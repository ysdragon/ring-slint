use crate::slint::{CALLBACK_ARGS, RingVmGuard};
use ring_lang_rs::{RingVM, ring_vm_callfunction_str};
use slint_interpreter::{ComponentInstance, Value};

pub fn set_callback(
    instance: &ComponentInstance,
    callback_name: &str,
    vm_ptr: RingVM,
    ring_func_name: String,
) -> Result<(), String> {
    instance
        .set_callback(callback_name, move |args: &[Value]| -> Value {
            CALLBACK_ARGS.with(|cell| {
                *cell.borrow_mut() = args.to_vec();
            });

            let func_without_parens = if ring_func_name.ends_with("()") {
                ring_func_name.trim_end_matches("()")
            } else {
                &ring_func_name
            };

            let _guard = RingVmGuard::new(vm_ptr);
            ring_vm_callfunction_str(vm_ptr, func_without_parens);

            Value::Void
        })
        .map_err(|e| format!("Set callback error: {:?}", e))
}

pub fn set_global_callback(
    instance: &ComponentInstance,
    global: &str,
    callback_name: &str,
    vm_ptr: RingVM,
    ring_func_name: String,
) -> Result<(), String> {
    instance
        .set_global_callback(global, callback_name, move |args: &[Value]| -> Value {
            CALLBACK_ARGS.with(|cell| {
                *cell.borrow_mut() = args.to_vec();
            });

            let func_without_parens = if ring_func_name.ends_with("()") {
                ring_func_name.trim_end_matches("()")
            } else {
                &ring_func_name
            };

            let _guard = RingVmGuard::new(vm_ptr);
            ring_vm_callfunction_str(vm_ptr, func_without_parens);

            Value::Void
        })
        .map_err(|e| format!("Set global callback error: {:?}", e))
}

pub fn get_callback_arg(index: usize) -> Option<Value> {
    CALLBACK_ARGS.with(|cell| {
        let args = cell.borrow();
        args.get(index).cloned()
    })
}

pub fn get_callback_args_count() -> usize {
    CALLBACK_ARGS.with(|cell| cell.borrow().len())
}
