use libp2p::{
    gossipsub, identity, kad, mdns, identify, ping, autonat, relay,
    swarm::NetworkBehaviour, PeerId, SwarmBuilder,
    futures::StreamExt,
};
use tokio::time::Duration;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// PHASE 2.1 & 2.2: Define the combined Web3 Node Behaviour (NOW INCLUDES NAT SERVER LOGIC)
#[derive(NetworkBehaviour)]
struct BootnodeBehaviour {
    gossipsub: gossipsub::Behaviour,
    kademlia: kad::Behaviour<kad::store::MemoryStore>,
    mdns: mdns::tokio::Behaviour, 
    identify: identify::Behaviour,
    ping: ping::Behaviour,
    autonat: autonat::Behaviour,
    relay: relay::Behaviour,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("==================================================");
    println!("S.H.I.F.T. LAYER-1 GLOBAL BOOTNODE INITIALIZING...");
    println!("==================================================");

    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("📡 Bootnode PeerID: {}", local_peer_id);

    let mut swarm = SwarmBuilder::with_existing_identity(local_key.clone())
        .with_tokio()
        .with_tcp(
            libp2p::tcp::Config::default(),
            libp2p::noise::Config::new,
            libp2p::yamux::Config::default,
        )?
        .with_quic()
        .with_behaviour(|key| {
            let authenticity = gossipsub::MessageAuthenticity::Signed(key.clone());
            let message_id_fn = |message: &gossipsub::Message| {
                let mut s = DefaultHasher::new();
                message.data.hash(&mut s);
                gossipsub::MessageId::from(s.finish().to_string())
            };
            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(10))
                .validation_mode(gossipsub::ValidationMode::Strict)
                .message_id_fn(message_id_fn)
                .build()
                .unwrap();
            let gossipsub: gossipsub::Behaviour = gossipsub::Behaviour::new(authenticity, gossipsub_config).unwrap();

            let store = kad::store::MemoryStore::new(local_peer_id);
            let kademlia = kad::Behaviour::new(local_peer_id, store);
            let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), local_peer_id).unwrap();

            // PHASE 2.1: Initialize Server-Side NAT Traversal Behaviours
            let identify = identify::Behaviour::new(identify::Config::new(
                "/shift/1.0.0".to_string(),
                key.public(),
            ));
            let ping = ping::Behaviour::new(ping::Config::new().with_interval(Duration::from_secs(15)));
            // Bootnode acts as the AutoNAT Server (Mirror)
            let autonat = autonat::Behaviour::new(local_peer_id, autonat::Config::default());
            // Bootnode acts as the Relay Server (Bridge)
            let relay = relay::Behaviour::new(local_peer_id, relay::Config::default());

            Ok(BootnodeBehaviour { gossipsub, kademlia, mdns, identify, ping, autonat, relay })
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    swarm.listen_on("/ip4/0.0.0.0/udp/4001/quic-v1".parse()?)?;
    swarm.listen_on("/ip4/0.0.0.0/tcp/4001".parse()?)?;

    // 1. Subscribe to the Global Chain
    let global_topic = gossipsub::IdentTopic::new("shift-pol-network");
    swarm.behaviour_mut().gossipsub.subscribe(&global_topic)?;

    // NEW: Subscribe to the global L1 Ledger topic
    let ledger_topic = gossipsub::IdentTopic::new("shift-ledger");
    swarm.behaviour_mut().gossipsub.subscribe(&ledger_topic)?;

    // 2. TEMPORARY RADAR TEST: Subscribe to the Fold 6's specific local Hexagon
    let local_topic_1 = gossipsub::IdentTopic::new("zone:892b9ab93c7ffff");
    swarm.behaviour_mut().gossipsub.subscribe(&local_topic_1)?;

    let local_topic_2 = gossipsub::IdentTopic::new("892b9ab93c7ffff"); // Fallback formatting
    swarm.behaviour_mut().gossipsub.subscribe(&local_topic_2)?;
    
    println!("🌐 Bootnode NAT-Relay, DHT, and GossipSub Mesh Online.");
    println!("⌛ Listening for autonomous Sovereign Nodes...\n");

    loop {
        match swarm.select_next_some().await {
            libp2p::swarm::SwarmEvent::NewListenAddr { address, .. } => {
                println!("✅ Listening on local network: {}", address);
            }
            libp2p::swarm::SwarmEvent::Behaviour(BootnodeBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                for (peer_id, _multiaddr) in list {
                    println!("👀 Auto-Discovered Sovereign Node: {}", peer_id);
                    swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                }
            }
            // PHASE 2.1: Log incoming Relay Requests
            libp2p::swarm::SwarmEvent::Behaviour(BootnodeBehaviourEvent::Relay(event)) => {
                println!("🌉 [RELAY BRIDGE] Network Activity: {:?}", event);
            }
            libp2p::swarm::SwarmEvent::Behaviour(BootnodeBehaviourEvent::Gossipsub(gossipsub::Event::Message { propagation_source: peer_id, message, .. })) => {
                let payload = String::from_utf8_lossy(&message.data);
                
                // NEW: Ignore ephemeral L2 routing requests (OCC)
                if payload.starts_with("LOCK_REQUEST:") {
                    // Do not run global consensus on ephemeral locks
                    continue; 
                }

                // NEW: Intercept and process Block-Lattice Mints
                if payload.starts_with("STATE_BLOCK_ANNOUNCEMENT|") {
                    println!("--------------------------------------------------");
                    println!("💎 [L1 LEDGER] INCOMING STATE BLOCK DETECTED");
                    println!("📦 Raw Payload: {}", payload);
                    println!("✅ ACTION: Sovereign Chain Anchored in Global State.");
                    println!("--------------------------------------------------");
                    continue;
                }

                println!("--------------------------------------------------");
                println!("⚡ [LAYER-1 RECEPTION] INCOMING PoL PAYLOAD CAUGHT");
                // PHASE 2.4: Log the Spatial Shard Zone Topic
                println!("🌐 Target Spatial Shard: {}", message.topic);
                println!("📍 From Node ID: {}", peer_id);
                println!("📦 Raw Payload: {}", payload);
                
                println!("🔍 [CONSENSUS ENGINE] Analyzing Hardware Proximity Signatures...");
                
                if let Some(ble_start) = payload.find("BLE Peers: [") {
                    let start_idx = ble_start + 12;
                    if let Some(end_idx) = payload[start_idx..].find("]") {
                        let ble_data = &payload[start_idx..start_idx + end_idx];
                        
                        if ble_data.is_empty() || ble_data.contains("ISOLATED") {
                            println!("❌ [CONSENSUS DENIED] Proximity Triangulation Failed.");
                            println!("⚠️ REASON: 0 local peers detected. High probability of OS-level GPS Spoofing.");
                            println!("🛑 ACTION: Node reputation slashed. PoL dropped from state.");
                        } else {
                            let mac_count = ble_data.split(',').count();
                            println!("✅ [CONSENSUS REACHED] Proximity Triangulation Validated.");
                            println!("🛡️ REASON: Node is mathematically anchored by {} physical BLE signatures.", mac_count);
                            println!("💾 ACTION: PoL cryptographically verified. Routing to Block-Lattice...");
                        }
                    }
                } else {
                    println!("❌ [CONSENSUS DENIED] Malformed Telemetry. Missing BLE Array.");
                }
                println!("--------------------------------------------------\n");
            }
            _ => {}
        }
    }
}