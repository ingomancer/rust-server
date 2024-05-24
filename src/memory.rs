use human_bytes::human_bytes;
use leptos::{
    component, create_signal, server, spawn_local, view, IntoView, ServerFnError, SignalGet,
    SignalSet,
};
use leptos_use::use_interval_fn;
use serde::{Deserialize, Serialize};
use sysinfo::System;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Memory {
    pub value: String,
}

#[server(UpdateMemory, "/api")]
async fn update_memory() -> Result<Memory, ServerFnError> {
    let mut sys = System::new_all();
    sys.refresh_all();
    let mem_str = format!(
        "{}/{}",
        human_bytes(sys.used_memory() as f64),
        human_bytes(sys.total_memory() as f64)
    );

    Ok(Memory { value: mem_str })
}

/// Displays system memory
#[component]
pub fn Memory() -> impl IntoView {
    let (mem, set_mem) = create_signal(Memory {
        value: "".to_string(),
    });

    use_interval_fn(
        move || {
            spawn_local(async move {
                set_mem.set(update_memory().await.unwrap());
            })
        },
        1000,
    );

    view! {
        <p> "Memory: " {move || mem.get().value} </p>
    }
}
