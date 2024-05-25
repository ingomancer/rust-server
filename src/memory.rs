use std::time::Duration;

#[cfg(feature = "ssr")]
use human_bytes::human_bytes;
use leptos::{
    component, create_effect, create_signal, server, set_interval, spawn_local, view, IntoView,
    ServerFnError, SignalGet, SignalSet,
};

#[cfg(feature = "ssr")]
use crate::sysinfo::SYSTEM;

#[server(UpdateMemory, "/api")]
async fn update_memory() -> Result<String, ServerFnError> {
    let sys = &mut SYSTEM.lock().unwrap().system;
    sys.refresh_memory();
    let mem_str = format!(
        "{}/{}",
        human_bytes(sys.used_memory() as f64),
        human_bytes(sys.total_memory() as f64)
    );

    Ok(mem_str)
}

/// Displays system memory
#[component]
pub fn Memory() -> impl IntoView {
    let (mem, set_mem) = create_signal("".to_string());

    create_effect(move |_| {
        set_interval(
            move || {
                spawn_local(async move {
                    set_mem.set(update_memory().await.unwrap());
                })
            },
            Duration::from_millis(1000),
        )
    });

    view! {
        <p> "Memory: " {move || mem.get()} </p>
    }
}
