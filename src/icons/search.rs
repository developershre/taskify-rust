use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SearchIconProps {
    #[props(default)]
    pub class: String,
}

#[component]
pub fn SearchIcon(props: SearchIconProps) -> Element {
    rsx! {
        svg {
            class: props.class,
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",

            g {
                clip_path: "url(#clip0_2497_25824)",

                path {
                    d: "M21 21L16.6569 16.6569M16.6569 16.6569C18.1046 15.2091 19 13.2091 19 11C19 6.58172 15.4183 3 11 3C6.58172 3 3 6.58172 3 11C3 15.4183 6.58172 19 11 19C13.2091 19 15.2091 18.1046 16.6569 16.6569Z",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }

            defs {
                clipPath {
                    id: "clip0_2497_25824",

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
