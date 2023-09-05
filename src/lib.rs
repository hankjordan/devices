#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![doc = include_str!("../README.md")]

mod error;
mod info;
mod path;

use cfg_if::cfg_if;
pub use error::Error;
pub use info::DeviceInfo;
pub use path::DevicePath;

#[cfg(unix)]
mod linux;

#[cfg(windows)]
mod win32;

/// Information about system devices.
pub struct Devices;

impl Devices {
    /// Retrieve a list of all connected devices.
    /// # Errors
    /// If the platform is unsupported or there is an issue retrieving the list of devices, an error is returned.
    pub fn get() -> Result<Vec<DeviceInfo>, Error> {
        let mut devices = Self::pci()?;
        devices.extend(Self::usb()?);

        Ok(devices)
    }
    
    /// Retrieve a list of all connected PCI devices.
    /// # Errors
    /// If the platform is unsupported or there is an issue retrieving the list of devices, an error is returned.
    pub fn pci() -> Result<Vec<DeviceInfo>, Error> {
        cfg_if! {
            if #[cfg(unix)] {
                linux::get_pci()
            } else if #[cfg(windows)] {
                win32::get_pci()
            } else {
                Err(Error::UnsupportedPlatform)
            }
        }
    }

    /// Retrieve a list of all connected USB devices.
    /// # Errors
    /// If the platform is unsupported or there is an issue retrieving the list of devices, an error is returned.
    pub fn usb() -> Result<Vec<DeviceInfo>, Error> {
        cfg_if! {
            if #[cfg(unix)] {
                linux::get_usb()
            } else if #[cfg(windows)] {
                win32::get_usb()
            } else {
                Err(Error::UnsupportedPlatform)
            }
        }
    }
}
