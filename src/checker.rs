use crate::status::WebsiteStatus;
use reqwest::blocking::Client;
use std::time::{Duration, Instant, SystemTime};

pub fn check_website(url: &str, timeout_secs: u64, retries: u32) -> WebsiteStatus {
    let client = Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .build()
        .expect("Failed to build HTTP client");

    let mut last_err = None;
    let start = Instant::now();

    for attempt in 0..=retries {
        match client.get(url).send() {
            Ok(resp) => {
                let code = resp.status().as_u16();
                let response_time = start.elapsed();
                return WebsiteStatus {
                    url: url.to_string(),
                    action_status: Ok(code),
                    response_time,
                    timestamp: SystemTime::now(),
                };
            }
            Err(e) => {
                last_err = Some(e.to_string());
                if attempt < retries {
                    std::thread::sleep(Duration::from_millis(100));
                }
            }
        }
    }
    WebsiteStatus {
        url: url.to_string(),
        action_status: Err(last_err.unwrap_or_else(|| "Unknown error".into())),
        response_time: start.elapsed(),
        timestamp: SystemTime::now(),
    }
}
