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

use components::TitleBar;
use views::Home;

mod components;
mod icons;
mod menus;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(TitleBar)]
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::new().with_menu(menus::create_menu()).with_window(
                WindowBuilder::new()
                    .with_title("Taskify")
                    .with_decorations(false)
                    .with_window_icon(load_window_icon()),
            ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    use_muda_event_handler(|event| match event.id().0.as_str() {
        "file.new" => println!("Action: New Task (Ctrl+N)"),
        "file.open" => println!("Action: Open File (Ctrl+O)"),
        "file.save" => println!("Action: Save (Ctrl+S)"),
        "file.save_as" => println!("Action: Save As (Ctrl+A)"),
        "file.exit" => std::process::exit(0),
        "edit.undo" => println!("Action: Undo (Ctrl+Z)"),
        "edit.redo" => println!("Action: Redo (Ctrl+Y)"),
        "view.zoom_in" => println!("Action: Zoom In (Ctrl++)"),
        "view.zoom_out" => println!("Action: Zoom Out (Ctrl+-)"),
        _ => {}
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
