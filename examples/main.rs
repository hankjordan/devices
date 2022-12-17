use devices::Devices;

fn main() {
    let devices = Devices::get();
    
    for device in devices {
        println!("Device {:?}", device);
    }
}