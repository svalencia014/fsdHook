use std::{io, any};
use pcap::{Device, Capture};

fn main() {
    //find all devices installed
    let mut devices = Vec::new();
    let mut i: u32 = 0;
    println!("Detected Devices:");
    for device in Device::list().expect("Device lookup failed") {
        println!("Device {i}: {:?}", device.name);
        devices.push(device);
        i = i + 1;
    }
    println!("Select a device:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let device = devices.get(input.trim().parse::<usize>().unwrap()).unwrap();
    println!("Using device: {:?}", device.name);

    //setup capture
    let mut cap = Capture::from_device(device)
        .unwrap()
        .immediate_mode(true)
        .open()
        .unwrap();

    println!("{:?}", cap.next_packet());
}
