use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GalleryVerticalEndIconProps {
    #[props(default)]
    pub class: String,
}

#[component]
pub fn GalleryVerticalEndIcon(props: GalleryVerticalEndIconProps) -> Element {
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

            path { d: "M7 2h10" }
            path { d: "M5 6h14" }
            rect {
                width: "18",
                height: "18",
                x: "3",
                y: "3",
                rx: "2",
            }
        }
    }
}
