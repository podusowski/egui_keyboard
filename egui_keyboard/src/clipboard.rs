#[cfg(target_os = "android")]
pub(crate) fn get_text() -> Option<String> {
    android_clipboard::get_text().ok()
}

#[cfg(not(target_os = "android"))]
pub(crate) fn get_text() -> Option<String> {
    None
}
