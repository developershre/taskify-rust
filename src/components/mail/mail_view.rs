use dioxus::prelude::*;

use super::mail_list::MailList;
use super::mail_nav::MailNav;
use super::mail_read::MailRead;
use super::types::{sample_mails, Mail};

/// Left nav width (w-52) + resize handle width (w-2).
const LIST_OFFSET: i64 = 208 + 8;

#[component]
pub fn MailView() -> Element {
    let mut mails = use_signal(sample_mails);
    let search = use_signal(String::new);
    let unread_only = use_signal(|| false);
    let mut active_folder = use_signal(|| "inbox".to_string());
    let mut active_label = use_signal(String::new);
    let mut selected_id = use_signal(|| {
        sample_mails().first().map(|m| m.id.clone())
    });
    let mut list_width = use_signal(|| 420_usize);
    let mut dragging = use_signal(|| false);

    let query = search.read().to_lowercase();
    let unread = *unread_only.read();
    let current_folder = active_folder();
    let current_label = active_label();

    let filtered: Vec<Mail> = mails
        .read()
        .iter()
        .filter(|m| {
            // Label filter takes precedence if selected
            let matches_category = if !current_label.is_empty() {
                m.labels.iter().any(|l| l.name.eq_ignore_ascii_case(&current_label))
            } else if current_folder == "starred" {
                m.is_starred
            } else {
                m.folder == current_folder
            };

            let matches_unread = !unread || !m.read;
            let matches_search = query.is_empty()
                || m.name.to_lowercase().contains(&query)
                || m.subject.to_lowercase().contains(&query)
                || m.preview.to_lowercase().contains(&query);

            matches_category && matches_unread && matches_search
        })
        .cloned()
        .collect();

    let selected = selected_id()
        .and_then(|id| mails.read().iter().find(|m| m.id == id).cloned());

    let title = if !current_label.is_empty() {
        current_label.clone()
    } else {
        match current_folder.as_str() {
            "inbox" => "Inbox".to_string(),
            "starred" => "Starred".to_string(),
            "drafts" => "Drafts".to_string(),
            "sent" => "Sent".to_string(),
            "junk" => "Junk".to_string(),
            "trash" => "Trash".to_string(),
            "archive" => "Archive".to_string(),
            other => other.to_string(),
        }
    };

    let on_select_mail = move |id: String| {
        selected_id.set(Some(id.clone()));
        let mut list = mails.write();
        if let Some(m) = list.iter_mut().find(|m| m.id == id) {
            m.read = true;
        }
    };

    let on_toggle_star = move |id: String| {
        let mut list = mails.write();
        if let Some(m) = list.iter_mut().find(|m| m.id == id) {
            m.is_starred = !m.is_starred;
        }
    };

    let on_archive_mail = move |id: String| {
        let mut list = mails.write();
        if let Some(m) = list.iter_mut().find(|m| m.id == id) {
            m.folder = "archive".to_string();
        }
    };

    let on_move_to_inbox = move |id: String| {
        let mut list = mails.write();
        if let Some(m) = list.iter_mut().find(|m| m.id == id) {
            m.folder = "inbox".to_string();
        }
    };

    let on_delete_mail = move |id: String| {
        let mut list = mails.write();
        if let Some(m) = list.iter_mut().find(|m| m.id == id) {
            if m.folder == "trash" {
                list.retain(|item| item.id != id);
            } else {
                m.folder = "trash".to_string();
            }
        }
    };

    let on_snooze_mail = move |id: String| {
        let mut list = mails.write();
        if let Some(m) = list.iter_mut().find(|m| m.id == id) {
            m.date = "Snoozed".to_string();
        }
    };

    let on_mark_unread = move |id: String| {
        let mut list = mails.write();
        if let Some(m) = list.iter_mut().find(|m| m.id == id) {
            m.read = false;
        }
    };

    let on_reply_mail = move |(id, text): (String, String)| {
        let mut list = mails.write();
        if let Some(m) = list.iter_mut().find(|m| m.id == id) {
            m.body.push(format!("You replied: {}", text));
            m.date = "Just now".to_string();
        }
    };

    let on_compose_mail = move |new_mail: Mail| {
        let id = new_mail.id.clone();
        mails.write().insert(0, new_mail);
        active_folder.set("sent".to_string());
        active_label.set(String::new());
        selected_id.set(Some(id));
    };

    rsx! {
        div {
            class: "flex h-full w-full min-h-0 overflow-hidden bg-background text-foreground select-none",

            onmousemove: move |e| {
                if dragging() {
                    let width = e.client_coordinates().x as i64 - LIST_OFFSET;
                    list_width.set(width.clamp(280, 680) as usize);
                }
            },
            onmouseup: move |_| dragging.set(false),

            // Left: folders & labels
            MailNav {
                active_folder: active_folder(),
                active_label: active_label(),
                mails: mails(),
                on_folder: move |id: String| {
                    active_folder.set(id);
                    active_label.set(String::new());
                },
                on_label: move |id: String| {
                    active_label.set(if active_label() == id { String::new() } else { id });
                },
                on_compose: on_compose_mail,
            }

            // Middle: mail list (resizable)
            div {
                class: "flex min-w-0 shrink-0 flex-col border-r border-border/40",
                style: format!("width: {}px", list_width()),

                MailList {
                    mails: filtered,
                    title: title,
                    selected_id: selected_id(),
                    on_select: on_select_mail,
                    search: search,
                    unread_only: unread_only,
                    on_toggle_star: on_toggle_star,
                    on_delete: on_delete_mail,
                }
            }

            // Resize handle
            div {
                class: "group flex w-2 shrink-0 cursor-ew-resize select-none items-center justify-center",
                onmousedown: move |_| dragging.set(true),
                div { class: "h-12 w-1 rounded-full bg-border transition-colors group-hover:bg-muted-foreground/60" }
            }

            // Right: reading pane
            MailRead {
                mail: selected,
                on_archive: on_archive_mail,
                on_move_to_inbox: on_move_to_inbox,
                on_delete: on_delete_mail,
                on_snooze: on_snooze_mail,
                on_toggle_star: on_toggle_star,
                on_mark_unread: on_mark_unread,
                on_reply: on_reply_mail,
            }
        }
    }
}
