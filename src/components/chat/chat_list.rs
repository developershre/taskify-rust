use super::types::{ChatContact, MessageStatus};
use crate::components::ui::dropdown_menu::DropdownMenuContext;
use crate::components::ui::{
    Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator, DropdownMenuTrigger,
};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ChatListProps {
    pub contacts: Vec<ChatContact>,
    pub selected_id: String,
    pub on_select: EventHandler<String>,
    #[props(default)]
    pub on_create_chat: Option<EventHandler<String>>,
    #[props(default)]
    pub on_toggle_pin: Option<EventHandler<String>>,
    #[props(default)]
    pub on_toggle_mute: Option<EventHandler<String>>,
    #[props(default)]
    pub on_delete_chat: Option<EventHandler<String>>,
    #[props(default)]
    pub class: String,
}

use crate::icons::{PlusIcon, SearchIcon, TrashIcon};

#[component]
pub fn ChatList(props: ChatListProps) -> Element {
    let mut search_query = use_signal(String::new);
    let mut new_chat_open = use_signal(|| false);
    let mut new_contact_name = use_signal(String::new);
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

    let pinned_contacts: Vec<ChatContact> = filtered_contacts
        .iter()
        .filter(|c| c.is_pinned)
        .cloned()
        .collect();

    let other_contacts: Vec<ChatContact> = filtered_contacts
        .iter()
        .filter(|c| !c.is_pinned)
        .cloned()
        .collect();

    rsx! {
        div {
            class: format!(
                "flex flex-col h-full w-full sm:w-56 md:w-80 shrink-0 border-r border-border/40 bg-card/40 select-none overflow-hidden {}",
                props.class
            ),

            // Top Header: "Chats" + "+" button
            div { class: "flex items-center justify-between p-3.5 pb-2.5 shrink-0",
                div { class: "flex items-center gap-2",
                    h2 { class: "text-lg font-bold tracking-tight text-foreground", "Chats" }
                    span { class: "rounded-full bg-muted/80 px-2 py-0.5 text-[11px] font-semibold text-muted-foreground",
                        "{props.contacts.len()}"
                    }
                }

                DropdownMenu {
                    DropdownMenuTrigger {
                        button {
                            r#type: "button",
                            class: "inline-flex size-8 items-center justify-center rounded-lg border border-border/50 text-muted-foreground hover:text-foreground hover:bg-muted/60 transition-colors cursor-pointer",
                            title: "New chat",
                            PlusIcon { class: "size-4" }
                        }
                    }
                    DropdownMenuContent {
                        align: "end",
                        side: "bottom",
                        side_offset: 6,
                        DropdownMenuItem {
                            onclick: move |_| new_chat_open.set(true),
                            "Start new conversation"
                        }
                        DropdownMenuItem {
                            onclick: move |_| new_chat_open.set(true),
                            "Create group chat"
                        }
                    }
                }
            }

            // Search Bar
            div { class: "px-3.5 pb-2.5 shrink-0",
                div { class: "relative flex items-center",
                    SearchIcon { class: "absolute left-3 size-3.5 text-muted-foreground/60 pointer-events-none" }
                    input {
                        r#type: "text",
                        class: "w-full rounded-xl border border-border/50 bg-background/60 pl-8.5 pr-3 py-1.5 text-xs text-foreground placeholder:text-muted-foreground/60 focus:outline-none focus:ring-1 focus:ring-primary/40 transition-all",
                        placeholder: "Search messages or contacts...",
                        value: search_query(),
                        oninput: move |e| search_query.set(e.value()),
                    }
                }
            }

            // Scrollable Contacts List
            div { class: "flex-1 min-h-0 overflow-y-auto px-2 space-y-1 pb-3",

                // Pinned section
                if !pinned_contacts.is_empty() {
                    div { class: "px-2 pt-1 pb-0.5 flex items-center gap-1.5 text-[10px] font-bold uppercase tracking-wider text-muted-foreground/60",
                        svg { class: "size-2.5 rotate-45", view_box: "0 0 24 24", fill: "currentColor",
                            path { d: "M16 3a1 1 0 0 1 1 1v6l2 2v2h-6v7l-1 1-1-1v-7H5v-2l2-2V4a1 1 0 0 1 1-1h8z" }
                        }
                        span { "Pinned Chats" }
                    }
                    for contact in pinned_contacts.clone() {
                        ChatListItem {
                            key: "{contact.id}",
                            is_active: props.selected_id == contact.id,
                            contact: contact,
                            on_select: props.on_select,
                            on_toggle_pin: props.on_toggle_pin,
                            on_toggle_mute: props.on_toggle_mute,
                            on_delete_chat: props.on_delete_chat,
                        }
                    }
                    div { class: "my-1.5 border-t border-border/30" }
                }

                // All messages section
                if !other_contacts.is_empty() {
                    if !pinned_contacts.is_empty() {
                        div { class: "px-2 pt-1 pb-0.5 text-[10px] font-bold uppercase tracking-wider text-muted-foreground/60",
                            "All Conversations"
                        }
                    }
                    for contact in other_contacts {
                        ChatListItem {
                            key: "{contact.id}",
                            is_active: props.selected_id == contact.id,
                            contact: contact,
                            on_select: props.on_select,
                            on_toggle_pin: props.on_toggle_pin,
                            on_toggle_mute: props.on_toggle_mute,
                            on_delete_chat: props.on_delete_chat,
                        }
                    }
                } else if filtered_contacts.is_empty() {
                    div { class: "py-8 text-center text-xs text-muted-foreground",
                        "No conversations found"
                    }
                }
            }

            // Dialog for New Chat
            Dialog {
                open: new_chat_open,
                DialogContent {
                    DialogHeader {
                        DialogTitle { "Start a New Chat" }
                        DialogDescription { "Enter the contact's name or team group to start messaging." }
                    }

                    div { class: "py-3 space-y-3",
                        label { class: "text-xs font-semibold text-foreground block", "Contact or Group Name" }
                        input {
                            r#type: "text",
                            class: "w-full rounded-xl border border-border bg-background px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-primary",
                            placeholder: "e.g. Alex Morgan or Frontend Team",
                            value: new_contact_name(),
                            oninput: move |e| new_contact_name.set(e.value()),
                            onkeydown: move |e| {
                                if e.key() == Key::Enter {
                                    let name = new_contact_name.read().trim().to_string();
                                    if !name.is_empty() {
                                        if let Some(handler) = &props.on_create_chat {
                                            handler.call(name);
                                        }
                                        new_contact_name.set(String::new());
                                        new_chat_open.set(false);
                                    }
                                }
                            }
                        }
                    }

                    DialogFooter {
                        button {
                            r#type: "button",
                            class: "h-8 rounded-lg border border-border px-3 text-xs font-medium text-foreground hover:bg-muted cursor-pointer",
                            onclick: move |_| new_chat_open.set(false),
                            "Cancel"
                        }
                        button {
                            r#type: "button",
                            class: "h-8 rounded-lg bg-primary px-3.5 text-xs font-medium text-primary-foreground hover:opacity-90 cursor-pointer",
                            onclick: move |_| {
                                let name = new_contact_name.read().trim().to_string();
                                if !name.is_empty() {
                                    if let Some(handler) = &props.on_create_chat {
                                        handler.call(name);
                                    }
                                    new_contact_name.set(String::new());
                                    new_chat_open.set(false);
                                }
                            },
                            "Create Chat"
                        }
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
    #[props(default)]
    on_toggle_pin: Option<EventHandler<String>>,
    #[props(default)]
    on_toggle_mute: Option<EventHandler<String>>,
    #[props(default)]
    on_delete_chat: Option<EventHandler<String>>,
}

#[component]
fn ChatListItem(props: ChatListItemProps) -> Element {
    let contact = props.contact;
    let id = contact.id.clone();
    let on_select = props.on_select;

    let active_class = if props.is_active {
        "bg-primary/10 text-foreground border border-primary/25 shadow-xs"
    } else {
        "hover:bg-muted/50 text-muted-foreground hover:text-foreground border border-transparent"
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
            class: "relative group/item flex items-center gap-2.5 p-2 rounded-xl transition-all cursor-pointer {active_class}",
            onclick: move |_| on_select.call(id.clone()),

            // Avatar with online status
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

            // Middle: Name & last message snippet
            div { class: "flex-1 min-w-0 flex flex-col gap-0.5",
                div { class: "flex items-center justify-between gap-1",
                    div { class: "flex items-center gap-1.5 min-w-0 truncate",
                        span { class: "truncate text-xs font-semibold text-foreground",
                            "{contact.name}"
                        }
                        if contact.is_muted {
                            svg { class: "size-3 text-muted-foreground/60 shrink-0", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                                path { d: "M18.36 6.64a9 9 0 0 1 0 12.72" }
                                line { x1: "2", x2: "22", y1: "2", y2: "22" }
                            }
                        }
                    }
                    span { class: "text-[10px] text-muted-foreground/70 shrink-0 font-normal",
                        "{contact.last_message_time}"
                    }
                }

                div { class: "relative flex items-center justify-between gap-2",
                    div { class: "flex items-center gap-1 min-w-0 truncate text-xs text-muted-foreground/90",
                        {status_tick}

                        if let Some(sender) = &contact.last_message_sender {
                            span { class: "font-medium text-foreground/80 shrink-0 text-[11px]", "{sender}:" }
                        }

                        span { class: "truncate text-[11px] text-muted-foreground",
                            "{contact.last_message}"
                        }
                    }

                    div { class: "flex items-center gap-1 shrink-0",
                        if contact.is_pinned {
                            svg { class: "size-3 text-primary/70 shrink-0 rotate-45", view_box: "0 0 24 24", fill: "currentColor",
                                path { d: "M16 3a1 1 0 0 1 1 1v6l2 2v2h-6v7l-1 1-1-1v-7H5v-2l2-2V4a1 1 0 0 1 1-1h8z" }
                            }
                        }

                        if contact.unread_count > 0 {
                            span { class: "flex size-4 min-w-4 items-center justify-center rounded-full bg-emerald-500 px-1 text-[10px] font-bold text-black shrink-0",
                                "{contact.unread_count}"
                            }
                        }
                    }
                }
            }

            // Context Menu Actions
            ChatRowMenu {
                contact_id: contact.id.clone(),
                is_pinned: contact.is_pinned,
                is_muted: contact.is_muted,
                on_toggle_pin: props.on_toggle_pin,
                on_toggle_mute: props.on_toggle_mute,
                on_delete_chat: props.on_delete_chat,
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ChatRowMenuProps {
    contact_id: String,
    is_pinned: bool,
    is_muted: bool,
    #[props(default)]
    on_toggle_pin: Option<EventHandler<String>>,
    #[props(default)]
    on_toggle_mute: Option<EventHandler<String>>,
    #[props(default)]
    on_delete_chat: Option<EventHandler<String>>,
}

#[component]
fn ChatRowMenu(props: ChatRowMenuProps) -> Element {
    let cid_pin = props.contact_id.clone();
    let cid_mute = props.contact_id.clone();
    let cid_del = props.contact_id.clone();
    let on_pin = props.on_toggle_pin;
    let on_mute = props.on_toggle_mute;
    let on_del = props.on_delete_chat;

    rsx! {
        div {
            class: "absolute top-2.5 right-2 z-10",
            DropdownMenu {
                ChatMenuButton {}

                DropdownMenuContent {
                    align: "end",
                    side_offset: 6,

                    DropdownMenuItem {
                        onclick: move |_| {
                            if let Some(h) = &on_pin {
                                h.call(cid_pin.clone());
                            }
                        },
                        if props.is_pinned { "Unpin chat" } else { "Pin to top" }
                    }
                    DropdownMenuItem {
                        onclick: move |_| {
                            if let Some(h) = &on_mute {
                                h.call(cid_mute.clone());
                            }
                        },
                        if props.is_muted { "Unmute notifications" } else { "Mute notifications" }
                    }

                    DropdownMenuSeparator {}

                    DropdownMenuItem {
                        variant: "destructive",
                        onclick: move |_| {
                            if let Some(h) = &on_del {
                                h.call(cid_del.clone());
                            }
                        },
                        TrashIcon { class: "size-4" }
                        span { "Delete chat" }
                    }
                }
            }
        }
    }
}

#[component]
fn ChatMenuButton() -> Element {
    let mut menu = use_context::<DropdownMenuContext>();
    let is_open = (menu.open)();

    let reveal_class = if is_open {
        "opacity-100 pointer-events-auto"
    } else {
        "opacity-0 pointer-events-none group-hover/item:opacity-100 group-hover/item:pointer-events-auto"
    };

    rsx! {
        div { class: "shrink-0 transition-opacity {reveal_class}",
            button {
                r#type: "button",
                "data-slot": "dropdown-menu-trigger",
                class: "inline-flex size-5 items-center justify-center rounded-full border border-border/50 bg-popover text-muted-foreground/80 shadow-xs hover:bg-muted hover:text-foreground transition-colors cursor-pointer outline-none",
                title: "Chat options",

                onmousedown: move |e| {
                    e.stop_propagation();
                },

                onclick: move |e| {
                    e.stop_propagation();
                    let current = (menu.open)();
                    menu.open.set(!current);
                },

                svg {
                    class: "size-3",
                    view_box: "0 0 24 24",
                    fill: "currentColor",
                    circle { cx: "5", cy: "12", r: "1.75" }
                    circle { cx: "12", cy: "12", r: "1.75" }
                    circle { cx: "19", cy: "12", r: "1.75" }
                }
            }
        }
    }
}
