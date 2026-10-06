use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MinimizeIconProps {
    #[props(default)]
    pub class: String,
}

#[component]
pub fn MinimizeIcon(props: MinimizeIconProps) -> Element {
    rsx! {
        svg {
            class: props.class,
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            xmlns: "http://www.w3.org/2000/svg",

            path {
                d: "M6 12h12",
            }
        }
    }
}
