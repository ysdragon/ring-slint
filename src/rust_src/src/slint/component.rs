use crate::slint::SlintInstanceWrapper;
use slint_interpreter::{ComponentHandle, ComponentInstance, Value};

pub fn create_instance(
    definition: &slint_interpreter::ComponentDefinition,
    vm_ptr: *mut libc::c_void,
) -> Result<SlintInstanceWrapper, String> {
    let instance = definition
        .create()
        .map_err(|e| format!("Failed to create instance: {:?}", e))?;
    Ok(SlintInstanceWrapper::new(instance, vm_ptr))
}

pub fn instance_get_property(instance: &ComponentInstance, name: &str) -> Result<Value, String> {
    instance
        .get_property(name)
        .map_err(|e| format!("Failed to get property '{}': {:?}", name, e))
}

pub fn instance_set_property(
    instance: &ComponentInstance,
    name: &str,
    value: Value,
) -> Result<(), String> {
    instance
        .set_property(name, value)
        .map_err(|e| format!("Failed to set property '{}': {:?}", name, e))
}

pub fn instance_invoke(
    instance: &ComponentInstance,
    name: &str,
    args: &[Value],
) -> Result<Value, String> {
    instance.invoke(name, args).map_err(|e| {
        format!(
            "Failed to invoke '{}' with {} args: {:?}",
            name,
            args.len(),
            e
        )
    })
}

pub fn instance_show(instance: &ComponentInstance) -> Result<(), String> {
    instance
        .show()
        .map_err(|e| format!("Failed to show window: {:?}", e))
}

pub fn instance_hide(instance: &ComponentInstance) -> Result<(), String> {
    instance
        .hide()
        .map_err(|e| format!("Failed to hide window: {:?}", e))
}

pub fn instance_run(instance: &ComponentInstance) -> Result<(), String> {
    instance
        .run()
        .map_err(|e| format!("Failed to run event loop: {:?}", e))
}

pub fn get_global_property(
    instance: &ComponentInstance,
    global: &str,
    property: &str,
) -> Result<Value, String> {
    instance.get_global_property(global, property).map_err(|e| {
        format!(
            "Failed to get property '{}' from global '{}': {:?}",
            property, global, e
        )
    })
}

pub fn set_global_property(
    instance: &ComponentInstance,
    global: &str,
    property: &str,
    value: Value,
) -> Result<(), String> {
    instance
        .set_global_property(global, property, value)
        .map_err(|e| {
            format!(
                "Failed to set property '{}' on global '{}': {:?}",
                property, global, e
            )
        })
}

pub fn invoke_global(
    instance: &ComponentInstance,
    global: &str,
    name: &str,
    args: &[Value],
) -> Result<Value, String> {
    instance.invoke_global(global, name, args).map_err(|e| {
        format!(
            "Failed to invoke '{}' on global '{}' with {} args: {:?}",
            name,
            global,
            args.len(),
            e
        )
    })
}

pub fn window_set_minimized(instance: &ComponentInstance, minimized: bool) {
    instance.window().set_minimized(minimized);
}

pub fn window_is_minimized(instance: &ComponentInstance) -> bool {
    instance.window().is_minimized()
}

pub fn window_set_maximized(instance: &ComponentInstance, maximized: bool) {
    instance.window().set_maximized(maximized);
}

pub fn window_is_maximized(instance: &ComponentInstance) -> bool {
    instance.window().is_maximized()
}

pub fn window_set_fullscreen(instance: &ComponentInstance, fullscreen: bool) {
    instance.window().set_fullscreen(fullscreen);
}

pub fn window_is_fullscreen(instance: &ComponentInstance) -> bool {
    instance.window().is_fullscreen()
}

pub fn window_set_position(instance: &ComponentInstance, x: i32, y: i32) {
    use slint_interpreter::PhysicalPosition;
    instance.window().set_position(PhysicalPosition::new(x, y));
}

pub fn window_get_position(instance: &ComponentInstance) -> (i32, i32) {
    let pos = instance.window().position();
    (pos.x, pos.y)
}

pub fn window_set_size(instance: &ComponentInstance, width: u32, height: u32) {
    use slint_interpreter::PhysicalSize;
    instance.window().set_size(PhysicalSize::new(width, height));
}

pub fn window_get_size(instance: &ComponentInstance) -> (u32, u32) {
    let size = instance.window().size();
    (size.width, size.height)
}

pub fn window_scale_factor(instance: &ComponentInstance) -> f32 {
    instance.window().scale_factor()
}

pub fn window_is_visible(instance: &ComponentInstance) -> bool {
    instance.window().is_visible()
}

pub fn window_request_redraw(instance: &ComponentInstance) {
    instance.window().request_redraw();
}

#[cfg(not(target_os = "android"))]
pub fn window_drag(instance: &ComponentInstance) -> Result<(), String> {
    use i_slint_backend_winit::WinitWindowAccessor;
    use slint_interpreter::ComponentHandle;

    let window = instance.window();

    let drag_result = window
        .with_winit_window(|winit_window| {
            winit_window
                .drag_window()
                .map_err(|e| format!("Failed to start window drag: {:?}", e))
        })
        .ok_or_else(|| "Window not available (not backed by winit)".to_string())?;

    // Wayland: compositor steals pointer during drag, so reset Slint's pointer state
    #[cfg(all(unix, not(target_os = "macos")))]
    if std::env::var("WAYLAND_DISPLAY").is_ok()
        || std::env::var("XDG_SESSION_TYPE")
            .map(|v| v == "wayland")
            .unwrap_or(false)
    {
        window.dispatch_event(i_slint_core::platform::WindowEvent::PointerReleased {
            position: i_slint_core::api::LogicalPosition::new(0.0, 0.0),
            button: i_slint_core::platform::PointerEventButton::Left,
        });
    }

    drag_result
}

#[cfg(not(target_os = "android"))]
pub fn window_set_always_on_top(instance: &ComponentInstance, always_on_top: bool) {
    use i_slint_backend_winit::WinitWindowAccessor;
    use slint_interpreter::ComponentHandle;

    instance.window().with_winit_window(|winit_window| {
        let level = if always_on_top {
            winit::window::WindowLevel::AlwaysOnTop
        } else {
            winit::window::WindowLevel::Normal
        };
        winit_window.set_window_level(level);
    });
}

#[cfg(not(target_os = "android"))]
pub fn window_set_icon(instance: &ComponentInstance, icon_path: &str) -> Result<(), String> {
    use i_slint_backend_winit::WinitWindowAccessor;
    use slint_interpreter::ComponentHandle;

    let image = image::open(icon_path)
        .map_err(|e| format!("Failed to load icon: {}", e))?
        .into_rgba8();
    let (width, height) = image.dimensions();
    let icon = winit::window::Icon::from_rgba(image.into_raw(), width, height)
        .map_err(|e| format!("Failed to create icon: {}", e))?;

    instance.window().with_winit_window(|winit_window| {
        winit_window.set_window_icon(Some(icon));
    });

    Ok(())
}
