use std::fmt;
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
    pub fn add_recieved_bytes(mut self, bytes: u32) {
        let mu = Mutex::new(0);
        let mu_status = mu.try_lock();
        match mu_status {
            Ok(v) => self.recieved += bytes,
            Err(e) => panic!()
        }
    }

    pub fn add_sent_bytes(mut self, bytes: u32) {
        let mu = Mutex::new(1);
        let mu_status = mu.try_lock();
        match mu_status {
            Ok(v) => self.sent += bytes,
            Err(e) => panic!()
        }
    }
}

impl fmt::Display for BandwidthMonitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Recieved: {} MBs, Sent: {} MBs", self.recieved/MEGABYTES, self.sent/MEGABYTES)
    }
}
