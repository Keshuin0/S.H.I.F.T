#![allow(non_snake_case)]
#![allow(dead_code)]

use libp2p::identity;
use ark_std::rand::{thread_rng, RngCore};
use std::time::{SystemTime, UNIX_EPOCH};
use log::{info, error};

pub struct RangingChallenge {
    pub nonce: [u8; 32],
    pub tx_timestamp_ns: u64,
}

pub struct RangingResponse {
    pub signature: Vec<u8>,
    pub compute_delay_ns: u64,
    pub rx_timestamp_ns: u64,
}

// =========================================================================
// 1. THE HARDWARE CHALLENGE (The Ping)
// =========================================================================
pub fn initiate_ranging_challenge() -> RangingChallenge {
    let mut nonce = [0u8; 32];
    thread_rng().fill_bytes(&mut nonce);
    
    // In the final Phase 1.6 AVF Hypervisor, this will call the hardware TSC (Time Stamp Counter)
    // For the POC, we extract nanoseconds directly from the System Clock
    let tx_timestamp_ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos() as u64;

    info!("📡 [RANGING] Blasting Radio Challenge Nonce: {:x?}", &nonce[0..8]);
    
    RangingChallenge {
        nonce,
        tx_timestamp_ns,
    }
}

// =========================================================================
// 2. THE FAST-RESPONSE (Executed by the surrounding peer's TEE)
// =========================================================================
pub fn process_ranging_challenge(nonce: &[u8; 32], local_key: &identity::Keypair) -> (Vec<u8>, u64) {
    let start_compute = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
    
    // Sign the random nonce using the TEE's private ed25519 key
    let signature = local_key.sign(nonce).expect("Hardware signing failed");
    
    let end_compute = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
    let compute_delay_ns = end_compute - start_compute;

    (signature, compute_delay_ns)
}

// =========================================================================
// 3. THE SPEED OF LIGHT CALCULATOR (The Wormhole Killer)
// =========================================================================
pub fn verify_time_of_flight(
    challenge: &RangingChallenge, 
    response: &RangingResponse, 
    peer_pub_key: &identity::PublicKey
) -> Result<(u64, u64), String> {
    
    // A. Verify the cryptographic signature (Prevents nonce-copying)
    if !peer_pub_key.verify(&challenge.nonce, &response.signature) {
        error!("❌ [RANGING] Signature verification failed. Mafia relay attack detected.");
        return Err("Signature invalid".to_string());
    }

    // B. Calculate the total Round Trip Time (RTT)
    let delta_t_nanos = response.rx_timestamp_ns.saturating_sub(challenge.tx_timestamp_ns);

    // C. Mathematical Speed of Light validation
    let t_flight = delta_t_nanos.saturating_sub(response.compute_delay_ns);
    
    // The speed of light is ~300 mm/ns.
    let distance_mm = t_flight * 300;

    info!("⏱️ [RANGING] Hardware RTT: {} ns | TEE Compute Delay: {} ns", delta_t_nanos, response.compute_delay_ns);
    info!("📏 [RANGING] Physical Proximity Bounded at: {:.2} meters", distance_mm as f64 / 1000.0);

    Ok((delta_t_nanos, response.compute_delay_ns))
}