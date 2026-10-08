use dioxus::prelude::*;

use super::types::Mail;
use crate::icons::SearchIcon;

#[derive(Props, Clone, PartialEq)]
pub struct MailListProps {
    pub mails: Vec<Mail>,
    pub title: String,
    pub selected_id: Option<String>,
    pub on_select: EventHandler<String>,
    pub search: Signal<String>,
    pub unread_only: Signal<bool>,
}

#[component]
pub fn MailList(props: MailListProps) -> Element {
    let mut search = props.search;
    let mut unread_only = props.unread_only;
    let on_select = props.on_select;
    let unread = unread_only();
    let selected_id = props.selected_id.clone();
    let mails = props.mails.clone();
    let title = props.title.clone();

    let pill_class = |active: bool| {
        if active {
            "rounded-md bg-background px-3 py-1 text-sm font-medium text-foreground shadow-xs cursor-pointer"
        } else {
            "rounded-md px-3 py-1 text-sm font-medium text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
        }
    };

    rsx! {
        div { class: "flex min-w-0 flex-1 flex-col overflow-hidden",

            // Header: Title + All / Unread toggle
            div { class: "flex shrink-0 items-center justify-between px-4 pb-3 pt-4",
                h2 { class: "text-xl font-bold tracking-tight text-foreground", "{title}" }

                div { class: "flex items-center rounded-lg bg-muted p-1",
                    button {
                        r#type: "button",
                        class: pill_class(!unread),
                        onclick: move |_| unread_only.set(false),
                        "All"
                    }
                    button {
                        r#type: "button",
                        class: pill_class(unread),
                        onclick: move |_| unread_only.set(true),
                        "Unread"
                    }
                }
            }

            // Search
            div { class: "shrink-0 px-4 pb-3",
                div { class: "relative",
                    SearchIcon { class: "pointer-events-none absolute left-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" }
                    input {
                        r#type: "text",
                        class: "w-full rounded-lg border border-border/60 bg-background py-2 pl-9 pr-3 text-sm text-foreground placeholder:text-muted-foreground/70 focus:border-primary/40 focus:outline-none focus:ring-1 focus:ring-primary/30 transition-all",
                        placeholder: "Search",
                        value: search(),
                        oninput: move |e| search.set(e.value()),
                    }
                }
            }

            // Mail cards
            div { class: "min-h-0 flex-1 space-y-3 overflow-y-auto px-4 pb-4",
                if mails.is_empty() {
                    div { class: "py-10 text-center text-sm text-muted-foreground",
                        "No mail found"
                    }
                }

                for mail in mails {
                    { mail_card(mail, selected_id.clone(), on_select) }
                }
            }
        }
    }
}

fn mail_card(mail: Mail, selected_id: Option<String>, on_select: EventHandler<String>) -> Element {
    let selected = selected_id.as_deref() == Some(mail.id.as_str());
    let id = mail.id.clone();

    let card_class = if selected {
        "flex w-full flex-col gap-1 rounded-lg border border-primary bg-card p-4 text-left shadow-xs cursor-pointer"
    } else {
        "flex w-full flex-col gap-1 rounded-lg border border-border/60 bg-card p-4 text-left hover:bg-muted/40 transition-colors cursor-pointer"
    };

    rsx! {
        div {
            key: "{id}",
            class: card_class,
            onclick: move |_| on_select.call(id.clone()),

            // Sender + date
            div { class: "flex items-center justify-between gap-2",
                div { class: "flex min-w-0 items-center gap-1.5",
                    span { class: "truncate text-sm font-semibold text-foreground",
                        "{mail.name}"
                    }
                    if mail.online {
                        span { class: "size-2 shrink-0 rounded-full bg-blue-500" }
                    }
                }
                span { class: "shrink-0 text-xs text-muted-foreground",
                    "{mail.date}"
                }
            }

            // Subject
            span { class: "text-sm font-medium text-foreground",
                "{mail.subject}"
            }

            // Preview
            p { class: "line-clamp-2 text-xs text-muted-foreground",
                "{mail.preview}"
            }

            // Labels
            if !mail.labels.is_empty() {
                div { class: "flex flex-wrap gap-1.5 pt-1.5",
                    for label in mail.labels.iter() {
                        span {
                            key: "{label.name}",
                            class: if label.dark {
                                "rounded-md bg-primary px-2 py-0.5 text-xs font-medium text-primary-foreground"
                            } else {
                                "rounded-md bg-secondary px-2 py-0.5 text-xs font-medium text-secondary-foreground"
                            },
                            "{label.name}"
                        }
                    }
                }
            }
        }
    }
}
