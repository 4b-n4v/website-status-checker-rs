use reqwest::blocking::Client;
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime};

use crate::cli::CLI;
use crate::status::WebsiteStatus;

pub fn check_website(url: &str, cli_args: &CLI) -> WebsiteStatus {
    let client = Client::builder()
        .timeout(Duration::from_secs(cli_args.timeout_secs))
        .build()
        .expect("Failed to build HTTP client");

    let mut last_err = None;
    let start = Instant::now();

    for attempt in 0..=cli_args.retries {
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
                if attempt < cli_args.retries {
                    sleep(Duration::from_millis(100));
                }
            }
        }
    }

    let response_time = start.elapsed();
    WebsiteStatus {
        url: url.to_string(),
        action_status: Err(last_err.unwrap_or_else(|| "Unknown error".to_string())),
        response_time,
        timestamp: SystemTime::now(),
    }
}
