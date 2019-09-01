use std::{fmt, io};
use std::thread;
use std::io::Write;
use std::sync::{Mutex};

static MEGABYTES: u32 = 1024 * 1024;

#[derive(Debug, Copy, Clone)]
pub struct BandwidthMonitor {
    recieved: u32,
    sent: u32,
}

impl BandwidthMonitor {
    pub fn new() -> BandwidthMonitor {
        BandwidthMonitor {
            recieved: 0,
            sent: 0
        }
    }
    pub fn add_recieved_bytes(&mut self, bytes: u32) {
        // let adder = thread::spawn(move || {    
        //     let mu = Mutex::new(self.recieved);
        //     let mut mu_status = mu.lock().unwrap();
        //     *mu_status += bytes;
        //     println!("Recieved bytes {}", mu_status);
        // });
        // adder.join().unwrap();
        self.recieved += bytes;
    }

    // pub fn add_sent_bytes(self, bytes: u32) {
    //     let mu = Mutex::new(self);
    //     let mut mu_status = mu.lock().unwrap();
    //     mu_status.sent += bytes;
    // }
    pub fn print(self) {
        // let mu = Mutex::new(self);
        // let print_mut = mu.lock().unwrap();
        let stdout = io::stdout();
        writeln!(&mut stdout.lock(),
                             "Recieved: {:.6} MBs, Sent: {:.6} MBs", (self.recieved/MEGABYTES) as f64, (self.sent/MEGABYTES) as f64).unwrap();                           
    }
}

impl fmt::Display for BandwidthMonitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Recieved: {} MBs, Sent: {} MBs", self.recieved/MEGABYTES, self.sent/MEGABYTES)
    }
}
