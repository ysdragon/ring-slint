pub mod slint;

#[cfg(target_os = "android")]
pub mod android;

use ring_lang_rs::*;
use slint_interpreter::Value;

#[cfg(target_os = "android")]
pub use android::*;

extern "C" fn free_slint_component(_state: *mut libc::c_void, ptr: *mut libc::c_void) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr as *mut slint::SlintComponentDef);
        }
    }
}

extern "C" fn free_slint_instance(_state: *mut libc::c_void, ptr: *mut libc::c_void) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr as *mut slint::SlintInstanceWrapper);
        }
    }
}

ring_func!(ring_slint_set_style, |p| {
    ring_check_paracount!(p, 1);
    ring_check_string!(p, 1);

    let style = ring_get_string!(p, 1);
    slint::set_style(style);
});

ring_func!(ring_slint_get_style, |p| {
    ring_check_paracount!(p, 0);

    if let Some(style) = slint::get_style() {
        ring_ret_string!(p, &style);
    }
});

ring_func!(ring_slint_add_library_path, |p| {
    ring_check_paracount!(p, 2);
    ring_check_string!(p, 1);
    ring_check_string!(p, 2);

    let name = ring_get_string!(p, 1);
    let path = ring_get_string!(p, 2);
    slint::add_library_path(name, path);
});

ring_func!(ring_slint_remove_library_path, |p| {
    ring_check_paracount!(p, 1);
    ring_check_string!(p, 1);

    let name = ring_get_string!(p, 1);
    slint::remove_library_path(name);
});

ring_func!(ring_slint_clear_library_paths, |p| {
    ring_check_paracount!(p, 0);
    slint::clear_library_paths();
});

ring_func!(ring_slint_load, |p| {
    ring_check_paracount!(p, 1);
    ring_check_string!(p, 1);

    let path = ring_get_string!(p, 1);

    match slint::compile_from_path(path) {
        Ok(definition) => {
            let wrapper = Box::new(slint::SlintComponentDef { definition });
            let ptr = Box::into_raw(wrapper);
            ring_ret_managed_cpointer!(p, ptr, slint::SLINT_COMPONENT_TYPE, free_slint_component);
        }
        Err(e) => {
            ring_error!(p, &e);
        }
    }
});

ring_func!(ring_slint_loadstring, |p| {
    ring_check_paracount!(p, 2);
    ring_check_string!(p, 1);
    ring_check_string!(p, 2);

    let source = ring_get_string!(p, 1);
    let path = ring_get_string!(p, 2);

    match slint::compile_from_source(source, path) {
        Ok(definition) => {
            let wrapper = Box::new(slint::SlintComponentDef { definition });
            let ptr = Box::into_raw(wrapper);
            ring_ret_managed_cpointer!(p, ptr, slint::SLINT_COMPONENT_TYPE, free_slint_component);
        }
        Err(e) => {
            ring_error!(p, &e);
        }
    }
});

ring_func!(ring_slint_create, |p| {
    ring_check_paracount!(p, 1);
    ring_check_cpointer!(p, 1);

    if let Some(comp) =
        ring_get_pointer!(p, 1, slint::SlintComponentDef, slint::SLINT_COMPONENT_TYPE)
    {
        match slint::create_instance(&comp.definition, p) {
            Ok(instance) => {
                let wrapper = Box::new(instance);
                let ptr = Box::into_raw(wrapper);
                ring_ret_managed_cpointer!(p, ptr, slint::SLINT_INSTANCE_TYPE, free_slint_instance);
            }
            Err(e) => {
                ring_error!(p, &e);
            }
        }
    } else {
        ring_error!(p, "Invalid SlintComponent pointer");
    }
});

ring_func!(ring_slint_show, |p| {
    ring_check_paracount!(p, 1);
    ring_check_cpointer!(p, 1);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        if let Err(e) = slint::instance_show(&wrapper.instance) {
            ring_error!(p, &e);
        }
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_hide, |p| {
    ring_check_paracount!(p, 1);
    ring_check_cpointer!(p, 1);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        if let Err(e) = slint::instance_hide(&wrapper.instance) {
            ring_error!(p, &e);
        }
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_run, |p| {
    let paracount = ring_api_paracount(p);

    if paracount == 0 {
        slint_interpreter::run_event_loop().ok();
    } else if paracount == 1 {
        ring_check_cpointer!(p, 1);
        if let Some(wrapper) = ring_get_pointer!(
            p,
            1,
            slint::SlintInstanceWrapper,
            slint::SLINT_INSTANCE_TYPE
        ) {
            if let Err(e) = slint::instance_run(&wrapper.instance) {
                ring_error!(p, &e);
            }
        } else {
            ring_error!(p, "Invalid SlintInstance pointer");
        }
    } else {
        ring_error!(p, "slint_run() expects 0 or 1 parameters");
    }
});

ring_func!(ring_slint_quit, |p| {
    ring_check_paracount!(p, 0);
    slint_interpreter::quit_event_loop().ok();
});

ring_func!(ring_slint_get, |p| {
    ring_check_paracount!(p, 2);
    ring_check_cpointer!(p, 1);
    ring_check_string!(p, 2);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let prop_name = ring_get_string!(p, 2);
        match slint::instance_get_property(&wrapper.instance, prop_name) {
            Ok(value) => {
                slint::slint_value_to_ring(p, &value);
            }
            Err(e) => {
                ring_error!(p, &e);
            }
        }
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_set, |p| {
    ring_check_paracount!(p, 3);
    ring_check_cpointer!(p, 1);
    ring_check_string!(p, 2);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let prop_name = ring_get_string!(p, 2);
        let value = slint::ring_list_to_slint_value(p, 3);

        if let Err(e) = slint::instance_set_property(&wrapper.instance, prop_name, value) {
            ring_error!(p, &e);
        }
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_set_bool, |p| {
    ring_check_paracount!(p, 3);
    ring_check_cpointer!(p, 1);
    ring_check_string!(p, 2);
    ring_check_number!(p, 3);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let prop_name = ring_get_string!(p, 2);
        let value = Value::Bool(ring_get_int!(p, 3) != 0);

        if let Err(e) = slint::instance_set_property(&wrapper.instance, prop_name, value) {
            ring_error!(p, &e);
        }
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_set_image, |p| {
    ring_check_paracount!(p, 3);
    ring_check_cpointer!(p, 1);
    ring_check_string!(p, 2);
    ring_check_string!(p, 3);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let prop_name = ring_get_string!(p, 2);
        let path = ring_get_string!(p, 3);

        match slint::ring_string_to_image(path) {
            Ok(value) => {
                if let Err(e) = slint::instance_set_property(&wrapper.instance, prop_name, value) {
                    ring_error!(p, &e);
                }
            }
            Err(e) => {
                ring_error!(p, &e);
            }
        }
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_set_string, |p| {
    ring_check_paracount!(p, 3);
    ring_check_cpointer!(p, 1);
    ring_check_string!(p, 2);
    ring_check_string!(p, 3);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let prop_name = ring_get_string!(p, 2);
        let s = ring_get_string!(p, 3);
        let value = Value::String(slint_interpreter::SharedString::from(s));

        if let Err(e) = slint::instance_set_property(&wrapper.instance, prop_name, value) {
            ring_error!(p, &e);
        }
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_set_number, |p| {
    ring_check_paracount!(p, 3);
    ring_check_cpointer!(p, 1);
    ring_check_string!(p, 2);
    ring_check_number!(p, 3);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let prop_name = ring_get_string!(p, 2);
        let value = Value::Number(ring_api_getnumber(p, 3));

        if let Err(e) = slint::instance_set_property(&wrapper.instance, prop_name, value) {
            ring_error!(p, &e);
        }
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_set_color, |p| {
    ring_check_paracount!(p, 3);
    ring_check_cpointer!(p, 1);
    ring_check_string!(p, 2);
    ring_check_string!(p, 3);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let prop_name = ring_get_string!(p, 2);
        let hex = ring_get_string!(p, 3);

        match slint::parse_hex_color_value(hex) {
            Some(value) => {
                if let Err(e) = slint::instance_set_property(&wrapper.instance, prop_name, value) {
                    ring_error!(p, &e);
                }
            }
            None => {
                ring_error!(
                    p,
                    &format!(
                        "Invalid color format '{}'. Expected hex color like #RRGGBB or #RRGGBBAA",
                        hex
                    )
                );
            }
        }
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_set_enum, |p| {
    ring_check_paracount!(p, 3);
    ring_check_cpointer!(p, 1);
    ring_check_string!(p, 2);
    ring_check_string!(p, 3);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let prop_name = ring_get_string!(p, 2);
        let s = ring_get_string!(p, 3);

        if let Some(dot_pos) = s.find('.') {
            let name = &s[..dot_pos];
            let variant = &s[dot_pos + 1..];
            let value = Value::EnumerationValue(name.to_string(), variant.to_string());

            if let Err(e) = slint::instance_set_property(&wrapper.instance, prop_name, value) {
                ring_error!(p, &e);
            }
        } else {
            ring_error!(
                p,
                &format!("Invalid enum format '{}'. Expected 'EnumName.variant'", s)
            );
        }
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_on, |p| {
    ring_check_paracount!(p, 3);
    ring_check_cpointer!(p, 1);
    ring_check_string!(p, 2);
    ring_check_string!(p, 3);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let callback_name = ring_get_string!(p, 2);
        let ring_func = ring_get_string!(p, 3).to_string();

        if let Err(e) =
            slint::set_callback(&wrapper.instance, callback_name, p as RingVM, ring_func)
        {
            ring_error!(p, &e);
        }
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_invoke, |p| {
    let paracount = ring_api_paracount(p);
    if !(2..=3).contains(&paracount) {
        ring_error!(
            p,
            "slint_invoke(window, callback, [args]) expects 2-3 parameters"
        );
        return;
    }

    ring_check_cpointer!(p, 1);
    ring_check_string!(p, 2);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let callback_name = ring_get_string!(p, 2);

        let args = if paracount == 3 && ring_api_islist(p, 3) {
            slint::ring_list_to_args(ring_api_getlist(p, 3))
        } else {
            Vec::new()
        };

        match slint::instance_invoke(&wrapper.instance, callback_name, &args) {
            Ok(result) => {
                slint::slint_value_to_ring(p, &result);
            }
            Err(e) => {
                ring_error!(p, &e);
            }
        }
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_callback_arg, |p| {
    ring_check_paracount!(p, 1);
    ring_check_number!(p, 1);

    let index = ring_get_int!(p, 1) as usize;
    if index == 0 {
        ring_error!(p, "Callback argument index starts at 1");
        return;
    }

    if let Some(value) = slint::get_callback_arg(index - 1) {
        slint::slint_value_to_ring(p, &value);
    } else {
        ring_ret_number!(p, 0);
    }
});

ring_func!(ring_slint_callback_args_count, |p| {
    ring_check_paracount!(p, 0);
    ring_ret_number!(p, slint::get_callback_args_count() as f64);
});

ring_func!(ring_slint_global_get, |p| {
    ring_check_paracount!(p, 3);
    ring_check_cpointer!(p, 1);
    ring_check_string!(p, 2);
    ring_check_string!(p, 3);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let global_name = ring_get_string!(p, 2);
        let prop_name = ring_get_string!(p, 3);

        match slint::get_global_property(&wrapper.instance, global_name, prop_name) {
            Ok(value) => {
                slint::slint_value_to_ring(p, &value);
            }
            Err(e) => {
                ring_error!(p, &e);
            }
        }
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_global_set, |p| {
    ring_check_paracount!(p, 4);
    ring_check_cpointer!(p, 1);
    ring_check_string!(p, 2);
    ring_check_string!(p, 3);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let global_name = ring_get_string!(p, 2);
        let prop_name = ring_get_string!(p, 3);
        let value = slint::ring_list_to_slint_value(p, 4);

        if let Err(e) = slint::set_global_property(&wrapper.instance, global_name, prop_name, value)
        {
            ring_error!(p, &e);
        }
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_global_on, |p| {
    ring_check_paracount!(p, 4);
    ring_check_cpointer!(p, 1);
    ring_check_string!(p, 2);
    ring_check_string!(p, 3);
    ring_check_string!(p, 4);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let global_name = ring_get_string!(p, 2);
        let callback_name = ring_get_string!(p, 3);
        let ring_func = ring_get_string!(p, 4).to_string();

        if let Err(e) = slint::set_global_callback(
            &wrapper.instance,
            global_name,
            callback_name,
            p as RingVM,
            ring_func,
        ) {
            ring_error!(p, &e);
        }
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_global_invoke, |p| {
    let paracount = ring_api_paracount(p);
    if !(3..=4).contains(&paracount) {
        ring_error!(
            p,
            "slint_global_invoke(window, global, callback, [args]) expects 3-4 parameters"
        );
        return;
    }

    ring_check_cpointer!(p, 1);
    ring_check_string!(p, 2);
    ring_check_string!(p, 3);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let global_name = ring_get_string!(p, 2);
        let callback_name = ring_get_string!(p, 3);

        let args = if paracount == 4 && ring_api_islist(p, 4) {
            slint::ring_list_to_args(ring_api_getlist(p, 4))
        } else {
            Vec::new()
        };

        match slint::invoke_global(&wrapper.instance, global_name, callback_name, &args) {
            Ok(result) => {
                slint::slint_value_to_ring(p, &result);
            }
            Err(e) => {
                ring_error!(p, &e);
            }
        }
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_timer_start, |p| {
    let paracount = ring_api_paracount(p);
    if !(2..=3).contains(&paracount) {
        ring_error!(
            p,
            "slint_timer_start(interval_ms, callback, [repeated]) expects 2-3 parameters"
        );
        return;
    }

    ring_check_number!(p, 1);
    ring_check_string!(p, 2);

    let interval_ms = ring_get_int!(p, 1) as u64;
    let callback_name = ring_get_string!(p, 2).to_string();

    let repeated = if paracount == 3 {
        ring_check_number!(p, 3);
        ring_get_int!(p, 3) != 0
    } else {
        true
    };

    let timer_id = slint::timer_start(interval_ms, repeated, p as RingVM, callback_name);
    ring_ret_number!(p, timer_id as f64);
});

ring_func!(ring_slint_timer_stop, |p| {
    ring_check_paracount!(p, 1);
    ring_check_number!(p, 1);

    let timer_id = ring_get_int!(p, 1) as u32;
    if let Err(e) = slint::timer_stop(timer_id) {
        ring_error!(p, &e);
    }
});

ring_func!(ring_slint_timer_running, |p| {
    ring_check_paracount!(p, 1);
    ring_check_number!(p, 1);

    let timer_id = ring_get_int!(p, 1) as u32;
    match slint::timer_running(timer_id) {
        Ok(running) => ring_ret_number!(p, if running { 1.0 } else { 0.0 }),
        Err(e) => ring_error!(p, &e),
    }
});

ring_func!(ring_slint_timer_restart, |p| {
    ring_check_paracount!(p, 1);
    ring_check_number!(p, 1);

    let timer_id = ring_get_int!(p, 1) as u32;
    if let Err(e) = slint::timer_restart(timer_id) {
        ring_error!(p, &e);
    }
});

ring_func!(ring_slint_timer_set_interval, |p| {
    ring_check_paracount!(p, 2);
    ring_check_number!(p, 1);
    ring_check_number!(p, 2);

    let timer_id = ring_get_int!(p, 1) as u32;
    let interval_ms = ring_get_int!(p, 2) as u64;
    if let Err(e) = slint::timer_set_interval(timer_id, interval_ms) {
        ring_error!(p, &e);
    }
});

ring_func!(ring_slint_model_create, |p| {
    ring_check_paracount!(p, 2);
    ring_check_cpointer!(p, 1);
    ring_check_string!(p, 2);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let prop_name = ring_get_string!(p, 2);
        match slint::model_create(&wrapper.instance, prop_name) {
            Ok(model_id) => ring_ret_number!(p, model_id as f64),
            Err(e) => ring_error!(p, &e),
        }
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_model_push, |p| {
    ring_check_paracount!(p, 2);
    ring_check_number!(p, 1);

    let model_id = ring_get_int!(p, 1) as u32;
    let value = slint::ring_param_to_model_value(p, 2);
    if let Err(e) = slint::model_push(model_id, value) {
        ring_error!(p, &e);
    }
});

ring_func!(ring_slint_model_remove, |p| {
    ring_check_paracount!(p, 2);
    ring_check_number!(p, 1);
    ring_check_number!(p, 2);

    let model_id = ring_get_int!(p, 1) as u32;
    let index = ring_get_int!(p, 2) as usize;
    if let Err(e) = slint::model_remove(model_id, index) {
        ring_error!(p, &e);
    }
});

ring_func!(ring_slint_model_set, |p| {
    ring_check_paracount!(p, 3);
    ring_check_number!(p, 1);
    ring_check_number!(p, 2);

    let model_id = ring_get_int!(p, 1) as u32;
    let index = ring_get_int!(p, 2) as usize;
    let value = slint::ring_param_to_model_value(p, 3);
    if let Err(e) = slint::model_set(model_id, index, value) {
        ring_error!(p, &e);
    }
});

ring_func!(ring_slint_model_count, |p| {
    ring_check_paracount!(p, 1);
    ring_check_number!(p, 1);

    let model_id = ring_get_int!(p, 1) as u32;
    match slint::model_count(model_id) {
        Ok(count) => ring_ret_number!(p, count as f64),
        Err(e) => ring_error!(p, &e),
    }
});

ring_func!(ring_slint_model_clear, |p| {
    ring_check_paracount!(p, 1);
    ring_check_number!(p, 1);

    let model_id = ring_get_int!(p, 1) as u32;
    if let Err(e) = slint::model_clear(model_id) {
        ring_error!(p, &e);
    }
});

ring_func!(ring_slint_model_insert, |p| {
    ring_check_paracount!(p, 3);
    ring_check_number!(p, 1);
    ring_check_number!(p, 2);

    let model_id = ring_get_int!(p, 1) as u32;
    let index = ring_get_int!(p, 2) as usize;
    let value = slint::ring_param_to_model_value(p, 3);
    if let Err(e) = slint::model_insert(model_id, index, value) {
        ring_error!(p, &e);
    }
});

ring_func!(ring_slint_model_destroy, |p| {
    ring_check_paracount!(p, 1);
    ring_check_number!(p, 1);

    let model_id = ring_get_int!(p, 1) as u32;
    if let Err(e) = slint::model_destroy(model_id) {
        ring_error!(p, &e);
    }
});

// Window management functions
ring_func!(ring_slint_window_set_minimized, |p| {
    ring_check_paracount!(p, 2);
    ring_check_cpointer!(p, 1);
    ring_check_number!(p, 2);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let minimized = ring_get_int!(p, 2) != 0;
        slint::window_set_minimized(&wrapper.instance, minimized);
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_window_is_minimized, |p| {
    ring_check_paracount!(p, 1);
    ring_check_cpointer!(p, 1);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let result = slint::window_is_minimized(&wrapper.instance);
        ring_ret_number!(p, if result { 1.0 } else { 0.0 });
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_window_set_maximized, |p| {
    ring_check_paracount!(p, 2);
    ring_check_cpointer!(p, 1);
    ring_check_number!(p, 2);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let maximized = ring_get_int!(p, 2) != 0;
        slint::window_set_maximized(&wrapper.instance, maximized);
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_window_is_maximized, |p| {
    ring_check_paracount!(p, 1);
    ring_check_cpointer!(p, 1);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let result = slint::window_is_maximized(&wrapper.instance);
        ring_ret_number!(p, if result { 1.0 } else { 0.0 });
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_window_set_fullscreen, |p| {
    ring_check_paracount!(p, 2);
    ring_check_cpointer!(p, 1);
    ring_check_number!(p, 2);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let fullscreen = ring_get_int!(p, 2) != 0;
        slint::window_set_fullscreen(&wrapper.instance, fullscreen);
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_window_is_fullscreen, |p| {
    ring_check_paracount!(p, 1);
    ring_check_cpointer!(p, 1);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let result = slint::window_is_fullscreen(&wrapper.instance);
        ring_ret_number!(p, if result { 1.0 } else { 0.0 });
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_window_set_position, |p| {
    ring_check_paracount!(p, 3);
    ring_check_cpointer!(p, 1);
    ring_check_number!(p, 2);
    ring_check_number!(p, 3);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let x = ring_get_int!(p, 2);
        let y = ring_get_int!(p, 3);
        slint::window_set_position(&wrapper.instance, x, y);
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_window_get_position, |p| {
    ring_check_paracount!(p, 1);
    ring_check_cpointer!(p, 1);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let (x, y) = slint::window_get_position(&wrapper.instance);
        let list = ring_api_newlist(p);
        ring_list_addint(list, x);
        ring_list_addint(list, y);
        ring_ret_list!(p, list);
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_window_set_size, |p| {
    ring_check_paracount!(p, 3);
    ring_check_cpointer!(p, 1);
    ring_check_number!(p, 2);
    ring_check_number!(p, 3);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let width = ring_get_int!(p, 2) as u32;
        let height = ring_get_int!(p, 3) as u32;
        slint::window_set_size(&wrapper.instance, width, height);
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_window_get_size, |p| {
    ring_check_paracount!(p, 1);
    ring_check_cpointer!(p, 1);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let (width, height) = slint::window_get_size(&wrapper.instance);
        let list = ring_api_newlist(p);
        ring_list_addint(list, width as i32);
        ring_list_addint(list, height as i32);
        ring_ret_list!(p, list);
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_window_scale_factor, |p| {
    ring_check_paracount!(p, 1);
    ring_check_cpointer!(p, 1);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let scale = slint::window_scale_factor(&wrapper.instance);
        ring_ret_number!(p, scale as f64);
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_window_is_visible, |p| {
    ring_check_paracount!(p, 1);
    ring_check_cpointer!(p, 1);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let visible = slint::window_is_visible(&wrapper.instance);
        ring_ret_number!(p, if visible { 1.0 } else { 0.0 });
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_window_request_redraw, |p| {
    ring_check_paracount!(p, 1);
    ring_check_cpointer!(p, 1);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        slint::window_request_redraw(&wrapper.instance);
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

ring_func!(ring_slint_definition_name, |p| {
    ring_check_paracount!(p, 1);
    ring_check_cpointer!(p, 1);

    if let Some(comp) =
        ring_get_pointer!(p, 1, slint::SlintComponentDef, slint::SLINT_COMPONENT_TYPE)
    {
        let name = slint::definition_name(&comp.definition);
        ring_ret_string!(p, &name);
    } else {
        ring_error!(p, "Invalid SlintComponent pointer");
    }
});

ring_func!(ring_slint_definition_properties, |p| {
    ring_check_paracount!(p, 1);
    ring_check_cpointer!(p, 1);

    if let Some(comp) =
        ring_get_pointer!(p, 1, slint::SlintComponentDef, slint::SLINT_COMPONENT_TYPE)
    {
        let props = slint::definition_properties(&comp.definition);
        let list = ring_api_newlist(p);
        for (name, prop_type) in props {
            let sublist = ring_list_newlist(list);
            ring_list_addstring_str(sublist, &name);
            ring_list_addstring_str(sublist, &prop_type);
        }
        ring_ret_list!(p, list);
    } else {
        ring_error!(p, "Invalid SlintComponent pointer");
    }
});

ring_func!(ring_slint_definition_callbacks, |p| {
    ring_check_paracount!(p, 1);
    ring_check_cpointer!(p, 1);

    if let Some(comp) =
        ring_get_pointer!(p, 1, slint::SlintComponentDef, slint::SLINT_COMPONENT_TYPE)
    {
        let callbacks = slint::definition_callbacks(&comp.definition);
        let list = ring_api_newlist(p);
        for cb in callbacks {
            ring_list_addstring_str(list, &cb);
        }
        ring_ret_list!(p, list);
    } else {
        ring_error!(p, "Invalid SlintComponent pointer");
    }
});

ring_func!(ring_slint_definition_functions, |p| {
    ring_check_paracount!(p, 1);
    ring_check_cpointer!(p, 1);

    if let Some(comp) =
        ring_get_pointer!(p, 1, slint::SlintComponentDef, slint::SLINT_COMPONENT_TYPE)
    {
        let functions = slint::definition_functions(&comp.definition);
        let list = ring_api_newlist(p);
        for func in functions {
            ring_list_addstring_str(list, &func);
        }
        ring_ret_list!(p, list);
    } else {
        ring_error!(p, "Invalid SlintComponent pointer");
    }
});

ring_func!(ring_slint_definition_globals, |p| {
    ring_check_paracount!(p, 1);
    ring_check_cpointer!(p, 1);

    if let Some(comp) =
        ring_get_pointer!(p, 1, slint::SlintComponentDef, slint::SLINT_COMPONENT_TYPE)
    {
        let globals = slint::definition_globals(&comp.definition);
        let list = ring_api_newlist(p);
        for g in globals {
            ring_list_addstring_str(list, &g);
        }
        ring_ret_list!(p, list);
    } else {
        ring_error!(p, "Invalid SlintComponent pointer");
    }
});

ring_func!(ring_slint_model_get, |p| {
    ring_check_paracount!(p, 2);
    ring_check_number!(p, 1);
    ring_check_number!(p, 2);

    let model_id = ring_get_int!(p, 1) as u32;
    let index = ring_get_int!(p, 2) as usize;
    match slint::model_get(model_id, index) {
        Ok(value) => slint::slint_value_to_ring(p, &value),
        Err(e) => ring_error!(p, &e),
    }
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_window_drag, |p| {
    ring_check_paracount!(p, 1);
    ring_check_cpointer!(p, 1);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        if let Err(e) = slint::window_drag(&wrapper.instance) {
            ring_error!(p, &e);
        }
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_file_open, |p| {
    let paracount = ring_api_paracount(p);
    if paracount < 1 {
        ring_error!(p, "slint_file_open() expects at least 1 parameter (title)");
        return;
    }
    ring_check_string!(p, 1);

    let title = ring_get_string!(p, 1);
    let filters: Vec<(String, Vec<String>)> = if paracount >= 2 && ring_api_islist(p, 2) {
        parse_file_filters(p, 2)
    } else {
        vec![]
    };

    if let Some(path) = slint::file_dialog_open(title, &filters) {
        ring_ret_string!(p, &path);
    }
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_file_open_multiple, |p| {
    let paracount = ring_api_paracount(p);
    if paracount < 1 {
        ring_error!(
            p,
            "slint_file_open_multiple() expects at least 1 parameter (title)"
        );
        return;
    }
    ring_check_string!(p, 1);

    let title = ring_get_string!(p, 1);
    let filters: Vec<(String, Vec<String>)> = if paracount >= 2 && ring_api_islist(p, 2) {
        parse_file_filters(p, 2)
    } else {
        vec![]
    };

    let files = slint::file_dialog_open_multiple(title, &filters);
    let list = ring_api_newlist(p);
    for file in files {
        ring_list_addstring2(list, file.as_bytes());
    }
    ring_ret_list!(p, list);
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_file_save, |p| {
    let paracount = ring_api_paracount(p);
    if paracount < 1 {
        ring_error!(p, "slint_file_save() expects at least 1 parameter (title)");
        return;
    }
    ring_check_string!(p, 1);

    let title = ring_get_string!(p, 1);
    let default_name = if paracount >= 2 {
        ring_check_string!(p, 2);
        ring_get_string!(p, 2)
    } else {
        ""
    };
    let filters: Vec<(String, Vec<String>)> = if paracount >= 3 && ring_api_islist(p, 3) {
        parse_file_filters(p, 3)
    } else {
        vec![]
    };

    if let Some(path) = slint::file_dialog_save(title, default_name, &filters) {
        ring_ret_string!(p, &path);
    }
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_folder_open, |p| {
    ring_check_paracount!(p, 1);
    ring_check_string!(p, 1);

    let title = ring_get_string!(p, 1);
    if let Some(path) = slint::folder_dialog(title) {
        ring_ret_string!(p, &path);
    }
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_folder_open_multiple, |p| {
    ring_check_paracount!(p, 1);
    ring_check_string!(p, 1);

    let title = ring_get_string!(p, 1);
    let folders = slint::folder_dialog_multiple(title);
    let list = ring_api_newlist(p);
    for folder in folders {
        ring_list_addstring2(list, folder.as_bytes());
    }
    ring_ret_list!(p, list);
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_msgbox, |p| {
    let paracount = ring_api_paracount(p);
    if paracount < 2 {
        ring_error!(
            p,
            "slint_msgbox() expects at least 2 parameters (title, message)"
        );
        return;
    }
    ring_check_string!(p, 1);
    ring_check_string!(p, 2);

    let title = ring_get_string!(p, 1);
    let message = ring_get_string!(p, 2);
    let msg_type = if paracount >= 3 {
        ring_check_string!(p, 3);
        ring_get_string!(p, 3)
    } else {
        "info"
    };

    match msg_type {
        "warning" | "warn" => slint::message_dialog_warning(title, message),
        "error" | "err" => slint::message_dialog_error(title, message),
        _ => slint::message_dialog_info(title, message),
    }
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_confirm, |p| {
    ring_check_paracount!(p, 2);
    ring_check_string!(p, 1);
    ring_check_string!(p, 2);

    let title = ring_get_string!(p, 1);
    let message = ring_get_string!(p, 2);
    let result = slint::message_dialog_confirm(title, message);
    ring_ret_number!(p, if result { 1.0 } else { 0.0 });
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_yesno, |p| {
    ring_check_paracount!(p, 2);
    ring_check_string!(p, 1);
    ring_check_string!(p, 2);

    let title = ring_get_string!(p, 1);
    let message = ring_get_string!(p, 2);
    let result = slint::message_dialog_yes_no(title, message);
    ring_ret_number!(p, if result { 1.0 } else { 0.0 });
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_notify, |p| {
    ring_check_paracount!(p, 2);
    ring_check_string!(p, 1);
    ring_check_string!(p, 2);

    let summary = ring_get_string!(p, 1);
    let body = ring_get_string!(p, 2);
    if let Err(e) = slint::notify(summary, body) {
        ring_error!(p, &e);
    }
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_notify_with_timeout, |p| {
    ring_check_paracount!(p, 3);
    ring_check_string!(p, 1);
    ring_check_string!(p, 2);
    ring_check_number!(p, 3);

    let summary = ring_get_string!(p, 1);
    let body = ring_get_string!(p, 2);
    let timeout = ring_get_int!(p, 3);
    if let Err(e) = slint::notify_with_timeout(summary, body, timeout) {
        ring_error!(p, &e);
    }
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_notify_with_icon, |p| {
    ring_check_paracount!(p, 3);
    ring_check_string!(p, 1);
    ring_check_string!(p, 2);
    ring_check_string!(p, 3);

    let summary = ring_get_string!(p, 1);
    let body = ring_get_string!(p, 2);
    let icon = ring_get_string!(p, 3);
    if let Err(e) = slint::notify_with_icon(summary, body, icon) {
        ring_error!(p, &e);
    }
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_notify_full, |p| {
    ring_check_paracount!(p, 4);
    ring_check_string!(p, 1);
    ring_check_string!(p, 2);
    ring_check_string!(p, 3);
    ring_check_number!(p, 4);

    let summary = ring_get_string!(p, 1);
    let body = ring_get_string!(p, 2);
    let icon = ring_get_string!(p, 3);
    let timeout = ring_get_int!(p, 4);
    if let Err(e) = slint::notify_full(summary, body, icon, timeout) {
        ring_error!(p, &e);
    }
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_clipboard_get, |p| {
    ring_check_paracount!(p, 0);
    if let Ok(text) = slint::clipboard_get_text() {
        ring_ret_string!(p, &text);
    }
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_clipboard_set, |p| {
    ring_check_paracount!(p, 1);
    ring_check_string!(p, 1);

    let text = ring_get_string!(p, 1);
    if let Err(e) = slint::clipboard_set_text(text) {
        ring_error!(p, &e);
    }
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_clipboard_clear, |p| {
    ring_check_paracount!(p, 0);
    if let Err(e) = slint::clipboard_clear() {
        ring_error!(p, &e);
    }
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_hotkey_register, |p| {
    ring_check_paracount!(p, 3);
    ring_check_string!(p, 1);
    ring_check_string!(p, 2);
    ring_check_string!(p, 3);

    let modifiers = ring_get_string!(p, 1);
    let key = ring_get_string!(p, 2);
    let callback = ring_get_string!(p, 3).to_string();
    match slint::hotkey_register(modifiers, key, p as ring_lang_rs::RingVM, callback) {
        Ok(id) => ring_ret_number!(p, id as f64),
        Err(e) => ring_error!(p, &e),
    }
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_hotkey_unregister, |p| {
    ring_check_paracount!(p, 1);
    ring_check_number!(p, 1);

    let id = ring_get_int!(p, 1) as u32;
    if let Err(e) = slint::hotkey_unregister(id) {
        ring_error!(p, &e);
    }
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_hotkey_unregister_all, |p| {
    ring_check_paracount!(p, 0);
    if let Err(e) = slint::hotkey_unregister_all() {
        ring_error!(p, &e);
    }
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_hotkey_poll, |p| {
    ring_check_paracount!(p, 0);
    slint::hotkey_poll();
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_tray_create, |p| {
    ring_check_paracount!(p, 1);
    ring_check_string!(p, 1);

    let tooltip = ring_get_string!(p, 1);
    if let Err(e) = slint::tray_create(tooltip) {
        ring_error!(p, &e);
    }
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_tray_create_with_icon, |p| {
    ring_check_paracount!(p, 2);
    ring_check_string!(p, 1);
    ring_check_string!(p, 2);

    let tooltip = ring_get_string!(p, 1);
    let icon_path = ring_get_string!(p, 2);
    if let Err(e) = slint::tray_create_with_icon(tooltip, icon_path) {
        ring_error!(p, &e);
    }
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_tray_set_icon, |p| {
    ring_check_paracount!(p, 1);
    ring_check_string!(p, 1);

    let icon_path = ring_get_string!(p, 1);
    if let Err(e) = slint::tray_set_icon(icon_path) {
        ring_error!(p, &e);
    }
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_tray_set_tooltip, |p| {
    ring_check_paracount!(p, 1);
    ring_check_string!(p, 1);

    let tooltip = ring_get_string!(p, 1);
    if let Err(e) = slint::tray_set_tooltip(tooltip) {
        ring_error!(p, &e);
    }
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_tray_add_item, |p| {
    ring_check_paracount!(p, 2);
    ring_check_string!(p, 1);
    ring_check_string!(p, 2);

    let label = ring_get_string!(p, 1);
    let callback = ring_get_string!(p, 2).to_string();
    match slint::tray_add_menu_item(label, p as ring_lang_rs::RingVM, callback) {
        Ok(id) => ring_ret_number!(p, id as f64),
        Err(e) => ring_error!(p, &e),
    }
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_tray_add_separator, |p| {
    ring_check_paracount!(p, 0);
    if let Err(e) = slint::tray_add_separator() {
        ring_error!(p, &e);
    }
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_tray_destroy, |p| {
    ring_check_paracount!(p, 0);
    slint::tray_destroy();
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_tray_poll, |p| {
    ring_check_paracount!(p, 0);
    slint::tray_poll();
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_window_set_always_on_top, |p| {
    ring_check_paracount!(p, 2);
    ring_check_cpointer!(p, 1);
    ring_check_number!(p, 2);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let always_on_top = ring_get_int!(p, 2) != 0;
        slint::window_set_always_on_top(&wrapper.instance, always_on_top);
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
ring_func!(ring_slint_window_set_icon, |p| {
    ring_check_paracount!(p, 2);
    ring_check_cpointer!(p, 1);
    ring_check_string!(p, 2);

    if let Some(wrapper) = ring_get_pointer!(
        p,
        1,
        slint::SlintInstanceWrapper,
        slint::SLINT_INSTANCE_TYPE
    ) {
        let icon_path = ring_get_string!(p, 2);
        if let Err(e) = slint::window_set_icon(&wrapper.instance, icon_path) {
            ring_error!(p, &e);
        }
    } else {
        ring_error!(p, "Invalid SlintInstance pointer");
    }
});

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn parse_file_filters(p: *mut libc::c_void, param: usize) -> Vec<(String, Vec<String>)> {
    let mut filters = Vec::new();
    let list = ring_api_getlist(p, param as i32);
    let count = ring_list_getsize(list);

    for i in 1..=count {
        if ring_list_islist(list, i) {
            let filter_list = ring_list_getlist(list, i);
            let filter_count = ring_list_getsize(filter_list);
            if filter_count >= 2 && ring_list_isstring(filter_list, 1) {
                let name = ring_list_getstring_str(filter_list, 1).to_string();
                let mut exts = Vec::new();
                for j in 2..=filter_count {
                    if ring_list_isstring(filter_list, j) {
                        exts.push(ring_list_getstring_str(filter_list, j).to_string());
                    }
                }
                filters.push((name, exts));
            }
        }
    }
    filters
}

ring_libinit! {
    "slint_set_style" => ring_slint_set_style,
    "slint_get_style" => ring_slint_get_style,
    "slint_add_library_path" => ring_slint_add_library_path,
    "slint_remove_library_path" => ring_slint_remove_library_path,
    "slint_clear_library_paths" => ring_slint_clear_library_paths,
    "slint_load" => ring_slint_load,
    "slint_loadstring" => ring_slint_loadstring,
    "slint_create" => ring_slint_create,
    "slint_show" => ring_slint_show,
    "slint_hide" => ring_slint_hide,
    "slint_run" => ring_slint_run,
    "slint_quit" => ring_slint_quit,
    "slint_get" => ring_slint_get,
    "slint_set" => ring_slint_set,
    "slint_set_bool" => ring_slint_set_bool,
    "slint_set_image" => ring_slint_set_image,
    "slint_set_string" => ring_slint_set_string,
    "slint_set_number" => ring_slint_set_number,
    "slint_set_color" => ring_slint_set_color,
    "slint_set_enum" => ring_slint_set_enum,
    "slint_on" => ring_slint_on,
    "slint_invoke" => ring_slint_invoke,
    "slint_callback_arg" => ring_slint_callback_arg,
    "slint_callback_args_count" => ring_slint_callback_args_count,
    "slint_global_get" => ring_slint_global_get,
    "slint_global_set" => ring_slint_global_set,
    "slint_global_on" => ring_slint_global_on,
    "slint_global_invoke" => ring_slint_global_invoke,
    "slint_timer_start" => ring_slint_timer_start,
    "slint_timer_stop" => ring_slint_timer_stop,
    "slint_timer_running" => ring_slint_timer_running,
    "slint_timer_restart" => ring_slint_timer_restart,
    "slint_timer_set_interval" => ring_slint_timer_set_interval,
    "slint_model_create" => ring_slint_model_create,
    "slint_model_push" => ring_slint_model_push,
    "slint_model_remove" => ring_slint_model_remove,
    "slint_model_set" => ring_slint_model_set,
    "slint_model_count" => ring_slint_model_count,
    "slint_model_clear" => ring_slint_model_clear,
    "slint_model_insert" => ring_slint_model_insert,
    "slint_model_destroy" => ring_slint_model_destroy,
    "slint_window_set_minimized" => ring_slint_window_set_minimized,
    "slint_window_is_minimized" => ring_slint_window_is_minimized,
    "slint_window_set_maximized" => ring_slint_window_set_maximized,
    "slint_window_is_maximized" => ring_slint_window_is_maximized,
    "slint_window_set_fullscreen" => ring_slint_window_set_fullscreen,
    "slint_window_is_fullscreen" => ring_slint_window_is_fullscreen,
    "slint_window_set_position" => ring_slint_window_set_position,
    "slint_window_get_position" => ring_slint_window_get_position,
    "slint_window_set_size" => ring_slint_window_set_size,
    "slint_window_get_size" => ring_slint_window_get_size,
    "slint_window_scale_factor" => ring_slint_window_scale_factor,
    "slint_window_is_visible" => ring_slint_window_is_visible,
    "slint_window_request_redraw" => ring_slint_window_request_redraw,
    "slint_definition_name" => ring_slint_definition_name,
    "slint_definition_properties" => ring_slint_definition_properties,
    "slint_definition_callbacks" => ring_slint_definition_callbacks,
    "slint_definition_functions" => ring_slint_definition_functions,
    "slint_definition_globals" => ring_slint_definition_globals,
    "slint_model_get" => ring_slint_model_get,
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        "slint_window_drag" => ring_slint_window_drag,
        "slint_file_open" => ring_slint_file_open,
        "slint_file_open_multiple" => ring_slint_file_open_multiple,
        "slint_file_save" => ring_slint_file_save,
        "slint_folder_open" => ring_slint_folder_open,
        "slint_folder_open_multiple" => ring_slint_folder_open_multiple,
        "slint_msgbox" => ring_slint_msgbox,
        "slint_confirm" => ring_slint_confirm,
        "slint_yesno" => ring_slint_yesno,
        "slint_notify" => ring_slint_notify,
        "slint_notify_with_timeout" => ring_slint_notify_with_timeout,
        "slint_notify_with_icon" => ring_slint_notify_with_icon,
        "slint_notify_full" => ring_slint_notify_full,
        "slint_clipboard_get" => ring_slint_clipboard_get,
        "slint_clipboard_set" => ring_slint_clipboard_set,
        "slint_clipboard_clear" => ring_slint_clipboard_clear,
        "slint_hotkey_register" => ring_slint_hotkey_register,
        "slint_hotkey_unregister" => ring_slint_hotkey_unregister,
        "slint_hotkey_unregister_all" => ring_slint_hotkey_unregister_all,
        "slint_hotkey_poll" => ring_slint_hotkey_poll,
        "slint_tray_create" => ring_slint_tray_create,
        "slint_tray_create_with_icon" => ring_slint_tray_create_with_icon,
        "slint_tray_set_icon" => ring_slint_tray_set_icon,
        "slint_tray_set_tooltip" => ring_slint_tray_set_tooltip,
        "slint_tray_add_item" => ring_slint_tray_add_item,
        "slint_tray_add_separator" => ring_slint_tray_add_separator,
        "slint_tray_destroy" => ring_slint_tray_destroy,
        "slint_tray_poll" => ring_slint_tray_poll,
        "slint_window_set_always_on_top" => ring_slint_window_set_always_on_top,
        "slint_window_set_icon" => ring_slint_window_set_icon,
    },
}
