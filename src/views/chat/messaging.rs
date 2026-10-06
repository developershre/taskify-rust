use dioxus::prelude::*;

use crate::components::ui::{Avatar, AvatarFallback, AvatarImage, Input};
use crate::views::PageHeader;

struct Message {
    text: String,
    from_me: bool,
    time: String,
}

fn send_message(mut draft: Signal<String>, mut messages: Signal<Vec<Message>>) {
    let text = draft.read().trim().to_string();
    if !text.is_empty() {
        messages.write().push(Message {
            text,
            from_me: true,
            time: "Now".to_string(),
        });
        draft.write().clear();
    }
}

#[component]
pub fn ChatMessaging() -> Element {
    let mut draft = use_signal(String::new);
    let messages = use_signal(|| {
        vec![
            Message {
                text: "Hey! How is the sidebar refactor going?".to_string(),
                from_me: false,
                time: "09:41".to_string(),
            },
            Message {
                text: "Almost done — popups now render with fixed positioning, so dialogs always stay on top.".to_string(),
                from_me: true,
                time: "09:42".to_string(),
            },
            Message {
                text: "Nice. Can you also check the breadcrumb on nested routes?".to_string(),
                from_me: false,
                time: "09:44".to_string(),
            },
            Message {
                text: "Already handled — it builds every segment from the URL.".to_string(),
                from_me: true,
                time: "09:45".to_string(),
            },
        ]
    });

    rsx! {
        div { class: "flex h-full flex-col",
            PageHeader {
                title: "Messaging",
                description: "Direct messages with your teammates.",
            }

            div { class: "flex min-h-0 flex-1 flex-col overflow-hidden rounded-xl border border-border/40 bg-card shadow-xs",
                // Thread header
                div { class: "flex items-center gap-3 border-b border-border/40 px-4 py-3",
                    Avatar { class: "size-9 rounded-lg overflow-hidden border border-border/40",
                        AvatarImage {
                            src: "https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=100&auto=format&fit=crop&q=80"
                                .to_string(),
                            alt: "Alice Kim".to_string(),
                        }
                        AvatarFallback { "AK" }
                    }
                    div { class: "flex min-w-0 flex-col",
                        span { class: "truncate text-sm font-semibold text-foreground",
                            "Alice Kim"
                        }
                        span { class: "text-xs text-primary", "Online" }
                    }
                }

                // Messages
                div { class: "flex min-h-0 flex-1 flex-col gap-2 overflow-y-auto p-4",
                    for (i, message) in messages.read().iter().enumerate() {
                        {
                            let bubble_class = if message.from_me {
                                "self-end bg-primary text-primary-foreground"
                            } else {
                                "self-start bg-muted text-foreground"
                            };
                            rsx! {
                                div { key: "{i}",
                                    class: "flex max-w-[75%] flex-col gap-1 rounded-2xl px-3 py-2 {bubble_class}",
                                    p { class: "text-sm leading-snug break-words", "{message.text}" }
                                    span { class: "text-[10px] opacity-70", "{message.time}" }
                                }
                            }
                        }
                    }
                }

                // Composer
                div { class: "flex items-center gap-2 border-t border-border/40 p-3",
                    Input {
                        value: draft.read().clone(),
                        placeholder: "Type a message…",
                        oninput: move |e: FormEvent| {
                            draft.set(e.value());
                        },
                        onkeydown: move |e: KeyboardEvent| {
                            if e.key() == Key::Enter {
                                send_message(draft, messages);
                            }
                        },
                    }
                    button {
                        r#type: "button",
                        class: "inline-flex size-9 shrink-0 items-center justify-center rounded-md bg-primary text-primary-foreground shadow-xs hover:bg-primary/90 transition-colors cursor-pointer disabled:opacity-50",
                        disabled: draft.read().trim().is_empty(),
                        onclick: move |_| {
                            send_message(draft, messages);
                        },
                        svg {
                            class: "size-4",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "m22 2-7 20-4-9-9-4Z" }
                            path { d: "M22 2 11 13" }
                        }
                        span { class: "sr-only", "Send" }
                    }
                }
            }
        }
    }
}
