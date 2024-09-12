use std::future::Future;

use common_data_model::data_models::{DiscoveredHost, ServicePort};
use sqlx::Error;

/// Defines the interface for database operations related to discovered hosts and service ports.
///
/// This trait provides asynchronous methods to interact with the database, including
/// inserting discovered hosts, finding service ports, and retrieving MAC vendor information.

pub trait DBInterface {
    /// Creates a new instance of the database interface.
    ///
    /// # Parameters
    ///
    /// * `max_conns` - An optional maximum number of connections to the database.
    ///
    /// # Returns
    ///
    /// An asynchronous future that resolves to an instance of the database interface.
    fn new(max_conns: Option<u32>) -> impl Future<Output=Self> + Send
    where
        Self: Sized;
    /// Inserts a discovered host into the database.
    ///
    /// # Parameters
    ///
    /// * `discovered_host` - The `DiscoveredHost` object to be inserted.
    ///
    /// # Returns
    ///
    /// An asynchronous future that resolves to the ID of the inserted host, if successful.
    fn insert_discovered_host(&self, discovered_host: DiscoveredHost) -> impl Future<Output=Result<Option<i32>, Error>> + Send;
    /// Retrieves all service ports from the database.
    ///
    /// # Returns
    ///
    /// An asynchronous future that resolves to a vector of `ServicePort` objects.
    fn find_all_service_ports(&self) -> impl Future<Output=Result<Vec<ServicePort>, Error>> + Send;
    /// Finds a service port by its port number.
    ///
    /// # Parameters
    ///
    /// * `port_number` - The port number of the service port to be retrieved.
    ///
    /// # Returns
    ///
    /// An asynchronous future that resolves to the `ServicePort` object if found.
    fn find_service_port_by_port_number(&self, port_number: i32) -> impl Future<Output=Result<ServicePort, Error>> + Send;
    /// Retrieves all MAC vendors and their minimal information from the database.
    ///
    /// # Returns
    ///
    /// An asynchronous future that resolves to a vector of tuples containing MAC address and vendor name.
    fn get_all_mac_vendors_minimal_info(&self) -> impl Future<Output=Result<Vec<(String, String)>, Error>> + Send;
}
