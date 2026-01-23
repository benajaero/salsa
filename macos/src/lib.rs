pub mod permissions {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum PermissionStatus {
        Unknown,
        Granted,
        Denied,
    }

    pub fn accessibility_status() -> PermissionStatus {
        PermissionStatus::Unknown
    }

    pub fn input_monitoring_status() -> PermissionStatus {
        PermissionStatus::Unknown
    }
}

pub fn is_supported_platform() -> bool {
    cfg!(target_os = "macos")
}
