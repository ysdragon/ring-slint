#![allow(non_snake_case)]

mod callback;
#[cfg(not(target_os = "android"))]
mod clipboard;
mod component;
#[cfg(not(target_os = "android"))]
mod dialogs;
#[cfg(not(target_os = "android"))]
mod hotkey;
mod interpreter;
mod model;
#[cfg(not(target_os = "android"))]
mod notification;
mod timer;
#[cfg(not(target_os = "android"))]
mod tray;
mod value;

pub use callback::*;
#[cfg(not(target_os = "android"))]
pub use clipboard::*;
pub use component::*;
#[cfg(not(target_os = "android"))]
pub use dialogs::*;
#[cfg(not(target_os = "android"))]
pub use hotkey::*;
pub use interpreter::*;
pub use model::*;
#[cfg(not(target_os = "android"))]
pub use notification::*;
pub use timer::*;
#[cfg(not(target_os = "android"))]
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
