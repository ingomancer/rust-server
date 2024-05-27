use lazy_static::lazy_static;
use sysinfo::{Disks, System};
use tokio::sync::RwLock;

pub struct SystemInformation {
    pub system: System,
    pub disks: Disks,
}

lazy_static! {
    pub static ref SYSTEM: RwLock<SystemInformation> = {
        let sysinfo = SystemInformation {
            system: System::new_all(),
            disks: Disks::new_with_refreshed_list(),
        };
        RwLock::new(sysinfo)
    };
}

#[cfg(feature = "ssr")]
pub async fn update_all_sysinfos() {
    let sysinfo = &mut SYSTEM.write().await;
    sysinfo.system.refresh_cpu();
    sysinfo.system.refresh_memory();
    sysinfo.disks.refresh_list();
}
