use std::{io, any};
use pcap::{Device, Capture, PacketHeader};

fn main() {
    //  find all devices installed
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
    let device: Device = devices.get(input.trim().parse::<usize>().unwrap()).unwrap().clone();
    std::mem::drop(devices);
    println!("Using device: {:?}", device.name);
    println!("Starting Capture");
    //setup capture
    let mut cap = Capture::from_device(device.clone())
    .unwrap()
    .immediate_mode(true)
    .open()
    .unwrap();
    //filter non fsd packets
    cap.filter("tcp port 6809", true).unwrap();
    loop {
        //  * Print packet for Debug Purposes
        //  TODO: Remove before prod
        println!("{:?}", cap.next_packet());
    }
}

fn parse_packet(packet: pcap::Packet) {
    let mut decodedPacket;
    decodedPacket.header = packet.header;
    decodedPacket.timestamp = packet.header.ts;
    decodedPacket.caplen = packet.header.caplen;
    decodedPacket.len = packet.header.len;
    decodedPacket.data = 
}

struct DecodedPacket {
    header: PacketHeader,
    timestamp: u64,
    caplen: u32,
    len: u32,
    data: Vec<u8>,
}