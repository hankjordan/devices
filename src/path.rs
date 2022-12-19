/// Device mount path.
#[derive(Debug)]
pub enum DevicePath {
    PCI { bus: u8, slot: u8, function: u8 },
    USB { bus: u8, device: u8 },
}
