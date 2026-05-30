use std::sync::OnceLock;
use ark_groth16::{Groth16, ProvingKey, PreparedVerifyingKey};
use ark_bls12_381::{Bls12_381, Fr};
use ark_snark::SNARK;
use ark_std::rand::thread_rng;
use log::{info, error};
use std::marker::PhantomData;
use crate::zk_engine::DistanceBoundingCircuit;

pub static PROVING_KEY: OnceLock<ProvingKey<Bls12_381>> = OnceLock::new();
pub static VERIFYING_KEY: OnceLock<PreparedVerifyingKey<Bls12_381>> = OnceLock::new();

// Static bytes baked at compile time from OUT_DIR
const PROVING_KEY_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/proving_key.bin"));
const VERIFYING_KEY_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/verification_key.bin"));

pub fn pre_initialize_keys() {
    info!("💎 [zkVM] Loading pre-compiled Groth16 Proving/Verification keys into memory...");
    
    // Fast unchecked deserialization directly from static memory bytes
    let pk = ark_serialize::CanonicalDeserialize::deserialize_uncompressed_unchecked(PROVING_KEY_BYTES)
        .expect("Failed to deserialize static proving key");
    let vk = ark_serialize::CanonicalDeserialize::deserialize_uncompressed_unchecked(VERIFYING_KEY_BYTES)
        .expect("Failed to deserialize static verifying key");
        
    let pvk = Groth16::<Bls12_381>::process_vk(&vk)
        .expect("Failed to process static verifying key");

    if PROVING_KEY.set(pk).is_err() {
        error!("❌ [zkVM] Proving key already initialized.");
    }
    if VERIFYING_KEY.set(pvk).is_err() {
        error!("❌ [zkVM] Verifying key already initialized.");
    }
}

pub fn generate_tof_proof(delta_t_nanos: u64, t_compute_nanos: u64, max_distance_mm: u64) -> String {
    info!("🧠 [zkVM] Spinning up Groth16 Prover using RAM static parameters...");
    
    let mut rng = thread_rng();
    
    // 1. Initialize Circuit with actual telemetry data
    let circuit = DistanceBoundingCircuit::<Fr> {
        delta_t_nanos: Some(Fr::from(delta_t_nanos)),
        t_compute_nanos: Some(Fr::from(t_compute_nanos)),
        speed_of_light_mm_per_ns: Some(Fr::from(300u32)),
        max_allowed_distance_mm: Some(Fr::from(max_distance_mm)),
        _engine: PhantomData,
    };

    // 2. Fetch PK from static cache
    let pk = PROVING_KEY.get().expect("ZK Proving Key not pre-initialized");

    // 3. Generate the Proof mathematically verifying distance <= max_distance
    info!("⚡ [zkVM] Executing ZK-SNARK Ranging Proof...");
    let proof = Groth16::<Bls12_381>::prove(pk, circuit, &mut rng)
        .expect("Failed to generate proof");

    // 4. Compress to byte array, then hex string for network transmission
    let mut proof_bytes = Vec::new();
    ark_serialize::CanonicalSerialize::serialize_compressed(&proof, &mut proof_bytes)
        .expect("Failed to serialize ZK proof");
    
    let proof_hex = hex::encode(proof_bytes);
    info!("💎 [zkVM] ZK-SNARK Ranging Proof Compressed: {} bytes", proof_hex.len() / 2);
    
    proof_hex
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_caching_proving_and_verification() {
        // Pre-initialize keys. This mimics REGISTER_NODE boot-up.
        pre_initialize_keys();
        
        let delta_t = 400;
        let t_compute = 100;
        let max_distance = 50_000;
        
        // Measure proving time
        let start = Instant::now();
        let proof_hex = generate_tof_proof(delta_t, t_compute, max_distance);
        let duration = start.elapsed();
        
        println!("🚀 [BENCHMARK] Proving time: {:?}", duration);
        assert!(duration.as_millis() < 100, "Proving latency exceeded 100ms");
        
        // Parse the proof back to verify
        let proof_bytes = hex::decode(proof_hex).unwrap();
        let proof: ark_groth16::Proof<Bls12_381> = ark_serialize::CanonicalDeserialize::deserialize_compressed(&proof_bytes[..]).unwrap();
        
        // Public inputs: speed of light (300), max_allowed_distance (50000), factor (2)
        // Order of inputs in circuit: speed_of_light, max_allowed_distance, 2
        let pvk = VERIFYING_KEY.get().expect("Verifying key missing");
        
        let public_inputs = vec![
            Fr::from(300u32),
            Fr::from(max_distance),
            Fr::from(2u32),
        ];
        
        let is_valid = Groth16::<Bls12_381>::verify_with_processed_vk(pvk, &public_inputs, &proof).unwrap();
        assert!(is_valid, "Proof verification failed");
        println!("✅ Proof verified successfully!");
    }

    #[test]
    #[should_panic]
    fn test_distance_out_of_bounds_fails() {
        // Pre-initialize keys.
        pre_initialize_keys();
        
        let delta_t = 1000;
        let t_compute = 100;
        let max_distance = 50_000;
        
        let proof_hex = generate_tof_proof(delta_t, t_compute, max_distance);
        let proof_bytes = hex::decode(proof_hex).unwrap();
        let proof: ark_groth16::Proof<Bls12_381> = ark_serialize::CanonicalDeserialize::deserialize_compressed(&proof_bytes[..]).unwrap();
        
        let pvk = VERIFYING_KEY.get().expect("Verifying key missing");
        
        let public_inputs = vec![
            Fr::from(300u32),
            Fr::from(max_distance),
            Fr::from(2u32),
        ];
        
        let is_valid = Groth16::<Bls12_381>::verify_with_processed_vk(pvk, &public_inputs, &proof).unwrap();
        assert!(!is_valid, "Proof with invalid distance should fail verification");
        println!("✅ Out-of-bounds proof was correctly rejected!");
    }
}
