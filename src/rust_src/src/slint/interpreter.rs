use once_cell::sync::Lazy;
use slint_interpreter::{Compiler, ComponentDefinition};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

static SLINT_STYLE: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));
static SLINT_LIBRARY_PATHS: Lazy<Mutex<HashMap<String, PathBuf>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn set_style(style: &str) {
    let mut guard = SLINT_STYLE.lock().unwrap_or_else(|e| e.into_inner());
    if style.is_empty() {
        *guard = None;
    } else {
        *guard = Some(style.to_string());
    }
}

pub fn get_style() -> Option<String> {
    SLINT_STYLE
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .clone()
}

pub fn add_library_path(name: &str, path: &str) {
    let mut guard = SLINT_LIBRARY_PATHS
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    guard.insert(name.to_string(), PathBuf::from(path));
}

pub fn remove_library_path(name: &str) {
    let mut guard = SLINT_LIBRARY_PATHS
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    guard.remove(name);
}

pub fn clear_library_paths() {
    let mut guard = SLINT_LIBRARY_PATHS
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    guard.clear();
}

fn create_compiler() -> Compiler {
    let mut compiler = Compiler::new();
    if let Some(style) = SLINT_STYLE
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .as_ref()
    {
        compiler.set_style(style.clone());
    }

    let library_paths = SLINT_LIBRARY_PATHS
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .clone();
    if !library_paths.is_empty() {
        compiler.set_library_paths(library_paths);
    }

    #[cfg(target_os = "android")]
    if let Some(assets_dir) = crate::android::get_assets_dir() {
        compiler.set_include_paths(vec![assets_dir.clone()]);
        log::debug!("Set include path to: {}", assets_dir.display());
    }

    compiler
}

pub fn compile_from_path(path: &str) -> Result<ComponentDefinition, String> {
    #[cfg(target_os = "android")]
    let actual_path = {
        if let Some(assets_dir) = crate::android::get_assets_dir() {
            let full_path = assets_dir.join(path);
            log::debug!("Android: loading from {}", full_path.display());
            full_path
        } else {
            PathBuf::from(path)
        }
    };

    #[cfg(not(target_os = "android"))]
    let actual_path = PathBuf::from(path);

    let compiler = create_compiler();
    let result = spin_on::spin_on(compiler.build_from_path(&actual_path));

    if result.has_errors() {
        let errors: Vec<String> = result
            .diagnostics()
            .map(|d| format!("{}: {}", d.line_column().0, d.message()))
            .collect();
        return Err(errors.join("\n"));
    }

    let name = result
        .component_names()
        .next()
        .ok_or_else(|| format!("No component found in '{}'", path))?;
    result
        .component(name)
        .ok_or_else(|| format!("Failed to get component '{}' from '{}'", name, path))
}

pub fn compile_from_source(source: &str, path: &str) -> Result<ComponentDefinition, String> {
    let compiler = create_compiler();
    let result =
        spin_on::spin_on(compiler.build_from_source(source.to_string(), PathBuf::from(path)));

    if result.has_errors() {
        let errors: Vec<String> = result
            .diagnostics()
            .map(|d| format!("{}: {}", d.line_column().0, d.message()))
            .collect();
        return Err(errors.join("\n"));
    }

    let name = result
        .component_names()
        .next()
        .ok_or_else(|| format!("No component found in source (path: '{}')", path))?;
    result
        .component(name)
        .ok_or_else(|| format!("Failed to get component '{}' from source", name))
}

pub fn get_component_by_name(path: &str, name: &str) -> Result<ComponentDefinition, String> {
    let compiler = create_compiler();
    let result = spin_on::spin_on(compiler.build_from_path(path));

    if result.has_errors() {
        let errors: Vec<String> = result
            .diagnostics()
            .map(|d| format!("{}: {}", d.line_column().0, d.message()))
            .collect();
        return Err(errors.join("\n"));
    }

    result
        .component(name)
        .ok_or_else(|| format!("Component '{}' not found", name))
}

pub fn definition_name(definition: &ComponentDefinition) -> String {
    definition.name().to_string()
}

pub fn definition_properties(definition: &ComponentDefinition) -> Vec<(String, String)> {
    definition
        .properties()
        .map(|(name, value_type)| (name, format!("{:?}", value_type)))
        .collect()
}

pub fn definition_callbacks(definition: &ComponentDefinition) -> Vec<String> {
    definition.callbacks().collect()
}

pub fn definition_functions(definition: &ComponentDefinition) -> Vec<String> {
    definition.functions().collect()
}

pub fn definition_globals(definition: &ComponentDefinition) -> Vec<String> {
    definition.globals().collect()
}
