use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct EmptyStateProps {
    pub title: String,
    pub description: String,
    #[props(default)]
    pub icon: Option<Element>,
}

#[component]
pub fn EmptyState(props: EmptyStateProps) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center gap-3 rounded-xl border border-dashed border-border/60 bg-card/50 px-6 py-12 text-center",
            div { class: "flex size-10 items-center justify-center rounded-lg bg-muted text-muted-foreground [&_svg]:size-5",
                {props.icon}
            }
            h3 { class: "text-sm font-semibold text-foreground", "{props.title}" }
            p { class: "max-w-sm text-xs text-muted-foreground", "{props.description}" }
        }
    }
}
