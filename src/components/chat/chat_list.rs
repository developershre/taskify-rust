use dioxus::prelude::*;
use super::types::{ChatContact, MessageStatus};

#[derive(Props, Clone, PartialEq)]
pub struct ChatListProps {
    pub contacts: Vec<ChatContact>,
    pub selected_id: String,
    pub on_select: EventHandler<String>,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn ChatList(props: ChatListProps) -> Element {
    let mut search_query = use_signal(String::new);
    let query_lower = search_query.read().to_lowercase();

    let filtered_contacts: Vec<ChatContact> = props
        .contacts
        .iter()
        .filter(|c| {
            if query_lower.is_empty() {
                true
            } else {
                c.name.to_lowercase().contains(&query_lower)
                    || c.last_message.to_lowercase().contains(&query_lower)
            }
        })
        .cloned()
        .collect();

    rsx! {
        div {
            class: format!(
                "flex flex-col h-full w-full sm:w-80 md:w-88 shrink-0 border-r border-border/40 bg-card/50 select-none overflow-hidden {}",
                props.class
            ),

            // Top Header: "Chats" + "+" button
            div { class: "flex items-center justify-between p-3.5 pb-2.5 shrink-0",
                h2 { class: "text-lg font-bold tracking-tight text-foreground", "Chats" }

                button {
                    r#type: "button",
                    class: "inline-flex size-8 items-center justify-center rounded-full border border-border/50 text-muted-foreground hover:text-foreground hover:bg-muted/60 transition-colors cursor-pointer",
                    title: "New chat",
                    onclick: move |_| {},
                    svg {
                        class: "size-4",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M5 12h14" }
                        path { d: "M12 5v14" }
                    }
                }
            }

            // Search Bar
            div { class: "px-3.5 pb-3 shrink-0",
                div { class: "relative flex items-center",
                    svg {
                        class: "absolute left-3 size-4 text-muted-foreground/60 pointer-events-none",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        circle { cx: "11", cy: "11", r: "8" }
                        path { d: "m21 21-4.3-4.3" }
                    }
                    input {
                        r#type: "text",
                        class: "w-full rounded-full border border-border/50 bg-background/60 pl-9 pr-3.5 py-1.5 text-xs text-foreground placeholder:text-muted-foreground/60 focus:outline-none focus:ring-1 focus:ring-primary/40 transition-all",
                        placeholder: "Chats search..",
                        value: search_query(),
                        oninput: move |e| search_query.set(e.value()),
                    }
                }
            }

            // Scrollable Contacts List
            div { class: "flex-1 min-h-0 overflow-y-auto px-2 space-y-0.5",
                for contact in filtered_contacts {
                    ChatListItem {
                        key: "{contact.id}",
                        is_active: props.selected_id == contact.id,
                        contact: contact,
                        on_select: props.on_select,
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ChatListItemProps {
    contact: ChatContact,
    is_active: bool,
    on_select: EventHandler<String>,
}

#[component]
fn ChatListItem(props: ChatListItemProps) -> Element {
    let contact = props.contact;
    let id = contact.id.clone();
    let on_select = props.on_select;

    let active_class = if props.is_active {
        "bg-muted/70 text-foreground shadow-xs"
    } else {
        "hover:bg-muted/30 text-muted-foreground hover:text-foreground"
    };

    let status_tick = match contact.status {
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

    rsx! {
        div {
            class: "flex items-center gap-3 p-2.5 rounded-xl transition-colors cursor-pointer {active_class}",
            onclick: move |_| on_select.call(id.clone()),

            // Avatar with online status
            div { class: "relative shrink-0",
                if let Some(avatar_url) = &contact.avatar_url {
                    img {
                        class: "size-10 rounded-full object-cover border border-border/40",
                        src: "{avatar_url}",
                        alt: "{contact.name}",
                    }
                } else if contact.is_group {
                    div { class: "flex size-10 items-center justify-center rounded-full bg-muted border border-border/40 text-muted-foreground",
                        svg {
                            class: "size-5",
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
                    div { class: "flex size-10 items-center justify-center rounded-full bg-primary/10 border border-primary/20 text-xs font-semibold text-primary",
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

            // Middle: Name & last message snippet
            div { class: "flex-1 min-w-0 flex flex-col gap-0.5",
                div { class: "flex items-center justify-between gap-1",
                    span { class: "truncate text-xs font-semibold text-foreground",
                        "{contact.name}"
                    }
                    span { class: "text-[11px] text-muted-foreground/80 shrink-0 font-normal",
                        "{contact.last_message_time}"
                    }
                }

                div { class: "flex items-center justify-between gap-2",
                    div { class: "flex items-center gap-1 min-w-0 truncate text-xs text-muted-foreground/90",
                        {status_tick}

                        if let Some(sender) = &contact.last_message_sender {
                            span { class: "font-medium text-foreground/80 shrink-0", "{sender}:" }
                        }

                        span { class: "truncate text-xs text-muted-foreground",
                            "{contact.last_message}"
                        }
                    }

                    if contact.unread_count > 0 {
                        span { class: "flex size-4.5 min-w-4.5 items-center justify-center rounded-full bg-emerald-500 px-1 text-[10px] font-bold text-black shrink-0",
                            "{contact.unread_count}"
                        }
                    }
                }
            }
        }
    }
}
