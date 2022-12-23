# Devices [![][img_crates]][crates] [![][img_doc]][doc]

`devices` is a cross-platform library for retrieving information about connected devices.

Combined with a library like [sysinfo](https://crates.io/crates/sysinfo), a more or less complete description of a system's hardware can be gathered.

## Supported platforms

- Linux (`lspci` and `lsusb` required)
- Windows (Windows 7+ / Wine not supported)

## Implementation Notes

Wine provides the APIs this library needs to function, but it does not return all the information necessary to build the `DeviceInfo` struct. When running on Wine, `Devices::get()` will return `Error::UnsupportedPlatform`.

On Linux, this library works by creating a subprocess to gather device information and parsing the result. Pulling device information from a platform-specific API would be preferred. PRs welcome.

[img_crates]: https://img.shields.io/crates/v/devices.svg
[img_doc]: https://img.shields.io/badge/rust-documentation-blue.svg

[crates]: https://crates.io/crates/devices
[doc]: https://docs.rs/devices/
