use crate::{
    cpus::Cpus,
    disks::Disks,
    error_template::{AppError, ErrorTemplate},
    memory::Memory,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

/// Main app entrypoint
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rust-server.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Clone)]
enum DisplayedInfo {
    Memory,
    Disks,
    Cpu,
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button

    let (displayed_info, set_displayed_info) = create_signal(DisplayedInfo::Memory);

    let mem_click = move |_| set_displayed_info.set(DisplayedInfo::Memory);
    let disk_click = move |_| set_displayed_info.set(DisplayedInfo::Disks);
    let cpu_click = move |_| set_displayed_info.set(DisplayedInfo::Cpu);

    view! {
        <div class="topnav">
            <h1>"ingo's server info webpage woop"</h1>
        </div>
        <div class="sidenav">
            <a on:click=mem_click>"Memory"</a>
            <a on:click=disk_click>"Disks"</a>
            <a on:click=cpu_click>"CPU"</a>
        </div>
        { move || match displayed_info.get() {
            DisplayedInfo::Memory => view! {<Memory/>},
            DisplayedInfo::Disks => view! {<Disks/>},
            DisplayedInfo::Cpu => view! {<Cpus/>},
        }}

    }
}
