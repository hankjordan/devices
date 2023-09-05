use devices::Devices;

fn main() {
    // Return all connected devices
    match Devices::get() {
        Ok(devices) => {
            for device in devices {
                println!("{:?}", device);
            }
        }
        Err(e) => {
            println!("Devices::get() returned Error {:?}", e);
        }
    }

    // Return only PCI devices
    match Devices::pci() {
        Ok(devices) => {
            for device in devices {
                println!("{:?}", device);
            }
        }
        Err(e) => {
            println!("Devices::get() returned Error {:?}", e);
        }
    }

    // Return only USB devices
    match Devices::usb() {
        Ok(devices) => {
            for device in devices {
                println!("{:?}", device);
            }
        }
        Err(e) => {
            println!("Devices::get() returned Error {:?}", e);
        }
    }
}
