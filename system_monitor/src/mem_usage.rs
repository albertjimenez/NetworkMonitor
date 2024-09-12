use serde::Deserialize;
use serde::Serialize;
use sysinfo::System;

use crate::unit_converter::UnitConverter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemUsage {
    pub used: u64,
    pub total: u64,
}

type Used = u64;
type Total = u64;

impl MemUsage {
    pub fn new(system: &System) -> Self {
        let used: Used = system.used_memory();
        let total: Total = system.total_memory();
        MemUsage { used, total }
    }
    pub fn get_mem_used_human_formatted(system: &System) -> String {
        let used = Self::new(system).used;
        UnitConverter::bytes_to_human_readable(used)
    }
    pub fn get_mem_total_human_formatted(system: &System) -> String {
        let used = Self::new(system).total;
        UnitConverter::bytes_to_human_readable(used)
    }
}