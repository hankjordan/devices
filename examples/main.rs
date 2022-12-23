use devices::Devices;

fn main() {
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
}
