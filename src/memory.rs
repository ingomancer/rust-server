use std::time::Duration;

#[cfg(feature = "ssr")]
use human_bytes::human_bytes;
use leptos::{
    component, create_effect, create_signal, on_cleanup, server, set_interval_with_handle,
    spawn_local, view, IntoView, ServerFnError, SignalGet, SignalSet,
};
use wasm_bindgen::JsValue;

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

    println!("Getting memory");
    Ok(mem_str)
}

/// Displays system memory
#[component]
pub fn Memory() -> impl IntoView {
    let (mem, set_mem) = create_signal("".to_string());
    let (handle, set_handle) = create_signal(Err(JsValue::null()));

    create_effect(move |_| {
        set_handle.set(set_interval_with_handle(
            move || {
                spawn_local(async move {
                    set_mem.set(update_memory().await.unwrap());
                })
            },
            Duration::from_millis(1000),
        ));
    });

    on_cleanup(move || handle.get().unwrap().clear());

    view! {
        <p> "Memory: " {move || mem.get()} </p>
    }
}
