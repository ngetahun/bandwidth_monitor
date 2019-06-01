use pcap::{Device};

#[macro_use]
extern crate clap;
use clap::{App};

fn main() {
    // let devices = Device::list().unwrap();
    // for device in devices.iter() {
    //     println!("Found a device {}", device.name);
    // }

    let yml = load_yaml!("../cli.yml");
    let m = App::from_yaml(yml).get_matches();


    if m.is_present("list") {
        println!("List flag present");
        let devices = Device::list().unwrap();
        let no_description = "No description found.";
        for device in devices.iter() {
            let description = device.desc.as_ref();
            println!("Found a device {} - {:?}", device.name, description.unwrap_or(&no_description.to_owned()));
        }
    }

    // let main_device = Device::lookup().unwrap();
    // println!("Main listening device {:?}", main_device.name);
    // let mut cap = Capture::from_device(main_device).unwrap()
    //                   .promisc(true)
    //                   .snaplen(5000)
    //                   .open().unwrap();

    // while let Ok(packet) = cap.next() {
    //     println!("received packet! {:?}", packet);
    // }
}

