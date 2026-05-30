use std::env;
use std::fs::File;
use std::path::Path;
use ark_groth16::Groth16;
use ark_snark::SNARK;
use ark_bls12_381::{Bls12_381, Fr};
use ark_std::marker::PhantomData;
use rand_chacha::rand_core::SeedableRng;

#[path = "src/zk_engine.rs"]
mod zk_engine;

fn main() {
    println!("cargo:rerun-if-changed=src/zk_engine.rs");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir);

    // Instantiate circuit with dummy inputs for trusted setup
    let circuit = zk_engine::DistanceBoundingCircuit::<Fr> {
        delta_t_nanos: None,
        t_compute_nanos: None,
        speed_of_light_mm_per_ns: None,
        max_allowed_distance_mm: None,
        _engine: PhantomData,
    };

    // Deterministic RNG with a static seed to ensure reproducible builds
    let mut rng = rand_chacha::ChaCha20Rng::from_seed([42u8; 32]);

    println!("cargo:warning=⚙️ [ZK-SETUP] Generating DistanceBounding parameters at compile-time...");
    let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit, &mut rng)
        .expect("Failed to run Groth16 trusted setup ceremony in build.rs");

    // Serialize keys to binary files in the build output directory
    let pk_file = File::create(dest_path.join("proving_key.bin")).unwrap();
    ark_serialize::CanonicalSerialize::serialize_uncompressed(&pk, pk_file)
        .expect("Failed to serialize proving key");

    let vk_file = File::create(dest_path.join("verification_key.bin")).unwrap();
    ark_serialize::CanonicalSerialize::serialize_uncompressed(&vk, vk_file)
        .expect("Failed to serialize verification key");

    println!("cargo:warning=💎 [ZK-SETUP] Done! Baked proving and verification keys successfully.");
}
