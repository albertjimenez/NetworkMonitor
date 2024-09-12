#[cfg(test)]
mod test_custom_traits {
    use common_data_model::data_models::{DiscoveredHost, ServicePort};
    use sqlx::Error;

    use crate::custom_traits::DBInterface;

    pub struct DummyDBHandler {}

    impl DBInterface for DummyDBHandler {
        async fn new(max_conns: Option<u32>) -> Self
        where
            Self: Sized,
        {
            DummyDBHandler {}
        }

        async fn insert_discovered_host(&self, discovered_host: DiscoveredHost) -> Result<Option<i32>, Error> {
            Ok(Some(1))
        }

        async fn find_all_service_ports(&self) -> Result<Vec<ServicePort>, Error> {
            Ok(vec![])
        }

        async fn find_service_port_by_port_number(&self, port_number: i32) -> Result<ServicePort, Error> {
            Ok(ServicePort::new(1, "T".to_owned()))
        }

        async fn get_all_mac_vendors_minimal_info(&self) -> Result<Vec<(String, String)>, Error> {
            Ok(vec![])
        }
    }

    fn create_dummy_db_handler() -> impl DBInterface {
        return DummyDBHandler {};
    }

    #[tokio::test]
    async fn insert_discovered_host() {
        let dummy = create_dummy_db_handler();
        let discovered_host = DiscoveredHost::new("".to_owned(), "".to_owned(), "".to_owned(), vec![]);
        let result = dummy.insert_discovered_host(discovered_host).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.is_some());
        assert_eq!(Some(1), result)
    }

    #[tokio::test]
    async fn find_all_service_ports() {
        let dummy = create_dummy_db_handler();
        let result = dummy.find_all_service_ports().await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.is_empty())
    }

    #[tokio::test]
    async fn find_service_port_by_port_number() {
        let dummy = create_dummy_db_handler();
        let result = dummy.find_service_port_by_port_number(1).await;
        assert!(result.is_ok());
        assert_eq!(1, result.unwrap().port);
    }

    #[tokio::test]
    async fn get_all_mac_vendors_minimal_info() {
        let dummy = create_dummy_db_handler();
        let result = dummy.get_all_mac_vendors_minimal_info().await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }
}