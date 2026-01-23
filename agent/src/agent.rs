use salsa_macos::permissions::{self, PermissionStatus};

pub struct Agent {
    pub accessibility: PermissionStatus,
    pub input_monitoring: PermissionStatus,
}

impl Agent {
    pub fn new() -> Self {
        let accessibility = permissions::accessibility_status();
        let input_monitoring = permissions::input_monitoring_status();
        Self {
            accessibility,
            input_monitoring,
        }
    }

    pub fn run(&self) {
        println!("accessibility: {:?}", self.accessibility);
        println!("input monitoring: {:?}", self.input_monitoring);
        println!("agent loop not wired yet");
    }
}
