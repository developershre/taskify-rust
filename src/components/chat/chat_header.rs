use dioxus::prelude::*;
use super::types::ChatContact;

#[derive(Props, Clone, PartialEq)]
pub struct ChatHeaderProps {
    pub contact: Option<ChatContact>,
    #[props(default)]
    pub on_back: Option<EventHandler<()>>,
}

#[component]
pub fn ChatHeader(props: ChatHeaderProps) -> Element {
    let Some(contact) = props.contact else {
        return rsx! {};
    };

    rsx! {
        div { class: "flex items-center justify-between px-4 py-3 border-b border-border/40 bg-card/60 backdrop-blur shrink-0",

            // Left: Back button (for mobile) + Avatar + Name + Online status
            div { class: "flex items-center gap-3 min-w-0",
                if let Some(on_back) = props.on_back {
                    button {
                        r#type: "button",
                        class: "sm:hidden inline-flex size-8 items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted/60 transition-colors cursor-pointer",
                        onclick: move |_| on_back.call(()),
                        svg {
                            class: "size-4",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "m15 18-6-6 6-6" }
                        }
                    }
                }

                div { class: "relative shrink-0",
                    if let Some(avatar_url) = &contact.avatar_url {
                        img {
                            class: "size-9 rounded-full object-cover border border-border/40",
                            src: "{avatar_url}",
                            alt: "{contact.name}",
                        }
                    } else if contact.is_group {
                        div { class: "flex size-9 items-center justify-center rounded-full bg-muted border border-border/40 text-muted-foreground",
                            svg {
                                class: "size-4.5",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" }
                                circle { cx: "9", cy: "7", r: "4" }
                                path { d: "M22 21v-2a4 4 0 0 0-3-3.87" }
                                path { d: "M16 3.13a4 4 0 0 1 0 7.75" }
                            }
                        }
                    } else {
                        div { class: "flex size-9 items-center justify-center rounded-full bg-primary/10 border border-primary/20 text-xs font-semibold text-primary",
                            "{contact.initials}"
                        }
                    }

                    if contact.is_online {
                        span {
                            class: "absolute bottom-0 right-0 size-2.5 rounded-full bg-emerald-500 ring-2 ring-card",
                            title: "Online",
                        }
                    }
                }

                div { class: "flex flex-col min-w-0",
                    span { class: "truncate text-xs sm:text-sm font-semibold text-foreground",
                        "{contact.name}"
                    }
                    if contact.is_online {
                        span { class: "text-[11px] font-medium text-emerald-400 leading-tight",
                            "Online"
                        }
                    } else {
                        span { class: "text-[11px] font-normal text-muted-foreground/80 leading-tight",
                            if contact.is_group { "Group chat" } else { "Offline" }
                        }
                    }
                }
            }

            // Right: Video call, Voice call, More actions
            div { class: "flex items-center gap-1 sm:gap-1.5 shrink-0",
                button {
                    r#type: "button",
                    class: "inline-flex size-8 items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted/60 transition-colors cursor-pointer",
                    title: "Start video call",
                    svg {
                        class: "size-4",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "m16 13 5.223 3.482a.5.5 0 0 0 .777-.416V7.87a.5.5 0 0 0-.752-.432L16 10.5" }
                        rect { x: "2", y: "6", width: "14", height: "12", rx: "2" }
                    }
                }

                button {
                    r#type: "button",
                    class: "inline-flex size-8 items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted/60 transition-colors cursor-pointer",
                    title: "Start voice call",
                    svg {
                        class: "size-4",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" }
                    }
                }

                button {
                    r#type: "button",
                    class: "inline-flex size-8 items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted/60 transition-colors cursor-pointer",
                    title: "More options",
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
            }
        }
    }
}
