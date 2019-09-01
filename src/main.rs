use std::io::{self, Write};
use pcap::{Device, Capture};

#[macro_use]
extern crate clap;
use clap::{App};
mod monitor;

fn main() {
    let yml = load_yaml!("../cli.yml");
    let m = App::from_yaml(yml).get_matches();


    if m.is_present("list") {
        let devices = Device::list().unwrap();
        let no_description = "No description found.";
        for device in devices.iter() {
            let description = device.desc.as_ref();
            println!("Found a device {} - {:?}", device.name, description.unwrap_or(&no_description.to_owned()));
        }
    }

    if m.is_present("default") {
        let main_device = Device::lookup().unwrap();
        println!("Main listening device {:?}", main_device.name);
    }

    if m.is_present("stat") {    
        let main_device = Device::lookup().unwrap();
        println!("Main listening device {:?}", main_device.name);
        let mut cap = Capture::from_device(main_device).unwrap()
                        .promisc(false)
                        .snaplen(5000)
                        .open().unwrap();
                        
        let mut bandwidth_monitor = monitor::BandwidthMonitor::new();
        while let Ok(packet) = cap.next() {
            // let stdio = io::stdio();
            // packet.filter
            bandwidth_monitor.add_recieved_bytes(packet.header.caplen);
            // println!("capture length: {}, packet length: {}", packet.header.caplen, packet.header.len);
            println!("{}", bandwidth_monitor);

            print!("{}[2J", 27 as char);
            // io::stdout().flush().unwrap();
            // bandwidth_monitor.print();
        }
    }
}
