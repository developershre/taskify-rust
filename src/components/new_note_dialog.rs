use dioxus::prelude::*;

use crate::components::ui::{
    Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, Input,
};
use crate::state::{use_app_state, AppState, OverlayState};

fn submit_new_note(
    mut app_state: AppState,
    title: Signal<String>,
    desc: Signal<String>,
    mut open: Signal<bool>,
) {
    let t = title.read().trim().to_string();
    if t.is_empty() {
        return;
    }
    let d = desc.read().trim().to_string();
    app_state.add_note(
        t,
        if d.is_empty() {
            "A simple item with title and description.".to_string()
        } else {
            d
        },
    );
    open.set(false);
}

#[component]
pub fn NewNoteDialog() -> Element {
    let app_state = use_app_state();
    let mut open = app_state.new_note_open;
    let mut title = use_signal(String::new);
    let mut desc = use_signal(String::new);

    let overlay = use_context::<OverlayState>();
    use_effect(move || {
        let is_open = *open.read();
        overlay.set_dialog_open(is_open);
        if !is_open {
            title.write().clear();
            desc.write().clear();
        }
    });

    use_drop(move || {
        overlay.set_dialog_open(false);
    });

    rsx! {
        Dialog { open: open,
            DialogContent { class: "max-w-md",
                DialogHeader {
                    DialogTitle { "New Note" }
                    DialogDescription { "Capture a quick note. It shows up in the calendar sidebar." }
                }

                div { class: "flex flex-col gap-4",
                    div { class: "flex flex-col gap-1.5",
                        label { class: "text-xs font-medium text-foreground", "Title" }
                        Input {
                            placeholder: "Note title...",
                            value: title(),
                            oninput: move |e: FormEvent| title.set(e.value()),
                            onkeydown: move |e: KeyboardEvent| {
                                if e.key() == Key::Enter {
                                    submit_new_note(app_state, title, desc, open);
                                }
                            },
                        }
                    }

                    div { class: "flex flex-col gap-1.5",
                        label { class: "text-xs font-medium text-foreground", "Description" }
                        textarea {
                            class: "min-h-20 w-full resize-none rounded-md border border-input bg-transparent px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring",
                            placeholder: "A simple item with title and description...",
                            value: "{desc}",
                            oninput: move |e| desc.set(e.value()),
                        }
                        span { class: "text-[11px] text-muted-foreground",
                            "Optional. Leave blank for the default description."
                        }
                    }
                }

                DialogFooter {
                    button {
                        r#type: "button",
                        class: "inline-flex h-8 items-center rounded-md border border-border px-3 text-xs font-medium text-foreground hover:bg-muted transition-colors cursor-pointer",
                        onclick: move |_| open.set(false),
                        "Cancel"
                    }
                    button {
                        r#type: "button",
                        class: "inline-flex h-8 items-center rounded-md bg-primary px-3 text-xs font-medium text-primary-foreground hover:opacity-90 transition-opacity cursor-pointer disabled:opacity-50",
                        disabled: title.read().trim().is_empty(),
                        onclick: move |_| submit_new_note(app_state, title, desc, open),
                        "Add Note"
                    }
                }
            }
        }
    }
}
