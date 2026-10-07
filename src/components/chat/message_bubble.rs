use dioxus::prelude::*;
use super::types::{ChatMessage, MessageContent, MessageStatus};

#[derive(Props, Clone, PartialEq)]
pub struct MessageBubbleProps {
    pub message: ChatMessage,
}

#[component]
pub fn MessageBubble(props: MessageBubbleProps) -> Element {
    let message = props.message;
    let is_outgoing = message.is_outgoing;
    let mut is_playing_audio = use_signal(|| false);

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
                        // Options menu dots
                        button {
                            r#type: "button",
                            class: "text-muted-foreground/60 hover:text-foreground transition-colors cursor-pointer p-0.5",
                            title: "Audio options",
                            svg {
                                class: "size-4",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                circle { cx: "12", cy: "12", r: "1" }
                                circle { cx: "19", cy: "12", r: "1" }
                                circle { cx: "5", cy: "12", r: "1" }
                            }
                        }

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
                                if *is_playing_audio.read() { "0:24 / 1:15" } else { "0:00 / 0:00" }
                            }
                            div { class: "flex-1 h-1.5 rounded-full bg-muted overflow-hidden relative cursor-pointer",
                                div {
                                    class: "h-full bg-primary/70 rounded-full transition-all duration-200",
                                    style: if *is_playing_audio.read() { "width: 32%" } else { "width: 0%" },
                                }
                            }
                        }

                        // Speaker / Volume icon
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

                        // Three dots vertical
                        svg {
                            class: "size-4 text-muted-foreground/70 shrink-0 cursor-pointer",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            circle { cx: "12", cy: "6", r: "1" }
                            circle { cx: "12", cy: "12", r: "1" }
                            circle { cx: "12", cy: "18", r: "1" }
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
        div { class: "flex flex-col gap-1 max-w-[85%] sm:max-w-[70%] md:max-w-[60%] {align_class}",
            {content_element}

            // Message timestamp and tick
            div { class: "flex items-center gap-1 px-1 self-end text-[11px] text-muted-foreground/80 font-normal",
                span { "{message.time}" }
                if is_outgoing {
                    {tick_element}
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
}

#[component]
fn MediaGridTile(props: MediaGridTileProps) -> Element {
    let item = props.item;
    rsx! {
        div {
            class: "relative rounded-xl overflow-hidden aspect-4/3 bg-muted group",
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

