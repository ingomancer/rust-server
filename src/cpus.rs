#[cfg(feature = "ssr")]
use std::fmt::Write;
use std::time::Duration;

use leptos::{
    component, create_effect, create_signal, server, set_interval, spawn_local, view, IntoView,
    ServerFnError, SignalGet, SignalSet,
};

#[cfg(feature = "ssr")]
use crate::sysinfo::SYSTEM;

#[server(UpdateCpus, "/api")]
async fn update_cpus() -> Result<String, ServerFnError> {
    let sys = &mut SYSTEM.lock().unwrap().system;
    sys.refresh_cpu();

    let string = sys
        .cpus()
        .iter()
        .enumerate()
        .fold(String::new(), |mut string, (index, cpu)| {
            let _ = write!(string, " {}: {:0>2.0}%,", index, cpu.cpu_usage());
            string
        });
    let string = string.trim_end_matches(',');
    Ok(string.to_string())
}

/// Displays system cpu usage
#[component]
pub fn Cpus() -> impl IntoView {
    let (cpus, set_cpus) = create_signal("".to_string());

    create_effect(move |_| {
        set_interval(
            move || {
                spawn_local(async move {
                    set_cpus.set(update_cpus().await.unwrap());
                })
            },
            Duration::from_millis(1000),
        )
    });

    view! {
        <p> "Cpu usage:" {move || cpus.get()} </p>
    }
}
