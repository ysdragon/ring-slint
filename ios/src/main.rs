use ring_lang_rs::*;
use std::path::PathBuf;

fn main() {
    eprintln!("[ring-slint] Starting...");

    let script_path = get_script_path();
    eprintln!(
        "[ring-slint] Script: {:?} exists={}",
        script_path,
        script_path.exists()
    );

    // Register Ring Slint extension
    ring_register_extension(ring_slint::ringlib_init);

    let state = ring_state_new();
    if state.is_null() {
        eprintln!("[ring-slint] Failed to create Ring state");
        return;
    }

    eprintln!("[ring-slint] Running script...");
    ring_state_runfile_str(state, script_path.to_str().unwrap());
    eprintln!("[ring-slint] Script finished");

    ring_state_delete(state);
}

fn get_script_path() -> PathBuf {
    if let Ok(exe) = std::env::current_exe() {
        eprintln!("[ring-slint] Executable: {:?}", exe);
        if let Some(bundle_dir) = exe.parent() {
            let script = bundle_dir.join("main.ring");
            if script.exists() {
                let _ = std::env::set_current_dir(bundle_dir);
                return script;
            }
        }
    }
    PathBuf::from("resources/main.ring")
}
