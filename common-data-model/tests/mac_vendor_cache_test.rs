use std::borrow::ToOwned;

use common_data_model::custom_traits::IMacVendorCache;
use common_data_model::mac_vendor_cache::MacVendorCache;

struct FakeDataRepository<T: IMacVendorCache> {
    mac_vendor_cache: T,
}

impl<T: IMacVendorCache> FakeDataRepository<T> {
    pub fn new(dependency_function: fn() -> T) -> Self {
        let dependency = dependency_function();
        FakeDataRepository {
            mac_vendor_cache: dependency,
        }
    }
    pub fn get_dummy_data(&self) -> Vec<(String, String)> {
        vec![
            ("MAC1".to_owned(), "Google".to_owned()),
            ("MAC2".to_owned(), "Apple".to_owned()),
            ("MAC3".to_owned(), "Microsoft".to_owned()),
        ]
    }
    pub fn get_cache(self) -> T {
        self.mac_vendor_cache
    }
}

#[test]
fn test_real_implementation() {
    let injection = || MacVendorCache::new();
    let repository = FakeDataRepository::new(injection);
    let data = repository.get_dummy_data();
    let cloned_data = data.clone();
    let expected_size = data.len();
    let cache = repository.get_cache();
    assert_eq!(cache.length(), 0);
    for (index, (mac, vendor)) in data.into_iter().enumerate() {
        cache.add(mac, vendor);
        assert_eq!(cache.length(), index + 1);
    }
    assert_eq!(expected_size, cache.length());
    for (mac, vendor) in cloned_data {
        assert!(cache.contains(&mac));
        let actual_vendor = cache.get(&mac);
        assert_eq!(actual_vendor, vendor)
    }
}
