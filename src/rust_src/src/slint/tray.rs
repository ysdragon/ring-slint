use crate::slint::RingVmGuard;
use ring_lang_rs::{RingVM, ring_vm_callfunction_str};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

static NEXT_MENU_ID: AtomicU32 = AtomicU32::new(1);

thread_local! {
    static TRAY_VM: RefCell<Option<RingVM>> = const { RefCell::new(None) };
    static MENU_CALLBACKS: RefCell<HashMap<u32, String>> = RefCell::new(HashMap::new());
}

// =============================================================================
// Linux/BSD implementation using ksni (D-Bus/SNI protocol)
// =============================================================================
#[cfg(all(unix, not(target_os = "macos"), not(target_os = "android")))]
mod platform {
    use super::*;
    use ksni::Tray;
    use ksni::blocking::TrayMethods;
    use ksni::menu::StandardItem;
    use std::sync::{Arc, Mutex};

    lazy_static::lazy_static! {
        static ref MENU_ITEMS: Arc<Mutex<Vec<(u32, String)>>> = Arc::new(Mutex::new(Vec::new()));
        static ref TRAY_TOOLTIP: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
        static ref PENDING_ACTIVATIONS: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(Vec::new()));
    }

    thread_local! {
        static TRAY_HANDLE: RefCell<Option<ksni::blocking::Handle<RingTray>>> = const { RefCell::new(None) };
    }

    struct RingTray;

    impl Tray for RingTray {
        fn id(&self) -> String {
            "ring-slint-app".into()
        }

        fn title(&self) -> String {
            TRAY_TOOLTIP.lock().unwrap().clone()
        }

        fn icon_name(&self) -> String {
            "application-x-executable".into()
        }

        fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
            let items = MENU_ITEMS.lock().unwrap();
            items
                .iter()
                .map(|(id, label)| {
                    let menu_id = *id;
                    ksni::MenuItem::Standard(StandardItem {
                        label: label.clone(),
                        activate: Box::new(move |_| {
                            if let Ok(mut pending) = PENDING_ACTIVATIONS.lock() {
                                pending.push(menu_id);
                            }
                        }),
                        ..Default::default()
                    })
                })
                .collect()
        }
    }

    pub fn create(tooltip: &str) -> Result<(), String> {
        *TRAY_TOOLTIP.lock().unwrap() = tooltip.to_string();
        MENU_ITEMS.lock().unwrap().clear();
        MENU_CALLBACKS.with(|c| c.borrow_mut().clear());

        let tray = RingTray;
        let handle = tray
            .spawn()
            .map_err(|e| format!("Failed to spawn tray: {}", e))?;

        TRAY_HANDLE.with(|h| {
            *h.borrow_mut() = Some(handle);
        });

        Ok(())
    }

    pub fn create_with_icon(tooltip: &str, _icon_path: &str) -> Result<(), String> {
        create(tooltip)
    }

    pub fn set_icon(_icon_path: &str) -> Result<(), String> {
        Ok(())
    }

    pub fn set_tooltip(tooltip: &str) -> Result<(), String> {
        *TRAY_TOOLTIP.lock().unwrap() = tooltip.to_string();
        TRAY_HANDLE.with(|h| {
            if let Some(handle) = h.borrow().as_ref() {
                handle.update(|_| {});
            }
        });
        Ok(())
    }

    pub fn add_menu_item(label: &str, vm: RingVM, callback: String) -> Result<u32, String> {
        let id = NEXT_MENU_ID.fetch_add(1, Ordering::SeqCst);

        MENU_ITEMS.lock().unwrap().push((id, label.to_string()));
        MENU_CALLBACKS.with(|c| c.borrow_mut().insert(id, callback));
        TRAY_VM.with(|v| *v.borrow_mut() = Some(vm));

        TRAY_HANDLE.with(|h| {
            if let Some(handle) = h.borrow().as_ref() {
                handle.update(|_| {});
            }
        });

        Ok(id)
    }

    pub fn add_separator() -> Result<(), String> {
        Ok(())
    }

    pub fn destroy() {
        TRAY_HANDLE.with(|h| {
            if let Some(handle) = h.borrow().as_ref() {
                handle.shutdown();
            }
            *h.borrow_mut() = None;
        });
        MENU_ITEMS.lock().unwrap().clear();
        MENU_CALLBACKS.with(|c| c.borrow_mut().clear());
    }

    pub fn poll() {
        let pending: Vec<u32> = {
            let mut p = PENDING_ACTIVATIONS.lock().unwrap();
            std::mem::take(&mut *p)
        };

        if pending.is_empty() {
            return;
        }

        TRAY_VM.with(|v| {
            if let Some(vm) = *v.borrow() {
                MENU_CALLBACKS.with(|c| {
                    let callbacks = c.borrow();
                    for menu_id in pending {
                        if let Some(callback) = callbacks.get(&menu_id) {
                            let _guard = RingVmGuard::new(vm);
                            ring_vm_callfunction_str(vm, callback);
                        }
                    }
                });
            }
        });
    }
}

// =============================================================================
// Windows/macOS implementation using tray-icon (native APIs)
// =============================================================================
#[cfg(any(windows, target_os = "macos"))]
mod platform {
    use super::*;
    use tray_icon::{
        Icon, TrayIcon, TrayIconBuilder,
        menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    };

    thread_local! {
        static TRAY_ICON: RefCell<Option<TrayIcon>> = const { RefCell::new(None) };
        static TRAY_MENU: RefCell<Option<Menu>> = const { RefCell::new(None) };
        static MENU_ID_MAP: RefCell<HashMap<String, u32>> = RefCell::new(HashMap::new());
    }

    fn load_icon(path: &str) -> Result<Icon, String> {
        let image = image::open(path)
            .map_err(|e| format!("Failed to load icon: {}", e))?
            .into_rgba8();
        let (width, height) = image.dimensions();
        Icon::from_rgba(image.into_raw(), width, height)
            .map_err(|e| format!("Failed to create icon: {}", e))
    }

    pub fn create(tooltip: &str) -> Result<(), String> {
        let menu = Menu::new();

        let tray = TrayIconBuilder::new()
            .with_menu(Box::new(menu.clone()))
            .with_tooltip(tooltip)
            .build()
            .map_err(|e| format!("Failed to create tray icon: {}", e))?;

        TRAY_ICON.with(|t| *t.borrow_mut() = Some(tray));
        TRAY_MENU.with(|m| *m.borrow_mut() = Some(menu));
        MENU_CALLBACKS.with(|c| c.borrow_mut().clear());
        MENU_ID_MAP.with(|m| m.borrow_mut().clear());

        Ok(())
    }

    pub fn create_with_icon(tooltip: &str, icon_path: &str) -> Result<(), String> {
        let icon = load_icon(icon_path)?;
        let menu = Menu::new();

        let tray = TrayIconBuilder::new()
            .with_menu(Box::new(menu.clone()))
            .with_tooltip(tooltip)
            .with_icon(icon)
            .build()
            .map_err(|e| format!("Failed to create tray icon: {}", e))?;

        TRAY_ICON.with(|t| *t.borrow_mut() = Some(tray));
        TRAY_MENU.with(|m| *m.borrow_mut() = Some(menu));
        MENU_CALLBACKS.with(|c| c.borrow_mut().clear());
        MENU_ID_MAP.with(|m| m.borrow_mut().clear());

        Ok(())
    }

    pub fn set_icon(icon_path: &str) -> Result<(), String> {
        let icon = load_icon(icon_path)?;
        TRAY_ICON.with(|t| {
            if let Some(tray) = t.borrow().as_ref() {
                tray.set_icon(Some(icon))
                    .map_err(|e| format!("Failed to set icon: {}", e))
            } else {
                Err("Tray not created".to_string())
            }
        })
    }

    pub fn set_tooltip(tooltip: &str) -> Result<(), String> {
        TRAY_ICON.with(|t| {
            if let Some(tray) = t.borrow().as_ref() {
                tray.set_tooltip(Some(tooltip))
                    .map_err(|e| format!("Failed to set tooltip: {}", e))
            } else {
                Err("Tray not created".to_string())
            }
        })
    }

    pub fn add_menu_item(label: &str, vm: RingVM, callback: String) -> Result<u32, String> {
        let id = NEXT_MENU_ID.fetch_add(1, Ordering::SeqCst);
        let item = MenuItem::new(label, true, None);
        let menu_id_str = item.id().0.clone();

        TRAY_MENU.with(|m| {
            if let Some(menu) = m.borrow().as_ref() {
                menu.append(&item)
                    .map_err(|e| format!("Failed to add menu item: {}", e))
            } else {
                Err("Tray menu not created".to_string())
            }
        })?;

        MENU_ID_MAP.with(|m| m.borrow_mut().insert(menu_id_str, id));
        MENU_CALLBACKS.with(|c| c.borrow_mut().insert(id, callback));
        TRAY_VM.with(|v| *v.borrow_mut() = Some(vm));

        Ok(id)
    }

    pub fn add_separator() -> Result<(), String> {
        TRAY_MENU.with(|m| {
            if let Some(menu) = m.borrow().as_ref() {
                menu.append(&PredefinedMenuItem::separator())
                    .map_err(|e| format!("Failed to add separator: {}", e))
            } else {
                Err("Tray menu not created".to_string())
            }
        })
    }

    pub fn destroy() {
        TRAY_ICON.with(|t| *t.borrow_mut() = None);
        TRAY_MENU.with(|m| *m.borrow_mut() = None);
        MENU_CALLBACKS.with(|c| c.borrow_mut().clear());
        MENU_ID_MAP.with(|m| m.borrow_mut().clear());
    }

    pub fn poll() {
        if let Ok(event) = MenuEvent::receiver().try_recv() {
            let menu_id_str = event.id.0;
            MENU_ID_MAP.with(|m| {
                if let Some(&id) = m.borrow().get(&menu_id_str) {
                    TRAY_VM.with(|v| {
                        if let Some(vm) = *v.borrow() {
                            MENU_CALLBACKS.with(|c| {
                                if let Some(callback) = c.borrow().get(&id) {
                                    let _guard = RingVmGuard::new(vm);
                                    ring_vm_callfunction_str(vm, callback);
                                }
                            });
                        }
                    });
                }
            });
        }
    }
}

// =============================================================================
// Android stub (no tray support)
// =============================================================================
#[cfg(target_os = "android")]
mod platform {
    use super::*;

    pub fn create(_tooltip: &str) -> Result<(), String> {
        Err("System tray not supported on Android".to_string())
    }

    pub fn create_with_icon(_tooltip: &str, _icon_path: &str) -> Result<(), String> {
        Err("System tray not supported on Android".to_string())
    }

    pub fn set_icon(_icon_path: &str) -> Result<(), String> {
        Err("System tray not supported on Android".to_string())
    }

    pub fn set_tooltip(_tooltip: &str) -> Result<(), String> {
        Err("System tray not supported on Android".to_string())
    }

    pub fn add_menu_item(_label: &str, _vm: RingVM, _callback: String) -> Result<u32, String> {
        Err("System tray not supported on Android".to_string())
    }

    pub fn add_separator() -> Result<(), String> {
        Err("System tray not supported on Android".to_string())
    }

    pub fn destroy() {}

    pub fn poll() {}
}

// =============================================================================
// Public API (delegates to platform module)
// =============================================================================
pub fn tray_create(tooltip: &str) -> Result<(), String> {
    platform::create(tooltip)
}

pub fn tray_create_with_icon(tooltip: &str, icon_path: &str) -> Result<(), String> {
    platform::create_with_icon(tooltip, icon_path)
}

pub fn tray_set_icon(icon_path: &str) -> Result<(), String> {
    platform::set_icon(icon_path)
}

pub fn tray_set_tooltip(tooltip: &str) -> Result<(), String> {
    platform::set_tooltip(tooltip)
}

pub fn tray_add_menu_item(label: &str, vm: RingVM, callback: String) -> Result<u32, String> {
    platform::add_menu_item(label, vm, callback)
}

pub fn tray_add_separator() -> Result<(), String> {
    platform::add_separator()
}

pub fn tray_destroy() {
    platform::destroy()
}

pub fn tray_poll() {
    platform::poll()
}
