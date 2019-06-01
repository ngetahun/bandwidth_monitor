use pcap::{Device, Capture};

#[macro_use]
extern crate clap;
use clap::{App};
mod monitor;

fn main() {
    // Load YAML file
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
                        .promisc(true)
                        .snaplen(5000)
                        .open().unwrap();
        // let bandwidth_monitor = monitor::BandwidthMonitor::new();
        while let Ok(packet) = cap.next() {
            println!("capture length: {}, packet length: {}", packet.header.caplen, packet.header.len);
            // bandwidth_monitor.add_recieved_bytes(packet.header.caplen);
            // println!("Bandwidth {:?}", bandwidth_monitor);
        }
    }
}
