pub(crate) fn get_text() -> Option<String> {
    android_clipboard::get_text().ok()
}
