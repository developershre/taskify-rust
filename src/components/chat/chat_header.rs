use crate::components::ui::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator, DropdownMenuTrigger,
};
use dioxus::prelude::*;
use super::types::ChatContact;

#[derive(Props, Clone, PartialEq)]
pub struct ChatHeaderProps {
    pub contact: Option<ChatContact>,
    #[props(default)]
    pub on_back: Option<EventHandler<()>>,
    #[props(default)]
    pub on_clear_chat: Option<EventHandler<()>>,
}

#[component]
pub fn ChatHeader(props: ChatHeaderProps) -> Element {
    let Some(contact) = props.contact else {
        return rsx! {};
    };

    let mut is_in_call = use_signal(|| false);
    let mut is_video_call = use_signal(|| false);
    let mut is_mic_muted = use_signal(|| false);
    let mut is_cam_off = use_signal(|| false);
    let mut show_info = use_signal(|| false);

    rsx! {
        div { class: "flex items-center justify-between px-4 py-3 border-b border-border/40 bg-card/60 backdrop-blur shrink-0 relative",

            // Active Call Modal Overlay
            if *is_in_call.read() {
                div { class: "absolute inset-0 z-30 flex items-center justify-between bg-card/95 px-4 backdrop-blur-md animate-in fade-in duration-200",
                    div { class: "flex items-center gap-3",
                        div { class: "relative",
                            span { class: "size-3 rounded-full bg-emerald-500 animate-ping absolute inset-0" }
                            span { class: "size-3 rounded-full bg-emerald-500 block relative" }
                        }
                        div { class: "flex flex-col",
                            span { class: "text-xs sm:text-sm font-semibold text-foreground",
                                if *is_video_call.read() { "Video Call with " } else { "Voice Call with " }
                                "{contact.name}"
                            }
                            span { class: "text-[11px] font-mono text-emerald-500", "01:24 (Connected)" }
                        }
                    }

                    div { class: "flex items-center gap-2",
                        // Mute Mic
                        button {
                            r#type: "button",
                            class: if *is_mic_muted.read() {
                                "size-8 rounded-full bg-destructive/20 text-destructive flex items-center justify-center cursor-pointer"
                            } else {
                                "size-8 rounded-full bg-muted text-foreground flex items-center justify-center hover:bg-muted/80 cursor-pointer"
                            },
                            title: if *is_mic_muted.read() { "Unmute Mic" } else { "Mute Mic" },
                            onclick: move |_| {
                                let cur = *is_mic_muted.read();
                                is_mic_muted.set(!cur);
                            },
                            svg { class: "size-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                                path { d: "M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3Z" }
                                path { d: "M19 10v2a7 7 0 0 1-14 0v-2" }
                            }
                        }

                        // Toggle Camera (for video calls)
                        if *is_video_call.read() {
                            button {
                                r#type: "button",
                                class: if *is_cam_off.read() {
                                    "size-8 rounded-full bg-destructive/20 text-destructive flex items-center justify-center cursor-pointer"
                                } else {
                                    "size-8 rounded-full bg-muted text-foreground flex items-center justify-center hover:bg-muted/80 cursor-pointer"
                                },
                                title: if *is_cam_off.read() { "Turn Camera On" } else { "Turn Camera Off" },
                                onclick: move |_| {
                                    let cur = *is_cam_off.read();
                                    is_cam_off.set(!cur);
                                },
                                svg { class: "size-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                                    path { d: "m16 13 5.223 3.482a.5.5 0 0 0 .777-.416V7.87a.5.5 0 0 0-.752-.432L16 10.5" }
                                    rect { x: "2", y: "6", width: "14", height: "12", rx: "2" }
                                }
                            }
                        }

                        // End Call
                        button {
                            r#type: "button",
                            class: "size-8 rounded-full bg-destructive text-destructive-foreground flex items-center justify-center hover:opacity-90 shadow-xs cursor-pointer",
                            title: "End Call",
                            onclick: move |_| is_in_call.set(false),
                            svg { class: "size-4 rotate-135", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.5",
                                path { d: "M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" }
                            }
                        }
                    }
                }
            }

            // Contact Info Popover Modal
            if *show_info.read() {
                div { class: "absolute top-14 right-4 z-40 w-72 rounded-2xl border border-border bg-popover p-4 shadow-2xl animate-in fade-in duration-150",
                    div { class: "flex items-center justify-between pb-3 border-b border-border/40",
                        span { class: "text-xs font-semibold text-foreground", "Contact Profile" }
                        button {
                            r#type: "button",
                            class: "text-muted-foreground hover:text-foreground text-xs cursor-pointer",
                            onclick: move |_| show_info.set(false),
                            "✕"
                        }
                    }
                    div { class: "flex flex-col items-center gap-2 py-3",
                        if let Some(avatar_url) = &contact.avatar_url {
                            img { class: "size-14 rounded-full object-cover border-2 border-border/60", src: "{avatar_url}", alt: "{contact.name}" }
                        } else {
                            div { class: "size-14 rounded-full bg-primary/10 border border-primary/20 flex items-center justify-center text-sm font-bold text-primary",
                                "{contact.initials}"
                            }
                        }
                        div { class: "text-center",
                            h4 { class: "text-sm font-semibold text-foreground", "{contact.name}" }
                            p { class: "text-xs text-muted-foreground",
                                if contact.is_online { "Active Now" } else { "Offline" }
                            }
                        }
                    }
                    div { class: "space-y-1.5 pt-2 border-t border-border/40 text-xs text-muted-foreground",
                        div { class: "flex justify-between", span { "Role:" }, span { class: "font-medium text-foreground", "Product Designer" } }
                        div { class: "flex justify-between", span { "Team:" }, span { class: "font-medium text-foreground", "Core UI / UX" } }
                        div { class: "flex justify-between", span { "Location:" }, span { class: "font-medium text-foreground", "San Francisco, CA" } }
                    }
                }
            }

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

                div { class: "relative shrink-0 cursor-pointer",
                    onclick: move |_| {
                        let cur = *show_info.read();
                        show_info.set(!cur);
                    },
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

                div { class: "flex flex-col min-w-0 cursor-pointer",
                    onclick: move |_| {
                        let cur = *show_info.read();
                        show_info.set(!cur);
                    },
                    span { class: "truncate text-xs sm:text-sm font-semibold text-foreground",
                        "{contact.name}"
                    }
                    if contact.is_online {
                        span { class: "text-[11px] font-medium text-emerald-500 leading-tight",
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

                // Video Call Button
                button {
                    r#type: "button",
                    class: "inline-flex size-8 items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted/60 transition-colors cursor-pointer",
                    title: "Start video call",
                    onclick: move |_| {
                        is_video_call.set(true);
                        is_in_call.set(true);
                    },
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

                // Voice Call Button
                button {
                    r#type: "button",
                    class: "inline-flex size-8 items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted/60 transition-colors cursor-pointer",
                    title: "Start voice call",
                    onclick: move |_| {
                        is_video_call.set(false);
                        is_in_call.set(true);
                    },
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

                // More options dropdown
                DropdownMenu {
                    DropdownMenuTrigger {
                        class: "inline-flex size-8 items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted/60 transition-colors cursor-pointer",
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
                    DropdownMenuContent {
                        align: "end",
                        side_offset: 6,
                        DropdownMenuItem {
                            onclick: move |_| show_info.set(true),
                            "View contact info"
                        }
                        DropdownMenuItem {
                            "Mute notifications"
                        }
                        DropdownMenuItem {
                            "Search in conversation"
                        }
                        DropdownMenuSeparator {}
                        if let Some(on_clear) = props.on_clear_chat {
                            DropdownMenuItem {
                                onclick: move |_| on_clear.call(()),
                                "Clear chat history"
                            }
                        }
                        DropdownMenuItem {
                            variant: "destructive",
                            "Block contact"
                        }
                    }
                }
            }
        }
    }
}
