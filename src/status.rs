use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct WebsiteStatus {
    pub url: String,
    pub action_status: Result<u16, String>, // HTTP code or error text
    pub response_time: Duration,
    pub timestamp: SystemTime,
}

impl WebsiteStatus {
    pub fn human_readable(&self) -> String {
        match &self.action_status {
            Ok(code) => format!("[OK] {} ({} ms)", self.url, self.response_time.as_millis()),
            Err(e) => format!("[ERR] {} ({})", self.url, e),
        }
    }
}
