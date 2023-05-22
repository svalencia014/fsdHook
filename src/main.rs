use std::io;
use pcap::{Device, Capture};

fn main() {
    //find all devices installed
    let mut devices = [];
    let mut i: u32 = 0;
    println!("Select a device:");
    for device in Device::list().expect("Device lookup failed") {
        println!("Device {i}: {:?}", device.name);
        devices[i] = device;
        i = i + 1;
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("Using device {:?}", devices[input]);
}
