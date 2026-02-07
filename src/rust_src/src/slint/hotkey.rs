use crate::slint::RingVmGuard;
use global_hotkey::{
	GlobalHotKeyEvent, GlobalHotKeyManager,
	hotkey::{Code, HotKey, Modifiers},
};
use ring_lang_rs::{RingVM, ring_vm_callfunction_str};
use std::cell::RefCell;
use std::collections::HashMap;
use std::str::FromStr;

thread_local! {
	static HOTKEY_MANAGER: RefCell<Option<GlobalHotKeyManager>> = const { RefCell::new(None) };
	static HOTKEY_CALLBACKS: RefCell<HashMap<u32, (RingVM, String)>> = RefCell::new(HashMap::new());
	static REGISTERED_HOTKEYS: RefCell<HashMap<u32, HotKey>> = RefCell::new(HashMap::new());
}

fn ensure_manager() -> Result<(), String> {
	HOTKEY_MANAGER.with(|m| {
		let mut m = m.borrow_mut();
		if m.is_none() {
			*m = Some(
				GlobalHotKeyManager::new()
					.map_err(|e| format!("Failed to init hotkey manager: {}", e))?,
			);
		}
		Ok(())
	})
}

fn parse_modifiers(mods: &str) -> Modifiers {
	let mut result = Modifiers::empty();
	let mods_lower = mods.to_lowercase();

	if mods_lower.contains("ctrl") || mods_lower.contains("control") {
		result |= Modifiers::CONTROL;
	}
	if mods_lower.contains("alt") {
		result |= Modifiers::ALT;
	}
	if mods_lower.contains("shift") {
		result |= Modifiers::SHIFT;
	}
	if mods_lower.contains("super") || mods_lower.contains("meta") || mods_lower.contains("win") {
		result |= Modifiers::SUPER;
	}

	result
}

fn parse_key(key: &str) -> Option<Code> {
	Code::from_str(key).ok()
}

pub fn hotkey_register(
	modifiers: &str,
	key: &str,
	vm: RingVM,
	callback: String,
) -> Result<u32, String> {
	ensure_manager()?;

	let mods = parse_modifiers(modifiers);
	let code = parse_key(key).ok_or_else(|| format!("Invalid key: {}", key))?;

	let hotkey = HotKey::new(Some(mods), code);
	let id = hotkey.id();

	HOTKEY_MANAGER.with(|m| {
		let m = m.borrow();
		m.as_ref()
			.unwrap()
			.register(hotkey)
			.map_err(|e| format!("Failed to register hotkey: {}", e))
	})?;

	REGISTERED_HOTKEYS.with(|h| {
		h.borrow_mut().insert(id, hotkey);
	});

	HOTKEY_CALLBACKS.with(|c| {
		c.borrow_mut().insert(id, (vm, callback));
	});

	Ok(id)
}

pub fn hotkey_unregister(id: u32) -> Result<(), String> {
	let hotkey = REGISTERED_HOTKEYS.with(|h| h.borrow_mut().remove(&id));

	if let Some(hk) = hotkey {
		HOTKEY_MANAGER.with(|m| {
			let m = m.borrow();
			if let Some(manager) = m.as_ref() {
				manager
					.unregister(hk)
					.map_err(|e| format!("Failed to unregister hotkey: {}", e))
			} else {
				Ok(())
			}
		})?;

		HOTKEY_CALLBACKS.with(|c| {
			c.borrow_mut().remove(&id);
		});
	}

	Ok(())
}

pub fn hotkey_unregister_all() -> Result<(), String> {
	let hotkeys: Vec<(u32, HotKey)> =
		REGISTERED_HOTKEYS.with(|h| h.borrow().iter().map(|(k, v)| (*k, *v)).collect());

	for (id, hk) in hotkeys {
		HOTKEY_MANAGER.with(|m| {
			let m = m.borrow();
			if let Some(manager) = m.as_ref() {
				let _ = manager.unregister(hk);
			}
		});
		HOTKEY_CALLBACKS.with(|c| {
			c.borrow_mut().remove(&id);
		});
		REGISTERED_HOTKEYS.with(|h| {
			h.borrow_mut().remove(&id);
		});
	}

	Ok(())
}

pub fn hotkey_poll() {
	if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
		HOTKEY_CALLBACKS.with(|c| {
			if let Some((vm, callback)) = c.borrow().get(&event.id) {
				let _guard = RingVmGuard::new(*vm);
				ring_vm_callfunction_str(*vm, callback);
			}
		});
	}
}
