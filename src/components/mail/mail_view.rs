use dioxus::prelude::*;

use super::mail_list::MailList;
use super::mail_nav::MailNav;
use super::mail_read::MailRead;
use super::types::{sample_folders, sample_mails, Mail};

/// Left nav width (w-52) + resize handle width (w-2).
const LIST_OFFSET: i64 = 208 + 8;

#[component]
pub fn MailView() -> Element {
    let mails = use_signal(sample_mails);
    let search = use_signal(String::new);
    let unread_only = use_signal(|| false);
    let mut active_folder = use_signal(|| "inbox".to_string());
    let mut active_label = use_signal(String::new);
    let mut selected_id = use_signal(|| Option::<String>::None);
    let mut list_width = use_signal(|| 440_usize);
    let mut dragging = use_signal(|| false);

    let query = search.read().to_lowercase();
    let unread = *unread_only.read();
    let filtered: Vec<Mail> = mails
        .read()
        .iter()
        .filter(|m| {
            (!unread || !m.read)
                && (query.is_empty()
                    || m.name.to_lowercase().contains(&query)
                    || m.subject.to_lowercase().contains(&query)
                    || m.preview.to_lowercase().contains(&query))
        })
        .cloned()
        .collect();

    let selected = selected_id()
        .and_then(|id| mails.read().iter().find(|m| m.id == id).cloned());

    let title = sample_folders()
        .iter()
        .find(|f| f.id == active_folder())
        .map(|f| f.name.to_string())
        .unwrap_or_else(|| "Inbox".to_string());

    rsx! {
        div {
            class: "z-0 flex h-full w-full min-h-0 overflow-hidden bg-background text-foreground select-none",

            onmousemove: move |e| {
                if dragging() {
                    let width = e.client_coordinates().x as i64 - LIST_OFFSET;
                    list_width.set(width.clamp(320, 760) as usize);
                }
            },
            onmouseup: move |_| dragging.set(false),

            // Left: folders & labels
            MailNav {
                active_folder: active_folder(),
                active_label: active_label(),
                on_folder: move |id: String| {
                    active_folder.set(id);
                    active_label.set(String::new());
                },
                on_label: move |id: String| {
                    active_label.set(if active_label() == id { String::new() } else { id });
                },
            }

            // Middle: mail list (resizable)
            div {
                class: "flex min-w-0 shrink-0 flex-col border-r border-border/40",
                style: format!("width: {}px", list_width()),

                MailList {
                    mails: filtered,
                    title: title,
                    selected_id: selected_id(),
                    on_select: move |id: String| selected_id.set(Some(id)),
                    search: search,
                    unread_only: unread_only,
                }
            }

            // Resize handle
            div {
                class: "group flex w-2 shrink-0 cursor-ew-resize select-none items-center justify-center",
                onmousedown: move |_| dragging.set(true),
                div { class: "h-12 w-1 rounded-full bg-border transition-colors group-hover:bg-muted-foreground/60" }
            }

            // Right: reading pane
            MailRead { mail: selected }
        }
    }
}
