#[cfg(feature = "bincode")]
use bincode::{
    Decode,
    Encode,
};

/// Device mount path.
#[cfg_attr(feature = "bincode", derive(Decode, Encode))]
#[derive(Debug)]
pub enum DevicePath {
    /// A PCI device path.
    PCI {
        /// PCI bus id.
        bus: u8,

        /// PCI slot id.
        ///
        /// Also known as `device` on Windows.
        slot: u8,

        /// PCI function.
        function: u8,
    },

    /// A USB device path.
    USB {
        /// USB bus id.
        ///
        /// Also known as `hub` on Windows.
        bus: u8,

        /// USB device id.
        ///
        /// Also known as `port` on Windows.
        device: u8,
    },
}
