use crate::cpu_usage::CPUUsage;
use crate::disk_usage::DiskUsage;
use crate::mem_usage::MemUsage;
use crate::network_usage::NetworkUsage;
use crate::temperature_info::TemperatureInfo;
use common_data_model::custom_traits::CustomSerializer;
use serde::{Deserialize, Serialize};
use sysinfo::System;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payload {
    pub cpu_usage: CPUUsage,
    pub disk_usage: DiskUsage,
    pub mem_usage: MemUsage,
    pub network_usage: NetworkUsage,
    pub temperature_info: Vec<TemperatureInfo>,
}
impl Payload {
    pub fn new(system: &System) -> Self {
        let cpu_usage = CPUUsage::new(system);
        let disk_usage = DiskUsage::new();
        let mem_usage = MemUsage::new(system);
        let network_usage = NetworkUsage::new();
        let temperature_info = TemperatureInfo::get_temperatures();
        Payload { cpu_usage, disk_usage, mem_usage, network_usage, temperature_info }
    }
}
impl CustomSerializer<Payload> for Payload {}