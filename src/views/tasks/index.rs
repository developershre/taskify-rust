use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
#[allow(dead_code)]
enum TaskView {
    Board,
    List,
    Table,
}

#[component]
pub fn Tasks() -> Element {
    rsx! {}
}
