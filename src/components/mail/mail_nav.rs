use dioxus::prelude::*;

use super::types::{sample_label_folders, Mail, MailFolder, MailLabelFolder};
use crate::components::ui::{
    Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator, DropdownMenuTrigger,
};

#[derive(Props, Clone, PartialEq)]
pub struct MailNavProps {
    pub active_folder: String,
    pub active_label: String,
    pub mails: Vec<Mail>,
    pub on_folder: EventHandler<String>,
    pub on_label: EventHandler<String>,
    #[props(default)]
    pub on_compose: Option<EventHandler<Mail>>,
}

#[component]
pub fn MailNav(props: MailNavProps) -> Element {
    let router = router();
    let label_folders = sample_label_folders();
    let on_folder = props.on_folder;
    let on_label = props.on_label;
    let active_folder = props.active_folder.clone();
    let active_label = props.active_label.clone();
    let mails = props.mails.clone();

    let mut compose_open = use_signal(|| false);
    let mut to_field = use_signal(String::new);
    let mut subject_field = use_signal(String::new);
    let mut body_field = use_signal(String::new);

    // Calculate dynamic counts
    let inbox_count = mails.iter().filter(|m| m.folder == "inbox" && !m.read).count();
    let drafts_count = mails.iter().filter(|m| m.folder == "drafts").count();
    let sent_count = mails.iter().filter(|m| m.folder == "sent").count();
    let junk_count = mails.iter().filter(|m| m.folder == "junk").count();
    let trash_count = mails.iter().filter(|m| m.folder == "trash").count();
    let archive_count = mails.iter().filter(|m| m.folder == "archive").count();
    let starred_count = mails.iter().filter(|m| m.is_starred).count();

    let folders = vec![
        MailFolder {
            id: "inbox",
            name: "Inbox",
            count: if inbox_count > 0 { Some(inbox_count) } else { None },
        },
        MailFolder {
            id: "starred",
            name: "Starred",
            count: if starred_count > 0 { Some(starred_count) } else { None },
        },
        MailFolder {
            id: "drafts",
            name: "Drafts",
            count: if drafts_count > 0 { Some(drafts_count) } else { None },
        },
        MailFolder {
            id: "sent",
            name: "Sent",
            count: if sent_count > 0 { Some(sent_count) } else { None },
        },
        MailFolder {
            id: "junk",
            name: "Junk",
            count: if junk_count > 0 { Some(junk_count) } else { None },
        },
        MailFolder {
            id: "trash",
            name: "Trash",
            count: if trash_count > 0 { Some(trash_count) } else { None },
        },
        MailFolder {
            id: "archive",
            name: "Archive",
            count: if archive_count > 0 { Some(archive_count) } else { None },
        },
    ];

    let folder_class = |active: bool| {
        if active {
            "flex w-full items-center gap-2.5 rounded-lg bg-primary px-3 py-2 text-sm font-medium text-primary-foreground shadow-xs cursor-pointer"
        } else {
            "flex w-full items-center gap-2.5 rounded-lg px-3 py-2 text-sm text-foreground hover:bg-muted/60 transition-colors cursor-pointer"
        }
    };

    let count_class = |active: bool| {
        if active {
            "ml-auto rounded-md bg-primary-foreground/15 px-1.5 py-0.5 text-xs font-medium text-primary-foreground"
        } else {
            "ml-auto rounded-md bg-muted px-1.5 py-0.5 text-xs font-medium text-muted-foreground"
        }
    };

    rsx! {
        aside { class: "w-52 shrink-0 border-r border-border/40 flex flex-col gap-3 p-3 overflow-y-auto select-none",

            // Account Dropdown
            DropdownMenu {
                DropdownMenuTrigger {
                    class: "w-full justify-between rounded-lg px-2 py-2 text-sm font-semibold text-foreground hover:bg-muted/60 cursor-pointer",
                    div { class: "flex min-w-0 items-center gap-2",
                        // Logo
                        svg {
                            class: "size-4.5 shrink-0 text-foreground",
                            view_box: "0 0 24 24",
                            fill: "currentColor",
                            fill_rule: "evenodd",
                            clip_rule: "evenodd",
                            path { d: "M12 2.5 22.5 20.5H1.5L12 2.5Zm0 5.4L6.4 17.8h11.2L12 7.9Z" }
                        }
                        span { class: "truncate", "Alicia Koch" }
                    }
                    svg {
                        class: "size-4 shrink-0 text-muted-foreground",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "m6 9 6 6 6-6" }
                    }
                }

                DropdownMenuContent {
                    align: "start",
                    side_offset: 6,
                    DropdownMenuItem { "Account" }
                    DropdownMenuItem {
                        onclick: move |_| {
                            let _ = router.push("/settings");
                        },
                        "Settings"
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuItem { "Log out" }
                }
            }

            // Compose Email Button
            button {
                r#type: "button",
                class: "flex w-full items-center justify-center gap-2 rounded-xl bg-primary px-3 py-2 text-xs font-semibold text-primary-foreground shadow-xs hover:bg-primary/90 active:scale-98 transition-all cursor-pointer",
                onclick: move |_| compose_open.set(true),
                svg {
                    class: "size-3.5",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M12 5v14" }
                    path { d: "M5 12h14" }
                }
                span { "Compose" }
            }

            // Folders
            nav { class: "flex flex-col gap-0.5",
                for folder in folders.iter() {
                    { folder_row(
                        folder,
                        count_class(active_folder == folder.id && active_label.is_empty()),
                        folder_class(active_folder == folder.id && active_label.is_empty()),
                        on_folder,
                    ) }
                }
            }

            div { class: "h-px bg-border/60" }

            // Labels
            nav { class: "flex flex-col gap-0.5",
                div { class: "px-2 pb-1 text-[10px] font-bold uppercase tracking-wider text-muted-foreground/60",
                    "Categories & Labels"
                }
                for label in label_folders.iter() {
                    { label_row(
                        label,
                        active_label == label.name,
                        count_class(false),
                        on_label,
                    ) }
                }
            }

            // Compose Mail Dialog
            Dialog {
                open: compose_open,
                DialogContent {
                    DialogHeader {
                        DialogTitle { "New Message" }
                        DialogDescription { "Draft and send an email to your team or contact." }
                    }

                    div { class: "py-3 space-y-3",
                        div { class: "space-y-1",
                            label { class: "text-xs font-semibold text-foreground", "To" }
                            input {
                                r#type: "email",
                                class: "w-full rounded-xl border border-border bg-background px-3 py-1.5 text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-primary",
                                placeholder: "recipient@example.com",
                                value: to_field(),
                                oninput: move |e| to_field.set(e.value()),
                            }
                        }

                        div { class: "space-y-1",
                            label { class: "text-xs font-semibold text-foreground", "Subject" }
                            input {
                                r#type: "text",
                                class: "w-full rounded-xl border border-border bg-background px-3 py-1.5 text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-primary",
                                placeholder: "Subject of your message...",
                                value: subject_field(),
                                oninput: move |e| subject_field.set(e.value()),
                            }
                        }

                        div { class: "space-y-1",
                            label { class: "text-xs font-semibold text-foreground", "Message" }
                            textarea {
                                class: "w-full rounded-xl border border-border bg-background p-3 text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-primary resize-none",
                                rows: "5",
                                placeholder: "Write your message here...",
                                value: body_field(),
                                oninput: move |e| body_field.set(e.value()),
                            }
                        }
                    }

                    DialogFooter {
                        button {
                            r#type: "button",
                            class: "h-8 rounded-lg border border-border px-3 text-xs font-medium text-foreground hover:bg-muted cursor-pointer",
                            onclick: move |_| compose_open.set(false),
                            "Discard"
                        }
                        button {
                            r#type: "button",
                            class: "h-8 rounded-lg bg-primary px-4 text-xs font-medium text-primary-foreground hover:opacity-90 cursor-pointer shadow-xs",
                            onclick: move |_| {
                                let to = to_field.read().trim().to_string();
                                let subj = subject_field.read().trim().to_string();
                                let body = body_field.read().trim().to_string();
                                if !to.is_empty() && !subj.is_empty() {
                                    let new_mail = Mail {
                                        id: format!("mail-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0)),
                                        name: to.split('@').next().unwrap_or(&to).to_string(),
                                        email: to.clone(),
                                        subject: subj.clone(),
                                        preview: body.chars().take(80).collect::<String>(),
                                        body: vec![body],
                                        date: "Just now".to_string(),
                                        read: true,
                                        online: true,
                                        is_starred: false,
                                        folder: "sent".to_string(),
                                        labels: vec![
                                            super::types::MailLabel {
                                                name: "work",
                                                dark: true,
                                            }
                                        ],
                                    };
                                    if let Some(h) = &props.on_compose {
                                        h.call(new_mail);
                                    }
                                    to_field.set(String::new());
                                    subject_field.set(String::new());
                                    body_field.set(String::new());
                                    compose_open.set(false);
                                }
                            },
                            "Send Email"
                        }
                    }
                }
            }
        }
    }
}

fn label_row(
    label: &MailLabelFolder,
    active: bool,
    count_class: &'static str,
    on_label: EventHandler<String>,
) -> Element {
    let name = label.name;
    let row_class = if active {
        "flex w-full items-center gap-2.5 rounded-lg bg-muted px-3 py-2 text-sm font-medium text-foreground cursor-pointer"
    } else {
        "flex w-full items-center gap-2.5 rounded-lg px-3 py-2 text-sm text-foreground hover:bg-muted/60 transition-colors cursor-pointer"
    };

    rsx! {
        button {
            key: "{name}",
            r#type: "button",
            class: row_class,
            onclick: move |_| on_label.call(name.to_string()),

            span {
                class: "size-2.5 shrink-0 rounded-full",
                style: format!("background-color: {}", label.color),
            }
            span { class: "truncate", "{label.name}" }
            span { class: count_class, "{label.count}" }
        }
    }
}

fn folder_row(
    folder: &MailFolder,
    count_class: &'static str,
    row_class: &'static str,
    on_folder: EventHandler<String>,
) -> Element {
    let id = folder.id;
    rsx! {
        button {
            key: "{id}",
            r#type: "button",
            class: row_class,
            onclick: move |_| on_folder.call(id.to_string()),

            {folder_icon(id)}
            span { class: "truncate", "{folder.name}" }
            if let Some(count) = folder.count {
                span { class: count_class, "{count}" }
            }
        }
    }
}

fn folder_icon(id: &str) -> Element {
    let svg_class = "size-4 shrink-0";
    match id {
        "inbox" => rsx! {
            svg {
                class: svg_class,
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.75",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M22 12h-6l-2 3h-4l-2-3H2" }
                path { d: "M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z" }
            }
        },
        "drafts" => rsx! {
            svg {
                class: svg_class,
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.75",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" }
                path { d: "M14 2v4a2 2 0 0 0 2 2h4" }
                path { d: "M10 9H8" }
                path { d: "M16 13H8" }
                path { d: "M16 17H8" }
            }
        },
        "sent" => rsx! {
            svg {
                class: svg_class,
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.75",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "m22 2-7 20-4-9-9-4Z" }
                path { d: "M22 2 11 13" }
            }
        },
        "junk" => rsx! {
            svg {
                class: svg_class,
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.75",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                rect { width: "18", height: "18", x: "3", y: "3", rx: "2" }
                path { d: "m9 9 6 6" }
                path { d: "m15 9-6 6" }
            }
        },
        "trash" => rsx! {
            svg {
                class: svg_class,
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.75",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M3 6h18" }
                path { d: "M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6" }
                path { d: "M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" }
                path { d: "M10 11v6" }
                path { d: "M14 11v6" }
            }
        },
        _ => rsx! {
            svg {
                class: svg_class,
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.75",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                rect { width: "20", height: "5", x: "2", y: "3", rx: "1" }
                path { d: "M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8" }
                path { d: "M10 12h4" }
            }
        },
    }
}
