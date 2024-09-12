use serde::Deserialize;
use serde::Serialize;
use sysinfo::System;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPUUsage {
    usage: f32,
}

impl CPUUsage {
    pub fn get_mean_cpu_usage(system: &System) -> f32 {
        (system.global_cpu_usage() * 100f32).round() / 100f32
    }
    pub fn new(system: &System) -> Self {
        let usage = Self::get_mean_cpu_usage(system);
        CPUUsage { usage }
    }
}