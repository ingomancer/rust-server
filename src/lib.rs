pub mod app;
mod cpus;
mod disks;
mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
mod memory;
mod sysinfo;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
