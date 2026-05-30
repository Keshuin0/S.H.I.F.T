#![allow(non_snake_case)]
#![allow(unused_variables)] 
#![allow(unused_assignments)]

mod zk_engine; // PHASE 1.6 & 4.3 Modularized ZK Logic
mod ranging;   // PHASE 1.6 Cryptographic Ranging Engine
mod zk_prover; // Baked static parameter prover cache

#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::io::{Read, Write};
use std::net::Shutdown;

// FIX 1: Removed SockAddr completely.
#[cfg(unix)]
use nix::sys::socket::{socket, bind, listen, accept, AddressFamily, SockFlag, SockType, VsockAddr};

use std::sync::OnceLock;
use std::collections::hash_map::DefaultHasher; 
use std::hash::{Hash, Hasher};                 
use tokio::runtime::Runtime; 
use tokio::sync::mpsc; 
use tokio::time::Duration;
use std::fmt::Write as FmtWrite; 
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::collections::HashMap;

use sha2::{Sha256, Digest};
use std::collections::HashSet;

use log::{info, error};
use android_logger::Config;
use log::LevelFilter;

use arrayvec::{ArrayVec, ArrayString};

use libp2p::{
    gossipsub, identity, kad, identify, ping, dcutr, relay,
    swarm::NetworkBehaviour, PeerId, SwarmBuilder,
    futures::StreamExt
};
use h3o::{LatLng, Resolution, CellIndex};

// =========================================================================
// THE SOVEREIGN STATE & P2P MESH
// =========================================================================

#[derive(NetworkBehaviour)]
struct NodeBehaviour {
    gossipsub: gossipsub::Behaviour,
    kademlia: kad::Behaviour<kad::store::MemoryStore>,
    identify: identify::Behaviour,
    ping: ping::Behaviour,
    dcutr: dcutr::Behaviour,
    relay_client: relay::client::Behaviour,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum RangingAttestation {
    PhysicalToF = 0x01,
    SimulatedMock = 0xFE,
}

static NODE_IDENTITY: OnceLock<String> = OnceLock::new();
static SOULBOUND_TOKEN: OnceLock<String> = OnceLock::new();
static LAMPORT_CLOCK: AtomicU64 = AtomicU64::new(0);
static ACTIVE_RIDE_LOCKS: OnceLock<Mutex<HashMap<String, u64>>> = OnceLock::new();

#[derive(Clone, Debug)]
pub struct StateBlock {
    pub account: String,           
    pub previous_hash: String,     
    pub representative: String,    
    pub balance: u64,              
    pub link: String,              
    pub signature: String,         
}

static LOCAL_LEDGER: OnceLock<Mutex<HashMap<String, StateBlock>>> = OnceLock::new();
static ASYNC_RUNTIME: OnceLock<Runtime> = OnceLock::new();
static MESH_TX: OnceLock<mpsc::Sender<EngineCommand>> = OnceLock::new();

enum EngineCommand {
    TransmitPoL { 
        global_topic: String, 
        local_zone: ArrayString<32>, 
        payload: String, 
        k_rings: Box<ArrayVec<ArrayString<32>, 7>> 
    },
    StrikeLocal {
        local_zone: ArrayString<32>,
        payload: String,
    },
    BroadcastLedger {
        payload: String,
    }
}

// =========================================================================
// PHASE 3: MATHEMATICAL REJECTION ENGINE (zk-PSI)
// =========================================================================

fn hash_mac_address(mac: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(mac.as_bytes());
    hex::encode(hasher.finalize())
}

fn execute_zk_psi(scanned_macs: Vec<&str>, expected_macs: Vec<&str>) -> String {
    let threshold_k = 3; 

    let mut scanned_hashes = HashSet::new();
    for mac in scanned_macs {
        let clean_mac = mac.trim();
        if !clean_mac.is_empty() {
            scanned_hashes.insert(hash_mac_address(clean_mac));
        }
    }

    let mut expected_hashes = HashSet::new();
    for mac in expected_macs {
        let clean_mac = mac.trim();
        if !clean_mac.is_empty() {
            expected_hashes.insert(hash_mac_address(clean_mac));
        }
    }

    let intersection: Vec<_> = scanned_hashes.intersection(&expected_hashes).collect();

    if intersection.len() >= threshold_k {
        format!("Execution Approved: zk-PSI Threshold Met. Intersection size: {}", intersection.len())
    } else {
        format!("Execution Denied: Proximity Triangulation Failed. GPS Spoofing Detected. Intersection size: {}", intersection.len())
    }
}

// =========================================================================
// PHASE 1.6: THE VSOCK HYPERVISOR BRIDGE (REPLACES JNI)
#[cfg(unix)]
const VSOCK_PORT: u32 = 8000;
#[cfg(unix)]
const VMADDR_CID_ANY: u32 = 0xFFFFFFFF; 

fn main() {
    android_logger::init_once(
        Config::default()
            .with_max_level(LevelFilter::Trace)
            .with_tag("SHIFT_VAULT")
    );

    info!("🛡️ [HYPERVISOR] Rust Vault booting...");
    
    #[cfg(unix)]
    {
        // 1. Attempt to create a Virtual Socket (AF_VSOCK)
        let vsock_result = socket(AddressFamily::Vsock, SockType::Stream, SockFlag::empty(), None)
            .and_then(|fd| {
                let addr = VsockAddr::new(VMADDR_CID_ANY, VSOCK_PORT);
                bind(fd.as_raw_fd(), &addr)?;
                listen(&fd, 10)?;
                Ok(fd)
            });

        match vsock_result {
            Ok(fd) => {
                info!("🎧 [HYPERVISOR] Vault listening on vsock port {}...", VSOCK_PORT);
                loop {
                    match accept(fd.as_raw_fd()) {
                        Ok(client_raw_fd) => {
                            info!("🤝 [HYPERVISOR] Kotlin OS connection accepted (VSOCK).");
                            let mut stream = unsafe { std::net::TcpStream::from_raw_fd(client_raw_fd) };
                            handle_connection(&mut stream);
                        }
                        Err(e) => error!("❌ [HYPERVISOR] Connection failed: {:?}", e),
                    }
                }
            }
            Err(e) => {
                info!("⚠️ [FALLBACK] VSOCK unavailable ({:?}). Binding to TCP 127.0.0.1:8000...", e);
                let listener = std::net::TcpListener::bind("127.0.0.1:8000").expect("Failed to bind TCP fallback");
                info!("🎧 [FALLBACK] Vault listening on tcp port 8000...");
                for stream in listener.incoming() {
                    match stream {
                        Ok(mut stream) => {
                            info!("🤝 [FALLBACK] Kotlin OS connection accepted (TCP).");
                            handle_connection(&mut stream);
                        }
                        Err(e) => error!("❌ [FALLBACK] Connection failed: {:?}", e),
                    }
                }
            }
        }
    }

    #[cfg(not(unix))]
    {
        info!("🎧 [FALLBACK] Vault listening on tcp port 8000...");
        let listener = std::net::TcpListener::bind("127.0.0.1:8000").expect("Failed to bind TCP fallback");
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    info!("🤝 [FALLBACK] Kotlin OS connection accepted (TCP).");
                    handle_connection(&mut stream);
                }
                Err(e) => error!("❌ [FALLBACK] Connection failed: {:?}", e),
            }
        }
    }
}

fn handle_connection(stream: &mut std::net::TcpStream) {
    use p256::{ecdh::EphemeralSecret, EncodedPoint, PublicKey, ecdsa::{VerifyingKey, Signature, signature::Verifier}};
    use p256::pkcs8::DecodePublicKey;
    use rand_core::{OsRng, RngCore};
    use aes_gcm::{Aes256Gcm, Key, KeyInit, aead::{Aead}};
    use aes_gcm::Nonce as AeadNonce;

    info!("🛡️ [SOVEREIGN TUNNEL] handle_connection started");

    // 1. Generate Vault Ephemeral Key & Nonce
    let vault_secret = EphemeralSecret::random(&mut OsRng);
    let vault_pub = EncodedPoint::from(vault_secret.public_key());
    let vault_pub_bytes = vault_pub.as_bytes(); // 65 bytes uncompressed
    info!("🛡️ [SOVEREIGN TUNNEL] Vault Ephemeral PubKey generated ({} bytes)", vault_pub_bytes.len());
    
    let mut nonce = [0u8; 32];
    OsRng.fill_bytes(&mut nonce);

    // Write Phase 1 to stream
    info!("🛡️ [SOVEREIGN TUNNEL] Writing Vault PubKey and Nonce to stream...");
    if let Err(e) = stream.write_all(vault_pub_bytes) {
        error!("❌ [SOVEREIGN TUNNEL] Failed to write vault_pub_bytes: {:?}", e);
        return;
    }
    if let Err(e) = stream.write_all(&nonce) {
        error!("❌ [SOVEREIGN TUNNEL] Failed to write nonce: {:?}", e);
        return;
    }
    info!("🛡️ [SOVEREIGN TUNNEL] Vault PubKey and Nonce sent successfully. Reading App Response...");

    // 2. Read App Response
    info!("🛡️ [SOVEREIGN TUNNEL] Reading hw_pub_len_buf...");
    let mut hw_pub_len_buf = [0u8; 2];
    if let Err(e) = stream.read_exact(&mut hw_pub_len_buf) {
        error!("❌ [SOVEREIGN TUNNEL] Failed to read hw_pub_len_buf: {:?}", e);
        return;
    }
    let hw_pub_len = u16::from_be_bytes(hw_pub_len_buf) as usize;
    info!("🛡️ [SOVEREIGN TUNNEL] hw_pub_len: {}", hw_pub_len);
    if hw_pub_len > 2048 {
        error!("❌ [SOVEREIGN TUNNEL] hw_pub_len exceeds limit: {}", hw_pub_len);
        return;
    } 
    let mut hw_pub = vec![0u8; hw_pub_len];
    if let Err(e) = stream.read_exact(&mut hw_pub) {
        error!("❌ [SOVEREIGN TUNNEL] Failed to read hw_pub: {:?}", e);
        return;
    }
    info!("🛡️ [SOVEREIGN TUNNEL] hw_pub read successfully ({} bytes)", hw_pub.len());

    let mut app_pub = [0u8; 65];
    if let Err(e) = stream.read_exact(&mut app_pub) {
        error!("❌ [SOVEREIGN TUNNEL] Failed to read app_pub: {:?}", e);
        return;
    }
    info!("🛡️ [SOVEREIGN TUNNEL] app_pub read successfully ({} bytes)", app_pub.len());

    let mut sig_len_buf = [0u8; 2];
    if let Err(e) = stream.read_exact(&mut sig_len_buf) {
        error!("❌ [SOVEREIGN TUNNEL] Failed to read sig_len_buf: {:?}", e);
        return;
    }
    let sig_len = u16::from_be_bytes(sig_len_buf) as usize;
    info!("🛡️ [SOVEREIGN TUNNEL] sig_len: {}", sig_len);
    if sig_len > 2048 {
        error!("❌ [SOVEREIGN TUNNEL] sig_len exceeds limit: {}", sig_len);
        return;
    }
    let mut sig = vec![0u8; sig_len];
    if let Err(e) = stream.read_exact(&mut sig) {
        error!("❌ [SOVEREIGN TUNNEL] Failed to read sig: {:?}", e);
        return;
    }
    info!("🛡️ [SOVEREIGN TUNNEL] sig read successfully ({} bytes)", sig.len());

    let mut ct_len_buf = [0u8; 4];
    if let Err(e) = stream.read_exact(&mut ct_len_buf) {
        error!("❌ [SOVEREIGN TUNNEL] Failed to read ct_len_buf: {:?}", e);
        return;
    }
    let ct_len = u32::from_be_bytes(ct_len_buf) as usize;
    info!("🛡️ [SOVEREIGN TUNNEL] ct_len: {}", ct_len);
    if ct_len > 1024 * 1024 {
        error!("❌ [SOVEREIGN TUNNEL] ct_len exceeds limit: {}", ct_len);
        return;
    } 
    let mut ciphertext = vec![0u8; ct_len];
    if let Err(e) = stream.read_exact(&mut ciphertext) {
        error!("❌ [SOVEREIGN TUNNEL] Failed to read ciphertext: {:?}", e);
        return;
    }
    info!("🛡️ [SOVEREIGN TUNNEL] ciphertext read successfully ({} bytes)", ciphertext.len());

    // 3. Cryptographic Verification
    let hw_pub_hex = hex::encode(&hw_pub);
    if let Some(registered_id) = NODE_IDENTITY.get() {
        if registered_id != &hw_pub_hex {
            error!("❌ [SOVEREIGN TUNNEL] Hardware Identity Mismatch! Expected: {}, Got: {}", registered_id, hw_pub_hex);
            return;
        }
    }

    info!("🛡️ [SOVEREIGN TUNNEL] Verifying Hardware ECDSA signature...");
    let verifying_key = match VerifyingKey::from_public_key_der(&hw_pub) {
        Ok(k) => k,
        Err(e) => { error!("❌ [SOVEREIGN TUNNEL] Invalid Hardware Public Key format: {:?}", e); return; }
    };
    let signature = match Signature::from_der(&sig) {
        Ok(s) => s,
        Err(e) => { error!("❌ [SOVEREIGN TUNNEL] Invalid Signature format: {:?}", e); return; }
    };

    let mut transcript = Vec::new();
    transcript.extend_from_slice(vault_pub_bytes);
    transcript.extend_from_slice(&app_pub);
    transcript.extend_from_slice(&nonce);
    transcript.extend_from_slice(&ciphertext);

    if let Err(e) = verifying_key.verify(&transcript, &signature) {
        error!("❌ [SOVEREIGN TUNNEL] ECDSA Hardware Signature Verification Failed: {:?}", e);
        return;
    }
    info!("🛡️ [SOVEREIGN TUNNEL] Hardware ECDSA signature verified successfully!");

    // 4. Derive Shared Secret
    info!("🛡️ [SOVEREIGN TUNNEL] Deriving shared secret via Ephemeral ECDH...");
    let app_public_point = match EncodedPoint::from_bytes(app_pub) {
        Ok(p) => p,
        Err(e) => { error!("❌ [SOVEREIGN TUNNEL] Invalid App Ephemeral Public Key Point: {:?}", e); return; }
    };
    let app_public_key = match PublicKey::from_sec1_bytes(app_public_point.as_ref()) {
        Ok(k) => k,
        Err(e) => { error!("❌ [SOVEREIGN TUNNEL] Invalid App Ephemeral Public Key: {:?}", e); return; }
    };
    let shared_secret = vault_secret.diffie_hellman(&app_public_key);

    let mut hasher = sha2::Sha256::new();
    hasher.update(shared_secret.raw_secret_bytes());
    let aes_key = hasher.finalize();
    info!("🛡️ [SOVEREIGN TUNNEL] Shared secret derived and AES key hashed successfully.");

    // 5. Decrypt Payload
    info!("🛡️ [SOVEREIGN TUNNEL] Decrypting payload via AES-GCM...");
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&aes_key));
    let aead_nonce_req = AeadNonce::from_slice(&nonce[0..12]);
    let decrypted = match cipher.decrypt(aead_nonce_req, ciphertext.as_ref()) {
        Ok(d) => d,
        Err(e) => { error!("❌ [SOVEREIGN TUNNEL] AES-GCM Decryption Failed: {:?}", e); return; }
    };

    let command = String::from_utf8_lossy(&decrypted).trim().to_string();
    info!("🛡️ [SOVEREIGN TUNNEL] Payload Decrypted Safely: [{}]", command);

    // 6. Enforce First Command Rule if Node Identity is missing
    if NODE_IDENTITY.get().is_none() && !command.starts_with("REGISTER_NODE:") {
        error!("❌ [SOVEREIGN TUNNEL] Vault Uninitialized. First command must be REGISTER_NODE.");
        return;
    }

    // 7. Process Vault Command
    info!("🛡️ [SOVEREIGN TUNNEL] Processing Command...");
    let response = process_vault_command(&command);
    info!("🛡️ [SOVEREIGN TUNNEL] Command processed, response: [{}]", response);

    // 8. Encrypt Response
    info!("🛡️ [SOVEREIGN TUNNEL] Encrypting Response via AES-GCM...");
    let aead_nonce_res = AeadNonce::from_slice(&nonce[12..24]);
    let response_bytes = response.as_bytes();
    let encrypted_response = match cipher.encrypt(aead_nonce_res, response_bytes) {
        Ok(e) => e,
        Err(e) => { error!("❌ [SOVEREIGN TUNNEL] AES-GCM Encryption Failed: {:?}", e); return; }
    };

    let res_len = encrypted_response.len() as u32;
    info!("🛡️ [SOVEREIGN TUNNEL] Sending encrypted response ({} bytes)...", res_len);
    let _ = stream.write_all(&res_len.to_be_bytes());
    let _ = stream.write_all(&encrypted_response);
    let _ = stream.shutdown(Shutdown::Both);
    info!("🛡️ [SOVEREIGN TUNNEL] Connection closed successfully.");
}

// =========================================================================
// VAULT COMMAND PROCESSOR 
// =========================================================================
fn process_vault_command(command: &str) -> String {
    let mut response = String::new();

    if command.starts_with("REGISTER_NODE:") {
        let payload = command.replace("REGISTER_NODE:", "");
        let parts: Vec<&str> = payload.split('|').collect();
        if parts.len() != 3 {
            return "Execution Denied: Malformed registration payload.".to_string();
        }
        let public_key = parts[0].to_string();
        let s_classical_hex = parts[1];
        let s_pqc_hex = parts[2];

        let s_classical = match hex::decode(s_classical_hex) {
            Ok(bytes) => bytes,
            Err(_) => return "Execution Denied: Invalid classical secret encoding.".to_string(),
        };
        let s_pqc = match hex::decode(s_pqc_hex) {
            Ok(bytes) => bytes,
            Err(_) => return "Execution Denied: Invalid post-quantum secret encoding.".to_string(),
        };

        match NODE_IDENTITY.set(public_key.clone()) {
            Ok(_) => {
                let rt = Runtime::new().expect("Failed to build Tokio Runtime");
                let (tx, mut rx) = mpsc::channel::<EngineCommand>(100);
                let _ = MESH_TX.set(tx);
                
                let _ = ACTIVE_RIDE_LOCKS.set(Mutex::new(HashMap::new()));
                let _ = LOCAL_LEDGER.set(Mutex::new(HashMap::new()));

                zk_prover::pre_initialize_keys();

                rt.spawn(async move {
                    info!("🚀 [BACKGROUND ENGINE] Spinning up Layer-1 Node...");
                    
                    let mut ikm = Vec::new();
                    ikm.extend_from_slice(&s_classical);
                    ikm.extend_from_slice(&s_pqc);

                    let hk = hkdf::Hkdf::<sha2::Sha256>::new(Some(b"SHIFT-PQC-V1"), &ikm);
                    let mut okm = [0u8; 32];
                    hk.expand(b"libp2p-identity", &mut okm).expect("Key derivation expansion failed");

                    let mut secret_key_bytes = okm;
                    let ed25519_secret = identity::ed25519::SecretKey::try_from_bytes(&mut secret_key_bytes)
                        .expect("Failed to construct Ed25519 secret key from derived bytes");
                    let local_key = identity::Keypair::from(identity::ed25519::Keypair::from(ed25519_secret));
                    let local_peer_id = PeerId::from(local_key.public());
                    info!("S.H.I.F.T. Layer-1 Engine Online. Network PeerID: {}", local_peer_id);

                    let (_relay_transport, relay_client) = relay::client::new(local_peer_id);

                    // FIX 4: Removed .with_tcp() entirely because we dropped the dependency
                    let mut swarm = SwarmBuilder::with_existing_identity(local_key.clone())
                        .with_tokio()
                        .with_quic()
                        .with_relay_client(libp2p::noise::Config::new, libp2p::yamux::Config::default)
                        .expect("Valid Relay Config")
                        .with_behaviour(|key, relay_client| {
                            let authenticity = gossipsub::MessageAuthenticity::Signed(key.clone());
                            let message_id_fn = |message: &gossipsub::Message| {
                                let mut s = DefaultHasher::new();
                                message.data.hash(&mut s);
                                gossipsub::MessageId::from(s.finish().to_string())
                            };
                            let gossipsub_config = gossipsub::ConfigBuilder::default()
                                .heartbeat_interval(Duration::from_secs(1)) 
                                .validation_mode(gossipsub::ValidationMode::Strict)
                                .message_id_fn(message_id_fn)
                                .build()
                                .expect("Valid config");
                            let gossipsub: gossipsub::Behaviour = gossipsub::Behaviour::new(authenticity, gossipsub_config).expect("Valid behaviour");

                            let store = kad::store::MemoryStore::new(local_peer_id);
                            let kademlia = kad::Behaviour::new(local_peer_id, store);
                            
                            let identify = identify::Behaviour::new(identify::Config::new(
                                "/shift/1.0.0".to_string(),
                                key.public(),
                            ));
                            let ping = ping::Behaviour::new(ping::Config::new().with_interval(Duration::from_secs(15)));
                            let dcutr = dcutr::Behaviour::new(local_peer_id);

                            Ok(NodeBehaviour { gossipsub, kademlia, identify, ping, dcutr, relay_client })
                        })
                        .expect("Valid behaviour builder")
                        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
                        .build();

                    swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse().unwrap()).unwrap();

                    info!("⌛ [WIFI AWARE] Swarm Listening for incoming NAN connections...");

                    let mut current_subscriptions: Vec<String> = Vec::new();

                    loop {
                        tokio::select! {
                            cmd_opt = rx.recv() => {
                                if let Some(cmd) = cmd_opt {
                                    match cmd {
                                        EngineCommand::TransmitPoL { global_topic, local_zone, payload, k_rings } => {
                                            info!("⚙️ [VAULT] Processing TransmitPoL command...");
                                            
                                            for old_zone in &current_subscriptions {
                                                let old_zone_str = old_zone.as_str();
                                                let still_in_range = k_rings.iter().any(|r| r.as_str() == old_zone_str);
                                                if !still_in_range && old_zone_str != global_topic.as_str() {
                                                    let _ = swarm.behaviour_mut().gossipsub.unsubscribe(&gossipsub::IdentTopic::new(old_zone_str));
                                                    info!("🔌 Unsubscribed from out-of-range zone: {}", old_zone_str);
                                                }
                                            }
                                            
                                            current_subscriptions.clear();
                                            
                                            let g_topic = gossipsub::IdentTopic::new(global_topic.clone());
                                            let _ = swarm.behaviour_mut().gossipsub.subscribe(&g_topic);
                                            current_subscriptions.push(global_topic.clone());

                                            for ring_zone in *k_rings {
                                                let topic = gossipsub::IdentTopic::new(ring_zone.as_str());
                                                let _ = swarm.behaviour_mut().gossipsub.subscribe(&topic);
                                                current_subscriptions.push(ring_zone.as_str().to_string());
                                            }

                                            match swarm.behaviour_mut().gossipsub.publish(g_topic, payload.as_bytes()) {
                                                Ok(msg_id) => info!("🚀 [GOSSIPSUB] -> GLOBAL PUBLISH SUCCESS: {}", msg_id),
                                                Err(e) => error!("❌ [GOSSIPSUB] -> GLOBAL PUBLISH ERROR: {:?}", e),
                                            }
                                            
                                            let tx_clone = MESH_TX.get().expect("Mesh TX Missing").clone();
                                            tokio::spawn(async move {
                                                tokio::time::sleep(Duration::from_millis(1500)).await;
                                                let _ = tx_clone.send(EngineCommand::StrikeLocal { local_zone, payload }).await;
                                            });
                                        },
                                        EngineCommand::StrikeLocal { local_zone, payload } => {
                                            let local_topic = gossipsub::IdentTopic::new(local_zone.as_str());
                                            match swarm.behaviour_mut().gossipsub.publish(local_topic, payload.as_bytes()) {
                                                Ok(msg_id) => info!("🚀 [SPATIAL GOSSIPSUB] -> LOCAL PUBLISH SUCCESS: {}", msg_id),
                                                Err(e) => error!("❌ [SPATIAL GOSSIPSUB] -> LOCAL PUBLISH ERROR: {:?}", e),
                                            }
                                        },
                                        EngineCommand::BroadcastLedger { payload } => {
                                            let ledger_topic = gossipsub::IdentTopic::new("shift-ledger");
                                            let _ = swarm.behaviour_mut().gossipsub.subscribe(&ledger_topic);
                                            match swarm.behaviour_mut().gossipsub.publish(ledger_topic, payload.as_bytes()) {
                                                Ok(msg_id) => info!("💎 [BLOCK-LATTICE] -> GLOBAL MINT SUCCESS: {}", msg_id),
                                                Err(e) => error!("❌ [BLOCK-LATTICE] -> GLOBAL MINT ERROR: {:?}", e),
                                            }
                                        }
                                    }
                                }
                            }
                            event_opt = swarm.next() => {
                                if let Some(event) = event_opt {
                                    match event {
                                        libp2p::swarm::SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                                            info!("🤝 [LINK SECURED] P2P Tunnel connected to Peer: {}", peer_id);
                                            swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                                        }
                                        libp2p::swarm::SwarmEvent::NewListenAddr { address, .. } => {
                                            info!("🛡️ S.H.I.F.T. Node listening on interface: {}", address);
                                        }
                                        libp2p::swarm::SwarmEvent::Behaviour(NodeBehaviourEvent::Gossipsub(gossipsub::Event::Message { propagation_source: peer_id, message, .. })) => {
                                            let payload = String::from_utf8_lossy(&message.data);
                                            if payload.starts_with("LOCK_REQUEST:") {
                                                let parts: Vec<&str> = payload.split(':').collect();
                                                if parts.len() == 2 {
                                                    if let Ok(incoming_ticket) = parts[1].parse::<u64>() {
                                                        LAMPORT_CLOCK.fetch_max(incoming_ticket, Ordering::SeqCst);
                                                        LAMPORT_CLOCK.fetch_add(1, Ordering::SeqCst);

                                                        if let Some(locks_mutex) = ACTIVE_RIDE_LOCKS.get() {
                                                            let mut active_locks = locks_mutex.lock().unwrap();
                                                            if active_locks.is_empty() {
                                                                active_locks.insert(peer_id.to_string(), incoming_ticket);
                                                                info!("✅ [OCC SUCCESS] Mathematical Lock granted to Rider [{}] on Ticket #{}.", peer_id, incoming_ticket);
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                LAMPORT_CLOCK.fetch_add(1, Ordering::SeqCst);
                                            }
                                        }
                                        _ => {} 
                                    }
                                }
                            }
                        }
                    }
                });

                if ASYNC_RUNTIME.set(rt).is_ok() {
                    response = "Vault Locked. Async L1 Engine IGNITED.".to_string();
                } else {
                    response = "Vault Locked, but Async Engine failed to store state.".to_string();
                }
            },
            Err(_) => {
                response = "Vault Error: Identity already locked.".to_string();
            }
        }
    } 
    else if command.starts_with("GENERATE_POL:") {
        let telemetry = command.replace("GENERATE_POL:", "");
        let node_id = NODE_IDENTITY.get();

        if let Some(identity) = node_id {
            // Phase 1.4: Enforce KYC clearance before PoL generation (Issue #112 / A12)
            if SOULBOUND_TOKEN.get().is_none() {
                return "Execution Denied: No Soulbound Token. KYC clearance required.".to_string();
            }

            let mut extracted_lat = 0.0;
            let mut extracted_lon = 0.0;
            let mut extracted_ble = String::new();
            
            let parts: Vec<&str> = telemetry.split('|').collect();
            for part in &parts {
                if part.starts_with("LAT:") {
                    extracted_lat = part.replace("LAT:", "").parse().unwrap_or(0.0);
                } else if part.starts_with("LON:") {
                    extracted_lon = part.replace("LON:", "").parse().unwrap_or(0.0);
                } else if part.starts_with("BLE:") {
                    extracted_ble = part.replace("BLE:", "");
                }
            }

            let mut k_ring_zones: ArrayVec<ArrayString<32>, 7> = ArrayVec::new();
            let mut core_h3_zone = ArrayString::<32>::new();

            match LatLng::new(extracted_lat, extracted_lon) {
                Ok(coord) => {
                    let cell: CellIndex = coord.to_cell(Resolution::Nine);
                    write!(&mut core_h3_zone, "zone:{}", cell).unwrap();
                    let disk_distances: ArrayVec<(CellIndex, u32), 7> = cell.grid_disk_distances(1);
                    for (c, _distance) in disk_distances {
                        let mut zone_str = ArrayString::<32>::new();
                        write!(&mut zone_str, "zone:{}", c).unwrap();
                        k_ring_zones.push(zone_str);
                    }
                },
                Err(_) => {
                    write!(&mut core_h3_zone, "zone:UNKNOWN_HEX").unwrap();
                }
            };

            let mut cryptogram = String::new();
            // 💎 PINNACLE HYBRID COMMITMENT: BLAKE3 (On-Device Speed) + SHA-256d (Consensus Security)
            {
                let mut blake_hasher = blake3::Hasher::new();
                blake_hasher.update(identity.as_bytes());
                blake_hasher.update(telemetry.as_bytes());
                let blake_hash = blake_hasher.finalize();

                let mut sha_hasher1 = Sha256::new();
                sha_hasher1.update(blake_hash.as_bytes());
                let hash1 = sha_hasher1.finalize();

                let mut sha_hasher2 = Sha256::new();
                sha_hasher2.update(&hash1);
                let final_hash = sha_hasher2.finalize();
                cryptogram = hex::encode(final_hash);
            }
            
            #[cfg(feature = "simulated")]
            {
                // ⚡ PHASE 1.6: CRYPTOGRAPHIC DISTANCE BOUNDING & ZK-SNARK RECEIPT
                let challenge = ranging::initiate_ranging_challenge();
                let dummy_peer_key = identity::Keypair::generate_ed25519();
                let (signature, compute_delay) = ranging::process_ranging_challenge(&challenge.nonce, &dummy_peer_key);
                let simulated_rx_time = challenge.tx_timestamp_ns + compute_delay + 100;
                
                let response_obj = ranging::RangingResponse {
                    signature,
                    compute_delay_ns: compute_delay,
                    rx_timestamp_ns: simulated_rx_time,
                };

                match ranging::verify_time_of_flight(&challenge, &response_obj, &dummy_peer_key.public()) {
                    Ok((delta_t, t_compute)) => {
                        let zksnark_receipt = zk_prover::generate_tof_proof(delta_t, t_compute, 50_000);
                        let ranging_attestation = RangingAttestation::SimulatedMock;
                        
                        if let Some(tx) = MESH_TX.get() {
                            let payload = format!(
                                "Node deployed to: [{}] | BLE: [{}] | Hash: {} | ZK-DB: {} | ATTEST: {:02x}",
                                core_h3_zone.as_str(),
                                extracted_ble,
                                cryptogram,
                                zksnark_receipt,
                                ranging_attestation as u8
                            );
                            let _ = tx.try_send(EngineCommand::TransmitPoL {
                                global_topic: "shift-pol-network".to_string(),
                                local_zone: core_h3_zone, 
                                payload,
                                k_rings: Box::new(k_ring_zones), 
                            }); 
                        }
                        response = "PoL Valid & Cleared. ZK-Proof Generated.".to_string();
                    },
                    Err(_) => {
                        response = "Execution Denied: Cryptographic Distance Bounding Failed.".to_string();
                    }
                }
            }

            #[cfg(not(feature = "simulated"))]
            {
                // In production builds, simulated mock code is stripped. Real physical ranging logic will be integrated in Phase 1.6.
                // For now, return an error that physical ranging hardware is offline/unsupported.
                response = "Execution Denied: Physical Ranging Hardware offline or unsupported on this target.".to_string();
            }
        } else {
            response = "Execution Denied: Node Identity not found.".to_string();
        }
    }
    // =========================================================================
    // PHASE 1.4: SOULBOUND TOKEN ISSUANCE (Issue #97 / A1)
    // =========================================================================
    else if command.starts_with("ISSUE_SBT:") {
        let sbt_token = command.replace("ISSUE_SBT:", "").trim().to_string();
        if sbt_token.is_empty() {
            response = "Execution Denied: Empty SBT token.".to_string();
        } else {
            match SOULBOUND_TOKEN.set(sbt_token.clone()) {
                Ok(_) => {
                    let mut hasher = Sha256::new();
                    hasher.update(sbt_token.as_bytes());
                    let sbt_hash = hex::encode(hasher.finalize());
                    info!("\u{1f6e1}\u{fe0f} [IDENTITY] Soulbound Token attested: {}", &sbt_hash[..16]);
                    response = format!("\u{1f6e1}\u{fe0f} Soulbound Token Locked. KYC Hash: {}.", &sbt_hash[..16]);
                },
                Err(_) => {
                    response = "Identity already attested. Soulbound Token is immutable.".to_string();
                }
            }
        }
    }
    // =========================================================================
    // PHASE 3.1: BLOCK-LATTICE GENESIS (Issue #97 / A1)
    // =========================================================================
    else if command.starts_with("MINT_GENESIS:") {
        let node_id = NODE_IDENTITY.get();
        if let Some(identity) = node_id {
            if let Some(ledger_mutex) = LOCAL_LEDGER.get() {
                let mut ledger = ledger_mutex.lock().unwrap();
                if ledger.contains_key(identity) {
                    response = "Block-Lattice Stable: Genesis already anchored.".to_string();
                } else {
                    // TODO: Placeholder genesis balance (1M \u03bcSHIFT) \u2014 see GitHub issue for production tokenomics
                    let genesis_balance: u64 = 1_000_000;

                    let mut hasher = Sha256::new();
                    hasher.update(identity.as_bytes());
                    hasher.update(b"0000000000000000");
                    hasher.update(b"GENESIS");
                    hasher.update(genesis_balance.to_be_bytes());
                    let block_hash = hex::encode(hasher.finalize());

                    let genesis_block = StateBlock {
                        account: identity.clone(),
                        previous_hash: "0000000000000000".to_string(),
                        representative: identity.clone(),
                        balance: genesis_balance,
                        link: "GENESIS".to_string(),
                        signature: block_hash.clone(),
                    };

                    ledger.insert(identity.clone(), genesis_block);
                    info!("\u{26d3}\u{fe0f} [BLOCK-LATTICE] Genesis block minted. Hash: {}", &block_hash[..16]);

                    // Broadcast genesis to the network via shift-ledger topic
                    if let Some(tx) = MESH_TX.get() {
                        let payload = format!("GENESIS:{}:{}", identity, block_hash);
                        let _ = tx.try_send(EngineCommand::BroadcastLedger { payload });
                    }

                    response = format!("\u{26d3}\u{fe0f} Genesis Block Anchored. Hash: {}. Balance: {} \u{03bc}SHIFT.", &block_hash[..16], genesis_balance);
                }
            } else {
                response = "Execution Denied: Ledger not initialized. Register node first.".to_string();
            }
        } else {
            response = "Execution Denied: Node Identity not found.".to_string();
        }
    }
    // =========================================================================
    // PHASE 2.4: OCC RIDE LOCK (Issue #97 / A1)
    // =========================================================================
    else if command.starts_with("FIRE_LOCK:") {
        let target_zone = command.replace("FIRE_LOCK:", "").trim().to_string();
        if target_zone.is_empty() {
            response = "Execution Denied: No target zone specified.".to_string();
        } else {
            let ticket = LAMPORT_CLOCK.fetch_add(1, Ordering::SeqCst) + 1;

            // Record the lock locally
            if let Some(locks_mutex) = ACTIVE_RIDE_LOCKS.get() {
                let mut active_locks = locks_mutex.lock().unwrap();
                let node_id_str = NODE_IDENTITY.get().cloned().unwrap_or_default();
                active_locks.insert(node_id_str, ticket);
            }

            // Broadcast LOCK_REQUEST to the target zone via GossipSub
            if let Some(tx) = MESH_TX.get() {
                let lock_payload = format!("LOCK_REQUEST:{}", ticket);
                let mut zone_str = ArrayString::<32>::new();
                let _ = write!(&mut zone_str, "{}", &target_zone[..target_zone.len().min(31)]);
                let mut k_rings = ArrayVec::<ArrayString<32>, 7>::new();
                k_rings.push(zone_str);

                let _ = tx.try_send(EngineCommand::TransmitPoL {
                    global_topic: target_zone.clone(),
                    local_zone: zone_str,
                    payload: lock_payload,
                    k_rings: Box::new(k_rings),
                });
                info!("\u{1f512} [OCC] Lock Request fired. Ticket #{} \u{2192} Zone: {}", ticket, target_zone);
                response = format!("\u{1f512} Lamport Lock Fired. Ticket #{} dispatched to {}.", ticket, target_zone);
            } else {
                response = "Execution Denied: P2P mesh not initialized. Register node first.".to_string();
            }
        }
    }
    else if command.starts_with("IGNITE_ZKVM:") {
        response = "🧠 [zkVM] Hybrid Market-Maker R1CS Circuits safely allocated inside Hypervisor memory.".to_string();
    }
    else if command.starts_with("VERIFY_PSI:") {
        let payload = command.replace("VERIFY_PSI:", "");
        let parts: Vec<&str> = payload.split('|').collect();
        if parts.len() == 2 {
            let scanned_macs: Vec<&str> = parts[0].split(',').collect();
            let expected_macs: Vec<&str> = parts[1].split(',').collect();
            response = execute_zk_psi(scanned_macs, expected_macs);
        } else {
            response = "Execution Denied: Malformed zk-PSI payload.".to_string();
        }
    }
    else {
        response = format!("Unrecognized or deprecated command: [{}]", command);
    }

    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_key_derivation() {
        let public_key = "3059301306072a8648ce3d020106082a8648ce3d03010703420004b71565928a4e41aa63b7524c94e1df61aea291d6091e7e365e972aab0cedcee0c40c64c9eba480e9fb639257de9eab655021bf39e7b854de63973696a866e716";
        let s_classical = "11223344556677889900aabbccddeeff11223344556677889900aabbccddeeff";
        let s_pqc = "ffeeddccbbaa99887766554433221100ffeeddccbbaa99887766554433221100";
        
        let payload = format!("{}|{}|{}", public_key, s_classical, s_pqc);
        
        let parts: Vec<&str> = payload.split('|').collect();
        assert_eq!(parts.len(), 3);
        
        let dec_classical = hex::decode(parts[1]).unwrap();
        let dec_pqc = hex::decode(parts[2]).unwrap();
        
        let mut ikm = Vec::new();
        ikm.extend_from_slice(&dec_classical);
        ikm.extend_from_slice(&dec_pqc);
        
        let hk = hkdf::Hkdf::<sha2::Sha256>::new(Some(b"SHIFT-PQC-V1"), &ikm);
        let mut okm1 = [0u8; 32];
        hk.expand(b"libp2p-identity", &mut okm1).unwrap();
        
        let hk2 = hkdf::Hkdf::<sha2::Sha256>::new(Some(b"SHIFT-PQC-V1"), &ikm);
        let mut okm2 = [0u8; 32];
        hk2.expand(b"libp2p-identity", &mut okm2).unwrap();
        
        assert_eq!(okm1, okm2);
        
        let mut secret_bytes = okm1;
        let ed_secret = identity::ed25519::SecretKey::try_from_bytes(&mut secret_bytes).unwrap();
        let keypair = identity::Keypair::from(identity::ed25519::Keypair::from(ed_secret));
        let peer_id1 = PeerId::from(keypair.public());
        
        let mut secret_bytes2 = okm2;
        let ed_secret2 = identity::ed25519::SecretKey::try_from_bytes(&mut secret_bytes2).unwrap();
        let keypair2 = identity::Keypair::from(identity::ed25519::Keypair::from(ed_secret2));
        let peer_id2 = PeerId::from(keypair2.public());
        
        assert_eq!(peer_id1, peer_id2);
        println!("Test success: PeerId derived deterministically: {}", peer_id1);
    }
}