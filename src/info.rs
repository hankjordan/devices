use crate::path::DevicePath;

/// Device information. Use accessors to extract information about connected devices.
#[derive(Debug)]
pub struct DeviceInfo {
    path: DevicePath,

    class: String,
    vendor: String,
    product: String,
    manufacturer: Option<String>,

    class_id: u16,
    vendor_id: u16,
    product_id: u16,
    manufacturer_id: Option<u16>,
}

impl DeviceInfo {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        path: DevicePath,
        class: &str,
        vendor: &str,
        product: &str,
        manufacturer: Option<&str>,
        class_id: u16,
        vendor_id: u16,
        product_id: u16,
        manufacturer_id: Option<u16>,
    ) -> Self {
        Self {
            path,
            class: class.to_owned(),
            vendor: vendor.to_owned(),
            product: product.to_owned(),
            manufacturer: manufacturer.map(|s| s.to_owned()),
            class_id,
            vendor_id,
            product_id,
            manufacturer_id
        }
    }

    /// Returns the path where the device is mounted.
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
    pub fn class_id(&self) -> u16 {
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