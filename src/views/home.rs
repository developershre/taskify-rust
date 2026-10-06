use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        h1 { class: "text-2xl sm:text-3xl font-bold tracking-tight text-foreground mt-4",
            "Welcome Back"
        }
    }
}
