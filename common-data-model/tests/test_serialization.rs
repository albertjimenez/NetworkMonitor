use common_data_model::custom_traits::CustomSerializer;
use common_data_model::data_models::{DiscoveredHost, ServiceDiscoveredHost, ServicePort};

#[test]
fn test_serialization() {
    let service_port = ServicePort::new(1, "port".to_owned());
    let string_service_port = r#"{"id":null,"port":1,"name":"port"}"#;
    assert_eq!(service_port.to_json_string(), string_service_port);
    assert_eq!(
        &ServicePort::from_json_string(string_service_port.to_owned()),
        &service_port
    );

    let discovered_host = DiscoveredHost::new(
        "127.0.0.1".to_owned(),
        "MAC".to_owned(),
        "Amazon".to_owned(),
        vec![service_port.clone()],
    );
    let string_discovered_host = r#"{"id":null,"ip":"127.0.0.1","mac_address":"MAC","vendor_name":"Amazon","discovered_at":"2024-08-29T16:18:01.795046Z","open_ports":[{"id":null,"port":1,"name":"port"}]}"#;
    assert_eq!(
        discovered_host.to_json_string().len(),
        string_discovered_host.len()
    );
    assert_eq!(
        DiscoveredHost::from_json_string(string_discovered_host.to_owned()).vendor_name,
        discovered_host.vendor_name
    );

    let service_discovered_host = ServiceDiscoveredHost::new(Some(1), Some(2));
    let string_service_discovered_host = r#"{"service_port_id":1,"discovered_host_id":2}"#;
    assert_eq!(
        service_discovered_host.to_json_string(),
        string_service_discovered_host
    );
    assert_eq!(
        ServiceDiscoveredHost::from_json_string(string_service_discovered_host.to_owned()),
        service_discovered_host
    );
}
