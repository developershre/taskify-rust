use dioxus::{
    desktop::tao::window::Icon,
    desktop::{use_muda_event_handler, Config, WindowBuilder},
    prelude::*,
};

fn load_window_icon() -> Option<Icon> {
    let bytes = include_bytes!("../assets/logo.png");

    let image = image::load_from_memory(bytes).ok()?.into_rgba8();

    let (width, height) = image.dimensions();

    Icon::from_rgba(image.into_raw(), width, height).ok()
}

use components::{AppFrame, TitleBar};
use views::{Home, PlaceholderPage};

mod components;
mod icons;
pub mod state;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(TitleBar)]
    #[layout(AppFrame)]
    #[route("/")]
    Home {},

    #[route("/tasks", PlaceholderPage)]
    Tasks {},
    #[route("/tasks/all", PlaceholderPage)]
    TasksAll {},
    #[route("/tasks/archived", PlaceholderPage)]
    TasksArchived {},
    #[route("/tasks/urgent", PlaceholderPage)]
    TasksUrgent {},

    #[route("/projects", PlaceholderPage)]
    Projects {},
    #[route("/projects/personal", PlaceholderPage)]
    ProjectsPersonal {},
    #[route("/projects/github", PlaceholderPage)]
    ProjectsGithub {},

    #[route("/calendar", PlaceholderPage)]
    Calendar {},

    #[route("/chat", PlaceholderPage)]
    Chat {},
    #[route("/chat/messaging", PlaceholderPage)]
    ChatMessaging {},
    #[route("/chat/mail", PlaceholderPage)]
    ChatMail {},
    #[route("/chat/issues", PlaceholderPage)]
    ChatIssues {},

    #[route("/analytics", PlaceholderPage)]
    Analytics {},

    #[route("/mcp", PlaceholderPage)]
    Mcp {},
}

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
        .with_cfg(
            Config::new()
                .with_window(
                    WindowBuilder::new()
                        .with_title("Taskify")
                        .with_decorations(false)
                        .with_window_icon(load_window_icon()),
                )
                .with_disable_drag_drop_handler(true),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    let mut app_state = state::use_init_app_state();

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
