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
    #[props(default)]
    pub on_toggle_star: Option<EventHandler<String>>,
    #[props(default)]
    pub on_delete: Option<EventHandler<String>>,
}

#[component]
pub fn MailList(props: MailListProps) -> Element {
    let mut search = props.search;
    let mut unread_only = props.unread_only;
    let on_select = props.on_select;
    let on_toggle_star = props.on_toggle_star;
    let unread = unread_only();
    let selected_id = props.selected_id.clone();
    let mails = props.mails.clone();
    let title = props.title.clone();

    let pill_class = |active: bool| {
        if active {
            "rounded-md bg-background px-3 py-1 text-xs font-semibold text-foreground shadow-xs cursor-pointer"
        } else {
            "rounded-md px-3 py-1 text-xs font-medium text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
        }
    };

    rsx! {
        div { class: "flex min-w-0 flex-1 flex-col overflow-hidden select-none",

            // Header: Title + All / Unread toggle
            div { class: "flex shrink-0 items-center justify-between px-4 pb-3 pt-4",
                div { class: "flex items-center gap-2",
                    h2 { class: "text-lg font-bold tracking-tight text-foreground", "{title}" }
                    span { class: "rounded-full bg-muted/80 px-2 py-0.5 text-[11px] font-semibold text-muted-foreground",
                        "{mails.len()}"
                    }
                }

                div { class: "flex items-center rounded-lg bg-muted p-0.5",
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
                    SearchIcon { class: "pointer-events-none absolute left-3 top-1/2 size-3.5 -translate-y-1/2 text-muted-foreground/60" }
                    input {
                        r#type: "text",
                        class: "w-full rounded-xl border border-border/50 bg-background/60 py-1.5 pl-8.5 pr-3 text-xs text-foreground placeholder:text-muted-foreground/60 focus:outline-none focus:ring-1 focus:ring-primary/40 transition-all",
                        placeholder: "Search emails...",
                        value: search(),
                        oninput: move |e| search.set(e.value()),
                    }
                }
            }

            // Mail cards
            div { class: "min-h-0 flex-1 space-y-2 overflow-y-auto px-3 pb-4",
                if mails.is_empty() {
                    div { class: "py-12 flex flex-col items-center justify-center gap-2 text-center text-xs text-muted-foreground",
                        svg {
                            class: "size-8 text-muted-foreground/40",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "1.5",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M22 12h-6l-2 3h-4l-2-3H2" }
                            path { d: "M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z" }
                        }
                        span { "No messages in this folder" }
                    }
                }

                for mail in mails {
                    { mail_card(mail, selected_id.clone(), on_select, on_toggle_star) }
                }
            }
        }
    }
}

fn mail_card(
    mail: Mail,
    selected_id: Option<String>,
    on_select: EventHandler<String>,
    on_toggle_star: Option<EventHandler<String>>,
) -> Element {
    let selected = selected_id.as_deref() == Some(mail.id.as_str());
    let id = mail.id.clone();
    let star_id = mail.id.clone();
    let is_starred = mail.is_starred;
    let is_unread = !mail.read;

    let card_class = if selected {
        "group relative flex w-full flex-col gap-1 rounded-xl border border-primary/40 bg-primary/5 p-3 text-left shadow-xs transition-all cursor-pointer"
    } else if is_unread {
        "group relative flex w-full flex-col gap-1 rounded-xl border border-border/80 bg-card p-3 text-left hover:bg-muted/40 transition-all cursor-pointer shadow-2xs"
    } else {
        "group relative flex w-full flex-col gap-1 rounded-xl border border-border/40 bg-card/60 p-3 text-left hover:bg-muted/40 opacity-85 hover:opacity-100 transition-all cursor-pointer"
    };

    rsx! {
        div {
            key: "{id}",
            class: card_class,
            onclick: move |_| on_select.call(id.clone()),

            // Sender + date + Star
            div { class: "flex items-center justify-between gap-2",
                div { class: "flex min-w-0 items-center gap-2",
                    if is_unread {
                        span { class: "size-2 shrink-0 rounded-full bg-primary ring-2 ring-background" }
                    }
                    span { class: format!("truncate text-xs font-semibold {}", if is_unread { "text-foreground font-bold" } else { "text-foreground/90" }),
                        "{mail.name}"
                    }
                }
                div { class: "flex items-center gap-1.5 shrink-0",
                    span { class: "text-[10px] text-muted-foreground/80",
                        "{mail.date}"
                    }
                    button {
                        r#type: "button",
                        class: if is_starred {
                            "text-amber-400 hover:scale-115 transition-transform cursor-pointer"
                        } else {
                            "text-muted-foreground/40 hover:text-amber-400 hover:scale-115 transition-all cursor-pointer"
                        },
                        title: if is_starred { "Unstar email" } else { "Star email" },
                        onclick: move |e| {
                            e.stop_propagation();
                            if let Some(h) = &on_toggle_star {
                                h.call(star_id.clone());
                            }
                        },
                        svg {
                            class: "size-3.5",
                            view_box: "0 0 24 24",
                            fill: if is_starred { "currentColor" } else { "none" },
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polygon { points: "12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" }
                        }
                    }
                }
            }

            // Subject
            span { class: format!("truncate text-xs {}", if is_unread { "font-semibold text-foreground" } else { "font-medium text-foreground/80" }),
                "{mail.subject}"
            }

            // Preview
            p { class: "line-clamp-2 text-[11px] text-muted-foreground leading-relaxed",
                "{mail.preview}"
            }

            // Labels
            if !mail.labels.is_empty() {
                div { class: "flex flex-wrap gap-1 pt-1",
                    for label in mail.labels.iter() {
                        span {
                            key: "{label.name}",
                            class: if label.dark {
                                "rounded-md bg-primary/15 border border-primary/20 px-1.5 py-0.2 text-[10px] font-semibold text-primary"
                            } else {
                                "rounded-md bg-secondary/80 border border-border/50 px-1.5 py-0.2 text-[10px] font-medium text-secondary-foreground"
                            },
                            "{label.name}"
                        }
                    }
                }
            }
        }
    }
}
