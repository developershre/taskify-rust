use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ChatInputProps {
    pub on_send: EventHandler<String>,
}

#[component]
pub fn ChatInput(props: ChatInputProps) -> Element {
    let message_text = use_signal(String::new);
    let mut show_emoji_picker = use_signal(|| false);

    let on_send = props.on_send;
    let send_message = move || {
        let text = message_text.read().trim().to_string();
        if !text.is_empty() {
            on_send.call(text);
            let mut mt = message_text;
            mt.set(String::new());
        }
    };

    let emojis = ["👍", "❤️", "🔥", "🎉", "👏", "😊", "🚀", "💡"];

    rsx! {
        div { class: "p-3 sm:p-4 border-t border-border/40 bg-card/60 backdrop-blur shrink-0 relative",

            // Quick emoji picker bubble if toggled
            if *show_emoji_picker.read() {
                div { class: "absolute bottom-16 right-4 sm:right-28 flex items-center gap-1.5 p-2 rounded-2xl bg-popover border border-border shadow-lg z-20",
                    for emoji in emojis {
                        button {
                            key: "{emoji}",
                            r#type: "button",
                            class: "size-8 flex items-center justify-center rounded-lg hover:bg-muted text-base transition-colors cursor-pointer",
                            onclick: move |_| {
                                let mut mt = message_text;
                                mt.write().push_str(emoji);
                                show_emoji_picker.set(false);
                            },
                            "{emoji}"
                        }
                    }
                }
            }

            div { class: "flex items-center gap-2 rounded-2xl border border-border/50 bg-background/70 px-3.5 py-2 shadow-xs focus-within:border-primary/50 focus-within:ring-1 focus-within:ring-primary/20 transition-all",

                input {
                    r#type: "text",
                    class: "flex-1 min-w-0 bg-transparent text-xs sm:text-sm text-foreground placeholder:text-muted-foreground/60 focus:outline-none",
                    placeholder: "Type a message...",
                    value: message_text(),
                    oninput: move |e| {
                        let mut mt = message_text;
                        mt.set(e.value());
                    },
                    onkeydown: move |e| {
                        if e.key() == Key::Enter && !e.modifiers().shift() {
                            send_message();
                        }
                    },
                }

                // Emoji button
                button {
                    r#type: "button",
                    class: "inline-flex size-7 items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted/60 transition-colors cursor-pointer",
                    title: "Insert emoji",
                    onclick: move |_| {
                        let cur = *show_emoji_picker.read();
                        show_emoji_picker.set(!cur);
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

                // Paperclip attachment button
                button {
                    r#type: "button",
                    class: "inline-flex size-7 items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted/60 transition-colors cursor-pointer",
                    title: "Attach file",
                    onclick: move |_| {},
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

                // Microphone voice button
                button {
                    r#type: "button",
                    class: "inline-flex size-7 items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted/60 transition-colors cursor-pointer",
                    title: "Record voice message",
                    onclick: move |_| {},
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
                    class: "inline-flex items-center gap-1.5 rounded-xl bg-primary px-3.5 py-1.5 text-xs font-semibold text-primary-foreground hover:opacity-90 active:scale-98 transition-all cursor-pointer shadow-xs",
                    onclick: move |_| send_message(),
                    "Send"
                }
            }
        }
    }
}
