# Common Data Model

## Overview

The `common-data-model` project provides a set of data models and utilities for handling network discovery and management. It includes functionality for interacting with NATS messaging systems, managing network host information, and caching MAC vendor data. This project is designed to be used as a component in the rest of the systems that require network scanning and data persistence.

## Features

- **Network Discovery:** Scan network interfaces to discover hosts and their associated information.
- **Data Serialization:** Convert data to and from JSON format.
- **Vendor Cache:** Maintain a thread-safe cache of MAC address to vendor name mappings.
- **Configurable Runtime Settings:** Retrieve and manage runtime settings from a NATS key-value store.

## Components

- **Data Models:** Define structures for network hosts, services, and caching.
- **Network Handler:** Perform network scans and manage host data.
- **Environment Variables:** Manage configuration through environment variables.
- **Logging:** Set up logging for the application.

## Installation

### Prerequisites

- Rust: Ensure you have Rust installed. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/).
- NATS Server: You need to have a running NATS server instance for message handling.
