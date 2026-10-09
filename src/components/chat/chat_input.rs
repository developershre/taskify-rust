use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ChatInputProps {
    pub on_send: EventHandler<String>,
    #[props(default)]
    pub on_send_doc: Option<EventHandler<(String, String)>>,
    #[props(default)]
    pub on_send_audio: Option<EventHandler<String>>,
    #[props(default)]
    pub on_send_image: Option<EventHandler<(String, String)>>,
}

use crate::icons::CloseIcon;

#[component]
pub fn ChatInput(props: ChatInputProps) -> Element {
    let mut message_text = use_signal(String::new);
    let mut show_emoji_picker = use_signal(|| false);
    let mut show_attach_menu = use_signal(|| false);
    let mut is_recording = use_signal(|| false);
    let mut record_seconds = use_signal(|| 0);

    let on_send = props.on_send;

    let emojis = [
        "👍", "❤️", "🔥", "🎉", "👏", "😊", "🚀", "💡", "🥳", "✨", "😂", "🙌", "💯", "🤝",
        "😍", "👀", "🙏", "💪", "😎", "🎯", "⚡", "⭐", "💼", "📎",
    ];

    rsx! {
        div { class: "p-3 sm:p-4 border-t border-border/40 bg-card/60 backdrop-blur shrink-0 relative",

            // 1. Emoji Picker Popover
            if *show_emoji_picker.read() {
                div { class: "absolute bottom-16 right-4 sm:right-28 p-3 rounded-2xl bg-popover border border-border shadow-xl z-20 w-72 max-w-[90vw]",
                    div { class: "flex items-center justify-between pb-2 mb-2 border-b border-border/40",
                        span { class: "text-xs font-semibold text-foreground", "Quick Reactions & Emojis" }
                        button {
                            r#type: "button",
                            class: "text-muted-foreground hover:text-foreground text-xs cursor-pointer",
                            onclick: move |_| show_emoji_picker.set(false),
                            CloseIcon{ class: "size-4" }
                        }
                    }
                    div { class: "grid grid-cols-6 gap-1.5",
                        for emoji in emojis {
                            button {
                                key: "{emoji}",
                                r#type: "button",
                                class: "size-8.5 flex items-center justify-center rounded-lg hover:bg-muted text-lg hover:scale-115 transition-all cursor-pointer",
                                onclick: move |_| {
                                    message_text.write().push_str(emoji);
                                    show_emoji_picker.set(false);
                                },
                                "{emoji}"
                            }
                        }
                    }
                }
            }

            // 2. Attachment Menu Popover
            if *show_attach_menu.read() {
                div { class: "absolute bottom-16 left-4 sm:left-14 p-2 rounded-2xl bg-popover border border-border shadow-xl z-20 flex flex-col gap-1 w-52",
                    button {
                        r#type: "button",
                        class: "flex items-center gap-2.5 px-3 py-2 rounded-xl text-xs font-medium text-foreground hover:bg-muted/80 transition-colors cursor-pointer text-left",
                        onclick: move |_| {
                            show_attach_menu.set(false);
                            if let Some(handler) = &props.on_send_image {
                                handler.call(("https://images.unsplash.com/photo-1517336714731-489689fd1ca8?w=800".to_string(), "Design Mockup Image".to_string()));
                            } else {
                                on_send.call("📷 Shared a photo: Design_Mockup.png".to_string());
                            }
                        },
                        span { class: "size-7 rounded-lg bg-emerald-500/15 text-emerald-500 flex items-center justify-center shrink-0",
                            svg { class: "size-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                                rect { width: "18", height: "18", x: "3", y: "3", rx: "2" }
                                circle { cx: "9", cy: "9", r: "2" }
                                path { d: "m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21" }
                            }
                        }
                        span { "Photo or Video" }
                    }
                    button {
                        r#type: "button",
                        class: "flex items-center gap-2.5 px-3 py-2 rounded-xl text-xs font-medium text-foreground hover:bg-muted/80 transition-colors cursor-pointer text-left",
                        onclick: move |_| {
                            show_attach_menu.set(false);
                            if let Some(handler) = &props.on_send_doc {
                                handler.call(("Project_Specifications_v2.pdf".to_string(), "1.2 MB".to_string()));
                            } else {
                                on_send.call("📄 Attached document: Project_Specifications_v2.pdf".to_string());
                            }
                        },
                        span { class: "size-7 rounded-lg bg-blue-500/15 text-blue-500 flex items-center justify-center shrink-0",
                            svg { class: "size-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                                path { d: "M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" }
                                path { d: "M14 2v4a2 2 0 0 0 2 2h4" }
                            }
                        }
                        span { "PDF Document" }
                    }
                    button {
                        r#type: "button",
                        class: "flex items-center gap-2.5 px-3 py-2 rounded-xl text-xs font-medium text-foreground hover:bg-muted/80 transition-colors cursor-pointer text-left",
                        onclick: move |_| {
                            show_attach_menu.set(false);
                            on_send.call("⚡ Linked Task: #TASK-402 - Auth Refactor".to_string());
                        },
                        span { class: "size-7 rounded-lg bg-purple-500/15 text-purple-500 flex items-center justify-center shrink-0",
                            svg { class: "size-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                                path { d: "M13 2 3 14h9l-1 8 10-12h-9l1-8z" }
                            }
                        }
                        span { "Share Task / Issue" }
                    }
                }
            }

            // 3. Input bar or Live Voice Recorder Bar
            if *is_recording.read() {
                div { class: "flex items-center justify-between gap-3 rounded-2xl border border-destructive/40 bg-destructive/10 px-4 py-2.5 shadow-xs transition-all",
                    div { class: "flex items-center gap-3",
                        span { class: "size-3 rounded-full bg-destructive animate-ping" }
                        span { class: "text-xs sm:text-sm font-semibold text-destructive",
                            "Recording Voice Note ({record_seconds()}s)..."
                        }
                    }
                    div { class: "flex items-center gap-2",
                        button {
                            r#type: "button",
                            class: "px-3 py-1 text-xs font-medium text-muted-foreground hover:text-foreground hover:bg-background/80 rounded-lg transition-colors cursor-pointer",
                            onclick: move |_| {
                                is_recording.set(false);
                                record_seconds.set(0);
                            },
                            "Cancel"
                        }
                        button {
                            r#type: "button",
                            class: "inline-flex items-center gap-1.5 rounded-xl bg-destructive px-3.5 py-1 text-xs font-semibold text-destructive-foreground hover:opacity-90 transition-opacity cursor-pointer shadow-xs",
                            onclick: move |_| {
                                is_recording.set(false);
                                if let Some(handler) = &props.on_send_audio {
                                    handler.call(format!("0:{:02}", record_seconds().max(3)));
                                } else {
                                    on_send.call("🎙️ Sent a voice memo".to_string());
                                }
                                record_seconds.set(0);
                            },
                            "Send Audio"
                        }
                    }
                }
            } else {
                div { class: "flex items-center gap-2 rounded-2xl border border-border/50 bg-background/70 px-3.5 py-2 shadow-xs focus-within:border-primary/50 focus-within:ring-1 focus-within:ring-primary/20 transition-all",

                    // Paperclip attachment button
                    button {
                        r#type: "button",
                        class: "inline-flex size-7 items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted/60 transition-colors cursor-pointer shrink-0",
                        title: "Attach file or media",
                        onclick: move |_| {
                            let cur = *show_attach_menu.read();
                            show_attach_menu.set(!cur);
                            show_emoji_picker.set(false);
                        },
                        svg {
                            class: "size-4",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "m21.44 11.05-9.19 9.19a6 6 0 0 1-8.49-8.49l8.57-8.57A4 4 0 1 1 18 8.84l-8.59 8.57a2 2 0 0 1-2.83-2.83l8.49-8.48" }
                        }
                    }

                    input {
                        r#type: "text",
                        class: "flex-1 min-w-0 bg-transparent text-xs sm:text-sm text-foreground placeholder:text-muted-foreground/60 focus:outline-none",
                        placeholder: "Type a message...",
                        value: message_text(),
                        oninput: move |e| {
                            message_text.set(e.value());
                        },
                        onkeydown: move |e| {
                            if e.key() == Key::Enter && !e.modifiers().shift() {
                                handle_send(message_text, on_send);
                            }
                        },
                    }

                    // Emoji button
                    button {
                        r#type: "button",
                        class: "inline-flex size-7 items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted/60 transition-colors cursor-pointer shrink-0",
                        title: "Insert emoji",
                        onclick: move |_| {
                            let cur = *show_emoji_picker.read();
                            show_emoji_picker.set(!cur);
                            show_attach_menu.set(false);
                        },
                        svg {
                            class: "size-4",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            circle { cx: "12", cy: "12", r: "10" }
                            path { d: "M8 14s1.5 2 4 2 4-2 4-2" }
                            line { x1: "9", x2: "9.01", y1: "9", y2: "9" }
                            line { x1: "15", x2: "15.01", y1: "9", y2: "9" }
                        }
                    }

                    // Microphone voice recording trigger
                    button {
                        r#type: "button",
                        class: "inline-flex size-7 items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted/60 transition-colors cursor-pointer shrink-0",
                        title: "Record voice message",
                        onclick: move |_| {
                            is_recording.set(true);
                            record_seconds.set(3);
                        },
                        svg {
                            class: "size-4",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3Z" }
                            path { d: "M19 10v2a7 7 0 0 1-14 0v-2" }
                            line { x1: "12", x2: "12", y1: "19", y2: "22" }
                        }
                    }

                    // Send Button
                    button {
                        r#type: "button",
                        class: "inline-flex items-center gap-1.5 rounded-xl bg-primary px-3.5 py-1.5 text-xs font-semibold text-primary-foreground hover:opacity-90 active:scale-98 transition-all cursor-pointer shadow-xs shrink-0",
                        onclick: move |_| handle_send(message_text, on_send),
                        "Send"
                    }
                }
            }
        }
    }
}

fn handle_send(mut text_sig: Signal<String>, handler: EventHandler<String>) {
    let text = text_sig.read().trim().to_string();
    if !text.is_empty() {
        handler.call(text);
        text_sig.set(String::new());
    }
}
