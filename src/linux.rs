use std::process::Command;

use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    error::Error,
    info::DeviceInfo,
    path::DevicePath,
};

lazy_static! {
    static ref LSPCI_REGEX: Regex = Regex::new("(^.*? )|(\".*?\")").unwrap();
    static ref LSUSB_PATH_REGEX: Regex = Regex::new("Bus [0-9]+ Device [0-9]+").unwrap();
    static ref LSUSB_CLASS_REGEX: Regex = Regex::new("\\s*?bDeviceClass\\s*?.*?\\n").unwrap();
    static ref LSUSB_VENDOR_REGEX: Regex = Regex::new("\\s*?idVendor\\s*?.*?\\n").unwrap();
    static ref LSUSB_PRODUCT_REGEX: Regex = Regex::new("\\s*?iProduct\\s*?.*?\\n").unwrap();
    static ref LSUSB_PRODUCT_ID_REGEX: Regex = Regex::new("\\s*?idProduct\\s*?.*?\\n").unwrap();
    static ref LSUSB_MANUFACTURER_REGEX: Regex =
        Regex::new("\\s*?iManufacturer\\s*?.*?\\n").unwrap();
}

trait RSplitAt {
    fn rsplit_at(&self, mid: usize) -> (&str, &str);
}

impl RSplitAt for &str {
    fn rsplit_at(&self, mid: usize) -> (&str, &str) {
        self.split_at(self.len() - mid)
    }
}

fn id_from_raw(raw: &str) -> Result<u16, Error> {
    let trimmed = raw.trim().trim_matches('[').trim_matches(']');

    u16::from_str_radix(trimmed, 16).map_err(|_| Error::ParseError)
}

pub(crate) fn get_pci() -> Result<Vec<DeviceInfo>, Error> {
    let output = Command::new("lspci")
        .arg("-mm")
        .arg("-nn")
        .output()
        .map_err(|_| Error::CommandError)?;

    let output_str = String::from_utf8(output.stdout).map_err(|_| Error::ParseError)?;

    let mut devices = Vec::new();

    for line in output_str.trim().split('\n') {
        let mut matches = Vec::new();

        for m in LSPCI_REGEX.find_iter(line) {
            matches.push(m.as_str().trim().trim_matches('"'));
        }

        let (bus, next) = matches.first().ok_or(Error::ParseError)?.split_at(2);
        let bus = u8::from_str_radix(bus, 16).map_err(|_| Error::ParseError)?;

        let (slot, next) = next.trim_matches(':').split_at(2);
        let slot = u8::from_str_radix(slot, 16).map_err(|_| Error::ParseError)?;

        let function = next.trim_matches('.');
        let function = u8::from_str_radix(function, 16).map_err(|_| Error::ParseError)?;

        let (class, class_id) = matches.get(1).ok_or(Error::ParseError)?.rsplit_at(7);
        let class_id = id_from_raw(class_id)?;

        let (vendor, vendor_id) = matches.get(2).ok_or(Error::ParseError)?.rsplit_at(7);
        let vendor_id = id_from_raw(vendor_id)?;

        let (product, product_id) = matches.get(3).ok_or(Error::ParseError)?.rsplit_at(7);
        let product_id = id_from_raw(product_id)?;

        devices.push(DeviceInfo {
            path: DevicePath::PCI {
                bus,
                slot,
                function,
            },
            class: class.to_owned(),
            vendor: vendor.to_owned(),
            product: product.to_owned(),
            manufacturer: None,
            class_id: Some(class_id),
            vendor_id,
            product_id,
            manufacturer_id: None,
        });
    }

    Ok(devices)
}

pub(crate) fn get_usb() -> Result<Vec<DeviceInfo>, Error> {
    let output = Command::new("lsusb")
        .arg("-v")
        .output()
        .map_err(|_| Error::CommandError)?;

    let output_str = String::from_utf8(output.stdout).map_err(|_| Error::ParseError)?;

    let mut devices = Vec::new();

    for dev in output_str.split("\n\n") {
        // Path

        let path_line = LSUSB_PATH_REGEX.find(dev).ok_or(Error::ParseError)?;

        let mut path = path_line.as_str().split(' ');

        let _ = path.next().ok_or(Error::ParseError)?;
        let bus = path.next().ok_or(Error::ParseError)?;
        let _ = path.next().ok_or(Error::ParseError)?;
        let device = path.next().ok_or(Error::ParseError)?;

        let bus = bus.parse::<u8>().map_err(|_| Error::ParseError)?;
        let device = device.parse::<u8>().map_err(|_| Error::ParseError)?;

        // Class

        let class_line = LSUSB_CLASS_REGEX.find(dev).ok_or(Error::ParseError)?;

        let class_line = class_line
            .as_str()
            .trim()
            .trim_start_matches("bDeviceClass")
            .trim()
            .trim_start_matches("0x");

        let (class_id, class) = class_line.split_once(' ').unwrap_or((class_line, "Other"));

        let class_id = class_id.parse::<u16>().map_err(|_| Error::ParseError)?;

        // Vendor

        let vendor_line = LSUSB_VENDOR_REGEX.find(dev).ok_or(Error::ParseError)?;

        let (vendor_id, vendor) = vendor_line
            .as_str()
            .trim()
            .trim_start_matches("idVendor")
            .trim()
            .trim_start_matches("0x")
            .split_at(4);

        let vendor_id = u16::from_str_radix(vendor_id, 16).map_err(|_| Error::ParseError)?;
        let vendor = vendor.trim();

        // Product

        let product_line_a = LSUSB_PRODUCT_REGEX.find(dev).ok_or(Error::ParseError)?;

        let (_, product_a) = product_line_a
            .as_str()
            .trim()
            .trim_start_matches("iProduct")
            .trim()
            .split_once(' ')
            .unwrap_or(("", ""));

        let product_line_b = LSUSB_PRODUCT_ID_REGEX.find(dev).ok_or(Error::ParseError)?;

        let (product_id, product_b) = product_line_b
            .as_str()
            .trim()
            .trim_start_matches("idProduct")
            .trim()
            .trim_start_matches("0x")
            .split_at(4);

        let product_b = product_b.trim();
        let product_id = u16::from_str_radix(product_id, 16).map_err(|_| Error::ParseError)?;

        let product = format!("{product_a} {product_b}");
        let product = product.trim();

        // Manufacturer

        let manufacturer_line = LSUSB_MANUFACTURER_REGEX
            .find(dev)
            .ok_or(Error::ParseError)?;

        let manufacturer_result = manufacturer_line
            .as_str()
            .trim()
            .trim_start_matches("iManufacturer")
            .trim()
            .split_once(' ');

        let mut manufacturer = None;
        let mut manufacturer_id = None;

        if let Some((man_id, man)) = manufacturer_result {
            let man_id = man_id.parse::<u16>().map_err(|_| Error::ParseError)?;

            manufacturer = Some(man);
            manufacturer_id = Some(man_id);
        }

        devices.push(DeviceInfo {
            path: DevicePath::USB { bus, device },
            class: class.to_owned(),
            vendor: vendor.to_owned(),
            product: product.to_owned(),
            manufacturer: manufacturer.map(|s| s.to_owned()),
            class_id: Some(class_id),
            vendor_id,
            product_id,
            manufacturer_id,
        });
    }

    Ok(devices)
}
