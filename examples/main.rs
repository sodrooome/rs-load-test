use std::time::Duration;

use rs_load_test::{AllowedRequestMethod, SimpleLoadTest};

#[tokio::main]
async fn main() {
    let load_test = SimpleLoadTest::new(
        2,
        10,
        Duration::from_secs(10),
        "https://test-api.k6.io/public/crocodiles/".to_string(),
        AllowedRequestMethod::GET,
        None
    );
    load_test.run().await;
    // tokio::runtime::Runtime::new().unwrap().block_on(load_test.run());
}
