use dioxus::prelude::*;

#[component]
pub fn CloseIcon(#[props(default = "16".to_string())] size: String) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",

            g {
                clip_path: "url(#clip0_2497_25965)",

                path {
                    d: "M7 7L17 17M7 17L17 7",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }

            defs {
                clipPath {
                    id: "clip0_2497_25965",

                    rect {
                        width: "24",
                        height: "24",
                        fill: "white",
                    }
                }
            }
        }
    }
}
