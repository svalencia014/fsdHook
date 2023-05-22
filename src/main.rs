use pcap::{Device, Capture};

fn main() {
    println!("Starting Packet Capture");
    let device = Device::lookup()
        .expect("Device lookup Failed")
        .expect("No Device available");
    println!("Using device {}", device.name);

    let mut cap = Capture::from_device(device)
        .unwrap()
        .immediate_mode(true)
        .open()
        .unwrap();
}
