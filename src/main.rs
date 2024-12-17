//! A P2P file sharing application built with libp2p
//! 
//! This application demonstrates basic peer-to-peer networking capabilities using libp2p,
//! including peer discovery, identification, and ping functionality.

use futures::StreamExt;
use libp2p::{
    identify,
    identity::Keypair,
    mdns,
    noise,
    ping,
    swarm::{NetworkBehaviour, Swarm, SwarmEvent},
    tcp,
    yamux,
    PeerId,
    Transport,
};
use std::error::Error;
use tokio;

/// Represents the network behavior of our P2P node.
/// This struct combines multiple behaviors:
/// - Identify: Helps peers exchange identification information
/// - Ping: Allows checking connectivity with peers
/// - MDNS: Enables automatic peer discovery on local networks
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MyBehaviourEvent")]
struct MyBehaviour {
    identify: identify::Behaviour,
    ping: ping::Behaviour,
    mdns: mdns::async_io::Behaviour,
}

/// Represents all possible events that can be emitted by our network behavior.
/// This enum combines events from all our behaviors (Identify, Ping, MDNS).
#[derive(Debug)]
enum MyBehaviourEvent {
    Identify(identify::Event),
    Ping(ping::Event),
    Mdns(mdns::Event),
}

// Implementation of From traits to convert specific behavior events into our custom event type
impl From<identify::Event> for MyBehaviourEvent {
    fn from(event: identify::Event) -> Self {
        MyBehaviourEvent::Identify(event)
    }
}

impl From<ping::Event> for MyBehaviourEvent {
    fn from(event: ping::Event) -> Self {
        MyBehaviourEvent::Ping(event)
    }
}

impl From<mdns::Event> for MyBehaviourEvent {
    fn from(event: mdns::Event) -> Self {
        MyBehaviourEvent::Mdns(event)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Generate a random Ed25519 keypair for secure communication
    let local_key = Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    // Set up the noise protocol for authentication
    let auth_config = noise::Config::new(&local_key).expect("signing libp2p-noise static keypair failed");
    
    // Create a transport layer with the following stack:
    // - TCP as the underlying transport
    // - Upgrade to secure channel using noise protocol
    // - Multiplex multiple substreams using yamux
    let transport = tcp::async_io::Transport::new(tcp::Config::default())
        .upgrade(libp2p::core::upgrade::Version::V1Lazy)
        .authenticate(auth_config)
        .multiplex(yamux::Config::default())
        .boxed();

    // Create a Swarm to manage peers and network events
    let mut swarm = {
        // Set up the identify protocol
        let identify = identify::Behaviour::new(identify::Config::new(
            "rust-p2p-example/1.0.0".to_string(),
            local_key.public(),
        ));
        
        // Set up the ping protocol
        let ping = ping::Behaviour::new(ping::Config::new());
        
        // Set up mDNS for peer discovery
        let mdns = mdns::async_io::Behaviour::new(mdns::Config::default(), local_peer_id)?;
        
        // Combine all protocols into a single behavior
        let behaviour = MyBehaviour {
            identify,
            ping,
            mdns,
        };
        
        // Create the swarm using tokio as the executor
        let config = libp2p::swarm::Config::with_tokio_executor();
        Swarm::new(transport, behaviour, local_peer_id, config)
    };

    // Listen on all interfaces with a random port
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Main event loop
    loop {
        match swarm.select_next_some().await {
            // New listening address has been established
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on {:?}", address);
            }
            // New peer discovered through mDNS
            SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(peers))) => {
                for (peer_id, addr) in peers {
                    println!("Discovered peer {} with addr {}", peer_id, addr);
                }
            }
            // Received an identify event
            SwarmEvent::Behaviour(MyBehaviourEvent::Identify(event)) => {
                println!("Identify event: {:?}", event);
            }
            // Received a ping event
            SwarmEvent::Behaviour(MyBehaviourEvent::Ping(event)) => {
                println!("Ping event: {:?}", event);
            }
            // Ignore all other events
            _ => {}
        }
    }
}
