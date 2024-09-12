use std::str::from_utf8;
use std::time::Duration;

use async_nats::jetstream::Context;
use async_nats::jetstream::kv::Config;
use common_data_model::channel_names::ChannelNames;
use common_data_model::runtime_properties::RuntimeProperties;
use log::warn;

pub struct RuntimeSettings {
    scan_interval: u64,
}

impl RuntimeSettings {
    pub const DEFAULT_SCAN_INTERVAL: u64 = RuntimeProperties::DEFAULT_SCAN_INTERVAL;
    /// Creates a new `RuntimeSettings` instance with the default scan interval.
    ///
    /// # Returns
    ///
    /// A `RuntimeSettings` instance initialized with the default scan interval.
    pub fn new() -> Self {
        RuntimeSettings { scan_interval: Self::DEFAULT_SCAN_INTERVAL }
    }
    /// Sets the host scan interval in seconds.
    ///
    /// This function updates the scan interval with the provided value. If the value
    /// is too low (10 seconds or less), a warning is logged.
    ///
    /// # Arguments
    ///
    /// * `secs` - The scan interval in seconds.
    fn set_interval(&mut self, secs: u64) {
        if secs <= 10 {
            warn!("Value {} too low, may not be enough to perform a full scan", &secs);
        }
        self.scan_interval = secs;
    }
    /// Retrieves the host scan interval from the NATS JetStream key-value store.
    ///
    /// This asynchronous function attempts to fetch the scan interval from the specified
    /// JetStream key-value bucket. If not found, it uses the default interval. The retrieved
    /// interval is then used to update the runtime settings.
    ///
    /// # Arguments
    ///
    /// * `jetstream` - A `Context` representing the JetStream context to interact with the key-value store.
    ///
    /// # Returns
    ///
    /// A `Result<Duration, async_nats::Error>` containing the scan interval duration if successful, or an error if something went wrong.
    pub async fn get_host_scan_interval(&mut self, jetstream: Context) -> Result<Duration, async_nats::Error> {
        let mut kv = jetstream
            .get_key_value(ChannelNames::SETTINGS_BUCKET.to_string())
            .await;
        if kv.is_err() {
            kv = Ok(jetstream
                .create_key_value(Config {
                    bucket: ChannelNames::SETTINGS_BUCKET.to_string(),
                    ..Default::default()
                }).await.unwrap());
        }
        let kv = kv.unwrap();
        let entry = kv.entry(ChannelNames::SETTINGS_HOST_SCAN_INTERVAL_NAME).await?;
        if let Some(entry) = entry {
            let value_from_bucket = from_utf8(&entry.value)?.parse().unwrap_or(Self::DEFAULT_SCAN_INTERVAL);
            self.set_interval(value_from_bucket);
            return Ok(Duration::from_secs(value_from_bucket));
        }
        Ok(Duration::from_secs(self.scan_interval))
    }
}