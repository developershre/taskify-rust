use dioxus::prelude::*;

use crate::components::ui::{
    Command, CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandList,
};

#[component]
pub fn TitleSearch() -> Element {
    let open = Signal::new(false);
    rsx! {
        div{
            CommandDialog{
                open: open,
                Command {
                    CommandInput {
                        placeholder: "Search...".to_string(),
                        value: Signal::new("".to_string()),
                        oninput: None,
                    }
                    CommandList {
                        CommandEmpty {
                            children: "No results found",
                        },
                        CommandGroup {
                            heading:"",
                            CommandList{
                                onclick: move |_| {
                                },
                                "Dashboard"
                            }
                        }
                    }
                }
            }
        }
    }
}
