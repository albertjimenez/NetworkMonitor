use crate::runtime_settings::RuntimeSettings;
use async_nats::jetstream;
use common_data_model::channel_names::ChannelNames;
use common_data_model::custom_traits::CustomSerializer;
use common_data_model::environment_variables::environment_variables;
use common_data_model::my_logger::init_logger;
use log::info;
use sysinfo::System;
use crate::payload::Payload;

mod unit_converter;
mod cpu_usage;
mod mem_usage;
mod disk_usage;
mod network_usage;
mod temperature_info;
mod runtime_settings;
mod payload;

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let nc_url = environment_variables::common::get_nats_url();
    let nc = async_nats::connect(&nc_url).await.expect("NATS server should be running");
    init_logger();
    let jetstream = jetstream::new(nc);
    let mut runtime_settings = RuntimeSettings::new();
    let mut sys = System::new_all();
    let mut interval = runtime_settings.get_system_monitor_refresh_interval(jetstream.clone()).await?;
    info!("Current interval {}", &interval.as_secs());
    loop {
        // Update all information
        sys.refresh_all();
        let payload = Payload::new(&sys);

        jetstream.publish(ChannelNames::SYSTEM_MONITOR_DATA_WEBSOCKET, payload.to_json_string().into()).await?;
        let duration = runtime_settings.get_system_monitor_refresh_interval(jetstream.clone()).await?;
        if interval != duration {
            info!("New Duration {}", &duration.as_secs());
        }
        interval = duration;
        tokio::time::sleep(interval).await;
    }
}
