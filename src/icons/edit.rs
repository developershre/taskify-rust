use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct EditIconProps {
    #[props(default)]
    pub class: String,
}

#[component]
pub fn EditIcon(props: EditIconProps) -> Element {
    rsx! {
        svg {
            class: props.class,
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",

            path {
                d: "M21 13V17C21 19.7614 18.7614 22 16 22H8C5.23858 22 3 19.7614 3 17V9C3 6.23858 5.23858 4 8 4H12",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
            }

            path {
                d: "M17.1721 3.28601C17.9525 2.50437 19.2184 2.50389 19.9994 3.28494L21.4887 4.77419C22.2631 5.54858 22.2711 6.80222 21.5067 7.58649L14.4467 14.8301C13.8823 15.4092 13.1082 15.7357 12.2998 15.7357L10.6026 15.7356C9.7498 15.7356 9.06872 15.0246 9.10459 14.1719L9.17869 12.4104C9.21028 11.6592 9.52251 10.9472 10.0536 10.4153L17.1721 3.28601Z",
                stroke: "currentColor",
                stroke_width: "2",
            }

            path {
                d: "M16.4364 4.16082C16.0458 3.77029 15.4129 3.77053 15.0227 4.16135C14.6325 4.55217 14.6327 5.18558 15.0232 5.5761L16.4364 4.16082ZM19.0372 9.59009C19.4277 9.98061 20.0607 9.98037 20.4509 9.58955C20.8411 9.19873 20.8409 8.56533 20.4503 8.1748L19.0372 9.59009ZM20.4503 8.1748L16.4364 4.16082L15.0232 5.5761L19.0372 9.59009L20.4503 8.1748Z",
                fill: "currentColor",
            }
        }
    }
}
