
# Basic WebSocket Server in Rust

A lightweight, asynchronous WebSocket server implementation using Rust and Tokio.

## Features

- Asynchronous WebSocket communication
- Echo server functionality
- Connection management with proper error handling
- Built with Tokio and Tokio-Tungstenite

## Requirements

- Rust 2024 edition
- Cargo package manager

## Installation

Clone the repository and build the project:

```bash
git clone https://github.com/LinuxUser255/websocket-server.git
cd websocket-server
cargo build --release
```

## Usage

Run the server:

```bash
cargo run
```

The WebSocket server will start on `ws://127.0.0.1:9001`.

### Testing the Server

You can test the server using various WebSocket clients:

- Browser-based clients like [WebSocket King](https://websocketking.com/)
- Command-line tools like [websocat](https://github.com/vi/websocat)
- Write your own WebSocket client

Example with websocat:

```bash
websocat ws://127.0.0.1:9001
```

Type messages and they will be echoed back to you.

## Project Structure

- `src/main.rs` - Server implementation with connection handling
- `Cargo.toml` - Project dependencies and configuration

## Dependencies

- `tokio` - Asynchronous runtime
- `tokio-tungstenite` - WebSocket implementation for Tokio
- `tungstenite` - WebSocket protocol implementation
- `futures` - Utilities for asynchronous programming

## License

[GPLv3](LICENSE)
```
