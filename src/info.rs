#[derive(Debug)]
pub struct DeviceInfo {
    path: String,

    class: String,
    vendor: String,
    product: String,

    class_id: u16,
    vendor_id: u16,
    product_id: u16,
}

impl DeviceInfo {
    pub(crate) fn new(
        path: &str,
        class: &str,
        vendor: &str,
        product: &str,
        class_id: u16,
        vendor_id: u16,
        product_id: u16
    ) -> Self {
        Self {
            path: path.to_owned(),
            class: class.to_owned(),
            vendor: vendor.to_owned(),
            product: product.to_owned(),
            class_id,
            vendor_id,
            product_id
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn class(&self) -> &str {
        &self.class
    }

    pub fn vendor(&self) -> &str {
        &self.vendor
    }

    pub fn product(&self) -> &str {
        &self.product
    }

    pub fn class_id(&self) -> u16 {
        self.class_id
    }

    pub fn vendor_id(&self) -> u16 {
        self.vendor_id
    }

    pub fn product_id(&self) -> u16 {
        self.product_id
    }
}