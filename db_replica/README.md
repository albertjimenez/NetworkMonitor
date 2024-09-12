# `Database Replica`

`db_replica` is a Rust application designed to listen to a NATS JetStream queue for network hosts from the `network-handler` project and insert them into a PostgreSQL database to keep them persisted. This service acts as a bridge between NATS messaging and a relational database.

## Architecture Overview

The system architecture consists of several key components:

1. **NATS JetStream**: A messaging system where the application listens for messages containing network host information.
2. **Database**: PostgreSQL database where the discovered hosts and their service ports are stored.
3. **Cache**: An in-memory cache for MAC vendors to optimize lookup operations.

Here is an ASCII diagram representing the flow of data through the system:
```text
   +--------------------+                 +----------------+
   |                    |                 |                |
   |   Network-Handler  |  --> (NATS) --> |   db_replica   |
   |                    |                 |                |
   +--------------------+                 +----------------+
                                                  |
                                                  |
                                                  v
                                          +-----------------+
                                          |                 |
                                          |  PostgreSQL DB  |
                                          |                 |
                                          +-----------------+
```

## Components

### `DBInterface`

Defines the interface for database operations related to discovered hosts and service ports.

#### Methods
- `new(max_conns: Option<u32>)`: Creates a new instance of the database interface.
- `insert_discovered_host(&self, discovered_host: DiscoveredHost)`: Inserts a discovered host into the database.
- `find_all_service_ports(&self)`: Retrieves all service ports from the database.
- `find_service_port_by_port_number(&self, port_number: i32)`: Finds a service port by its port number.
- `get_all_mac_vendors_minimal_info(&self)`: Retrieves all MAC vendors and their minimal information from the database.

### `DBHandler`

Implements `DBInterface` and manages database interactions for discovered hosts and service ports.

#### Methods
- `process_service_ports(&self, discovered_host: &DiscoveredHost)`: Processes and inserts service ports for a discovered host.
- `insert_host(&self, discovered_host: &DiscoveredHost)`: Inserts a discovered host into the database.
- `link_host_with_ports(&self, host_id: i32, service_ports: &[ServicePort])`: Links a host with its service ports in the database.
- `insert_many_to_many_services_hosts(&self, service_port_id: Option<i32>, discovered_host_id: Option<i32>)`: Inserts a many-to-many relationship between a service port and a discovered host.
- `insert_service_port(&self, port: i32, name: &str)`: Inserts a service port into the database.

### `QueueListener`

Listens for messages from a NATS JetStream queue and processes them to update the database.

#### Methods
- `new(db_handler: T, cache: M)`: Creates a new instance of `QueueListener`.
- `handle_consumers(&self)`: Handles incoming messages from the NATS queue and processes them.

## Getting Started

### Prerequisites

- Rust and Cargo installed
- PostgreSQL database set up
- NATS JetStream server running

### Configuration 
1. Configure environment variables. Create a `.env` file in the root directory with the following entries:
    ```sh
    DATABASE_URL=postgres://user:password@localhost/dbname
    NATS_URL=nats://localhost:4222
    ```