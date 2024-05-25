use std::sync::Mutex;

use lazy_static::lazy_static;
use sysinfo::{Disks, System};

pub struct SystemInformation {
    pub system: System,
    pub disks: Disks,
}

lazy_static! {
    pub static ref SYSTEM: Mutex<SystemInformation> = {
        let sysinfo = SystemInformation {
            system: System::new_all(),
            disks: Disks::new_with_refreshed_list(),
        };
        Mutex::new(sysinfo)
    };
}
