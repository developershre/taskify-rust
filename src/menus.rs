use dioxus::desktop::muda::{
    accelerator::{Accelerator, Code, Modifiers},
    Menu, MenuId, MenuItem, Submenu,
};

fn file_menu() -> Submenu {
    let new = MenuItem::with_id(
        MenuId::new("file.new"),
        "New",
        true,
        Some(Accelerator::new(Some(Modifiers::CONTROL), Code::KeyN)),
    );
    let open = MenuItem::with_id(
        MenuId::new("file.open"),
        "Open",
        true,
        Some(Accelerator::new(Some(Modifiers::CONTROL), Code::KeyO)),
    );
    let save = MenuItem::with_id(
        MenuId::new("file.save"),
        "Save",
        true,
        Some(Accelerator::new(Some(Modifiers::CONTROL), Code::KeyS)),
    );

    let save_as = MenuItem::with_id(
        MenuId::new("file.save_as"),
        "Save As",
        true,
        Some(Accelerator::new(Some(Modifiers::CONTROL), Code::KeyA)),
    );

    let exit = MenuItem::with_id(
        MenuId::new("file.exit"),
        "Exit",
        true,
        Some(Accelerator::new(Some(Modifiers::CONTROL), Code::KeyQ)),
    );
    Submenu::with_items("File", true, &[&new, &open, &save, &save_as, &exit])
        .expect("failed to build File menu")
}

fn edit_menu() -> Submenu {
    let undo = MenuItem::with_id(
        MenuId::new("edit.undo"),
        "Undo",
        true,
        Some(Accelerator::new(Some(Modifiers::CONTROL), Code::KeyZ)),
    );
    let redo = MenuItem::with_id(
        MenuId::new("edit.redo"),
        "Redo",
        true,
        Some(Accelerator::new(Some(Modifiers::CONTROL), Code::KeyY)),
    );

    Submenu::with_items("Edit", true, &[&undo, &redo]).expect("failed to build Edit menu")
}

fn view_menu() -> Submenu {
    let zoom_in = MenuItem::with_id(
        MenuId::new("view.zoom_in"),
        "Zoom In",
        true,
        Some(Accelerator::new(Some(Modifiers::CONTROL), Code::Equal)),
    );
    let zoom_out = MenuItem::with_id(
        MenuId::new("view.zoom_out"),
        "Zoom Out",
        true,
        Some(Accelerator::new(Some(Modifiers::CONTROL), Code::Minus)),
    );

    Submenu::with_items("View", true, &[&zoom_in, &zoom_out]).expect("failed to build View menu")
}

pub fn create_menu() -> Menu {
    let menu = Menu::new();

    menu.append_items(&[&file_menu(), &edit_menu(), &view_menu()])
        .expect("failed to build menu");

    menu
}
