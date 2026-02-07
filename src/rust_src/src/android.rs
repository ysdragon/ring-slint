#![cfg(target_os = "android")]

use once_cell::sync::OnceCell;
use ring_lang_rs::*;
use std::ffi::CString;
use std::io::Read;
use std::path::PathBuf;

static ANDROID_APP: OnceCell<slint::android::AndroidApp> = OnceCell::new();
static ASSETS_DIR: OnceCell<PathBuf> = OnceCell::new();

pub fn set_android_app(app: slint::android::AndroidApp) {
    let _ = ANDROID_APP.set(app);
}

pub fn get_android_app() -> Option<&'static slint::android::AndroidApp> {
    ANDROID_APP.get()
}

pub fn get_assets_dir() -> Option<&'static PathBuf> {
    ASSETS_DIR.get()
}

fn load_asset_text(app: &slint::android::AndroidApp, path: &str) -> Option<String> {
    let asset_manager = app.asset_manager();
    let cpath = CString::new(path).ok()?;
    let mut asset = asset_manager.open(&cpath)?;
    let mut content = String::new();
    asset.read_to_string(&mut content).ok()?;
    Some(content)
}

fn load_asset_bytes(app: &slint::android::AndroidApp, path: &str) -> Option<Vec<u8>> {
    let asset_manager = app.asset_manager();
    let cpath = CString::new(path).ok()?;
    let mut asset = asset_manager.open(&cpath)?;
    let mut content = Vec::new();
    asset.read_to_end(&mut content).ok()?;
    Some(content)
}

pub fn load_asset_content(path: &str) -> Option<String> {
    let app = get_android_app()?;
    load_asset_text(app, path)
}

fn extract_asset_to_file(
    app: &slint::android::AndroidApp,
    asset_path: &str,
    dest: &PathBuf,
) -> bool {
    if let Some(bytes) = load_asset_bytes(app, asset_path) {
        if let Some(parent) = dest.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if std::fs::write(dest, &bytes).is_ok() {
            log::debug!("Extracted {} -> {}", asset_path, dest.display());
            return true;
        }
    }
    log::warn!("Failed to extract asset: {}", asset_path);
    false
}

fn extract_all_assets(app: &slint::android::AndroidApp, dest_dir: &PathBuf) -> bool {
    extract_assets_recursive(app, "", dest_dir)
}

fn extract_assets_recursive(
    app: &slint::android::AndroidApp,
    subdir: &str,
    dest_dir: &PathBuf,
) -> bool {
    let asset_manager = app.asset_manager();
    let path = CString::new(subdir).unwrap_or_else(|_| CString::new("").unwrap());

    let Some(mut dir) = asset_manager.open_dir(&path) else {
        log::debug!("Could not open asset dir: '{}'", subdir);
        return true;
    };

    while let Some(entry) = dir.next() {
        let name = entry.to_string_lossy();
        let asset_path = if subdir.is_empty() {
            name.to_string()
        } else {
            format!("{}/{}", subdir, name)
        };

        let cpath = match CString::new(asset_path.as_str()) {
            Ok(p) => p,
            Err(_) => continue,
        };

        if asset_manager.open(&cpath).is_some() {
            let dest = dest_dir.join(&asset_path);
            extract_asset_to_file(app, &asset_path, &dest);
        } else {
            let sub_dest = dest_dir.join(&asset_path);
            let _ = std::fs::create_dir_all(&sub_dest);
            extract_assets_recursive(app, &asset_path, dest_dir);
        }
    }
    true
}

pub fn run_ring_file(file_path: &str, working_dir: &std::path::Path) -> Result<(), String> {
    if let Err(e) = std::env::set_current_dir(working_dir) {
        log::warn!("Failed to set working directory: {}", e);
    }

    ring_register_extension(crate::ringlib_init);

    let state = ring_state_new();
    if state.is_null() {
        return Err("Failed to create Ring state".to_string());
    }

    let result = ring_state_runfile_str(state, file_path);
    log::info!("ring_state_runfile returned: {}", result);
    ring_state_delete(state);

    Ok(())
}

#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_tag("RingSlint"),
    );

    log::info!("Ring-Slint Android starting...");

    set_android_app(app.clone());

    if let Err(e) = slint::android::init(app.clone()) {
        log::error!("Failed to init Slint: {:?}", e);
        return;
    }

    let assets_dir = app
        .internal_data_path()
        .map(|p| p.join("assets"))
        .unwrap_or_else(|| PathBuf::from("/data/local/tmp/ring_slint"));

    log::info!("Assets directory: {}", assets_dir.display());

    let _ = std::fs::create_dir_all(&assets_dir);
    extract_all_assets(&app, &assets_dir);

    let _ = ASSETS_DIR.set(assets_dir.clone());
    log::info!("Assets extracted");

    let main_ring = assets_dir.join("main.ring");
    if !main_ring.exists() {
        log::error!("main.ring not found in assets");
        return;
    }

    log::info!("Running: {}", main_ring.display());
    if let Err(e) = run_ring_file(main_ring.to_str().unwrap_or("main.ring"), &assets_dir) {
        log::error!("Ring error: {}", e);
    }
}
