#[cfg(feature = "ssr")]
use human_bytes::human_bytes;

#[cfg(feature = "ssr")]
use std::fmt::Write;
use std::time::Duration;

use leptos::{
    component, create_effect, create_signal, server, set_interval, spawn_local, view, IntoView,
    ServerFnError, SignalGet, SignalSet,
};

#[cfg(feature = "ssr")]
use crate::sysinfo::SYSTEM;

#[server(UpdateDisks, "/api")]
async fn update_disks() -> Result<String, ServerFnError> {
    let sys = &mut SYSTEM.lock().unwrap().disks;
    sys.refresh_list();

    let string = sys
        .list()
        .iter()
        .filter(|disk| match disk.kind() {
            sysinfo::DiskKind::HDD | sysinfo::DiskKind::SSD => true,
            sysinfo::DiskKind::Unknown(_) => false,
        })
        .fold(String::new(), |mut string, disk| {
            let _ = write!(
                string,
                " {:?}: {}/{},",
                disk.name(),
                human_bytes(disk.total_space() as f64 - disk.available_space() as f64),
                human_bytes(disk.total_space() as f64),
            );
            string
        });
    let string = string.trim_end_matches(',');
    Ok(string.to_string())
}

/// Displays system disk usage
#[component]
pub fn Disks() -> impl IntoView {
    let (disks, set_disks) = create_signal("".to_string());

    create_effect(move |_| {
        set_interval(
            move || {
                spawn_local(async move {
                    set_disks.set(update_disks().await.unwrap());
                })
            },
            Duration::from_millis(1000),
        )
    });

    view! {
        <p> "Disk usage:" {move || disks.get()} </p>
    }
}
