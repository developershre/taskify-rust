use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CommandIconProps {
    #[props(default)]
    pub class: String,
}

#[component]
pub fn CommandIcon(props: CommandIconProps) -> Element {
    rsx! {
        svg {
            class: props.class,
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            xmlns: "http://www.w3.org/2000/svg",

            path {
                d: "M15 6v12a3 3 0 1 0 3-3H6a3 3 0 1 0 3 3V6a3 3 0 1 0-3 3h12a3 3 0 1 0-3-3",
            }
        }
    }
}
