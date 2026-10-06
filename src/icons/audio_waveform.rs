use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AudioWaveformIconProps {
    #[props(default)]
    pub class: String,
}

#[component]
pub fn AudioWaveformIcon(props: AudioWaveformIconProps) -> Element {
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
                d: "M2 13a2 2 0 0 0 2-2V7a2 2 0 0 1 4 0v13a2 2 0 0 0 4 0V4a2 2 0 0 1 4 0v13a2 2 0 0 0 4 0v-4a2 2 0 0 1 2-2",
            }
        }
    }
}
