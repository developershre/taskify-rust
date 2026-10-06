use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ChevronRightIconProps {
    #[props(default)]
    pub class: String,
}

#[component]
pub fn ChevronRightIcon(props: ChevronRightIconProps) -> Element {
    rsx! {
        svg {
            class: props.class,
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",

            path {
                d: "M10 17L15 12",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }

            path {
                d: "M15 12L10 7",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
