# Devices

A simple cross-platform library for retrieving information about connected devices.

## Supported platforms

- Linux (`lspci` and `lsusb` required)
- Windows

## Implementation Note

This library works by creating a subprocess to gather device information and parsing the result.

Pulling device information from a platform-specific API would be preferred. PRs welcome.