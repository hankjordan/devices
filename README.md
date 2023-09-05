# Devices
[![][img_version]][crates] [![][img_doc]][doc] [![][img_license]][license] [![][img_downloads]][crates]

`devices` is a cross-platform library for retrieving information about connected devices.

Combined with a library like [sysinfo](https://crates.io/crates/sysinfo), a more or less complete description of a system's hardware can be gathered.

## Supported platforms

- Linux (`lspci` and `lsusb` required)
- Windows (Windows 7+ / Wine not supported)

## Implementation Notes

Wine provides the APIs this library needs to function, but it does not return all the information necessary to build the `DeviceInfo` struct. When running on Wine, all device-retrieving methods will return `Error::UnsupportedPlatform`.

On Linux, this library works by creating a subprocess to gather device information and parsing the result. Pulling device information from a platform-specific API would be preferred. PRs welcome.

## Cargo Features

- `bincode`: Support for bincode v2 encoding and decoding. Enabled by default.
- `serde`: Support for serde serialization and deserialization. Enabled by default.

## License

`devices` is dual-licensed under MIT and Apache-2.0.

[img_version]: https://img.shields.io/crates/v/devices.svg
[img_doc]: https://img.shields.io/badge/rust-documentation-blue.svg
[img_license]: https://img.shields.io/badge/license-MIT%2FApache-blue.svg
[img_downloads]:https://img.shields.io/crates/d/devices.svg

[crates]: https://crates.io/crates/devices
[doc]: https://docs.rs/devices/
[license]: https://github.com/hankjordan/devices#license
