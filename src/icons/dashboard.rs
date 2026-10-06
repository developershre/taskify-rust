use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LayoutIconProps {
    #[props(default)]
    pub class: String,
}

#[component]
pub fn LayoutIcon(props: LayoutIconProps) -> Element {
    rsx! {
        svg {
            class: props.class,
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",

            rect {
                x: "3.5",
                y: "4",
                width: "7",
                height: "4",
                rx: "2",
                stroke: "currentColor",
                stroke_width: "2",
            }

            rect {
                x: "3.5",
                y: "11",
                width: "7",
                height: "10",
                rx: "2.5",
                stroke: "currentColor",
                stroke_width: "2",
            }

            rect {
                x: "13.5",
                y: "4",
                width: "7",
                height: "10",
                rx: "2.5",
                stroke: "currentColor",
                stroke_width: "2",
            }

            rect {
                x: "13.5",
                y: "17",
                width: "7",
                height: "4",
                rx: "2",
                stroke: "currentColor",
                stroke_width: "2",
            }
        }
    }
}
