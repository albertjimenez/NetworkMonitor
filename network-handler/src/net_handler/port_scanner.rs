use std::net::IpAddr;
use std::thread;
use std::time::Duration;

use common_data_model::data_models::ServicePort;
use netdev::Interface;
use netscan::host::{Host, Port, PortStatus};
use netscan::scan::scanner::PortScanner;
use netscan::scan::setting::{PortScanSetting, PortScanType};

use crate::net_handler::service_db_map::PORT_SERVICE_MAP;

pub struct MyPortScanner {}

impl MyPortScanner {
    /// Scans the specified IP address for open service ports.
    ///
    /// This function performs a TCP SYN scan on the given IP address, targeting
    /// predefined ports. It returns a vector of `ServicePort` objects representing
    /// the open ports and their associated services.
    ///
    /// # Arguments
    ///
    /// * `interface` - A reference to the network interface to use for scanning.
    /// * `ip_addr` - A reference to the IP address to scan.
    ///
    /// # Returns
    ///
    /// A `Vec<ServicePort>` containing details of the open ports and their associated services.
    pub fn scan(interface: &Interface, ip_addr: &IpAddr) -> Vec<ServicePort> {
        let dst_ip = ip_addr.clone();
        let dst: Host = Host::new(dst_ip, String::new()).with_ports(vec![22, 80, 443, 5000, 8080]);
        let scan_setting = PortScanSetting::default()
            .set_if_index(interface.index)
            .set_scan_type(PortScanType::TcpSynScan)
            .add_target(dst)
            .set_timeout(Duration::from_millis(10000))
            .set_wait_time(Duration::from_millis(500))
            .set_send_rate(Duration::from_millis(0));
        let port_scanner = PortScanner::new(scan_setting);

        let rx = port_scanner.get_progress_receiver();
        let handle = thread::spawn(move || { port_scanner.scan() });
        while let Ok(_socket_addr) = rx.lock().unwrap().recv() {}
        let result = handle.join().unwrap();
        result.hosts.into_iter().map(|host| {
            get_service_ports(&host.ports)
        }).flatten().collect()
    }
}
/// Filters open ports and retrieves their associated service names.
///
/// This helper function iterates over the provided vector of ports, filters out closed ports, and
/// returns a vector of `ServicePort` objects for the open ports with their associated service names.
///
/// # Arguments
///
/// * `ports` - A reference to a vector of `Port` objects representing the ports to be filtered.
///
/// # Returns
///
/// A vector of `ServicePort` objects representing the open ports and their associated services.
fn get_service_ports(ports: &Vec<Port>) -> Vec<ServicePort> {
    return ports.iter().filter_map(|port| {
        if port.status != PortStatus::Open {
            return None;
        }
        let name = PORT_SERVICE_MAP.get(&port.number).unwrap_or(&"Unknown").to_string();
        Some(ServicePort::new(port.number as i32, name))
    }).collect();
}