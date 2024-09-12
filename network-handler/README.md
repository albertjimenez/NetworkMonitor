# Host Scanner with NATS Integration

This Rust project provides a host scanning tool integrated with NATS for publishing discovered host data. The tool scans a network interface, identifies active hosts, and publishes the data to a NATS JetStream channel.

## Features

- Host Scanning: Uses ICMP ping scan to detect active hosts on the network.
- Port Scanning: Scans specific ports on discovered hosts to identify open services.
- NATS JetStream Integration: Publishes discovered host data to a NATS JetStream channel.
- Runtime Configuration: Uses NATS key-value store to dynamically update scan intervals.
## Prerequisites
- `sudo` access
- Rust: Ensure you have Rust installed. If not, install it from [Rust-Lang](rust-lang.org).
- NATS Server: A running NATS server is required. You can download and run it from [NATS.io](nats.io).
Setup

## Setup
### Install Dependencies
`cargo build`
### Configure Environment Variables
You will need to set up the following environment variables. Since the application may require sudo privileges to access network interfaces, use sudo -E to preserve these variables when running with sudo.
`export NATS_URL="nats://localhost:4222"`

### Running the Application
Use the following command to run the application with elevated privileges, ensuring that environment variables are preserved:

`sudo -E cargo run`

### Diagram
```text
+---------------------+           +-------------------+         +-----------------------------+
|                     |  Retrieve |                   |         |                             |
|     NATS Server     | <-------- |  Network Handler  | <------ |  NATS Key-Value Store       |
|                     |           |                   |         | (Interval Duration Storage) |
|  (JetStream Queue)  |           |                   |         |                             |
|                     |           +-------------------+         +-----------------------------+
|                     |
|                     |
|                     |           +-------------------+         +-----------------------------+
|                     | <-------- |  Network Handler  |  Insert |  NATS JetStream Queue       |
|                     |  Publish  |                   | ------> | (Discovered Hosts Storage)  |
|                     |           +-------------------+         +-----------------------------+
+---------------------+
```
#### NATS Server
- Acts as the central communication hub, managing both the key-value store and the JetStream queue.
#### Network Handler
- Retrieves the interval duration from the NATS Key-Value Store (shown by the arrow pointing from the key-value store to the handler).
- Scans the network to discover hosts.
- Publishes the discovered hosts to the NATS JetStream queue (shown by the arrow pointing from the network handler to the NATS server).
