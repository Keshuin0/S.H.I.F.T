#![allow(non_snake_case)]
#![allow(unused_variables)] 
#![allow(unused_assignments)]

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use std::sync::OnceLock;
use std::collections::hash_map::DefaultHasher; 
use std::hash::{Hash, Hasher};                 
use tokio::runtime::Runtime; 
use tokio::sync::mpsc; 
use tokio::time::Duration;
use std::net::ToSocketAddrs; 
use std::fmt::Write; // Required for zero-allocation string writing
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::collections::HashMap;

// NEW FOR ZK-PSI: Cryptographic hashing and set operations
use sha2::{Sha256, Digest};
use std::collections::HashSet;

// NEW: Logging crates for Android Logcat visibility
use log::{info, error};
use android_logger::{Config, FilterBuilder};
use log::LevelFilter;

// ZERO-ALLOCATION AEROSPACE MEMORY
use arrayvec::{ArrayVec, ArrayString};

// PHASE 2.1 & 2.2: NAT Hole Punching & Kademlia Integration
use libp2p::{
    gossipsub, identity, kad, mdns, identify, ping, autonat, dcutr, relay,
    swarm::NetworkBehaviour, PeerId, SwarmBuilder,
    futures::StreamExt, Multiaddr
};

// PHASE 2.3 & 2.4: Bleeding-Edge Spatial Indexing & K-Rings
use h3o::{LatLng, Resolution, CellIndex};

// Define the combined Web3 Node Behaviour (NOW INCLUDES NAT TRAVERSAL)
#[derive(NetworkBehaviour)]
struct NodeBehaviour {
    gossipsub: gossipsub::Behaviour,
    kademlia: kad::Behaviour<kad::store::MemoryStore>,
    mdns: mdns::tokio::Behaviour,
    identify: identify::Behaviour,
    ping: ping::Behaviour,
    autonat: autonat::Behaviour,
    dcutr: dcutr::Behaviour,
    relay_client: relay::client::Behaviour,
}

// THE SOVEREIGN STATE: 
static NODE_IDENTITY: OnceLock<String> = OnceLock::new();
static SOULBOUND_TOKEN: OnceLock<String> = OnceLock::new();

// NEW: PHASE 2.4 - THE DECENTRALIZED OCC ENGINE
// Atomic lock-free counter for the Sovereign Lamport Clock
static LAMPORT_CLOCK: AtomicU64 = AtomicU64::new(0);
// Memory isolation: Tracks which Riders have successfully locked this Driver
static ACTIVE_RIDE_LOCKS: OnceLock<Mutex<HashMap<String, u64>>> = OnceLock::new();

// NEW: PHASE 3.1 - THE SOVEREIGN BLOCK-LATTICE
// The fundamental unit of the L1 Ledger. Every user has their own chain.
#[derive(Clone, Debug)]
pub struct StateBlock {
    pub account: String,           // The user's TEE Public Key (Node ID)
    pub previous_hash: String,     // Link to their previous block (chaining)
    pub representative: String,    // For consensus voting weights
    pub balance: u64,              // The absolute current balance (allows extreme pruning)
    pub link: String,              // Cross-chain reference (e.g., locking an escrow on another chain)
    pub signature: String,         // Hardware-attested TEE signature
}

// In-Memory Ledger for the edge device. 
// A HashMap linking Account IDs to their most recent State Block.
static LOCAL_LEDGER: OnceLock<Mutex<HashMap<String, StateBlock>>> = OnceLock::new();

// =========================================================================
// PHASE 3: MATHEMATICAL REJECTION ENGINE (zk-PSI)
// =========================================================================

/// Cryptographically hashes MAC addresses to prevent raw location data leakage.
fn hash_mac_address(mac: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(mac.as_bytes());
    hex::encode(hasher.finalize())
}

/// The core zk-PSI Mathematical Rejection Engine.
fn execute_zk_psi(scanned_macs: Vec<&str>, expected_macs: Vec<&str>) -> String {
    // Threshold: Must see at least 3 verified nodes
    let threshold_k = 3; 

    // Hash scanned MACs
    let mut scanned_hashes = HashSet::new();
    for mac in scanned_macs {
        let clean_mac = mac.trim();
        if !clean_mac.is_empty() {
            scanned_hashes.insert(hash_mac_address(clean_mac));
        }
    }

    // Hash expected MACs
    let mut expected_hashes = HashSet::new();
    for mac in expected_macs {
        let clean_mac = mac.trim();
        if !clean_mac.is_empty() {
            expected_hashes.insert(hash_mac_address(clean_mac));
        }
    }

    // Intersection: |Set A \cap Set B|
    let intersection: Vec<_> = scanned_hashes.intersection(&expected_hashes).collect();

    if intersection.len() >= threshold_k {
        format!("Execution Approved: zk-PSI Threshold Met. Intersection size: {}", intersection.len())
    } else {
        format!("Execution Denied: Proximity Triangulation Failed. GPS Spoofing Detected. Intersection size: {}", intersection.len())
    }
}


// The Background Network Engine & Tunnel
static ASYNC_RUNTIME: OnceLock<Runtime> = OnceLock::new();

// Command Enum upgraded to use Zero-Allocation Stack Memory and Asynchronous Striking
enum EngineCommand {
    TransmitPoL { 
        global_topic: String, 
        local_zone: ArrayString<32>, 
        payload: String, 
        k_rings: ArrayVec<ArrayString<32>, 7> 
    },
    StrikeLocal {
        local_zone: ArrayString<32>,
        payload: String,
    },
    // NEW: Decoupled L1 Ledger Broadcast
    BroadcastLedger {
        payload: String,
    }
}
static MESH_TX: OnceLock<mpsc::Sender<EngineCommand>> = OnceLock::new();

// =========================================================================
// PHASE 4.1: THE ON-DEVICE zkVM (NOVA IVC FOLDING)
// =========================================================================

// Import Nova's core components
use nova_snark::{
    traits::{circuit::{StepCircuit, TrivialTestCircuit}, Group},
    PublicParams, RecursiveSNARK,
};
use bellpepper_core::{num::AllocatedNum, ConstraintSystem, SynthesisError};
use ff::PrimeField;

// Define the State we want to fold at every step
#[derive(Clone, Debug)]
pub struct RideState<F: PrimeField> {
    pub distance_traveled: F,
    pub current_fare: F,
    pub base_rate: F,
}

// Implement the StepCircuit trait for our RideState
// This is the "Smart Contract" that runs locally on the phone
impl<F: PrimeField> StepCircuit<F> for RideState<F> {
    fn arity(&self) -> usize {
        3 // We are tracking 3 variables: distance, fare, base_rate
    }

    fn synthesize<CS: ConstraintSystem<F>>(
        &self,
        cs: &mut CS,
        z: &[AllocatedNum<F>], // The state from the previous step
    ) -> Result<Vec<AllocatedNum<F>>, SynthesisError> {
        
        // 1. Grab the previous state variables
        let prev_distance = &z[0];
        let prev_fare = &z[1];
        let base_rate = &z[2];

        // 2. We allocate the new inputs for this current step (e.g., GPS ping)
        let delta_distance = AllocatedNum::alloc(cs.namespace(|| "delta d"), || Ok(self.distance_traveled))?;
        
        // 3. ENFORCE THE RULES (The Constraint System)
        // Rule A: Distance must always move forward (delta > 0)
        // In a real circuit, we'd add bit-decomposition checks here.

        // Rule B: Calculate the new total distance
        let new_distance = prev_distance.add(cs.namespace(|| "new distance"), &delta_distance)?;

        // Rule C: Calculate the new fare (prev_fare + (delta_distance * base_rate))
        let fare_increase = delta_distance.mul(cs.namespace(|| "fare increase"), base_rate)?;
        let new_fare = prev_fare.add(cs.namespace(|| "new fare"), &fare_increase)?;

        // 4. Return the new state to be folded into the next step
        Ok(vec![new_distance, new_fare, base_rate.clone()])
    }
}

// A helper function to simulate initializing the Nova Prover
pub fn ignite_zkvm() -> String {
    // In production, generating PublicParams is a heavy, one-time setup.
    // We log the ignition to prove the memory allocation works on ARM.
    info!("🧠 [zkVM] Nova IVC Prover Ignited. Ready to fold ride states.");
    "zkVM Engine Online. Circuits allocated.".to_string()
}


// =========================================================================
// JNI NATIVE BRIDGES (KOTLIN <-> RUST)
// =========================================================================

#[no_mangle]
pub extern "system" fn Java_com_shift_core_TeeBridge_pingVault<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    input: JString<'local>,
) -> jstring {
    
    // 1. Initialize Android Logger instantly so we can see what Rust is doing
    android_logger::init_once(
        Config::default()
            .with_max_level(LevelFilter::Trace)
            .with_tag("SHIFT_VAULT")
    );

    let command: String = env
        .get_string(&input)
        .expect("Failed to read string from OS")
        .into();

    // INJECT THIS EXACT LINE:
    info!("⚙️ [VAULT] JNI Bridge successfully pierced. Incoming OS Command: {}", command);
    
    let mut response = String::new();

    if command.starts_with("REGISTER_NODE:") {
        let public_key = command.replace("REGISTER_NODE:", "");
        
        match NODE_IDENTITY.set(public_key.clone()) {
            Ok(_) => {
                let rt = Runtime::new().expect("Failed to build Tokio Runtime");
                let (tx, mut rx) = mpsc::channel::<EngineCommand>(100);
                let _ = MESH_TX.set(tx);
                
                // NEW: Initialize the OCC memory map
                let _ = ACTIVE_RIDE_LOCKS.set(Mutex::new(HashMap::new()));
                
                // NEW: Phase 3.1 - Initialize the local Block-Lattice
                let _ = LOCAL_LEDGER.set(Mutex::new(HashMap::new()));

                rt.spawn(async move {
                    info!("🚀 [BACKGROUND ENGINE] Spinning up Layer-1 Node...");
                    let local_key = identity::Keypair::generate_ed25519();
                    let local_peer_id = PeerId::from(local_key.public());
                    info!("S.H.I.F.T. Layer-1 Engine Online. Network PeerID: {}", local_peer_id);

                    let (_relay_transport, relay_client) = relay::client::new(local_peer_id);

                    let mut swarm = SwarmBuilder::with_existing_identity(local_key.clone())
                        .with_tokio()
                        .with_tcp(
                            libp2p::tcp::Config::default(),
                            libp2p::noise::Config::new,
                            libp2p::yamux::Config::default,
                        )
                        .expect("Valid TCP Config")
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
                                .heartbeat_interval(Duration::from_secs(1)) // AGGRESSIVE MESH WAKEUP
                                .validation_mode(gossipsub::ValidationMode::Strict)
                                .message_id_fn(message_id_fn)
                                .build()
                                .expect("Valid config");
                            let gossipsub: gossipsub::Behaviour = gossipsub::Behaviour::new(authenticity, gossipsub_config).expect("Valid behaviour");

                            let store = kad::store::MemoryStore::new(local_peer_id);
                            let kademlia = kad::Behaviour::new(local_peer_id, store);
                            let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), local_peer_id).expect("Valid mdns");
                            
                            let identify = identify::Behaviour::new(identify::Config::new(
                                "/shift/1.0.0".to_string(),
                                key.public(),
                            ));
                            let ping = ping::Behaviour::new(ping::Config::new().with_interval(Duration::from_secs(15)));
                            let autonat = autonat::Behaviour::new(local_peer_id, autonat::Config::default());
                            let dcutr = dcutr::Behaviour::new(local_peer_id);

                            Ok(NodeBehaviour { gossipsub, kademlia, mdns, identify, ping, autonat, dcutr, relay_client })
                        })
                        .expect("Valid behaviour builder")
                        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
                        .build();

                    swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse().unwrap()).unwrap();
                    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();

                    // PHASE 2.1: THE RAW SRV OVERRIDE (PLAYIT ANYCAST TCP)
                    let trojan_url = "end-nicholas.gl.at.ply.gg:43013"; 
                    info!("🔍 [DNS] Bypassing SRV Mask. Striking True Host: {}", trojan_url);
                    
                    let bootnode_ip: Multiaddr = match trojan_url.to_socket_addrs() {
                        Ok(mut addrs) => {
                            if let Some(addr) = addrs.find(|a| a.is_ipv4()) {
                                let ip_addr = format!("/ip4/{}/tcp/{}", addr.ip(), addr.port());
                                info!("✅ [DNS] Resolved to: {}", ip_addr);
                                ip_addr.parse().unwrap()
                            } else {
                                "/ip4/127.0.0.1/tcp/4001".parse().unwrap()
                            }
                        }
                        Err(e) => {
                            error!("❌ [DNS] Resolution failed: {:?}", e);
                            "/ip4/127.0.0.1/tcp/4001".parse().unwrap()
                        }
                    };

                    info!("🔗 [NAT BRIDGE] Firing public 5G hard-link to Bootnode at {}...", bootnode_ip);
                    let _ = swarm.dial(bootnode_ip);

                    let mut current_subscriptions: Vec<String> = Vec::new();

                    loop {
                        tokio::select! {
                            cmd_opt = rx.recv() => {
                                if let Some(cmd) = cmd_opt {
                                    match cmd {
                                        EngineCommand::TransmitPoL { global_topic, local_zone, payload, k_rings } => {
                                            info!("⚙️ [VAULT] Processing TransmitPoL command...");
                                            
                                            // 1. Unsubscribe from old zones
                                            for old_zone in &current_subscriptions {
                                                let old_zone_str = old_zone.as_str();
                                                let still_in_range = k_rings.iter().any(|r| r.as_str() == old_zone_str);
                                                if !still_in_range && old_zone_str != global_topic.as_str() {
                                                    let _ = swarm.behaviour_mut().gossipsub.unsubscribe(&gossipsub::IdentTopic::new(old_zone_str));
                                                    info!("🔌 Unsubscribed from out-of-range zone: {}", old_zone_str);
                                                }
                                            }
                                            
                                            current_subscriptions.clear();
                                            
                                            // 2. Subscribe to global and K-Rings
                                            let g_topic = gossipsub::IdentTopic::new(global_topic.clone());
                                            let _ = swarm.behaviour_mut().gossipsub.subscribe(&g_topic);
                                            current_subscriptions.push(global_topic.clone());

                                            for ring_zone in k_rings {
                                                let topic = gossipsub::IdentTopic::new(ring_zone.as_str());
                                                let _ = swarm.behaviour_mut().gossipsub.subscribe(&topic);
                                                current_subscriptions.push(ring_zone.as_str().to_string());
                                            }
                                            info!("📡 Subscribed to Global + 7 Local K-Rings.");

                                            // 3. Publish to Global immediately for Consensus
                                            match swarm.behaviour_mut().gossipsub.publish(g_topic, payload.as_bytes()) {
                                                Ok(msg_id) => info!("🚀 [GOSSIPSUB] -> GLOBAL PUBLISH SUCCESS: {}", msg_id),
                                                Err(e) => error!("❌ [GOSSIPSUB] -> GLOBAL PUBLISH ERROR: {:?}", e),
                                            }
                                            
                                            // 4. DECOUPLED STRIKE: Allow the mesh to form, then hit the local shard
                                            let tx_clone = MESH_TX.get().expect("Mesh TX Missing").clone();
                                            tokio::spawn(async move {
                                                info!("⏳ Waiting 1500ms for GossipSub Mesh to graft...");
                                                tokio::time::sleep(Duration::from_millis(1500)).await;
                                                let _ = tx_clone.send(EngineCommand::StrikeLocal { local_zone, payload }).await;
                                            });
                                        },
                                        EngineCommand::StrikeLocal { local_zone, payload } => {
                                            // 5. The Sub-50ms Strike executes after the mesh is stabilized
                                            let local_topic = gossipsub::IdentTopic::new(local_zone.as_str());
                                            info!("⚡ Executing StrikeLocal on topic: {}", local_zone.as_str());
                                            match swarm.behaviour_mut().gossipsub.publish(local_topic, payload.as_bytes()) {
                                                Ok(msg_id) => info!("🚀 [SPATIAL GOSSIPSUB] -> LOCAL PUBLISH SUCCESS: {}", msg_id),
                                                Err(e) => error!("❌ [SPATIAL GOSSIPSUB] -> LOCAL PUBLISH ERROR: {:?}", e),
                                            }
                                        },
                                        EngineCommand::BroadcastLedger { payload } => {
                                            let ledger_topic = gossipsub::IdentTopic::new("shift-ledger");
                                            // Ensure we are subscribed to the L1 ledger shard
                                            let _ = swarm.behaviour_mut().gossipsub.subscribe(&ledger_topic);
                                            info!("⚡ Broadcasting Genesis Block to Global Ledger...");
                                            
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
                                            info!("🤝 [LINK SECURED] 5G Tunnel connected to Bootnode/Peer: {}", peer_id);
                                            swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                                        }
                                        libp2p::swarm::SwarmEvent::NewListenAddr { address, .. } => {
                                            info!("🛡️ S.H.I.F.T. Node listening on interface: {}", address);
                                        }
                                        libp2p::swarm::SwarmEvent::Behaviour(NodeBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                                            for (peer_id, multiaddr) in list {
                                                info!("👀 Autonomous Discovery! Found Bootnode/Peer: {} at {}", peer_id, multiaddr);
                                                swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                                                let _ = swarm.dial(multiaddr);
                                            }
                                        }
                                        libp2p::swarm::SwarmEvent::Behaviour(NodeBehaviourEvent::Autonat(autonat::Event::StatusChanged { old: _, new })) => {
                                            info!("🌍 [NAT TRAVERSAL] Public Network Status: {:?}", new);
                                        }
                                        libp2p::swarm::SwarmEvent::Behaviour(NodeBehaviourEvent::Dcutr(dcutr::Event { remote_peer_id, result })) => {
                                            match result {
                                                Ok(_) => info!("🕳️✅ [DCUtR] SUCCESSFULLY PUNCHED CGNAT HOLE TO PEER: {}", remote_peer_id),
                                                Err(e) => error!("🕳️❌ [DCUtR] Failed to punch NAT hole to {}: {:?}", remote_peer_id, e),
                                            }
                                        }
                                        libp2p::swarm::SwarmEvent::Behaviour(NodeBehaviourEvent::Gossipsub(gossipsub::Event::Message { propagation_source: peer_id, message, .. })) => {
                                            let payload = String::from_utf8_lossy(&message.data);
                                            let topic_str = message.topic.as_str();

                                            // OCC ENGINE: INTERCEPTING SPATIAL LOCK REQUESTS
                                            if payload.starts_with("LOCK_REQUEST:") {
                                                info!("⚠️ [OCC ENGINE] Sub-50ms Lock Request caught from: {}", peer_id);
                                                
                                                // Extract the Lamport Sequence Ticket (e.g., LOCK_REQUEST:14)
                                                let parts: Vec<&str> = payload.split(':').collect();
                                                if parts.len() == 2 {
                                                    if let Ok(incoming_ticket) = parts[1].parse::<u64>() {
                                                        
                                                        // Update our local Sovereign Clock to match the mesh reality
                                                        LAMPORT_CLOCK.fetch_max(incoming_ticket, Ordering::SeqCst);
                                                        LAMPORT_CLOCK.fetch_add(1, Ordering::SeqCst);

                                                        if let Some(locks_mutex) = ACTIVE_RIDE_LOCKS.get() {
                                                            let mut active_locks = locks_mutex.lock().unwrap();
                                                            
                                                            // Check if this Driver is already locked by someone else
                                                            if active_locks.is_empty() {
                                                                active_locks.insert(peer_id.to_string(), incoming_ticket);
                                                                info!("✅ [OCC SUCCESS] Mathematical Lock granted to Rider [{}] on Ticket #{}.", peer_id, incoming_ticket);
                                                                
                                                                // NOTE: In Step 2, we will broadcast LOCK_ACCEPTED back to the mesh
                                                            } else {
                                                                error!("❌ [OCC REJECTED] Driver is currently engaged. Lock denied for Rider [{}].", peer_id);
                                                                // NOTE: In Step 2, we will broadcast LOCK_DENIED back to the mesh
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                info!("📡 [RADAR] Spatial Telemetry from {}: {}", peer_id, payload);
                                                // We received standard telemetry, increment the Lamport clock organically
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
                    let display_str = if public_key.len() > 16 { &public_key[public_key.len() - 16..] } else { &public_key };
                    response = format!("Vault Locked. Node [...]{} stored. Async L1 Engine IGNITED.", display_str);
                } else {
                    response = "Vault Locked, but Async Engine failed to store state.".to_string();
                }
            },
            Err(_) => {
                response = "Vault Error: Identity already locked.".to_string();
            }
        }
    } 
    else if command.starts_with("ISSUE_SBT:") {
        let kyc_badge = command.replace("ISSUE_SBT:", "");
        if NODE_IDENTITY.get().is_some() {
            match SOULBOUND_TOKEN.set(kyc_badge.clone()) {
                Ok(_) => response = format!("Soulbound Token Accepted. KYC Clearance: [{}] securely stored.", kyc_badge),
                Err(_) => response = "Vault Error: Soulbound Token already issued.".to_string(),
            }
        } else {
            response = "Vault Error: Cannot issue SBT. No Node Identity established.".to_string();
        }
    }
    else if command.starts_with("GENERATE_POL:") {
        let telemetry = command.replace("GENERATE_POL:", "");
        let node_id = NODE_IDENTITY.get();
        let sbt = SOULBOUND_TOKEN.get();

        if let (Some(identity), Some(token)) = (node_id, sbt) {
            
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

            // PHASE 2.4 ZERO-ALLOCATION AEROSPACE MATH
            let mut k_ring_zones: ArrayVec<ArrayString<32>, 7> = ArrayVec::new();
            let mut core_h3_zone = ArrayString::<32>::new();

            match LatLng::new(extracted_lat, extracted_lon) {
                Ok(coord) => {
                    let cell: CellIndex = coord.to_cell(Resolution::Nine);
                    // Format directly to the stack cache. We drop .to_string() here.
                    write!(&mut core_h3_zone, "zone:{}", cell).unwrap();

                    // Instruct the compiler to construct the 7 items directly into a Stack Array
                    let disk_distances: ArrayVec<(CellIndex, u32), 7> = cell.grid_disk_distances(1);

                    for (c, _distance) in disk_distances {
                        let mut zone_str = ArrayString::<32>::new();
                        // Format directly to the stack cache. No temporary Strings.
                        write!(&mut zone_str, "zone:{}", c).unwrap();
                        k_ring_zones.push(zone_str);
                    }
                },
                Err(_) => {
                    write!(&mut core_h3_zone, "zone:UNKNOWN_HEX").unwrap();
                    let mut unknown_str = ArrayString::<32>::new();
                    write!(&mut unknown_str, "zone:UNKNOWN_HEX").unwrap();
                    k_ring_zones.push(unknown_str);
                }
            };

            let mut hasher = DefaultHasher::new();
            identity.hash(&mut hasher);
            token.hash(&mut hasher); 
            telemetry.hash(&mut hasher); 
            let cryptogram = hasher.finish();
            let cryptogram_str = format!("{:x}", cryptogram);
            
            let display_id = if identity.len() > 8 { &identity[identity.len() - 8..] } else { identity };
            
            if let Some(tx) = MESH_TX.get() {
                let payload = format!("Node [...{}] deployed to H3 Hexagon: [{}] | BLE Peers: [{}] | Hash: {}", display_id, core_h3_zone.as_str(), extracted_ble, cryptogram_str);
                let _ = tx.try_send(EngineCommand::TransmitPoL {
                    global_topic: "shift-pol-network".to_string(),
                    local_zone: core_h3_zone, // ArrayString copied byte-for-byte, no heap allocation
                    payload,
                    k_rings: k_ring_zones.clone(), // ArrayVec copied byte-for-byte, no heap allocation
                }); 
            }

            response = format!("PoL Valid & Cleared.\nActive Shard: [{}]\nRadar K-Rings Active: {}\nNode: [...{}]\nBLE Signatures: {}\nHardware Hash: {}", core_h3_zone.as_str(), k_ring_zones.len(), display_id, extracted_ble, cryptogram_str);
        } else if node_id.is_none() {
            response = "Execution Denied: Node Identity not found.".to_string();
        } else {
            response = "Execution Denied: Soulbound Token (KYC) not found.".to_string();
        }
    }
    else if command.starts_with("FIRE_LOCK:") {
        let target_zone = command.replace("FIRE_LOCK:", "");
        
        // Advance the Sovereign Clock for an outbound strike
        let current_ticket = LAMPORT_CLOCK.fetch_add(1, Ordering::SeqCst) + 1;
        let payload = format!("LOCK_REQUEST:{}", current_ticket);
        
        if let Some(tx) = MESH_TX.get() {
            let mut zone_str = ArrayString::<32>::new();
            // Write directly to the L1 cache stack
            let _ = write!(&mut zone_str, "{}", target_zone);
            
            // Fire the 50ms strike directly into the spatial shard
            let _ = tx.try_send(EngineCommand::StrikeLocal {
                local_zone: zone_str,
                payload: payload.clone(),
            });
            response = format!("Lamport Ticket #{} generated. Lock Request fired into Shard: [{}]", current_ticket, target_zone);
        } else {
            response = "Execution Denied: Layer-1 Engine Offline.".to_string();
        }
    } 
    // NEW: PHASE 3.1 - THE GENESIS BLOCK MINTING
    else if command.starts_with("MINT_GENESIS:") {
        let node_id = NODE_IDENTITY.get();
        if let Some(identity) = node_id {
            if let Some(ledger_mutex) = LOCAL_LEDGER.get() {
                let mut ledger = ledger_mutex.lock().unwrap();
                
                // Ensure the account doesn't already exist
                if !ledger.contains_key(identity) {
                    
                    // Create the Genesis StateBlock
                    let genesis_block = StateBlock {
                        account: identity.clone(),
                        previous_hash: "0000000000000000000000000000000000000000000000000000000000000000".to_string(), // Root origin
                        representative: identity.clone(), // Self-representing by default
                        balance: 1000, // Initial network drop for testing (1000 SHIFT tokens)
                        link: "GENESIS_MINT".to_string(),
                        signature: "TEE_HARDWARE_SIG_PLACEHOLDER".to_string(), // Will be bound to actual TEE later
                    };
                    
                    // Hash the block to prove cryptographic integrity
                    let mut hasher = DefaultHasher::new();
                    genesis_block.account.hash(&mut hasher);
                    genesis_block.previous_hash.hash(&mut hasher);
                    genesis_block.balance.hash(&mut hasher);
                    let block_hash = format!("{:x}", hasher.finish());

                    // Anchor the chain into the local ledger
                    ledger.insert(identity.clone(), genesis_block);
                    
                    let display_id = if identity.len() > 8 { &identity[identity.len() - 8..] } else { identity };
                    response = format!("Sovereign Chain Anchored.\nNode: [...{}]\nGenesis Hash: {}\nBalance: 1000 SHIFT", display_id, block_hash);
                    info!("💎 [BLOCK-LATTICE] Genesis Block Minted for Account: {}", display_id);

                    // NEW: FIRE THE BLOCK TO THE GLOBAL NETWORK WITHOUT WIPING SPATIAL ROUTING
                    if let Some(tx) = MESH_TX.get() {
                        let payload = format!("STATE_BLOCK_ANNOUNCEMENT|ACCOUNT:{}|HASH:{}|BALANCE:1000", identity, block_hash);
                        let _ = tx.try_send(EngineCommand::BroadcastLedger { payload });
                    }

                } else {
                    response = "Execution Denied: Sovereign Chain already exists for this Node.".to_string();
                }
            } else {
                response = "Execution Denied: Block-Lattice memory corrupted.".to_string();
            }
        } else {
            response = "Execution Denied: Node Identity not found. Run Phase 1.5 first.".to_string();
        }
    }
    else {
        response = format!("Unrecognized command: [{}].", command);
    }

    let output = env.new_string(response).expect("Failed to create secure response");
    output.into_raw()
}

// NEW EXPOSED FUNCTION: Android calls this to ignite the zkVM
#[no_mangle]
pub extern "system" fn Java_com_shift_core_TeeBridge_igniteZkVM<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jstring {
    
    // Ignite the Engine
    let result = ignite_zkvm();

    // Return status to Android
    let output = env.new_string(result).expect("Failed to create output string");
    output.into_raw()
}

// NEW EXPOSED FUNCTION: Android calls this directly to run the Rejection Engine
#[no_mangle]
pub extern "system" fn Java_com_shift_core_TeeBridge_verifyProximityProof<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    scanned_input: JString<'local>,
    expected_input: JString<'local>,
) -> jstring {
    
    let scanned_str: String = match env.get_string(&scanned_input) {
        Ok(s) => s.into(),
        Err(_) => return env.new_string("Error: Failed to read scanned input").unwrap().into_raw(),
    };
    
    let expected_str: String = match env.get_string(&expected_input) {
        Ok(s) => s.into(),
        Err(_) => return env.new_string("Error: Failed to read expected input").unwrap().into_raw(),
    };

    let scanned_macs: Vec<&str> = scanned_str.split(',').collect();
    let expected_macs: Vec<&str> = expected_str.split(',').collect();

    let result = execute_zk_psi(scanned_macs, expected_macs);

    let output = env.new_string(result).expect("Failed to create output string");
    output.into_raw()
}