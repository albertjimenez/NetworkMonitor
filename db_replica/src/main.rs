use common_data_model::custom_traits::IMacVendorCache;
use common_data_model::mac_vendor_cache::MacVendorCache;
use common_data_model::my_logger::init_logger;
use dotenv::dotenv;
use crate::custom_traits::DBInterface;

use crate::db_handler::DBHandler;
use crate::queue_listener::QueueListener;

mod db_handler;
mod queue_listener;
pub mod custom_traits;
mod test_custom_traits;

#[tokio::main]
async fn main() {
    init_logger();
    dotenv().ok();
    let db_handler = DBHandler::new(None).await;
    let cache = MacVendorCache::new();
    let queue_listener = QueueListener::new(db_handler, cache).await.expect("NATS server should be running");
    queue_listener.handle_consumers().await.expect("Queue issue, could not connect to nats")
}
