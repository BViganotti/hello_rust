# Rust P2P File Sharing Application

A peer-to-peer file sharing application built with Rust and libp2p. This project demonstrates how to create a decentralized network where peers can discover each other and share files locally.

## Features

Current features:
- 🔍 Automatic peer discovery using mDNS (local network)
- 🤝 Peer identification and metadata exchange
- ❤️ Connection health monitoring with ping/pong
- 🔒 Secure communication using noise protocol
- 📡 TCP transport with yamux multiplexing

Planned features:
- 📁 File listing and sharing
- 🔍 File search functionality
- ⬇️ File download with progress tracking
- 📊 Transfer statistics
- 🎯 Selective file downloading

## Prerequisites

- Rust 1.70.0 or higher
- Cargo package manager

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd hello_rust
```

2. Build the project:
```bash
cargo build
```

3. Run the application:
```bash
cargo run
```

## Usage

When you start the application, it will:
1. Generate a unique PeerID for your node
2. Start listening on a random port
3. Automatically discover other peers on your local network
4. Display events such as peer discovery, identification, and ping results

## Project Structure

```
src/
  ├── main.rs          # Main application code
  └── (more to come)   # Future modules for file handling, etc.
```

## Dependencies

- `libp2p`: Core P2P networking functionality
- `tokio`: Async runtime
- `futures`: Async utilities
- `serde`: Serialization framework
- Other dependencies listed in `Cargo.toml`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [libp2p](https://libp2p.io/) for the P2P networking framework
- Rust and tokio communities for excellent documentation and support

---
**Note**: This is a work in progress. Features and documentation will be updated as the project evolves.
