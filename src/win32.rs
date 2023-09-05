use std::{
    mem,
    ptr,
};

use widestring::Utf16String;
use winapi::{
    shared::{
        devpkey::DEVPKEY_Device_LocationInfo,
        devpropdef::DEVPROPKEY,
        guiddef::GUID,
        minwindef::DWORD,
        windef::HWND,
    },
    um::setupapi::{
        SetupDiDestroyDeviceInfoList,
        SetupDiEnumDeviceInfo,
        SetupDiGetClassDevsW,
        SetupDiGetDevicePropertyW,
        SetupDiGetDeviceRegistryPropertyW,
        DIGCF_ALLCLASSES,
        HDEVINFO,
        SPDRP_CLASS,
        SPDRP_DEVICEDESC,
        SPDRP_HARDWAREID,
        SPDRP_MFG,
        SP_DEVINFO_DATA,
    },
};

use crate::{
    DeviceInfo,
    DevicePath,
    Error,
};

#[derive(Debug)]
struct FromUtf16BytesError(());

trait FromUtf16Bytes {
    fn from_utf16_bytes(bytes: &[u8]) -> Result<Self, FromUtf16BytesError>
    where
        Self: Sized;
}

impl FromUtf16Bytes for String {
    fn from_utf16_bytes(bytes: &[u8]) -> Result<Self, FromUtf16BytesError> {
        let (front, slice, back) = unsafe { bytes.align_to::<u16>() };
        if front.is_empty() && back.is_empty() {
            String::from_utf16(slice).map_err(|_| FromUtf16BytesError(()))
        } else {
            Err(FromUtf16BytesError(()))
        }
    }
}

struct SetupDiClassDevs {
    flags: DWORD,
    class: Option<GUID>,
    enumerator: Option<Utf16String>,
    parent: Option<HWND>,
}

impl SetupDiClassDevs {
    pub fn build(flags: DWORD) -> SetupDiClassDevs {
        Self {
            flags,
            class: None,
            enumerator: None,
            parent: None,
        }
    }

    #[allow(dead_code)]
    pub fn class(&mut self, class: GUID) -> &mut Self {
        self.class = Some(class);
        self
    }

    pub fn enumerator(&mut self, enumerator: &str) -> &mut Self {
        self.enumerator = Some(enumerator.into());
        self
    }

    #[allow(dead_code)]
    pub fn parent(&mut self, parent: HWND) -> &mut Self {
        self.parent = Some(parent);
        self
    }

    pub fn get(&self) -> IterSetupDiClassDevs {
        unsafe {
            let class: *const GUID = if let Some(c) = self.class {
                ptr::addr_of!(c)
            } else {
                ptr::null_mut()
            };

            let enumerator = self.enumerator.as_ref().map_or(ptr::null(), |e| e.as_ptr());
            let parent = self.parent.unwrap_or(ptr::null_mut());

            let hset = SetupDiGetClassDevsW(class, enumerator, parent, self.flags);
            IterSetupDiClassDevs { index: 0, hset }
        }
    }
}

struct IterSetupDiClassDevs {
    index: usize,
    hset: HDEVINFO,
}

#[allow(clippy::cast_possible_truncation)]
impl Iterator for IterSetupDiClassDevs {
    type Item = DevInfo;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut data: SP_DEVINFO_DATA = mem::zeroed();
            data.cbSize = mem::size_of::<SP_DEVINFO_DATA>() as u32;

            if SetupDiEnumDeviceInfo(self.hset, self.index as u32, &mut data) == 0 {
                None
            } else {
                self.index += 1;
                Some(DevInfo {
                    hset: self.hset,
                    data,
                })
            }
        }
    }
}

impl Drop for IterSetupDiClassDevs {
    fn drop(&mut self) {
        unsafe {
            assert_eq!(SetupDiDestroyDeviceInfoList(self.hset), 1);
        }
    }
}

struct DevInfo {
    hset: HDEVINFO,
    data: SP_DEVINFO_DATA,
}

trait GetRegistryProperty<T> {
    fn get_registry_property(&mut self, property: DWORD) -> Result<T, Error>;
}

trait GetProperty<T> {
    fn get_property(&mut self, property: DEVPROPKEY) -> Result<T, Error>;
}

impl GetRegistryProperty<Vec<u8>> for DevInfo {
    fn get_registry_property(&mut self, property: DWORD) -> Result<Vec<u8>, Error> {
        unsafe {
            let mut buf: [u8; 1024] = mem::zeroed();
            let mut size = mem::zeroed();

            let r = SetupDiGetDeviceRegistryPropertyW(
                self.hset,
                &mut self.data,
                property,
                &mut 0,
                <*mut _>::cast(&mut buf),
                1024,
                &mut size,
            );

            if r == 1 {
                let (buf, _) = buf.split_at(size as usize);
                Ok(buf.into())
            } else {
                Err(Error::CommandError)
            }
        }
    }
}

impl GetProperty<Vec<u8>> for DevInfo {
    fn get_property(&mut self, property: DEVPROPKEY) -> Result<Vec<u8>, Error> {
        unsafe {
            let mut ptype = mem::zeroed();
            let mut buf: [u8; 1024] = mem::zeroed();
            let mut size = mem::zeroed();

            let r = SetupDiGetDevicePropertyW(
                self.hset,
                &mut self.data,
                &property,
                &mut ptype,
                <*mut _>::cast(&mut buf),
                1024,
                &mut size,
                0,
            );

            if r == 1 {
                let (buf, _) = buf.split_at(size as usize);
                Ok(buf.into())
            } else {
                Err(Error::CommandError)
            }
        }
    }
}

impl GetRegistryProperty<String> for DevInfo {
    fn get_registry_property(&mut self, property: DWORD) -> Result<String, Error> {
        let buf: Vec<u8> = self.get_registry_property(property)?;
        let (buf, _) = buf.split_at(buf.len() - 2);

        let s = String::from_utf16_bytes(buf).map_err(|_| Error::ParseError)?;
        Ok(s)
    }
}

impl GetProperty<String> for DevInfo {
    fn get_property(&mut self, property: DEVPROPKEY) -> Result<String, Error> {
        let buf: Vec<u8> = self.get_property(property)?;
        let (buf, _) = buf.split_at(buf.len() - 2);

        let s = String::from_utf16_bytes(buf).map_err(|_| Error::ParseError)?;
        Ok(s)
    }
}

impl GetRegistryProperty<Vec<String>> for DevInfo {
    fn get_registry_property(&mut self, property: DWORD) -> Result<Vec<String>, Error> {
        let buf: String = self.get_registry_property(property)?;

        let mut result = Vec::new();

        for item in buf.split('\0') {
            if !item.is_empty() {
                result.push(item.to_owned());
            }
        }

        Ok(result)
    }
}

fn parse_hwids(hwids: &[String]) -> Result<(u16, u16), Error> {
    let hwid = hwids.first().ok_or(Error::ParseError)?;
    let (_, hwid) = hwid.split_at(4);

    let mut iter = hwid.split('&');

    let (_, v_id) = iter.next().ok_or(Error::ParseError)?.split_at(4);
    let vendor_id = u16::from_str_radix(v_id, 16).map_err(|_| Error::ParseError)?;

    let (_, p_id) = iter.next().ok_or(Error::ParseError)?.split_at(4);
    let product_id = u16::from_str_radix(p_id, 16).map_err(|_| Error::ParseError)?;

    Ok((vendor_id, product_id))
}

macro_rules! ok_or_next {
    ($e:expr) => {
        if let Ok(value) = $e {
            value
        } else {
            continue;
        }
    };
}

macro_rules! or_next {
    ($e:expr) => {
        if let Some(value) = $e {
            value
        } else {
            continue;
        }
    };
}

pub(crate) fn get_pci() -> Result<Vec<DeviceInfo>, Error> {
    let mut devices = Vec::new();

    // Return Error::UnsupportedPlatform for Wine
    let mut devs = SetupDiClassDevs::build(DIGCF_ALLCLASSES)
        .enumerator("WINEBUS")
        .get();

    if devs.next().is_some() {
        return Err(Error::UnsupportedPlatform);
    }

    // Native Windows

    let devs = SetupDiClassDevs::build(DIGCF_ALLCLASSES)
        .enumerator("PCI")
        .get();

    for mut info in devs {
        // Path (Location)

        let location: String = info.get_property(DEVPKEY_Device_LocationInfo)?;
        let mut iter = location.split(", ");

        let (_, bus) = iter.next().ok_or(Error::ParseError)?.split_at(8);
        let bus = bus.parse::<u8>().map_err(|_| Error::ParseError)?;

        let (_, slot) = iter.next().ok_or(Error::ParseError)?.split_at(7);
        let slot = slot.parse::<u8>().map_err(|_| Error::ParseError)?;

        let (_, function) = iter.next().ok_or(Error::ParseError)?.split_at(9);
        let function = function.parse::<u8>().map_err(|_| Error::ParseError)?;

        // Class Name, Vendor Name, Product Name

        let class: String = info.get_registry_property(SPDRP_CLASS)?;
        let vendor: String = info.get_registry_property(SPDRP_MFG)?;
        let product: String = info.get_registry_property(SPDRP_DEVICEDESC)?;

        // Vendor ID, Product ID

        let hwids: Vec<String> = info.get_registry_property(SPDRP_HARDWAREID)?;
        let (vendor_id, product_id) = parse_hwids(&hwids)?;

        devices.push(DeviceInfo {
            path: DevicePath::PCI {
                bus,
                slot,
                function,
            },
            class,
            vendor,
            product,
            manufacturer: None,
            class_id: None,
            vendor_id,
            product_id,
            manufacturer_id: None,
        });
    }

    Ok(devices)
}

pub(crate) fn get_usb() -> Result<Vec<DeviceInfo>, Error> {
    let mut devices = Vec::new();

    // Return Error::UnsupportedPlatform for Wine
    let mut devs = SetupDiClassDevs::build(DIGCF_ALLCLASSES)
        .enumerator("WINEBUS")
        .get();

    if devs.next().is_some() {
        return Err(Error::UnsupportedPlatform);
    }

    // Native Windows

    let devs = SetupDiClassDevs::build(DIGCF_ALLCLASSES)
        .enumerator("USB")
        .get();

    for mut info in devs {
        // Path (Location)

        let location: String = ok_or_next!(info.get_property(DEVPKEY_Device_LocationInfo));

        let (device, bus) = or_next!(location.split_once('.'));

        let (_, device) = device.split_at(6);
        let device = ok_or_next!(u8::from_str_radix(device, 16));

        let (_, bus) = bus.split_at(5);
        let bus = ok_or_next!(u8::from_str_radix(bus, 16));

        // Class Name, Vendor Name, Product Name, Manufacturer Name

        let class: String = info.get_registry_property(SPDRP_CLASS)?;
        let vendor: String = info.get_registry_property(SPDRP_MFG)?;
        let product: String = info.get_registry_property(SPDRP_DEVICEDESC)?;
        let manufacturer: String = info.get_registry_property(SPDRP_MFG)?;

        // Vendor ID, Product ID

        let hwids: Vec<String> = ok_or_next!(info.get_registry_property(SPDRP_HARDWAREID));
        let (vendor_id, product_id) = ok_or_next!(parse_hwids(&hwids));

        devices.push(DeviceInfo {
            path: DevicePath::USB { bus, device },
            class,
            vendor,
            product,
            manufacturer: Some(manufacturer),
            class_id: None,
            vendor_id,
            product_id,
            manufacturer_id: None,
        });
    }

    Ok(devices)
}
