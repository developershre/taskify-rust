use dioxus::prelude::*;
use super::types::{ChatMessage, MessageContent, MessageStatus};

#[derive(Props, Clone, PartialEq)]
pub struct MessageBubbleProps {
    pub message: ChatMessage,
    #[props(default)]
    pub on_react: Option<EventHandler<(String, String)>>,
}

#[component]
pub fn MessageBubble(props: MessageBubbleProps) -> Element {
    let message = props.message;
    let is_outgoing = message.is_outgoing;
    let mut is_playing_audio = use_signal(|| false);
    let mut show_reaction_menu = use_signal(|| false);
    let mut preview_image_url = use_signal(|| Option::<String>::None);
    let mut local_reactions = use_signal(|| message.reactions.clone());

    let align_class = if is_outgoing {
        "items-end ml-auto"
    } else {
        "items-start mr-auto"
    };

    let tick_element = match message.status {
        MessageStatus::Read => rsx! {
            svg {
                class: "size-3.5 text-emerald-500 shrink-0",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M18 6 7 17l-5-5" }
                path { d: "m22 10-7.5 7.5L13 16" }
            }
        },
        MessageStatus::Delivered => rsx! {
            svg {
                class: "size-3.5 text-muted-foreground/60 shrink-0",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M18 6 7 17l-5-5" }
                path { d: "m22 10-7.5 7.5L13 16" }
            }
        },
        MessageStatus::Sent => rsx! {
            svg {
                class: "size-3.5 text-muted-foreground/60 shrink-0",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M20 6 9 17l-5-5" }
            }
        },
    };

    let content_element = match message.content {
        MessageContent::Document { name, size, url: _ } => rsx! {
            div { class: "flex flex-col gap-2 rounded-2xl border border-border/50 bg-card/90 p-3 sm:p-3.5 shadow-xs w-72 sm:w-80",
                div { class: "flex items-center gap-3",
                    div { class: "flex size-11 items-center justify-center rounded-xl bg-muted/70 text-muted-foreground border border-border/40 shrink-0",
                        svg {
                            class: "size-6",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "1.75",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" }
                            path { d: "M14 2v4a2 2 0 0 0 2 2h4" }
                            path { d: "M10 13H8" }
                            path { d: "M16 17H8" }
                            path { d: "M16 13h-2" }
                        }
                    }
                    div { class: "flex flex-col min-w-0 flex-1",
                        span { class: "truncate text-xs sm:text-sm font-semibold text-foreground",
                            "{name}"
                        }
                        span { class: "text-[11px] text-muted-foreground/80 font-normal",
                            "{size}"
                        }
                    }
                }

                div { class: "flex items-center gap-2 pt-1",
                    button {
                        r#type: "button",
                        class: "px-3 py-1 rounded-full bg-muted/80 text-foreground hover:bg-muted text-xs font-medium transition-colors cursor-pointer",
                        "Download"
                    }
                    button {
                        r#type: "button",
                        class: "px-3 py-1 rounded-full bg-muted/80 text-foreground hover:bg-muted text-xs font-medium transition-colors cursor-pointer",
                        "Preview"
                    }
                }
            }
        },

        MessageContent::Audio { duration: _, url: _ } => rsx! {
            div { class: "flex items-center gap-2.5 rounded-2xl border border-border/50 bg-card/90 px-3.5 py-2.5 shadow-xs w-72 sm:w-88",
                // Play/Pause button
                button {
                    r#type: "button",
                    class: "flex size-8 items-center justify-center rounded-full bg-primary/20 text-primary hover:bg-primary/30 transition-colors cursor-pointer shrink-0",
                    onclick: move |_| {
                        let cur = *is_playing_audio.read();
                        is_playing_audio.set(!cur);
                    },
                    if *is_playing_audio.read() {
                        svg {
                            class: "size-3.5 fill-current",
                            view_box: "0 0 24 24",
                            rect { x: "6", y: "4", width: "4", height: "16" }
                            rect { x: "14", y: "4", width: "4", height: "16" }
                        }
                    } else {
                        svg {
                            class: "size-3.5 fill-current ml-0.5",
                            view_box: "0 0 24 24",
                            polygon { points: "5 3 19 12 5 21 5 3" }
                        }
                    }
                }

                // Audio scrub track / timer
                div { class: "flex-1 flex items-center gap-2 min-w-0",
                    span { class: "text-[11px] font-mono text-muted-foreground shrink-0",
                        if *is_playing_audio.read() { "0:24 / 1:15" } else { "0:00 / 1:15" }
                    }
                    div { class: "flex-1 h-1.5 rounded-full bg-muted overflow-hidden relative cursor-pointer",
                        div {
                            class: "h-full bg-primary rounded-full transition-all duration-300",
                            style: if *is_playing_audio.read() { "width: 45%" } else { "width: 0%" },
                        }
                    }
                }

                // Speaker icon
                svg {
                    class: "size-4 text-muted-foreground/70 shrink-0",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    polygon { points: "11 5 6 9 2 9 2 15 6 15 11 19 11 5" }
                    path { d: "M15.54 8.46a5 5 0 0 1 0 7.07" }
                    path { d: "M19.07 4.93a10 10 0 0 1 0 14.14" }
                }
            }
        },

        MessageContent::Video { duration, thumbnail_url, video_url: _ } => rsx! {
            div { class: "relative rounded-2xl overflow-hidden border border-border/50 bg-card group w-72 sm:w-88 aspect-video shadow-xs",
                img {
                    class: "w-full h-full object-cover group-hover:scale-102 transition-transform duration-300",
                    src: "{thumbnail_url}",
                    alt: "Video preview",
                }

                div { class: "absolute inset-0 bg-black/30 group-hover:bg-black/20 transition-colors flex items-center justify-center",
                    button {
                        r#type: "button",
                        class: "flex size-11 items-center justify-center rounded-full bg-black/60 backdrop-blur-sm border border-white/20 text-white shadow-xl hover:scale-110 transition-transform cursor-pointer",
                        onclick: move |_| {
                            preview_image_url.set(Some("https://images.unsplash.com/photo-1517245386807-bb43f82c33c4?w=1200".to_string()));
                        },
                        svg {
                            class: "size-5 fill-current ml-0.5",
                            view_box: "0 0 24 24",
                            polygon { points: "6 4 20 12 6 20 6 4" }
                        }
                    }
                }

                span { class: "absolute bottom-2.5 right-2.5 rounded-md bg-black/75 px-1.5 py-0.5 text-[10px] font-semibold text-white/90 backdrop-blur-xs",
                    "{duration}"
                }
            }
        },

        MessageContent::MediaGrid { images, extra_count } => rsx! {
            div { class: "rounded-2xl border border-border/50 bg-card/80 p-1.5 grid grid-cols-2 gap-1.5 w-72 sm:w-88 shadow-xs",
                for (idx, item) in images.iter().enumerate() {
                    MediaGridTile {
                        key: "{item.url}",
                        item: item.clone(),
                        is_last: idx == images.len().saturating_sub(1),
                        extra_count: extra_count,
                        on_click: move |url: String| preview_image_url.set(Some(url)),
                    }
                }
            }
        },

        MessageContent::Text(text) => rsx! {
            div {
                class: format!(
                    "rounded-2xl px-4 py-2.5 text-xs sm:text-sm leading-relaxed border shadow-xs {}",
                    if is_outgoing {
                        "bg-primary text-primary-foreground border-primary/20 rounded-tr-xs"
                    } else {
                        "bg-card text-foreground border-border/50 rounded-tl-xs"
                    }
                ),
                "{text}"
            }
        },
    };

    rsx! {
        div { class: "group/msg relative flex flex-col gap-1 max-w-[85%] sm:max-w-[70%] md:max-w-[60%] {align_class}",

            // Quick Reaction Trigger Button on hover
            div {
                class: format!(
                    "absolute -top-3.5 {} opacity-0 group-hover/msg:opacity-100 transition-opacity z-10",
                    if is_outgoing { "left-0 -translate-x-full pr-1" } else { "right-0 translate-x-full pl-1" }
                ),
                button {
                    r#type: "button",
                    class: "size-6 rounded-full bg-popover border border-border shadow-xs flex items-center justify-center text-xs text-muted-foreground hover:text-foreground hover:scale-110 transition-all cursor-pointer",
                    title: "Add reaction",
                    onclick: move |_| {
                        let cur = *show_reaction_menu.read();
                        show_reaction_menu.set(!cur);
                    },
                    "😊"
                }

                // Reaction Palette
                if *show_reaction_menu.read() {
                    div { class: "absolute top-7 left-0 p-1.5 rounded-full bg-popover border border-border shadow-xl flex items-center gap-1 z-20",
                        for emoji in ["❤️", "👍", "🔥", "😂", "🎉"] {
                            button {
                                key: "{emoji}",
                                r#type: "button",
                                class: "size-6 rounded-full hover:bg-muted text-sm flex items-center justify-center hover:scale-125 transition-all cursor-pointer",
                                onclick: move |_| {
                                    local_reactions.write().push(emoji.to_string());
                                    show_reaction_menu.set(false);
                                },
                                "{emoji}"
                            }
                        }
                    }
                }
            }

            {content_element}

            // Reactions badges below message
            if !local_reactions.read().is_empty() {
                div { class: "flex flex-wrap gap-1 px-1 -mt-0.5",
                    for (i, reaction) in local_reactions.read().iter().enumerate() {
                        span {
                            key: "{i}",
                            class: "rounded-full bg-muted/80 border border-border/40 px-2 py-0.5 text-xs select-none shadow-2xs animate-in zoom-in-50",
                            "{reaction}"
                        }
                    }
                }
            }

            // Message timestamp and tick
            div { class: "flex items-center gap-1 px-1 self-end text-[11px] text-muted-foreground/80 font-normal",
                span { "{message.time}" }
                if is_outgoing {
                    {tick_element}
                }
            }

            // Lightbox Modal Preview
            if let Some(url) = preview_image_url() {
                div {
                    class: "fixed inset-0 z-50 bg-black/85 backdrop-blur-md flex items-center justify-center p-4",
                    onclick: move |_| preview_image_url.set(None),
                    div { class: "relative max-w-4xl max-h-[85vh] rounded-2xl overflow-hidden shadow-2xl",
                        img { class: "max-w-full max-h-[85vh] object-contain", src: "{url}", alt: "Preview" }
                        button {
                            r#type: "button",
                            class: "absolute top-3 right-3 size-9 rounded-full bg-black/60 text-white flex items-center justify-center hover:bg-black/80 transition-colors cursor-pointer",
                            onclick: move |_| preview_image_url.set(None),
                            "✕"
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct MediaGridTileProps {
    item: super::types::MediaItem,
    is_last: bool,
    extra_count: usize,
    on_click: EventHandler<String>,
}

#[component]
fn MediaGridTile(props: MediaGridTileProps) -> Element {
    let item = props.item;
    let url = item.url.clone();
    let on_click = props.on_click;

    rsx! {
        div {
            class: "relative rounded-xl overflow-hidden aspect-4/3 bg-muted group cursor-pointer",
            onclick: move |_| on_click.call(url.clone()),
            img {
                class: "w-full h-full object-cover group-hover:scale-105 transition-transform duration-300",
                src: "{item.url}",
                alt: "{item.title}",
            }

            if props.is_last && props.extra_count > 0 {
                div { class: "absolute inset-0 bg-black/60 backdrop-blur-[1px] flex items-center justify-center",
                    span { class: "text-lg sm:text-xl font-bold text-white select-none",
                        "+{props.extra_count}"
                    }
                }
            }
        }
    }
}

