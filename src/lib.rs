use crate::info::DeviceInfo;

mod error;
pub mod info;

#[cfg(target_os = "windows")]
mod win32;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux::*;

pub struct Devices;

impl Devices {
    pub fn get() -> Vec<DeviceInfo> {
        let mut devices = Vec::new();

        devices.extend(lspci().unwrap_or_default());

        devices
    }
}
