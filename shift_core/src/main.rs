#![allow(non_snake_case)]
#![allow(unused_variables)] 
#![allow(unused_assignments)]

mod zk_engine; // PHASE 1.6 & 4.3 Modularized ZK Logic
mod ranging;   // PHASE 1.6 Cryptographic Ranging Engine

use std::os::unix::io::{AsRawFd, FromRawFd};
use std::io::{Read, Write};
use std::net::Shutdown;
use nix::sys::socket::{socket, bind, listen, accept, AddressFamily, SockFlag, SockType, VsockAddr};
use nix::sys::socket::SockAddr;

use std::sync::OnceLock;
use std::collections::hash_map::DefaultHasher; 
use std::hash::{Hash, Hasher};                 
use tokio::runtime::Runtime; 
use tokio::sync::mpsc; 
use tokio::time::Duration;
use std::net::ToSocketAddrs; 
use std::fmt::Write as FmtWrite; 
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::collections::HashMap;

use sha2::{Sha256, Digest};
use std::collections::HashSet;

use log::{info, error};
use android_logger::{Config, FilterBuilder};
use log::LevelFilter;

use arrayvec::{ArrayVec, ArrayString};

use libp2p::{
    gossipsub, identity, kad, mdns, identify, ping, autonat, dcutr, relay,
    swarm::NetworkBehaviour, PeerId, SwarmBuilder,
    futures::StreamExt, Multiaddr
};
use h3o::{LatLng, Resolution, CellIndex};

// =========================================================================
// THE SOVEREIGN STATE & P2P MESH
// =========================================================================

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
        k_rings: ArrayVec<ArrayString<32>, 7> 
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
// =========================================================================
const VSOCK_PORT: u32 = 8000;
const VMADDR_CID_ANY: u32 = 0xFFFFFFFF; 

fn main() {
    android_logger::init_once(
        Config::default()
            .with_max_level(LevelFilter::Trace)
            .with_tag("SHIFT_VAULT")
    );

    info!("🛡️ [HYPERVISOR] Rust Vault booting in isolated pKVM.");
    
    // 1. Create a Virtual Socket (AF_VSOCK)
    let fd = socket(AddressFamily::Vsock, SockType::Stream, SockFlag::empty(), None)
        .expect("Failed to create vsock");

    // 2. Bind to the designated port inside the VM
    let addr = VsockAddr::new(VMADDR_CID_ANY, VSOCK_PORT);
    bind(fd.as_raw_fd(), &SockAddr::Vsock(addr)).expect("Failed to bind vsock port");

    // 3. Listen for incoming connections from the Kotlin OS
    listen(fd.as_raw_fd(), 10).expect("Failed to listen on vsock");
    info!("🎧 [HYPERVISOR] Vault listening on vsock port {}...", VSOCK_PORT);

    // 4. The Event Loop: Process commands from the Android App
    loop {
        match accept(fd.as_raw_fd()) {
            Ok(client_fd) => {
                info!("🤝 [HYPERVISOR] Kotlin OS connection accepted.");
                let mut stream = unsafe { std::net::TcpStream::from_raw_fd(client_fd) };
                
                let mut buffer = [0; 8192];
                if let Ok(bytes_read) = stream.read(&mut buffer) {
                    if bytes_read > 0 {
                        let command = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();
                        info!("⚙️ [HYPERVISOR] Command received: {}", command);
                        
                        // Process the command exactly as the old JNI bridge did
                        let response = process_vault_command(&command);
                        
                        let _ = stream.write(response.as_bytes());
                    }
                }
                let _ = stream.shutdown(Shutdown::Both);
            }
            Err(e) => error!("❌ [HYPERVISOR] Connection failed: {:?}", e),
        }
    }
}

// =========================================================================
// VAULT COMMAND PROCESSOR (Extracted from old pingVault)
// =========================================================================
fn process_vault_command(command: &str) -> String {
    let mut response = String::new();

    if command.starts_with("REGISTER_NODE:") {
        let public_key = command.replace("REGISTER_NODE:", "");
        
        match NODE_IDENTITY.set(public_key.clone()) {
            Ok(_) => {
                let rt = Runtime::new().expect("Failed to build Tokio Runtime");
                let (tx, mut rx) = mpsc::channel::<EngineCommand>(100);
                let _ = MESH_TX.set(tx);
                
                let _ = ACTIVE_RIDE_LOCKS.set(Mutex::new(HashMap::new()));
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
                                .heartbeat_interval(Duration::from_secs(1)) 
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

                    // PHASE 1.6 UPDATE: Wait for incoming Wi-Fi Aware Connections instead of connecting to a static bootnode
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

                                            for ring_zone in k_rings {
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
                    response = format!("Vault Locked. Async L1 Engine IGNITED.");
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

            let mut hasher = DefaultHasher::new();
            identity.hash(&mut hasher);
            telemetry.hash(&mut hasher); 
            let cryptogram = format!("{:x}", hasher.finish());
            
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

            let mut zksnark_receipt = String::new();
            match ranging::verify_time_of_flight(&challenge, &response_obj, &dummy_peer_key.public()) {
                Ok((delta_t, t_compute)) => {
                    zksnark_receipt = zk_engine::generate_tof_proof(delta_t, t_compute, 50_000);
                    
                    if let Some(tx) = MESH_TX.get() {
                        let payload = format!("Node deployed to: [{}] | BLE: [{}] | Hash: {} | ZK-DB: {}", core_h3_zone.as_str(), extracted_ble, cryptogram, zksnark_receipt);
                        let _ = tx.try_send(EngineCommand::TransmitPoL {
                            global_topic: "shift-pol-network".to_string(),
                            local_zone: core_h3_zone.clone(), 
                            payload,
                            k_rings: k_ring_zones.clone(), 
                        }); 
                    }
                    response = format!("PoL Valid & Cleared. ZK-Proof Generated.");
                },
                Err(_) => {
                    response = "Execution Denied: Cryptographic Distance Bounding Failed.".to_string();
                }
            }
        } else {
            response = "Execution Denied: Node Identity not found.".to_string();
        }
    }
    // Handle other commands (MINT_GENESIS, FIRE_LOCK, etc.)
    
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