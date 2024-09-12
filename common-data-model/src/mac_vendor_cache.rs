use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::custom_traits::IMacVendorCache;

type Vendor = String;
type MacAddress = String;

/// A thread-safe cache for storing and retrieving MAC addresses and their associated vendor names.
///
/// The `MacVendorCache` struct is designed to store mappings between MAC addresses and vendor names
/// in a thread-safe manner using an `Arc<Mutex<HashMap<MacAddress, Vendor>>>`. This ensures that
/// the cache can be accessed and modified safely across multiple threads.
#[derive(Clone)]
pub struct MacVendorCache {
    cache: Arc<Mutex<HashMap<MacAddress, Vendor>>>,
}

impl IMacVendorCache for MacVendorCache {
    /// Creates a new instance of `MacVendorCache`.
    ///
    /// The cache is initialized as an empty `HashMap` protected by a `Mutex`
    /// to allow safe concurrent access.
    ///
    /// # Returns
    /// A new, empty `MacVendorCache`.
    fn new() -> Self {
        MacVendorCache {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    /// Adds a new MAC address and its associated vendor name to the cache.
    ///
    /// If the MAC address already exists in the cache, its associated vendor name
    /// will be updated.
    ///
    /// # Arguments
    ///
    /// * `mac_addr` - The MAC address to be added to the cache.
    /// * `vendor_name` - The name of the vendor associated with the MAC address.
    fn add(&self, mac_addr: MacAddress, vendor_name: Vendor) {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(mac_addr, vendor_name);
    }
    /// Checks if a given MAC address exists in the cache.
    ///
    /// # Arguments
    ///
    /// * `mac_addr` - The MAC address to check in the cache.
    ///
    /// # Returns
    /// `true` if the MAC address is found in the cache, `false` otherwise.

    fn contains(&self, mac_addr: &MacAddress) -> bool {
        let cache = self.cache.lock().unwrap();
        cache.contains_key(mac_addr)
    }
    /// Retrieves the vendor name associated with a given MAC address.
    ///
    /// If the MAC address is not found in the cache, the function returns `"Unknown"`.
    ///
    /// # Arguments
    ///
    /// * `mac_addr` - The MAC address whose associated vendor name is to be retrieved.
    ///
    /// # Returns
    /// The vendor name associated with the MAC address, or `"Unknown"` if the MAC address
    /// is not found in the cache.
    fn get(&self, mac_addr: &MacAddress) -> String {
        let cache = self.cache.lock().unwrap();
        cache
            .get(mac_addr)
            .unwrap_or(&"Unknown".to_string())
            .to_owned()
    }
    /// Returns the number of entries in the cache.
    ///
    /// This method returns the total count of MAC address to vendor name mappings
    /// currently stored in the cache.
    ///
    /// # Returns
    /// The number of entries in the cache.
    fn length(&self) -> usize {
        self.cache.lock().unwrap().len()
    }
}
