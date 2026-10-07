use dioxus::desktop::{tao::window::Icon, Config, WindowBuilder};

fn load_window_icon() -> Option<Icon> {
    let bytes = include_bytes!("../assets/logo.png");

    let image = image::load_from_memory(bytes).ok()?.into_rgba8();

    let (width, height) = image.dimensions();

    Icon::from_rgba(image.into_raw(), width, height).ok()
}

pub fn desktop_config() -> Config {
    Config::new()
        .with_window(
            WindowBuilder::new()
                .with_title("Taskify")
                .with_decorations(false)
                .with_window_icon(load_window_icon()),
        )
        .with_disable_drag_drop_handler(true)
}
