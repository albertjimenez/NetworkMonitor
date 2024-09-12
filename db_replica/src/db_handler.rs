use common_data_model::data_models::{DiscoveredHost, ServicePort};
use common_data_model::environment_variables::environment_variables;
use futures::future::join_all;
use sqlx::{Error, PgPool, Row};
use sqlx::postgres::PgPoolOptions;

use crate::custom_traits::DBInterface;

/// Manages database interactions for discovered hosts and service ports.
///
/// This struct implements the `DBInterface` trait and provides methods to insert discovered hosts,
/// process service ports, and manage many-to-many relationships between services and hosts.
#[derive(Debug)]
pub struct DBHandler {
    pool_connection: PgPool,
}


impl DBHandler {
    const DEFAULT_MAX_CONNS: u8 = 5;
    /// Processes and inserts service ports for a discovered host.
    ///
    /// # Parameters
    ///
    /// * `discovered_host` - The `DiscoveredHost` object containing service ports to be processed.
    ///
    /// # Returns
    ///
    /// An asynchronous future that resolves to a vector of `ServicePort` objects.
    async fn process_service_ports(&self, discovered_host: &DiscoveredHost) -> Result<Vec<ServicePort>, Error> {
        let futures = discovered_host.open_ports.iter().map(|a_port| {
            let port_clone = a_port.clone();
            async move {
                match self.insert_service_port(port_clone.port, &port_clone.name).await {
                    Ok(Some(inserted_id)) => ServicePort::new_with_id(inserted_id, port_clone.port, port_clone.name),
                    Ok(None) => self.find_service_port_by_port_number(port_clone.port).await.unwrap(),
                    Err(_) => panic!("Failed to insert or find service port"),
                }
            }
        });
        Ok(join_all(futures).await)
    }
    /// Inserts a discovered host into the database.
    ///
    /// # Parameters
    ///
    /// * `discovered_host` - The `DiscoveredHost` object to be inserted.
    ///
    /// # Returns
    ///
    /// An asynchronous future that resolves to the ID of the inserted host, if successful.
    async fn insert_host(&self, discovered_host: &DiscoveredHost) -> Result<Option<i32>, Error> {
        let query = r#"
            INSERT INTO discovered_hosts (ip, mac_address, vendor_name, discovered_at)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT DO NOTHING
            RETURNING id
        "#;
        sqlx::query_as::<_, (i32,)>(query)
            .bind(&discovered_host.ip)
            .bind(&discovered_host.mac_address)
            .bind(&discovered_host.vendor_name)
            .bind(&discovered_host.discovered_at)
            .fetch_optional(&self.pool_connection)
            .await
            .map(|opt| opt.map(|row| row.0))
    }
    /// Links a host with its service ports in the database.
    ///
    /// # Parameters
    ///
    /// * `host_id` - The ID of the host to be linked.
    /// * `service_ports` - A slice of `ServicePort` objects to be linked with the host.
    ///
    /// # Returns
    ///
    /// An asynchronous future that resolves to an empty result if successful.
    async fn link_host_with_ports(&self, host_id: i32, service_ports: &[ServicePort]) -> Result<(), Error> {
        let futures = service_ports.iter().map(|a_port| async {
            self.insert_many_to_many_services_hosts(a_port.id, Some(host_id)).await
        });
        join_all(futures).await.into_iter().collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }
    /// Inserts a many-to-many relationship between a service port and a discovered host.
    ///
    /// # Parameters
    ///
    /// * `service_port_id` - The ID of the service port.
    /// * `discovered_host_id` - The ID of the discovered host.
    ///
    /// # Returns
    ///
    /// An asynchronous future that resolves to an empty result if successful.
    async fn insert_many_to_many_services_hosts(&self, service_port_id: Option<i32>, discovered_host_id: Option<i32>) -> Result<(), Error> {
        let query = r#"
            INSERT INTO services_discovered (service_port_id, discovered_host_id)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
        "#;
        sqlx::query(query)
            .bind(service_port_id)
            .bind(discovered_host_id)
            .execute(&self.pool_connection)
            .await
            .map(|_| ())
    }
    /// Inserts a service port into the database.
    ///
    /// # Parameters
    ///
    /// * `port` - The port number of the service port.
    /// * `name` - The name of the service port.
    ///
    /// # Returns
    ///
    /// An asynchronous future that resolves to the ID of the inserted service port, if successful.
    async fn insert_service_port(&self, port: i32, name: &str) -> Result<Option<i32>, Error> {
        let query = r#"
            INSERT INTO service_ports (port, name)
            SELECT $1, $2
            ON CONFLICT DO NOTHING
            RETURNING id
        "#;
        sqlx::query_as::<_, (i32,)>(query)
            .bind(port)
            .bind(name)
            .fetch_optional(&self.pool_connection)
            .await
            .map(|opt| opt.map(|row| row.0))
    }
}

impl DBInterface for DBHandler {
    async fn new(max_conns: Option<u32>) -> Self
    where
        Self: Sized,
    {
        // dotenv().ok();
        // REVIEW: the dot file might clash with an environment variable and I moved it to main
        let database_url = environment_variables::database::get_db_url();
        let connections = max_conns.unwrap_or(Self::DEFAULT_MAX_CONNS as u32);
        let pool_connection = PgPoolOptions::new()
            .max_connections(connections)
            .connect(&database_url)
            .await.expect("Could not connect to the DB, review the values");
        DBHandler { pool_connection }
    }

    async fn insert_discovered_host(&self, discovered_host: DiscoveredHost) -> Result<Option<i32>, Error> {
        let service_ports = self.process_service_ports(&discovered_host).await?;
        let host_id = self.insert_host(&discovered_host).await?;

        if let Some(id) = host_id {
            self.link_host_with_ports(id, &service_ports).await?;
        }

        Ok(host_id)
    }

    async fn find_all_service_ports(&self) -> Result<Vec<ServicePort>, Error> {
        sqlx::query_as!(ServicePort, "SELECT id, port, name FROM service_ports")
            .fetch_all(&self.pool_connection)
            .await
    }

    async fn find_service_port_by_port_number(&self, port_number: i32) -> Result<ServicePort, Error> {
        sqlx::query_as!(ServicePort, "SELECT * FROM service_ports WHERE port = $1", port_number)
            .fetch_one(&self.pool_connection)
            .await
    }
    async fn get_all_mac_vendors_minimal_info(&self) -> Result<Vec<(String, String)>, Error> {
        let query = r#"
        SELECT * FROM discovered_hosts
        "#;
        return sqlx::query(query).fetch_all(&self.pool_connection).await.map(|all_rows| {
            all_rows.iter().map(|row| {
                let mac_address: String = row.get(2);
                let vendor_name: String = row.get(3);
                return (mac_address, vendor_name);
            }).collect::<Vec<(String, String)>>()
        });
    }
}
