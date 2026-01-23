pub fn is_supported_platform() -> bool {
    cfg!(target_os = "macos")
}
