use notify_rust::{Notification, Timeout};

pub fn notify(summary: &str, body: &str) -> Result<(), String> {
	Notification::new()
		.summary(summary)
		.body(body)
		.show()
		.map(|_| ())
		.map_err(|e| format!("Failed to show notification: {}", e))
}

pub fn notify_with_timeout(summary: &str, body: &str, timeout_ms: i32) -> Result<(), String> {
	Notification::new()
		.summary(summary)
		.body(body)
		.timeout(Timeout::Milliseconds(timeout_ms as u32))
		.show()
		.map(|_| ())
		.map_err(|e| format!("Failed to show notification: {}", e))
}

pub fn notify_with_icon(summary: &str, body: &str, icon: &str) -> Result<(), String> {
	Notification::new()
		.summary(summary)
		.body(body)
		.icon(icon)
		.show()
		.map(|_| ())
		.map_err(|e| format!("Failed to show notification: {}", e))
}

pub fn notify_full(summary: &str, body: &str, icon: &str, timeout_ms: i32) -> Result<(), String> {
	Notification::new()
		.summary(summary)
		.body(body)
		.icon(icon)
		.timeout(Timeout::Milliseconds(timeout_ms as u32))
		.show()
		.map(|_| ())
		.map_err(|e| format!("Failed to show notification: {}", e))
}
