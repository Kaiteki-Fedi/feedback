use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackForm {
    pub category: String,
    pub email: Option<String>,
    pub message: String,
    pub device_details: Option<DeviceDetailsForm>,
    pub exceptions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDetailsForm {
    pub version_name: String,
    pub platform: Platform,
    pub platform_version: String,
    pub branch: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Platform {
    Android,
    iOS,
    Linux,
    Windows,
    Fuchsia,
    Web,
    macOS,
    Other,
}
