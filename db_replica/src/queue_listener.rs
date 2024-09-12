use std::str::from_utf8;
use std::sync::{Arc, Mutex};

use async_nats::{Client, connect};
use common_data_model::channel_names::ChannelNames;
use common_data_model::custom_traits::{CustomSerializer, IMacVendorCache};
use common_data_model::data_models::DiscoveredHost;
use common_data_model::environment_variables::environment_variables;
use futures::StreamExt;
use log::info;

use crate::custom_traits::DBInterface;
/// Listens for messages from a NATS JetStream queue and processes them to insert into the database.
///
/// This struct handles the connection to the NATS server, initializes the MAC vendor cache,
/// and processes incoming messages to update the database with discovered hosts.
pub struct QueueListener<T: DBInterface, M: IMacVendorCache> {
    nc: Client,
    db_handler: T,
    mac_vendor_cache: Arc<Mutex<M>>,
}

impl<T: DBInterface, M: IMacVendorCache> QueueListener<T, M> {
    /// Creates a new instance of `QueueListener`.
    ///
    /// # Parameters
    ///
    /// * `db_handler` - The database handler implementing `DBInterface`.
    /// * `cache` - The MAC vendor cache implementing `IMacVendorCache`.
    ///
    /// # Returns
    ///
    /// An asynchronous future that resolves to an instance of `QueueListener`.
    pub async fn new(db_handler: T, cache: M) -> Result<Self, async_nats::Error> {
        let nc_url = environment_variables::common::get_nats_url();
        let nc = connect(&nc_url).await?;
        let all_vendors = db_handler.get_all_mac_vendors_minimal_info().await.unwrap();
        all_vendors.into_iter().for_each(|(mac, vendor)|{
            cache.add(mac, vendor)
        });
        info!("Cache init with {} elements", &cache.length());
        let mac_vendor_cache = Arc::new(Mutex::new(cache));
        Ok(QueueListener { nc, db_handler, mac_vendor_cache })
    }
    /// Handles incoming messages from the NATS queue and processes them.
    ///
    /// # Returns
    ///
    /// An asynchronous future that resolves to an empty result if successful.
    pub async fn handle_consumers(&self) -> Result<(), async_nats::Error> {
        let mut subscription = self.nc.subscribe(ChannelNames::MONITORING_DATA_WEBSOCKET).await?;
        while let Some(message) = subscription.next().await {
            let payload_str = from_utf8(&*message.payload)?;
            let discovered_host = DiscoveredHost::from_json_string(payload_str.to_owned());
            let local_cache = self.mac_vendor_cache.lock().unwrap();
            if local_cache.contains(&discovered_host.mac_address) {
                continue;
            } else {
                let cloned_host = discovered_host.clone();
                local_cache.add(cloned_host.mac_address, cloned_host.vendor_name);
            }
            let result = self.db_handler.insert_discovered_host(discovered_host).await?;
            if result.is_some() {
                info!("Added new row with id={} to the DB", result.unwrap());
            }
        }
        Ok(())
    }
}