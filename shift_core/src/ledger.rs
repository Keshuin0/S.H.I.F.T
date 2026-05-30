use std::sync::OnceLock;
use std::sync::Mutex;
use std::collections::HashMap;
use sha2::{Sha256, Digest};
use p256::ecdsa::{VerifyingKey, Signature, signature::Verifier};
use p256::pkcs8::DecodePublicKey;
use log::info;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct StateBlock {
    pub account: String,           // SECP256R1 Master Public Key (hex DER)
    pub previous_hash: String,     // Hex hash of the previous block
    pub representative: String,    // Delegated representative address
    pub balance: u64,              // Account balance after this transaction (microSHIFT)
    pub link: String,              // Link payload (recipient pubkey, Send block hash, or delegation cert)
    pub signature: String,         // Hex signature (Master SECP256R1 or Session Ed25519)
}

#[derive(Clone, Debug, PartialEq)]
pub enum BlockType {
    Open,
    Send,
    Receive,
    Delegate,
}

#[derive(Clone, Debug)]
pub struct DelegationCert {
    pub session_pubkey: String,       // Ed25519 Public Key (hex)
    pub expiration_timestamp: u64,    // Unix timestamp (seconds)
    pub spend_limit: u64,             // Max microSHIFT allowed per autonomous Send block
    pub master_signature: String,     // Hex signature signed by Master Key
}

pub static LOCAL_LEDGER: OnceLock<Mutex<HashMap<String, Vec<StateBlock>>>> = OnceLock::new();
pub static ACTIVE_DELEGATIONS: OnceLock<Mutex<HashMap<String, DelegationCert>>> = OnceLock::new();
pub static PENDING_RECEIVES: OnceLock<Mutex<HashMap<String, (u64, String)>>> = OnceLock::new(); // hash -> (amount, recipient)

pub fn get_ledger() -> &'static Mutex<HashMap<String, Vec<StateBlock>>> {
    LOCAL_LEDGER.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn get_delegations() -> &'static Mutex<HashMap<String, DelegationCert>> {
    ACTIVE_DELEGATIONS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn get_pending_receives() -> &'static Mutex<HashMap<String, (u64, String)>> {
    PENDING_RECEIVES.get_or_init(|| Mutex::new(HashMap::new()))
}

// Hashing helper
#[allow(dead_code)]
pub fn hash_block(block: &StateBlock) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(block.account.as_bytes());
    hasher.update(block.previous_hash.as_bytes());
    hasher.update(block.representative.as_bytes());
    hasher.update(block.balance.to_be_bytes());
    hasher.update(block.link.as_bytes());
    hasher.finalize().to_vec()
}

pub fn serialize_block_payload(block: &StateBlock) -> Vec<u8> {
    let mut payload = Vec::new();
    payload.extend_from_slice(block.account.as_bytes());
    payload.extend_from_slice(block.previous_hash.as_bytes());
    payload.extend_from_slice(block.representative.as_bytes());
    payload.extend_from_slice(&block.balance.to_be_bytes());
    payload.extend_from_slice(block.link.as_bytes());
    payload
}

// Verify master delegation certificate
pub fn verify_delegation_cert(account: &str, cert: &DelegationCert) -> Result<(), String> {
    let mut payload = Vec::new();
    payload.extend_from_slice(cert.session_pubkey.as_bytes());
    payload.extend_from_slice(&cert.expiration_timestamp.to_be_bytes());
    payload.extend_from_slice(&cert.spend_limit.to_be_bytes());

    let hw_pub = hex::decode(account)
        .map_err(|e| format!("Invalid account hex: {:?}", e))?;
    let sig_bytes = hex::decode(&cert.master_signature)
        .map_err(|e| format!("Invalid delegation signature hex: {:?}", e))?;

    let verifying_key = VerifyingKey::from_public_key_der(&hw_pub)
        .map_err(|e| format!("Invalid Master Public Key DER: {:?}", e))?;
    let signature = Signature::from_der(&sig_bytes)
        .map_err(|e| format!("Invalid signature DER format: {:?}", e))?;

    verifying_key.verify(&payload, &signature)
        .map_err(|e| format!("Delegation Certificate Signature Verification Failed: {:?}", e))?;

    Ok(())
}

// Main operational validation engine
pub fn validate_and_process_block(block: &StateBlock) -> Result<(), String> {
    let payload = serialize_block_payload(block);
    let block_hash = {
        let mut hasher = Sha256::new();
        hasher.update(&payload);
        hex::encode(hasher.finalize())
    };

    let sig_bytes = hex::decode(&block.signature)
        .map_err(|e| format!("Invalid block signature hex: {:?}", e))?;

    // 1. Check if signed directly by the Master Key (SECP256R1)
    let is_signed_by_master = {
        if let Ok(hw_pub) = hex::decode(&block.account) {
            if let Ok(verifying_key) = VerifyingKey::from_public_key_der(&hw_pub) {
                if let Ok(signature) = Signature::from_der(&sig_bytes) {
                    verifying_key.verify(&payload, &signature).is_ok()
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    };

    if !is_signed_by_master {
        // 2. Validate session delegation
        let delegations = get_delegations().lock().unwrap();
        let cert = delegations.get(&block.account)
            .ok_or_else(|| "Signature invalid: Block not signed by Master Key, and no active delegation certificate found.".to_string())?;

        // Expiration check
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if current_time > cert.expiration_timestamp {
            return Err("Delegation certificate has expired.".to_string());
        }

        // Verify signature using Session Key (Ed25519)
        let session_pub_bytes = hex::decode(&cert.session_pubkey)
            .map_err(|e| format!("Invalid session public key hex: {:?}", e))?;
        let ed_pubkey = libp2p::identity::ed25519::PublicKey::try_from_bytes(&session_pub_bytes)
            .map_err(|e| format!("Invalid Ed25519 session public key: {:?}", e))?;
        let peer_pubkey = libp2p::identity::PublicKey::from(ed_pubkey);

        if !peer_pubkey.verify(&payload, &sig_bytes) {
            return Err("Signature invalid: Session Key signature verification failed.".to_string());
        }
        info!("🔐 [LEDGER] Block signed by authorized delegated Session Key: {}", &cert.session_pubkey[..8]);
    } else {
        info!("🛡️ [LEDGER] Block signed directly by Master Key.");
    }

    // 3. Chain continuity check
    let mut ledger = get_ledger().lock().unwrap();
    let chain = ledger.entry(block.account.clone()).or_insert_with(Vec::new);

    if chain.is_empty() {
        if block.previous_hash != "0000000000000000" {
            return Err("Chain empty: First block must have previous_hash of 0000000000000000.".to_string());
        }
    } else {
        let last_block = &chain[chain.len() - 1];
        let last_payload = serialize_block_payload(last_block);
        let last_hash = {
            let mut hasher = Sha256::new();
            hasher.update(&last_payload);
            hex::encode(hasher.finalize())
        };
        if block.previous_hash != last_hash {
            return Err(format!("Chain continuity broken: previous_hash matches {} instead of current head {}", block.previous_hash, last_hash));
        }
    }

    // 4. State transition rules
    let old_balance = if chain.is_empty() {
        0
    } else {
        chain[chain.len() - 1].balance
    };

    let block_type = if block.link.starts_with("DELEGATE:") {
        BlockType::Delegate
    } else if chain.is_empty() {
        BlockType::Open
    } else if block.balance < old_balance {
        BlockType::Send
    } else {
        BlockType::Receive
    };

    match block_type {
        BlockType::Delegate => {
            let raw_cert = block.link.replace("DELEGATE:", "");
            let parts: Vec<&str> = raw_cert.split('|').collect();
            if parts.len() != 4 {
                return Err("Malformed Delegate link payload.".to_string());
            }
            let cert = DelegationCert {
                session_pubkey: parts[0].to_string(),
                expiration_timestamp: parts[1].parse().map_err(|e| format!("Invalid expiration: {:?}", e))?,
                spend_limit: parts[2].parse().map_err(|e| format!("Invalid limit: {:?}", e))?,
                master_signature: parts[3].to_string(),
            };

            verify_delegation_cert(&block.account, &cert)?;

            let mut delegations = get_delegations().lock().unwrap();
            delegations.insert(block.account.clone(), cert);
            info!("✅ [LEDGER] New delegation cert registered for account: {}", &block.account[..8]);
        }
        BlockType::Open => {
            info!("✅ [LEDGER] Open block processed. Starting balance: {}", block.balance);
        }
        BlockType::Send => {
            let amount = old_balance - block.balance;
            if !is_signed_by_master {
                let delegations = get_delegations().lock().unwrap();
                if let Some(cert) = delegations.get(&block.account) {
                    if amount > cert.spend_limit {
                        return Err(format!("Transaction amount {} exceeds the authorized session limit of {}.", amount, cert.spend_limit));
                    }
                }
            }
            let mut pending = get_pending_receives().lock().unwrap();
            pending.insert(block_hash.clone(), (amount, block.link.clone()));
            info!("✅ [LEDGER] Send block processed. Amount: {}, Recipient: {}", amount, block.link);
        }
        BlockType::Receive => {
            let mut pending = get_pending_receives().lock().unwrap();
            let (amount, recipient) = {
                let (amt, rcpt) = pending.get(&block.link)
                    .ok_or_else(|| "Receive block rejected: Mismatched or non-existent Send block link hash.".to_string())?;
                (*amt, rcpt.clone())
            };

            if recipient != block.account {
                return Err("Receive block rejected: Caller is not the designated recipient of the Send block.".to_string());
            }

            if block.balance != old_balance + amount {
                return Err(format!("Receive block rejected: Balance update must match Balance_old + Amount ({} != {} + {})", block.balance, old_balance, amount));
            }

            pending.remove(&block.link);
            info!("✅ [LEDGER] Receive block processed. Claimed amount: {}", amount);
        }
    }

    chain.push(block.clone());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use p256::ecdsa::SigningKey;
    use p256::pkcs8::EncodePublicKey;
    use p256::ecdsa::signature::Signer;
    use rand_core::OsRng;

    // Helper to generate a SECP256R1 Keypair for Master Key
    fn generate_master_key() -> (SigningKey, Vec<u8>) {
        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = verifying_key_to_der_bytes(&signing_key);
        (signing_key, verifying_key)
    }

    fn verifying_key_to_der_bytes(signing_key: &SigningKey) -> Vec<u8> {
        let verifying_key = signing_key.verifying_key();
        let pub_doc = verifying_key.to_public_key_der().unwrap();
        pub_doc.to_vec()
    }

    fn sign_master_payload(signing_key: &SigningKey, payload: &[u8]) -> String {
        let signature: Signature = signing_key.sign(payload);
        hex::encode(signature.to_der().to_bytes())
    }

    #[test]
    fn test_genesis_block_and_ledger_validation() {
        let (master_sign_key, master_pub_der) = generate_master_key();
        let master_pub_hex = hex::encode(&master_pub_der);

        // 1. Create a Master-Signed Genesis Open block
        let mut genesis_block = StateBlock {
            account: master_pub_hex.clone(),
            previous_hash: "0000000000000000".to_string(),
            representative: master_pub_hex.clone(),
            balance: 100_000,
            link: "GENESIS".to_string(),
            signature: "".to_string(),
        };

        let payload = serialize_block_payload(&genesis_block);
        let sig = sign_master_payload(&master_sign_key, &payload);
        genesis_block.signature = sig;

        // Process block
        let result = validate_and_process_block(&genesis_block);
        assert!(result.is_ok(), "Failed to validate genesis block: {:?}", result.err());

        // Confirm balance
        {
            let ledger = get_ledger().lock().unwrap();
            let chain = ledger.get(&master_pub_hex).unwrap();
            assert_eq!(chain.len(), 1);
            assert_eq!(chain[0].balance, 100_000);
        }

        // 2. Register Delegation Certificate signed by Master Key
        let session_key_raw = libp2p::identity::ed25519::Keypair::generate();
        let session_pub_bytes = session_key_raw.public().to_bytes();
        let session_pub_hex = hex::encode(session_pub_bytes);
        let session_key = libp2p::identity::Keypair::from(session_key_raw);

        let expiration = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() + 3600; // 1 hour validity
        let spend_limit: u64 = 5000;

        let mut cert_payload = Vec::new();
        cert_payload.extend_from_slice(session_pub_hex.as_bytes());
        cert_payload.extend_from_slice(&expiration.to_be_bytes());
        cert_payload.extend_from_slice(&spend_limit.to_be_bytes());

        let master_cert_sig = sign_master_payload(&master_sign_key, &cert_payload);

        let delegate_link = format!("DELEGATE:{}|{}|{}|{}", session_pub_hex, expiration, spend_limit, master_cert_sig);

        // Create delegation block on-chain
        let last_payload = serialize_block_payload(&genesis_block);
        let previous_hash = hex::encode(Sha256::digest(&last_payload));

        let mut delegate_block = StateBlock {
            account: master_pub_hex.clone(),
            previous_hash,
            representative: master_pub_hex.clone(),
            balance: 100_000,
            link: delegate_link,
            signature: "".to_string(),
        };

        let payload2 = serialize_block_payload(&delegate_block);
        delegate_block.signature = sign_master_payload(&master_sign_key, &payload2);

        let result2 = validate_and_process_block(&delegate_block);
        assert!(result2.is_ok(), "Failed to validate delegate block: {:?}", result2.err());

        // Verify active delegation
        {
            let delegations = get_delegations().lock().unwrap();
            let active = delegations.get(&master_pub_hex).unwrap();
            assert_eq!(active.session_pubkey, session_pub_hex);
            assert_eq!(active.spend_limit, spend_limit);
        }

        // 3. Create a Session-Signed SEND Block (Low Value)
        let last_payload2 = serialize_block_payload(&delegate_block);
        let previous_hash2 = hex::encode(Sha256::digest(&last_payload2));

        let recipient_pubkey = "recipient_account_key".to_string();
        let mut send_block = StateBlock {
            account: master_pub_hex.clone(),
            previous_hash: previous_hash2,
            representative: master_pub_hex.clone(),
            balance: 98_000, // spent 2_000, which is <= spend_limit (5_000)
            link: recipient_pubkey.clone(),
            signature: "".to_string(),
        };

        let send_payload = serialize_block_payload(&send_block);
        let session_sig = session_key.sign(&send_payload).unwrap();
        send_block.signature = hex::encode(session_sig);

        let result3 = validate_and_process_block(&send_block);
        assert!(result3.is_ok(), "Failed to validate session-signed send block: {:?}", result3.err());

        // Verify pending receive
        {
            let send_payload_raw = serialize_block_payload(&send_block);
            let send_hash = hex::encode(Sha256::digest(&send_payload_raw));
            let pending = get_pending_receives().lock().unwrap();
            let (amount, recipient) = pending.get(&send_hash).unwrap();
            assert_eq!(*amount, 2_000);
            assert_eq!(recipient, &recipient_pubkey);
        }

        // 4. Over Limit Rejection Test
        let last_payload3 = serialize_block_payload(&send_block);
        let previous_hash3 = hex::encode(Sha256::digest(&last_payload3));

        let mut over_limit_block = StateBlock {
            account: master_pub_hex.clone(),
            previous_hash: previous_hash3,
            representative: master_pub_hex.clone(),
            balance: 90_000, // spent 8_000, which is > spend_limit (5_000)
            link: recipient_pubkey.clone(),
            signature: "".to_string(),
        };

        let over_payload = serialize_block_payload(&over_limit_block);
        let session_sig_over = session_key.sign(&over_payload).unwrap();
        over_limit_block.signature = hex::encode(session_sig_over);

        let result_over = validate_and_process_block(&over_limit_block);
        assert!(result_over.is_err(), "Block exceeding spend limit should fail validation");
        println!("✅ Spend limit correctly enforced: {:?}", result_over.err().unwrap());
    }

    #[test]
    fn test_session_expiration() {
        let (master_sign_key, master_pub_der) = generate_master_key();
        let master_pub_hex = hex::encode(&master_pub_der);

        // 1. Create and process Master-Signed Genesis block
        let mut genesis_block = StateBlock {
            account: master_pub_hex.clone(),
            previous_hash: "0000000000000000".to_string(),
            representative: master_pub_hex.clone(),
            balance: 100_000,
            link: "GENESIS".to_string(),
            signature: "".to_string(),
        };
        let payload = serialize_block_payload(&genesis_block);
        genesis_block.signature = sign_master_payload(&master_sign_key, &payload);
        validate_and_process_block(&genesis_block).unwrap();

        // 2. Register an expired Delegation Certificate (expiration = 1)
        let session_key_raw = libp2p::identity::ed25519::Keypair::generate();
        let session_pub_bytes = session_key_raw.public().to_bytes();
        let session_pub_hex = hex::encode(session_pub_bytes);
        let session_key = libp2p::identity::Keypair::from(session_key_raw);

        let expiration: u64 = 1; // Expired in 1970
        let spend_limit: u64 = 5000;

        let mut cert_payload = Vec::new();
        cert_payload.extend_from_slice(session_pub_hex.as_bytes());
        cert_payload.extend_from_slice(&expiration.to_be_bytes());
        cert_payload.extend_from_slice(&spend_limit.to_be_bytes());

        let master_cert_sig = sign_master_payload(&master_sign_key, &cert_payload);
        let delegate_link = format!("DELEGATE:{}|{}|{}|{}", session_pub_hex, expiration, spend_limit, master_cert_sig);

        let last_payload = serialize_block_payload(&genesis_block);
        let previous_hash = hex::encode(Sha256::digest(&last_payload));

        let mut delegate_block = StateBlock {
            account: master_pub_hex.clone(),
            previous_hash,
            representative: master_pub_hex.clone(),
            balance: 100_000,
            link: delegate_link,
            signature: "".to_string(),
        };
        let payload2 = serialize_block_payload(&delegate_block);
        delegate_block.signature = sign_master_payload(&master_sign_key, &payload2);
        validate_and_process_block(&delegate_block).unwrap();

        // 3. Attempt to sign a SEND block using the expired session key
        let last_payload2 = serialize_block_payload(&delegate_block);
        let previous_hash2 = hex::encode(Sha256::digest(&last_payload2));

        let recipient_pubkey = "recipient_account_key".to_string();
        let mut send_block = StateBlock {
            account: master_pub_hex.clone(),
            previous_hash: previous_hash2,
            representative: master_pub_hex.clone(),
            balance: 98_000,
            link: recipient_pubkey.clone(),
            signature: "".to_string(),
        };
        let send_payload = serialize_block_payload(&send_block);
        let session_sig = session_key.sign(&send_payload).unwrap();
        send_block.signature = hex::encode(session_sig);

        let result = validate_and_process_block(&send_block);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Delegation certificate has expired.");
    }

    #[test]
    fn test_receive_and_double_claim_protection() {
        let (sender_sign_key, sender_pub_der) = generate_master_key();
        let sender_pub_hex = hex::encode(&sender_pub_der);

        let (recipient_sign_key, recipient_pub_der) = generate_master_key();
        let recipient_pub_hex = hex::encode(&recipient_pub_der);

        // 1. Initialize sender chain
        let mut sender_genesis = StateBlock {
            account: sender_pub_hex.clone(),
            previous_hash: "0000000000000000".to_string(),
            representative: sender_pub_hex.clone(),
            balance: 100_000,
            link: "GENESIS".to_string(),
            signature: "".to_string(),
        };
        let payload_s = serialize_block_payload(&sender_genesis);
        sender_genesis.signature = sign_master_payload(&sender_sign_key, &payload_s);
        validate_and_process_block(&sender_genesis).unwrap();

        // 2. Initialize recipient chain
        let mut recipient_genesis = StateBlock {
            account: recipient_pub_hex.clone(),
            previous_hash: "0000000000000000".to_string(),
            representative: recipient_pub_hex.clone(),
            balance: 0,
            link: "GENESIS".to_string(),
            signature: "".to_string(),
        };
        let payload_r = serialize_block_payload(&recipient_genesis);
        recipient_genesis.signature = sign_master_payload(&recipient_sign_key, &payload_r);
        validate_and_process_block(&recipient_genesis).unwrap();

        // 3. Sender sends 10,000 microSHIFT to Recipient
        let last_payload_s = serialize_block_payload(&sender_genesis);
        let previous_hash_s = hex::encode(Sha256::digest(&last_payload_s));

        let mut send_block = StateBlock {
            account: sender_pub_hex.clone(),
            previous_hash: previous_hash_s,
            representative: sender_pub_hex.clone(),
            balance: 90_000,
            link: recipient_pub_hex.clone(),
            signature: "".to_string(),
        };
        let send_payload = serialize_block_payload(&send_block);
        send_block.signature = sign_master_payload(&sender_sign_key, &send_payload);
        validate_and_process_block(&send_block).unwrap();

        let send_hash = hex::encode(Sha256::digest(&send_payload));

        // 4. Recipient receives the block
        let last_payload_r = serialize_block_payload(&recipient_genesis);
        let previous_hash_r = hex::encode(Sha256::digest(&last_payload_r));

        let mut receive_block = StateBlock {
            account: recipient_pub_hex.clone(),
            previous_hash: previous_hash_r,
            representative: recipient_pub_hex.clone(),
            balance: 10_000, // old balance 0 + 10_000 amount
            link: send_hash.clone(),
            signature: "".to_string(),
        };
        let receive_payload = serialize_block_payload(&receive_block);
        receive_block.signature = sign_master_payload(&recipient_sign_key, &receive_payload);

        let result = validate_and_process_block(&receive_block);
        assert!(result.is_ok(), "Failed to validate receive block: {:?}", result.err());

        // 5. Attempt to claim the same Send block again (Double claim)
        let last_payload_r2 = serialize_block_payload(&receive_block);
        let previous_hash_r2 = hex::encode(Sha256::digest(&last_payload_r2));

        let mut double_receive_block = StateBlock {
            account: recipient_pub_hex.clone(),
            previous_hash: previous_hash_r2,
            representative: recipient_pub_hex.clone(),
            balance: 20_000, // old balance 10_000 + 10_000 amount
            link: send_hash,
            signature: "".to_string(),
        };
        let double_receive_payload = serialize_block_payload(&double_receive_block);
        double_receive_block.signature = sign_master_payload(&recipient_sign_key, &double_receive_payload);

        let result_double = validate_and_process_block(&double_receive_block);
        assert!(result_double.is_err(), "Double claim should fail");
        assert_eq!(result_double.err().unwrap(), "Receive block rejected: Mismatched or non-existent Send block link hash.");
    }
}
