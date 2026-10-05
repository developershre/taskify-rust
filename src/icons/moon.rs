use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MoonIconProps {
    #[props(default = "size-4".to_string())]
    pub class: String,
}

#[component]
pub fn MoonIcon(props: MoonIconProps) -> Element {
    rsx! {
        svg {
            class: props.class,
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" }
        }
    }
}
