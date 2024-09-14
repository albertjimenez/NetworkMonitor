use async_nats::jetstream;
use common_data_model::channel_names::ChannelNames;
use common_data_model::custom_traits::{CustomSerializer, IMacVendorCache};
use common_data_model::mac_vendor_cache::MacVendorCache;
use common_data_model::my_logger::init_logger;
use futures::StreamExt;
use log::{debug, info};
use netdev::get_default_interface;

use crate::net_handler::host_scanner::HostScanner;
use crate::net_handler::runtime_settings::RuntimeSettings;
use common_data_model::environment_variables::environment_variables;

pub mod net_handler;

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    rustls::crypto::ring::default_provider().install_default().expect("Failed to install rustls crypto provider");
    let nc_url = environment_variables::common::get_nats_url();
    let nc = async_nats::connect(&nc_url).await?;
    init_logger();
    let interface = get_default_interface().unwrap();
    let jetstream = jetstream::new(nc);
    let mut runtime_settings = RuntimeSettings::new();
    let cache = MacVendorCache::new();
    let host_scanner = HostScanner::new(cache);
    let mut interval = runtime_settings.get_host_scan_interval(jetstream.clone()).await?;
    info!("Current interval {}", &interval.as_secs());
    loop {
        let mut host_stream = host_scanner.scan(&interface);
        while let Some(discovered_host) = host_stream.next().await {
            debug!("Device found {:?}", &discovered_host);
            jetstream.publish(ChannelNames::MONITORING_DATA_WEBSOCKET, discovered_host.to_json_string().into()).await?;
        }
        let duration = runtime_settings.get_host_scan_interval(jetstream.clone()).await?;
        info!("Duration {}", &duration.as_secs());
        interval = duration;
        tokio::time::sleep(interval).await;
    }
}


