use serde::Deserialize;
use serde::Serialize;
use sysinfo::Networks;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkUsage {
    pub name: String,
    pub tx: u64,
    pub rx: u64,
}

impl NetworkUsage {
    pub fn new() -> Self {
        let networks = Networks::new_with_refreshed_list();
        let first_active_network = networks.into_iter().find(|(_, net)| net.ip_networks().len() == 2);
        if first_active_network.is_some() {
            let first_active_network = first_active_network.unwrap();
            let tx = first_active_network.1.total_transmitted();
            let rx = first_active_network.1.total_received();
            let name = first_active_network.0.to_owned();
            return NetworkUsage { name, tx, rx };
        }
        NetworkUsage { name: "".to_owned(), tx: 0, rx: 0 }
    }
}