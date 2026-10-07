use dioxus::prelude::*;
use crate::components::ChatView;

#[component]
pub fn Chat() -> Element {
    rsx! {
        div { class: "flex h-full min-h-0 flex-col",
            ChatView {}
        }
    }
}
