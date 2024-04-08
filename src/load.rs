use reqwest::{Client, Error, Response};
use serde_json::Value;
// use serde::{Deserialize, Serialize};
// use serde_json;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tokio::time::sleep;

use crate::report;

#[derive(Clone)]
pub enum AllowedRequestMethod {
    GET,
    POST,
}

// #[derive(Serialize, Deserialize)]
pub struct SimpleLoadTest {
    num_of_workers: usize,
    num_of_users: usize,
    duration: Duration,
    target_url: String,
    http_method: AllowedRequestMethod,
    json: Option<Value>,
    http_client: Client,
}

impl SimpleLoadTest {
    pub fn new(
        num_of_workers: usize,
        num_of_users: usize,
        duration: Duration,
        url: String,
        http_method: AllowedRequestMethod,
        json: Option<Value>,
    ) -> Self {
        SimpleLoadTest {
            num_of_workers,
            num_of_users,
            duration,
            target_url: url,
            http_method,
            http_client: Client::new(),
            json,
        }
    }

    pub async fn run(self) {
        let semaphore = Arc::new(Semaphore::new(self.num_of_users));
        let num_of_workers = self.num_of_workers;
        let start_time = Instant::now();
        let mut handles = vec![];
        let total_requests = Arc::new(tokio::sync::Mutex::new(0));
        // let request_method = self.http_method.clone();

        // design choice please read more detail
        // on this documentation
        // https://docs.rs/tokio/latest/tokio/sync/struct.Semaphore.html#limit-the-number-of-outgoing-requests-being-sent-at-the-same-time
        for task_id in 0..num_of_workers {
            let semaphore = semaphore.clone();
            let request_url = self.target_url.clone();
            let request_method = self.http_method.clone();
            let total_requests = total_requests.clone();
            let json_payload = self.json.clone();

            let handle = tokio::spawn(async move {
                while Instant::now() - start_time < self.duration {
                    let _permit = semaphore
                        .acquire()
                        .await
                        .expect("unable to acquire the permit before sending request");

                    drop(_permit);

                    match request_method {
                        AllowedRequestMethod::GET => {
                            match SimpleLoadTest::send_get_request(&request_url).await {
                                Ok(_response) => {
                                    println!("Success sending a request: {:?}", task_id)
                                }
                                Err(_err) => println!("Error when sending a request"),
                            }
                        }
                        AllowedRequestMethod::POST => {
                            match SimpleLoadTest::send_post_request(&request_url, &json_payload)
                                .await
                            {
                                Ok(_response) => {
                                    println!("Success sending a request: {:?}", task_id)
                                }
                                Err(_err) => println!("Error when sending a request"),
                            }
                        }
                    }

                    *total_requests.lock().await += 1;
                    sleep(Duration::from_millis(100)).await;
                }
            });

            handles.push(handle);
        }

        let mut responses = Vec::new();
        for handle in handles {
            let response = handle.await.unwrap();
            responses.push(response);
            // handle.await.expect("Something went wrong");
        }

        let elapsed_time = self.duration.as_secs_f64();
        let total_requests = *total_requests.lock().await;
        let rps = total_requests as f64 / elapsed_time;
        println!("Total requests: {:?}", total_requests);
        println!("RPS: {:.2}/secs", rps);
        println!("Receiving the responses: {:?}", responses);

        let generate_html = report::generate_report_as_html(total_requests, rps);
        drop(generate_html);
        // println!("{}", generate_html);
    }

    async fn send_get_request(url: &str) -> Result<Response, Error> {
        Client::new().get(url).send().await
    }

    async fn send_post_request(url: &str, json: &Option<Value>) -> Result<Response, Error> {
        Client::new().post(url).json(json).send().await
    }
}
