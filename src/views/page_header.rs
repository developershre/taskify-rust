use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PageHeaderProps {
    pub title: String,
    #[props(default)]
    pub description: String,
    #[props(default)]
    pub actions: Option<Element>,
}

#[component]
pub fn PageHeader(props: PageHeaderProps) -> Element {
    rsx! {
        div { class: "flex items-start justify-between gap-4 pt-4 pb-4",
            div { class: "flex flex-col gap-1 min-w-0",
                h1 { class: "text-2xl sm:text-3xl font-bold tracking-tight text-foreground",
                    "{props.title}"
                }
                if !props.description.is_empty() {
                    p { class: "text-sm text-muted-foreground", "{props.description}" }
                }
            }
            {props.actions}
        }
    }
}
