use std::sync::Mutex;

use lazy_static::lazy_static;
use sysinfo::System;

lazy_static! {
    pub static ref SYSTEM: Mutex<System> = Mutex::new(System::new_all());
}
