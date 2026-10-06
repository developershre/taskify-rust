use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct StatCardProps {
    pub label: String,
    pub value: String,
    #[props(default)]
    pub hint: String,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn StatCard(props: StatCardProps) -> Element {
    rsx! {
        div { class: "flex flex-col gap-1 rounded-xl border border-border/40 bg-card p-4 shadow-xs {props.class}",
            span { class: "text-xs font-medium uppercase tracking-wide text-muted-foreground",
                "{props.label}"
            }
            span { class: "text-2xl font-bold tracking-tight text-foreground",
                "{props.value}"
            }
            if !props.hint.is_empty() {
                span { class: "text-xs text-muted-foreground", "{props.hint}" }
            }
        }
    }
}
