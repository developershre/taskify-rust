use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ZapIconProps {
    #[props(default)]
    pub class: String,
}

#[component]
pub fn ZapIcon(props: ZapIconProps) -> Element {
    rsx! {
        svg {
            class: props.class,
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",

            path {
                d: "M13 3V10H19L11 21V14H5L13 3Z",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
