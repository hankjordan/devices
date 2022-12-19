use devices::Devices;

fn main() {
    if let Ok(devices) = Devices::get() {
        for device in devices {
            println!("Device {:?}", device);
        }
    }
}