use std::process::Command;

use lazy_static::lazy_static;
use regex::Regex;

use crate::{info::DeviceInfo, error::Error};

lazy_static! {
    static ref LSPCI_REGEX: Regex = Regex::new("(^.*? )|(\".*?\")").unwrap();
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

    u16::from_str_radix(trimmed, 16).map_err(|_| Error)
}

pub(crate) fn lspci() -> Result<Vec<DeviceInfo>, Error> {
    let output = Command::new("lspci").arg("-mm").arg("-nn").output().map_err(|_| Error)?;
    
    let output_str = String::from_utf8(output.stdout).map_err(|_| Error)?;

    let mut devices = Vec::new();

    for line in output_str.trim().split('\n') {
        let mut matches = Vec::new();

        for m in LSPCI_REGEX.find_iter(line) {
            matches.push(m.as_str().trim().trim_matches('"'));
        }

        let path = matches.first().ok_or(Error)?;

        let (class, class_id_raw) = matches.get(1).ok_or(Error)?.rsplit_at(7);
        let class_id = id_from_raw(class_id_raw)?;

        let (vendor, vendor_id_raw) = matches.get(2).ok_or(Error)?.rsplit_at(7);
        let vendor_id = id_from_raw(vendor_id_raw)?;

        let (product, product_id_raw) = matches.get(3).ok_or(Error)?.rsplit_at(7);
        let product_id = id_from_raw(product_id_raw)?;

        devices.push(
            DeviceInfo::new(
                path,
                class,
                vendor,
                product,
                class_id,
                vendor_id,
                product_id
            )
        );
    }
    
    Ok(devices)
}
