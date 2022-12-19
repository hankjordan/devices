use crate::{DeviceInfo, Error};
use winapi::shared::minwindef::DWORD;
use winapi::um::setupapi::{
    SetupDiDestroyDeviceInfoList, SetupDiEnumDeviceInfo, SetupDiGetClassDevsW,
    SetupDiGetDeviceRegistryPropertyW, DIGCF_ALLCLASSES, HDEVINFO, SPDRP_HARDWAREID,
    SP_DEVINFO_DATA,
};

use std::mem;
use std::ptr;

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

struct RegError;

#[allow(clippy::fn_to_numeric_cast_with_truncation)]
unsafe fn get_registry_property(
    hset: HDEVINFO,
    mut data: SP_DEVINFO_DATA,
    property: DWORD,
) -> Result<Vec<u8>, RegError> {
    let mut buf: [u8; 1024] = mem::zeroed();
    let mut size = mem::zeroed();

    let r = SetupDiGetDeviceRegistryPropertyW(
        hset,
        &mut data,
        property,
        &mut 0,
        <*mut _>::cast(&mut buf),
        mem::size_of::<[u8; 1024]> as u32,
        &mut size,
    );

    if r == 1 {
        let (buf, _) = buf.split_at(size as usize - 4);
        Ok(buf.into())
    } else {
        Err(RegError)
    }
}

unsafe fn get_registry_property_string(
    hset: HDEVINFO,
    data: SP_DEVINFO_DATA,
    property: DWORD,
) -> Result<String, RegError> {
    if let Ok(buf) = get_registry_property(hset, data, property) {
        let s = String::from_utf16_bytes(&buf).map_err(|_| RegError)?;
        Ok(s)
    } else {
        Err(RegError)
    }
}

unsafe fn get_registry_property_string_array(
    hset: HDEVINFO,
    data: SP_DEVINFO_DATA,
    property: DWORD,
) -> Result<Vec<String>, RegError> {
    if let Ok(string) = get_registry_property_string(hset, data, property) {
        let mut result = Vec::new();

        for item in string.split('\0') {
            if !item.is_empty() {
                result.push(item.to_owned());
            }
        }

        Ok(result)
    } else {
        Err(RegError)
    }
}

#[allow(clippy::pedantic)]
pub(crate) fn get_devices() -> Result<Vec<DeviceInfo>, Error> {
    unsafe {
        let hset = SetupDiGetClassDevsW(
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            DIGCF_ALLCLASSES,
        );

        let mut i = 0;

        loop {
            let mut data: SP_DEVINFO_DATA = mem::zeroed();
            data.cbSize = mem::size_of::<SP_DEVINFO_DATA>() as u32;

            if SetupDiEnumDeviceInfo(hset, i, &mut data) == 0 {
                break;
            }

            if let Ok(data) = get_registry_property_string_array(hset, data, SPDRP_HARDWAREID) {
                println!("Data {:?}", data);
            }

            i += 1;
        }

        let d = SetupDiDestroyDeviceInfoList(hset);
        println!("destroy result {:?}", d);
    }

    Ok(Vec::new())
}
