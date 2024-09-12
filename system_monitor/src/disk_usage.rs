use serde::Deserialize;
use serde::Serialize;
use sysinfo::Disks;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskUsage {
    pub available: u64,
    pub total: u64,
}

impl DiskUsage {
    pub fn new() -> Self {
        let available = Self::get_available_disk_usage();
        let total = Self::get_total_disk_usage();
        DiskUsage { available, total }
    }
    pub fn get_available_disk_usage() -> u64 {
        let disks = Disks::new_with_refreshed_list();
        let first = disks.first();
        if first.is_some() {
            return first.unwrap().available_space();
        }
        0
    }
    pub fn get_total_disk_usage() -> u64 {
        let disks = Disks::new_with_refreshed_list();
        let first = disks.first();
        if first.is_some() {
            return first.unwrap().total_space();
        }
        0
    }
}