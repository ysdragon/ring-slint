use arboard::Clipboard;
use std::cell::RefCell;

thread_local! {
	static CLIPBOARD: RefCell<Option<Clipboard>> = const { RefCell::new(None) };
}

fn with_clipboard<F, R>(f: F) -> Result<R, String>
where
	F: FnOnce(&mut Clipboard) -> Result<R, arboard::Error>,
{
	CLIPBOARD.with(|cb| {
		let mut cb = cb.borrow_mut();
		if cb.is_none() {
			*cb = Some(Clipboard::new().map_err(|e| format!("Failed to init clipboard: {}", e))?);
		}
		f(cb.as_mut().unwrap()).map_err(|e| format!("Clipboard error: {}", e))
	})
}

pub fn clipboard_get_text() -> Result<String, String> {
	with_clipboard(|cb| cb.get_text())
}

pub fn clipboard_set_text(text: &str) -> Result<(), String> {
	with_clipboard(|cb| cb.set_text(text.to_string()))
}

pub fn clipboard_clear() -> Result<(), String> {
	with_clipboard(|cb| cb.clear())
}
