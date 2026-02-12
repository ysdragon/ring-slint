#![cfg(not(any(target_os = "android", target_os = "ios")))]

use rfd::{FileDialog, MessageButtons, MessageDialog, MessageLevel};

pub fn file_dialog_open(title: &str, filters: &[(String, Vec<String>)]) -> Option<String> {
    let mut dialog = FileDialog::new().set_title(title);

    for (name, exts) in filters {
        let ext_refs: Vec<&str> = exts.iter().map(|s| s.as_str()).collect();
        dialog = dialog.add_filter(name, &ext_refs);
    }

    dialog.pick_file().map(|p| p.to_string_lossy().to_string())
}

pub fn file_dialog_open_multiple(title: &str, filters: &[(String, Vec<String>)]) -> Vec<String> {
    let mut dialog = FileDialog::new().set_title(title);

    for (name, exts) in filters {
        let ext_refs: Vec<&str> = exts.iter().map(|s| s.as_str()).collect();
        dialog = dialog.add_filter(name, &ext_refs);
    }

    dialog
        .pick_files()
        .map(|files| {
            files
                .into_iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect()
        })
        .unwrap_or_default()
}

pub fn file_dialog_save(
    title: &str,
    default_name: &str,
    filters: &[(String, Vec<String>)],
) -> Option<String> {
    let mut dialog = FileDialog::new()
        .set_title(title)
        .set_file_name(default_name);

    for (name, exts) in filters {
        let ext_refs: Vec<&str> = exts.iter().map(|s| s.as_str()).collect();
        dialog = dialog.add_filter(name, &ext_refs);
    }

    dialog.save_file().map(|p| p.to_string_lossy().to_string())
}

pub fn folder_dialog(title: &str) -> Option<String> {
    FileDialog::new()
        .set_title(title)
        .pick_folder()
        .map(|p| p.to_string_lossy().to_string())
}

pub fn folder_dialog_multiple(title: &str) -> Vec<String> {
    FileDialog::new()
        .set_title(title)
        .pick_folders()
        .map(|folders| {
            folders
                .into_iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect()
        })
        .unwrap_or_default()
}

pub fn message_dialog_info(title: &str, message: &str) {
    MessageDialog::new()
        .set_level(MessageLevel::Info)
        .set_title(title)
        .set_description(message)
        .set_buttons(MessageButtons::Ok)
        .show();
}

pub fn message_dialog_warning(title: &str, message: &str) {
    MessageDialog::new()
        .set_level(MessageLevel::Warning)
        .set_title(title)
        .set_description(message)
        .set_buttons(MessageButtons::Ok)
        .show();
}

pub fn message_dialog_error(title: &str, message: &str) {
    MessageDialog::new()
        .set_level(MessageLevel::Error)
        .set_title(title)
        .set_description(message)
        .set_buttons(MessageButtons::Ok)
        .show();
}

pub fn message_dialog_confirm(title: &str, message: &str) -> bool {
    MessageDialog::new()
        .set_level(MessageLevel::Info)
        .set_title(title)
        .set_description(message)
        .set_buttons(MessageButtons::OkCancel)
        .show()
        == rfd::MessageDialogResult::Ok
}

pub fn message_dialog_yes_no(title: &str, message: &str) -> bool {
    MessageDialog::new()
        .set_level(MessageLevel::Info)
        .set_title(title)
        .set_description(message)
        .set_buttons(MessageButtons::YesNo)
        .show()
        == rfd::MessageDialogResult::Yes
}
