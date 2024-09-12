use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use common_data_model::custom_traits::IMacVendorCache;
use common_data_model::data_models::DiscoveredHost;
use futures::stream::Stream;
use ipnet::Ipv4Net;
use log::{debug, info};
use netdev::Interface;
use netscan::host::Host;
use netscan::scan::setting::{HostScanSetting, HostScanType};
use tokio::sync::mpsc;
use tokio::task;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::net_handler::port_scanner::MyPortScanner;

pub struct HostScanner<T: IMacVendorCache> {
    mac_vendor_cache: Arc<Mutex<T>>,
}

impl<T: IMacVendorCache + Send + 'static> HostScanner<T> {
    /// Creates a new `HostScanner` instance.
    ///
    /// # Arguments
    ///
    /// * `cache` - An instance of a type that implements the `IMacVendorCache` trait, used for caching MAC address vendor information.
    ///
    /// # Returns
    ///
    /// A `HostScanner` instance that can be used to scan a network interface for active hosts.
    pub fn new(cache: T) -> Self {
        HostScanner {
            mac_vendor_cache: Arc::new(Mutex::new(cache)),
        }
    }
    /// Retrieves the vendor name for a given MAC address.
    ///
    /// This asynchronous function checks the provided MAC address in the cache.
    /// If not found, it fetches the vendor name from the `macvendors.com` API and caches the result.
    ///
    /// # Arguments
    ///
    /// * `mac_address` - A string slice representing the MAC address.
    /// * `mac_vendor_cache` - An `Arc<Mutex<T>>` where `T` implements the `IMacVendorCache` trait, representing the cache.
    ///
    /// # Returns
    ///
    /// A `String` containing the vendor name associated with the given MAC address. Returns `"Unknown"` if the vendor cannot be determined.
    pub fn scan(&self, interface: &Interface) -> impl Stream<Item=DiscoveredHost> {
        let (tx, rx) = mpsc::unbounded_channel();
        let mac_vendor_cache = Arc::clone(&self.mac_vendor_cache);

        let interface_clone = interface.clone();
        task::spawn(async move {
            info!("Starting host scanner");

            let mut scan_setting: HostScanSetting = HostScanSetting::default()
                .set_if_index(interface_clone.index)
                .set_scan_type(HostScanType::IcmpPingScan)
                .set_timeout(Duration::from_millis(10000))
                .set_wait_time(Duration::from_millis(500))
                .set_async_scan(true);

            let src_ip: Ipv4Addr = interface_clone.ipv4[0].addr;
            let net: Ipv4Net = Ipv4Net::new(src_ip, 24).unwrap();
            let nw_addr = Ipv4Net::new(net.network(), 24).unwrap();
            let hosts: Vec<Ipv4Addr> = nw_addr.hosts().collect();

            for host in hosts {
                let dst: Host = Host::new(IpAddr::V4(host), String::new());
                scan_setting.add_target(dst);
            }

            let host_scanner: netscan::scan::scanner::HostScanner = netscan::scan::scanner::HostScanner::new(scan_setting);
            let rx = host_scanner.get_progress_receiver();

            let handle = task::spawn_blocking(move || host_scanner.scan());
            while let Ok(_socket_addr) = rx.lock().unwrap().recv() {}

            let result = handle.await.unwrap();
            info!("Status: {:?}", result.scan_status);

            for host in result.hosts {
                let ip = host.ip_addr;
                let mac_addr = host.mac_addr.to_string();
                let service_ports = MyPortScanner::scan(&interface_clone, &ip);

                let vendor = Self::get_mac_vendor(&mac_addr, mac_vendor_cache.clone()).await;

                debug!(
                    "IP: {:?} | MAC: {} | Vendor: {} | Service Ports: {:?}",
                    &ip, &mac_addr, &vendor, &service_ports
                );

                let discovered_host = DiscoveredHost::new(ip.to_string(), mac_addr, vendor, service_ports);
                if let Err(_err) = tx.send(discovered_host) {
                    break;
                }
            }
        });

        UnboundedReceiverStream::new(rx)
    }

    async fn get_mac_vendor(mac_address: &str, mac_vendor_cache: Arc<Mutex<T>>) -> String {
        let mac_addr = &mac_address.to_string();
        {
            let cache = mac_vendor_cache.lock().unwrap();
            if cache.contains(mac_addr) {
                debug!("Cached vendor found");
                return cache.get(mac_addr);
            }
        }

        let url = format!("https://api.macvendors.com/{}", mac_address);
        let response = ureq::get(&url).call();
        let unknown_text = "Unknown".to_owned();
        if response.is_ok() {
            let vendor = response.unwrap().into_string().unwrap_or(unknown_text.clone());
            {
                let cache = mac_vendor_cache.lock().unwrap();
                cache.add(mac_addr.to_owned(), vendor.clone());
            }
            return vendor;
        }
        unknown_text
    }
}