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
            Ok(_code) => format!("[OK] {} ({} ms)", self.url, self.response_time.as_millis()),
            Err(e) => format!("[ERR] {} ({})", self.url, e),
        }
    }

    pub fn to_json_object(&self) -> String {
        let status = match &self.action_status {
            Ok(code) => format!(r#""status_code": {}"#, code),
            Err(e) => format!(r#""error": "{}""#, escape_json(e)),
        };

        let response_time_ms = self.response_time.as_millis();
        let timestamp = self
            .timestamp
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        format!(
            r#"{{
    "url": "{}",
    {},
    "response_time_ms": {},
    "timestamp": {}
}}"#,
            escape_json(&self.url),
            status,
            response_time_ms,
            timestamp
        )
    }
}
// kinda bandaid lowkey but i dunno
fn escape_json(text: &str) -> String {
    text.replace('\\', "\\\\").replace('"', "\\\"")
}
