use crate::path::DevicePath;

/// Device information. Use accessors to extract information about connected devices.
#[cfg_attr(feature = "bincode", derive(bincode::Decode, bincode::Encode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DeviceInfo {
    pub(crate) path: DevicePath,

    pub(crate) class: String,
    pub(crate) vendor: String,
    pub(crate) product: String,
    pub(crate) manufacturer: Option<String>,

    pub(crate) class_id: Option<u16>,
    pub(crate) vendor_id: u16,
    pub(crate) product_id: u16,
    pub(crate) manufacturer_id: Option<u16>,
}

impl DeviceInfo {
    /// Returns the path where the device is mounted.
    ///
    /// Also known as `Location` on Windows.
    pub fn path(&self) -> &DevicePath {
        &self.path
    }

    /// Returns the class name of the device.
    pub fn class(&self) -> &str {
        &self.class
    }

    /// Returns the name of the device's vendor.
    pub fn vendor(&self) -> &str {
        &self.vendor
    }

    /// Returns the device's product name.
    pub fn product(&self) -> &str {
        &self.product
    }

    /// Returns the name of the device's manufacturer, if known.
    /// # Note
    /// Always returns `None` for PCI devices.
    pub fn manufacturer(&self) -> &Option<String> {
        &self.manufacturer
    }

    /// Returns the class id of the device.
    /// # Note
    /// Always returns `None` on Windows.
    pub fn class_id(&self) -> Option<u16> {
        self.class_id
    }

    /// Returns the id of the device's vendor.
    pub fn vendor_id(&self) -> u16 {
        self.vendor_id
    }

    /// Returns the device's product id.
    pub fn product_id(&self) -> u16 {
        self.product_id
    }

    /// Returns the id of the device's manufacturer, if known.
    /// # Note
    /// Always returns `None` for PCI devices.
    pub fn manufacturer_id(&self) -> Option<u16> {
        self.manufacturer_id
    }
}
