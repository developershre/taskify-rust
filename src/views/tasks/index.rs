use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum TaskView {
    Board,
    List,
    Table,
}

#[component]
pub fn Tasks() -> Element {
    rsx! {}
}
