use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ChevronLeftIconProps {
    #[props(default)]
    pub class: String,
}

#[component]
pub fn ChevronLeftIcon(props: ChevronLeftIconProps) -> Element {
    rsx! {
        svg {
            class: props.class,
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",

            path {
                d: "M14 7L9 12",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }

            path {
                d: "M9 12L14 17",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
