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
    let mut selected_id = use_signal(|| "jacquenetta".to_string());
    let mut mobile_show_chat = use_signal(|| true);

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

    let on_send_message = move |text: String| {
        let new_msg = ChatMessage {
            id: format!("msg-{}", js_sys_time()),
            sender_id: "me".to_string(),
            sender_name: "You".to_string(),
            time: "Just now".to_string(),
            is_outgoing: true,
            status: MessageStatus::Read,
            content: MessageContent::Text(text.clone()),
        };

        // Append to messages
        messages_by_contact
            .write()
            .entry(current_id.clone())
            .or_default()
            .push(new_msg);

        // Update contact last message
        let mut list = contacts.write();
        if let Some(c) = list.iter_mut().find(|c| c.id == current_id) {
            c.last_message = text;
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
                "flex h-full w-full min-h-0 overflow-hidden rounded-2xl border border-border/40 bg-card shadow-xs select-none {}",
                props.class
            ),

            // Left Panel: Chats List
            ChatList {
                class: list_class.to_string(),
                contacts: contacts(),
                selected_id: selected_id(),
                on_select: on_select_contact,
            }

            // Right Panel: Active Conversation
            div {
                class: format!(
                    "flex-1 min-w-0 flex flex-col h-full bg-background/50 overflow-hidden {}",
                    conversation_class
                ),

                // Conversation Header
                ChatHeader {
                    contact: current_contact,
                    on_back: move |_| mobile_show_chat.set(false),
                }

                // Messages Timeline Stream
                div { class: "flex-1 min-h-0 overflow-y-auto p-4 sm:p-5 flex flex-col gap-4",

                    // Date / Time Divider Badge
                    div { class: "flex items-center justify-center my-2",
                        span { class: "rounded-full bg-muted/60 px-3 py-1 text-[11px] font-medium text-muted-foreground select-none",
                            "05:23 PM"
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
                }
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
