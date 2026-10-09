use dioxus::prelude::*;
use std::collections::HashMap;

use super::chat_header::ChatHeader;
use super::chat_input::ChatInput;
use super::chat_list::ChatList;
use super::message_bubble::MessageBubble;
use super::types::{
    sample_contacts, sample_messages_for, ChatMessage, MessageContent, MessageStatus,
};

#[derive(Props, Clone, PartialEq)]
pub struct ChatViewProps {
    #[props(default)]
    pub class: String,
}

#[component]
pub fn ChatView(props: ChatViewProps) -> Element {
    let mut contacts = use_signal(sample_contacts);
    let mut selected_id = use_signal(String::new);
    let mut mobile_show_chat = use_signal(|| false);

    let mut messages_by_contact = use_signal(|| {
        let mut map: HashMap<String, Vec<ChatMessage>> = HashMap::new();
        for c in sample_contacts() {
            map.insert(c.id.clone(), sample_messages_for(&c.id));
        }
        map
    });

    let current_id = selected_id();
    let current_contact = contacts.read().iter().find(|c| c.id == current_id).cloned();

    let current_messages = messages_by_contact
        .read()
        .get(&current_id)
        .cloned()
        .unwrap_or_default();

    let on_select_contact = move |id: String| {
        selected_id.set(id.clone());
        mobile_show_chat.set(true);

        // Mark as read
        let mut list = contacts.write();
        if let Some(c) = list.iter_mut().find(|c| c.id == id) {
            c.unread_count = 0;
            c.status = MessageStatus::Read;
        }
    };

    let on_create_chat = move |name: String| {
        let id = format!("chat_{}", js_sys_time());
        let initials: String = name
            .split_whitespace()
            .filter_map(|w| w.chars().next())
            .take(2)
            .collect();
        let initials = if initials.is_empty() {
            name.chars().take(2).collect()
        } else {
            initials
        };

        let new_contact = super::types::ChatContact {
            id: id.clone(),
            name: name.clone(),
            avatar_url: None,
            initials: initials.to_uppercase(),
            is_online: true,
            is_group: name.to_lowercase().contains("team") || name.to_lowercase().contains("group"),
            is_pinned: false,
            is_muted: false,
            last_message: "Chat started".to_string(),
            last_message_time: "Just now".to_string(),
            last_message_sender: None,
            status: MessageStatus::Read,
            unread_count: 0,
        };

        contacts.write().insert(0, new_contact);
        messages_by_contact.write().insert(id.clone(), vec![
            ChatMessage {
                id: format!("msg-{}", js_sys_time()),
                sender_id: "system".to_string(),
                sender_name: "System".to_string(),
                time: "Just now".to_string(),
                is_outgoing: false,
                status: MessageStatus::Read,
                content: MessageContent::Text(format!("You started a new conversation with {}.", name)),
                reactions: vec![],
            }
        ]);
        selected_id.set(id);
        mobile_show_chat.set(true);
    };

    let on_toggle_pin = move |id: String| {
        let mut list = contacts.write();
        if let Some(c) = list.iter_mut().find(|c| c.id == id) {
            c.is_pinned = !c.is_pinned;
        }
    };

    let on_toggle_mute = move |id: String| {
        let mut list = contacts.write();
        if let Some(c) = list.iter_mut().find(|c| c.id == id) {
            c.is_muted = !c.is_muted;
        }
    };

    let on_delete_chat = move |id: String| {
        contacts.write().retain(|c| c.id != id);
        messages_by_contact.write().remove(&id);
        if selected_id() == id {
            selected_id.set(String::new());
            mobile_show_chat.set(false);
        }
    };

    let on_clear_chat = move |_| {
        let cid = selected_id();
        if !cid.is_empty() {
            messages_by_contact.write().insert(cid, vec![]);
        }
    };

    let on_send_message = move |text: String| {
        let cid = selected_id();
        let new_msg = ChatMessage {
            id: format!("msg-{}", js_sys_time()),
            sender_id: "me".to_string(),
            sender_name: "You".to_string(),
            time: "Just now".to_string(),
            is_outgoing: true,
            status: MessageStatus::Read,
            content: MessageContent::Text(text.clone()),
            reactions: vec![],
        };

        // Append to messages
        messages_by_contact
            .write()
            .entry(cid.clone())
            .or_default()
            .push(new_msg);

        // Update contact last message
        let mut list = contacts.write();
        if let Some(c) = list.iter_mut().find(|c| c.id == cid) {
            c.last_message = text;
            c.last_message_time = "Just now".to_string();
            c.status = MessageStatus::Read;
        }
    };

    let on_send_doc = move |(name, size): (String, String)| {
        let cid = selected_id();
        let new_msg = ChatMessage {
            id: format!("msg-{}", js_sys_time()),
            sender_id: "me".to_string(),
            sender_name: "You".to_string(),
            time: "Just now".to_string(),
            is_outgoing: true,
            status: MessageStatus::Read,
            content: MessageContent::Document {
                name: name.clone(),
                size,
                url: "#".to_string(),
            },
            reactions: vec![],
        };

        messages_by_contact
            .write()
            .entry(cid.clone())
            .or_default()
            .push(new_msg);

        let mut list = contacts.write();
        if let Some(c) = list.iter_mut().find(|c| c.id == cid) {
            c.last_message = format!("📄 {}", name);
            c.last_message_time = "Just now".to_string();
            c.status = MessageStatus::Read;
        }
    };

    let on_send_audio = move |duration: String| {
        let cid = selected_id();
        let new_msg = ChatMessage {
            id: format!("msg-{}", js_sys_time()),
            sender_id: "me".to_string(),
            sender_name: "You".to_string(),
            time: "Just now".to_string(),
            is_outgoing: true,
            status: MessageStatus::Read,
            content: MessageContent::Audio {
                duration,
                url: "#".to_string(),
            },
            reactions: vec![],
        };

        messages_by_contact
            .write()
            .entry(cid.clone())
            .or_default()
            .push(new_msg);

        let mut list = contacts.write();
        if let Some(c) = list.iter_mut().find(|c| c.id == cid) {
            c.last_message = "🎙️ Voice message".to_string();
            c.last_message_time = "Just now".to_string();
            c.status = MessageStatus::Read;
        }
    };

    let on_send_image = move |(url, title): (String, String)| {
        let cid = selected_id();
        let new_msg = ChatMessage {
            id: format!("msg-{}", js_sys_time()),
            sender_id: "me".to_string(),
            sender_name: "You".to_string(),
            time: "Just now".to_string(),
            is_outgoing: true,
            status: MessageStatus::Read,
            content: MessageContent::MediaGrid {
                images: vec![super::types::MediaItem { url, title }],
                extra_count: 0,
            },
            reactions: vec![],
        };

        messages_by_contact
            .write()
            .entry(cid.clone())
            .or_default()
            .push(new_msg);

        let mut list = contacts.write();
        if let Some(c) = list.iter_mut().find(|c| c.id == cid) {
            c.last_message = "📷 Photo".to_string();
            c.last_message_time = "Just now".to_string();
            c.status = MessageStatus::Read;
        }
    };

    let list_class = if *mobile_show_chat.read() {
        "hidden sm:flex"
    } else {
        "flex"
    };

    let conversation_class = if *mobile_show_chat.read() {
        "flex"
    } else {
        "hidden sm:flex"
    };

    rsx! {
        div {
            class: format!(
                "flex h-full w-full min-h-0 overflow-hidden select-none {}",
                props.class
            ),

            // Left Panel: Chats List
            ChatList {
                class: list_class.to_string(),
                contacts: contacts(),
                selected_id: selected_id(),
                on_select: on_select_contact,
                on_create_chat: on_create_chat,
                on_toggle_pin: on_toggle_pin,
                on_toggle_mute: on_toggle_mute,
                on_delete_chat: on_delete_chat,
            }

            // Right Panel: Active Conversation (only when a chat is selected)
            div {
                class: format!(
                    "flex-1 min-w-0 flex flex-col h-full overflow-hidden {}",
                    conversation_class
                ),

                if current_contact.is_some() {
                    // Conversation Header
                    ChatHeader {
                        contact: current_contact,
                        on_back: move |_| mobile_show_chat.set(false),
                        on_clear_chat: on_clear_chat,
                    }

                    // Messages Timeline Stream
                    div { class: "flex-1 min-h-0 overflow-y-auto flex flex-col gap-4",

                        // Date / Time Divider Badge
                        div { class: "flex items-center justify-center my-2",
                            span { class: "rounded-full bg-muted/60 px-3 py-1 text-[11px] font-medium text-muted-foreground select-none",
                                "Today"
                            }
                        }

                        for msg in current_messages {
                            MessageBubble {
                                key: "{msg.id}",
                                message: msg,
                            }
                        }
                    }

                    // Bottom Input Field
                    ChatInput {
                        on_send: on_send_message,
                        on_send_doc: on_send_doc,
                        on_send_audio: on_send_audio,
                        on_send_image: on_send_image,
                    }
                } else {
                    NoChatSelected {
                        on_select_first: move |_| {
                            let first = contacts.read().first().map(|c| c.id.clone());
                            if let Some(id) = first {
                                selected_id.set(id.clone());
                                mobile_show_chat.set(true);
                                let mut list = contacts.write();
                                if let Some(c) = list.iter_mut().find(|c| c.id == id) {
                                    c.unread_count = 0;
                                    c.status = MessageStatus::Read;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct NoChatSelectedProps {
    on_select_first: EventHandler<()>,
}

#[component]
fn NoChatSelected(props: NoChatSelectedProps) -> Element {
    let on_first = props.on_select_first;
    rsx! {
        div { class: "flex-1 min-h-0 flex flex-col items-center justify-center gap-5 p-8 text-center",

            div { class: "flex size-14 items-center justify-center rounded-2xl bg-primary/10 border border-primary/20 text-primary shadow-xs",
                svg {
                    class: "size-7",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "1.75",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
                    path { d: "M13 10h6" }
                    path { d: "m16 7 3 3-3 3" }
                }
            }

            div { class: "flex flex-col gap-1.5",
                h3 { class: "text-lg font-bold text-foreground tracking-tight",
                    "Select a Conversation"
                }
                p { class: "max-w-xs sm:max-w-sm text-xs leading-relaxed text-muted-foreground",
                    "Choose a chat from the sidebar to view message history, collaborate in real time, or start a new conversation."
                }
            }

            button {
                r#type: "button",
                class: "rounded-xl bg-primary px-5 py-2.5 text-xs font-semibold text-primary-foreground hover:bg-primary/90 transition-all active:scale-98 cursor-pointer shadow-sm",
                onclick: move |_| on_first.call(()),
                "Open First Conversation"
            }
        }
    }
}

fn js_sys_time() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::now() as u64
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0)
    }
}
