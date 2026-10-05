use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CloseIconProps {
    #[props(default)]
    pub class: String,
}

#[component]
pub fn CloseIcon(props: CloseIconProps) -> Element {
    rsx! {
        svg {
            class: props.class,
            view_box: "0 0 22 22",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",

            path {
                d: "M1 1L21 21M1 21L21 1",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
