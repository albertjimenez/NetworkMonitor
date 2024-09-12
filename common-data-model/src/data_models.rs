use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::custom_traits::CustomSerializer;

/// Represents a host discovered on the network.
///
/// This struct contains details such as the host's IP address, MAC address,
/// vendor name, and open ports.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Serialize, Deserialize, FromRow)]
pub struct DiscoveredHost {
    pub id: Option<i32>,
    pub ip: String,
    pub mac_address: String,
    pub vendor_name: String,
    pub discovered_at: DateTime<Utc>,
    pub open_ports: Vec<ServicePort>,
}

impl DiscoveredHost {
    /// Creates a new instance of `DiscoveredHost`.
    ///
    /// # Arguments
    ///
    /// * `ip` - The IP address of the host.
    /// * `mac_address` - The MAC address of the host.
    /// * `vendor_name` - The vendor name associated with the MAC address.
    /// * `open_ports` - A vector of open service ports on the host.
    pub fn new(
        ip: String,
        mac_address: String,
        vendor_name: String,
        open_ports: Vec<ServicePort>,
    ) -> Self {
        let discovered_at = Utc::now();
        DiscoveredHost {
            id: None,
            ip,
            mac_address,
            vendor_name,
            discovered_at,
            open_ports,
        }
    }
}
/// Represents an open service port on a discovered host.
///
/// This struct contains the port number, the name of the service running on that port,
/// and an optional ID which can be used when storing in a database.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Serialize, Deserialize, FromRow)]
pub struct ServicePort {
    pub id: Option<i32>,
    pub port: i32,
    pub name: String,
}

impl ServicePort {
    /// Creates a new instance of `ServicePort` without an ID.
    ///
    /// This constructor is useful for newly discovered service ports that
    /// haven't been persisted to a database yet.
    ///
    /// # Arguments
    ///
    /// * `port` - The port number of the service.
    /// * `name` - The name of the service running on the port.
    ///
    /// # Returns
    /// A new instance of `ServicePort` with the specified port and service name.
    pub fn new(port: i32, name: String) -> Self {
        ServicePort {
            id: None,
            port,
            name,
        }
    }
    /// Creates a new instance of `ServicePort` with a specified ID.
    ///
    /// This constructor is useful for service ports that have been persisted
    /// to a database and have a corresponding ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The database ID of the service port.
    /// * `port` - The port number of the service.
    /// * `name` - The name of the service running on the port.
    ///
    /// # Returns
    /// A new instance of `ServicePort` with the specified ID, port, and service name.
    pub fn new_with_id(id: i32, port: i32, name: String) -> Self {
        ServicePort {
            id: Some(id),
            port,
            name,
        }
    }
}

/// Represents the association between a `ServicePort` and a `DiscoveredHost`.
///
/// This struct is used to link a discovered host with the service ports
/// it has open, typically for database associations.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Serialize, Deserialize, FromRow)]
pub struct ServiceDiscoveredHost {
    pub service_port_id: Option<i32>,
    pub discovered_host_id: Option<i32>,
}

impl ServiceDiscoveredHost {
    /// Creates a new instance of `ServiceDiscoveredHost`.
    ///
    /// This constructor links a service port with a discovered host.
    ///
    /// # Arguments
    ///
    /// * `service_port_id` - The ID of the associated service port.
    /// * `discovered_host_id` - The ID of the associated discovered host.
    ///
    /// # Returns
    /// A new instance of `ServiceDiscoveredHost` with the specified associations.
    pub fn new(service_port_id: Option<i32>, discovered_host_id: Option<i32>) -> Self {
        ServiceDiscoveredHost {
            service_port_id,
            discovered_host_id,
        }
    }
}

impl CustomSerializer<ServiceDiscoveredHost> for ServiceDiscoveredHost {}

impl CustomSerializer<ServicePort> for ServicePort {}

impl CustomSerializer<DiscoveredHost> for DiscoveredHost {}
