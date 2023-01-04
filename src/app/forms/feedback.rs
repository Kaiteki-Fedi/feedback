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

impl TryFrom<i32> for Platform {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == Platform::Android as i32 => Ok(Platform::Android),
            x if x == Platform::iOS as i32 => Ok(Platform::iOS),
            x if x == Platform::Linux as i32 => Ok(Platform::Linux),
            x if x == Platform::Windows as i32 => Ok(Platform::Windows),
            x if x == Platform::Fuchsia as i32 => Ok(Platform::Fuchsia),
            x if x == Platform::Web as i32 => Ok(Platform::Web),
            x if x == Platform::macOS as i32 => Ok(Platform::macOS),
            x if x == Platform::Other as i32 => Ok(Platform::Other),
            _ => Err(()),
        }
    }
}
