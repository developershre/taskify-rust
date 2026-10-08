use crate::components::MailView;
use dioxus::prelude::*;

#[component]
pub fn ChatMail() -> Element {
    rsx! {
        div { class: "flex h-full min-h-0 flex-col",
            MailView {}
        }
    }
}
