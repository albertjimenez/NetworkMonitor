pub struct ChannelNames {}

impl ChannelNames {
    pub const MONITORING_DATA_WEBSOCKET: &'static str = "monitoring.data";
    pub const SYSTEM_MONITOR_DATA_WEBSOCKET: &'static str = "system_monitor.data";
    pub const SETTINGS_BUCKET: &'static str = "settings";
    pub const SETTINGS_HOST_SCAN_INTERVAL_NAME: &'static str = "host_scan.interval";
    pub const SETTINGS_SYSTEM_MONITOR_REFRESH_INTERVAL_NAME: &'static str = "system_monitor.refresh_time";
}
