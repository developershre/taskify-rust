use dioxus::{desktop::use_muda_event_handler, prelude::*};

mod components;
mod icons;
pub mod routes;
pub mod state;
mod views;
mod window;

pub use routes::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(target_os = "linux")]
extern "C" {
    fn malloc_trim(pad: usize) -> i32;
}

/// Releases freed heap memory back to the operating system on Linux
fn trim_memory() {
    #[cfg(target_os = "linux")]
    unsafe {
        malloc_trim(0);
    }
}

fn main() {
    // Periodically release unused heap memory back to the operating system
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(4));
        trim_memory();

        loop {
            std::thread::sleep(std::time::Duration::from_secs(30));
            trim_memory();
        }
    });

    dioxus::LaunchBuilder::desktop()
        .with_cfg(window::desktop_config())
        .launch(App);
}

#[component]
fn App() -> Element {
    let mut app_state = state::use_init_app_state();
    state::use_overlay_state();

    use_muda_event_handler(move |event| match event.id().0.as_str() {
        "file.new" => println!("Action: New Task (Ctrl+N)"),
        "file.open" => println!("Action: Open File (Ctrl+O)"),
        "file.save" => println!("Action: Save (Ctrl+S)"),
        "file.save_as" => println!("Action: Save As (Ctrl+A)"),
        "file.exit" => std::process::exit(0),
        "edit.undo" => println!("Action: Undo (Ctrl+Z)"),
        "edit.redo" => println!("Action: Redo (Ctrl+Y)"),
        "view.zoom_in" => println!("Action: Zoom In (Ctrl++)"),
        "view.zoom_out" => println!("Action: Zoom Out (Ctrl+-)"),
        "view.toggle_theme" => app_state.toggle_theme(),
        _ => {}
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
