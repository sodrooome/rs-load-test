## rs-load-testing

Simple, stupid and sufficient load testing written over Rust (with ~~unsteady~~ minimalistic HTML report)

### Building and installing from source

```bash
$ git clone https://github.com/sodrooome/rs-load-test
$ cd rs-load-test
$ cargo build --release
```

### Minimalist setup

```rs
use rs_load_test::{AllowedRequestMethod, SimpleLoadTest};

#[tokio::main]
async fn main() {
    let load_test = SimpleLoadTest::new(
        2,  // num of workers
        10, // num of concurrent users
        Duration::from_secs(10), // duration for ramp-up to the targeted server
        "https://test-api.k6.io/public/crocodiles/".to_string(), // targeted server URL
        AllowedRequestMethod::GET, // HTTP methods, right now only support GET & POST method
        None // request body as JSON
    );
    load_test.run().await;
}
```

**Notes**: you'll be given the trace log of `task_id` (between 0 and 1) when a request is successfully requested. For now, just ignore this thing (lolz)
