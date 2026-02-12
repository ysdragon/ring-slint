#![allow(non_snake_case)]

mod callback;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
mod clipboard;
mod component;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
mod dialogs;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
mod hotkey;
mod interpreter;
mod model;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
mod notification;
mod timer;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
mod tray;
mod value;

pub use callback::*;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub use clipboard::*;
pub use component::*;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub use dialogs::*;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub use hotkey::*;
pub use interpreter::*;
pub use model::*;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub use notification::*;
pub use timer::*;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub use tray::*;
pub use value::*;

use slint_interpreter::{ComponentInstance, Value};
use std::cell::RefCell;

pub const SLINT_COMPONENT_TYPE: &[u8] = b"SlintComponent\0";
pub const SLINT_INSTANCE_TYPE: &[u8] = b"SlintInstance\0";

pub struct SlintComponentDef {
    pub definition: slint_interpreter::ComponentDefinition,
}

pub struct SlintInstanceWrapper {
    pub instance: ComponentInstance,
    pub vm_ptr: *mut libc::c_void,
}

impl SlintInstanceWrapper {
    pub fn new(instance: ComponentInstance, vm_ptr: *mut libc::c_void) -> Self {
        Self { instance, vm_ptr }
    }
}

thread_local! {
    pub static CALLBACK_ARGS: RefCell<Vec<Value>> = const { RefCell::new(Vec::new()) };
}
