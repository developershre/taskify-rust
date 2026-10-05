use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    #[props(default = "default".to_string())]
    pub variant: String,

    #[props(default)]
    pub class: String,

    pub children: Element,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let variant_class = match props.variant.as_str() {
        "secondary" => "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80",
        "destructive" => "border-transparent bg-destructive text-destructive-foreground shadow-xs hover:bg-destructive/80",
        "outline" => "border-border text-foreground",
        _ => "border-transparent bg-primary text-primary-foreground shadow-xs hover:bg-primary/80",
    };

    let class = format!(
        "inline-flex items-center gap-1 rounded-md border px-2.5 py-0.5 text-xs font-semibold \
         transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 \
         select-none {} {}",
        variant_class, props.class
    );

    rsx! {
        div {
            "data-slot": "badge",
            class: class,
            {props.children}
        }
    }
}
