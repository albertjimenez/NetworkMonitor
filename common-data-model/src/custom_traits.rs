use serde::{Deserialize, Serialize};

/// A trait for custom serialization and deserialization.
///
/// Provides methods to convert the implementing type to and from JSON strings.
pub trait CustomSerializer<T> {
    /// Serializes the implementor into a JSON string.
    ///
    /// # Errors
    /// Returns an error if serialization fails.
    fn to_json_string(&self) -> String
    where
        Self: Serialize,
        Self: Sized,
    {
        serde_json::to_string(&self).expect("Error during serialization")
    }
    /// Deserializes an instance of the implementing type from a JSON string.
    ///
    /// # Arguments
    ///
    /// * `data_string` - A string slice containing the JSON representation.
    ///
    /// # Errors
    /// Returns an error if deserialization fails.

    fn from_json_string(data_string: String) -> T
    where
        T: for<'a> Deserialize<'a>,
    {
        serde_json::from_str(&data_string).expect("Error during deserialization")
    }
}

pub trait IMacVendorCache {
    fn new() -> Self;
    fn add(&self, mac_addr: String, vendor_name: String);
    fn contains(&self, mac_addr: &String) -> bool;
    fn get(&self, mac_addr: &String) -> String;
    fn length(&self) -> usize;
}
