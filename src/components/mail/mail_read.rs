use dioxus::prelude::*;

use super::types::Mail;
use crate::components::ui::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator, DropdownMenuTrigger,
};

const TOOL_CLASS: &str =
    "inline-flex size-8 items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted/60 transition-colors cursor-pointer";

#[derive(Props, Clone, PartialEq)]
pub struct MailReadProps {
    pub mail: Option<Mail>,
    #[props(default)]
    pub on_archive: Option<EventHandler<String>>,
    #[props(default)]
    pub on_move_to_inbox: Option<EventHandler<String>>,
    #[props(default)]
    pub on_delete: Option<EventHandler<String>>,
    #[props(default)]
    pub on_snooze: Option<EventHandler<String>>,
    #[props(default)]
    pub on_toggle_star: Option<EventHandler<String>>,
    #[props(default)]
    pub on_mark_unread: Option<EventHandler<String>>,
    #[props(default)]
    pub on_reply: Option<EventHandler<(String, String)>>,
}

#[component]
pub fn MailRead(props: MailReadProps) -> Element {
    let mut reply_text = use_signal(String::new);
    let mut reply_mode = use_signal(|| false);
    let toast_message = use_signal(String::new);

    let mail = props.mail.clone();
    let current_id = mail.as_ref().map(|m| m.id.clone()).unwrap_or_default();
    let is_starred = mail.as_ref().map(|m| m.is_starred).unwrap_or(false);

    let on_archive = props.on_archive;
    let on_move_to_inbox = props.on_move_to_inbox;
    let on_delete = props.on_delete;
    let on_snooze = props.on_snooze;
    let on_toggle_star = props.on_toggle_star;
    let on_mark_unread = props.on_mark_unread;
    let on_reply = props.on_reply;

    rsx! {
        section { class: "flex min-w-0 flex-1 flex-col overflow-hidden bg-background relative select-none",

            // Toast feedback
            if !toast_message.read().is_empty() {
                div { class: "absolute top-3 right-4 z-50 flex items-center gap-2 rounded-xl border border-border bg-card px-3 py-2 text-xs font-semibold text-foreground shadow-lg animate-in fade-in duration-150",
                    span { "{toast_message}" }
                    button {
                        r#type: "button",
                        class: "text-muted-foreground hover:text-foreground text-xs cursor-pointer ml-2",
                        onclick: move |_| {
                            let mut t = toast_message;
                            t.set(String::new());
                        },
                        "✕"
                    }
                }
            }

            // Toolbar
            div { class: "flex shrink-0 items-center justify-between border-b border-border/40 px-3 py-2 bg-card/20 backdrop-blur",

                div { class: "flex items-center gap-1",
                    // Archive
                    {
                        let cid = current_id.clone();
                        rsx! {
                            button {
                                r#type: "button",
                                class: TOOL_CLASS,
                                title: "Archive",
                                onclick: move |_| {
                                    if !cid.is_empty() {
                                        if let Some(h) = &on_archive {
                                            h.call(cid.clone());
                                            set_toast(toast_message, "Conversation archived");
                                        }
                                    }
                                },
                                svg { class: "size-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.75", stroke_linecap: "round", stroke_linejoin: "round",
                                    rect { width: "20", height: "5", x: "2", y: "3", rx: "1" }
                                    path { d: "M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8" }
                                    path { d: "M10 12h4" }
                                }
                            }
                        }
                    }

                    // Move to inbox
                    {
                        let cid = current_id.clone();
                        rsx! {
                            button {
                                r#type: "button",
                                class: TOOL_CLASS,
                                title: "Move to inbox",
                                onclick: move |_| {
                                    if !cid.is_empty() {
                                        if let Some(h) = &on_move_to_inbox {
                                            h.call(cid.clone());
                                            set_toast(toast_message, "Moved to inbox");
                                        }
                                    }
                                },
                                svg { class: "size-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.75", stroke_linecap: "round", stroke_linejoin: "round",
                                    rect { width: "20", height: "5", x: "2", y: "3", rx: "1" }
                                    path { d: "M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8" }
                                    path { d: "M12 11v5" }
                                    path { d: "m9.5 13.5 2.5 2.5 2.5-2.5" }
                                }
                            }
                        }
                    }

                    // Delete
                    {
                        let cid = current_id.clone();
                        rsx! {
                            button {
                                r#type: "button",
                                class: TOOL_CLASS,
                                title: "Delete",
                                onclick: move |_| {
                                    if !cid.is_empty() {
                                        if let Some(h) = &on_delete {
                                            h.call(cid.clone());
                                            set_toast(toast_message, "Moved to trash");
                                        }
                                    }
                                },
                                svg { class: "size-4 text-destructive/80 hover:text-destructive", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.75", stroke_linecap: "round", stroke_linejoin: "round",
                                    path { d: "M3 6h18" }
                                    path { d: "M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6" }
                                    path { d: "M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" }
                                    path { d: "M10 11v6" }
                                    path { d: "M14 11v6" }
                                }
                            }
                        }
                    }

                    // Snooze
                    {
                        let cid = current_id.clone();
                        rsx! {
                            button {
                                r#type: "button",
                                class: TOOL_CLASS,
                                title: "Snooze",
                                onclick: move |_| {
                                    if !cid.is_empty() {
                                        if let Some(h) = &on_snooze {
                                            h.call(cid.clone());
                                            set_toast(toast_message, "Snoozed until tomorrow");
                                        }
                                    }
                                },
                                svg { class: "size-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.75", stroke_linecap: "round", stroke_linejoin: "round",
                                    circle { cx: "12", cy: "12", r: "10" }
                                    path { d: "M12 6v6l4 2" }
                                }
                            }
                        }
                    }

                    // Star / Unstar
                    {
                        let cid = current_id.clone();
                        rsx! {
                            button {
                                r#type: "button",
                                class: TOOL_CLASS,
                                title: if is_starred { "Unstar" } else { "Star" },
                                onclick: move |_| {
                                    if !cid.is_empty() {
                                        if let Some(h) = &on_toggle_star {
                                            h.call(cid.clone());
                                        }
                                    }
                                },
                                svg {
                                    class: if is_starred { "size-4 text-amber-400" } else { "size-4" },
                                    view_box: "0 0 24 24",
                                    fill: if is_starred { "currentColor" } else { "none" },
                                    stroke: "currentColor",
                                    stroke_width: "1.75",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    polygon { points: "12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" }
                                }
                            }
                        }
                    }
                }

                div { class: "flex items-center gap-1",
                    // Reply
                    button {
                        r#type: "button",
                        class: TOOL_CLASS,
                        title: "Reply",
                        onclick: move |_| {
                            reply_mode.set(true);
                        },
                        svg { class: "size-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.75", stroke_linecap: "round", stroke_linejoin: "round",
                            polyline { points: "9 17 4 12 9 7" }
                            path { d: "M20 18v-2a4 4 0 0 0-4-4H4" }
                        }
                    }

                    // Reply all
                    button {
                        r#type: "button",
                        class: TOOL_CLASS,
                        title: "Reply all",
                        onclick: move |_| {
                            reply_mode.set(true);
                        },
                        svg { class: "size-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.75", stroke_linecap: "round", stroke_linejoin: "round",
                            polyline { points: "7 17 2 12 7 7" }
                            polyline { points: "12 17 7 12 12 7" }
                            path { d: "M22 18v-2a4 4 0 0 0-4-4H6" }
                        }
                    }

                    // Forward
                    button {
                        r#type: "button",
                        class: TOOL_CLASS,
                        title: "Forward",
                        onclick: move |_| {
                            reply_mode.set(true);
                        },
                        svg { class: "size-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.75", stroke_linecap: "round", stroke_linejoin: "round",
                            polyline { points: "15 17 20 12 15 7" }
                            path { d: "M4 18v-2a4 4 0 0 1 4-4h12" }
                        }
                    }

                    div { class: "mx-1 h-5 w-px bg-border/60" }

                    // More options dropdown
                    {
                        let cid_unread = current_id.clone();
                        let cid_delete = current_id.clone();
                        rsx! {
                            DropdownMenu {
                                DropdownMenuTrigger {
                                    class: "rounded-lg p-1.5 text-muted-foreground hover:text-foreground hover:bg-muted/60 cursor-pointer",
                                    svg {
                                        class: "size-4",
                                        view_box: "0 0 24 24",
                                        fill: "currentColor",
                                        circle { cx: "12", cy: "5", r: "1.5" }
                                        circle { cx: "12", cy: "12", r: "1.5" }
                                        circle { cx: "12", cy: "19", r: "1.5" }
                                    }
                                }
                                DropdownMenuContent {
                                    align: "end",
                                    side_offset: 6,
                                    DropdownMenuItem {
                                        onclick: move |_| {
                                            if !cid_unread.is_empty() {
                                                if let Some(h) = &on_mark_unread {
                                                    h.call(cid_unread.clone());
                                                    set_toast(toast_message, "Marked as unread");
                                                }
                                            }
                                        },
                                        "Mark as unread"
                                    }
                                    DropdownMenuItem { "Mute thread" }
                                    DropdownMenuItem { "Print conversation" }
                                    DropdownMenuSeparator {}
                                    DropdownMenuItem {
                                        variant: "destructive",
                                        onclick: move |_| {
                                            if !cid_delete.is_empty() {
                                                if let Some(h) = &on_delete {
                                                    h.call(cid_delete.clone());
                                                }
                                            }
                                        },
                                        "Delete message"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if let Some(mail) = &props.mail {
                {
                    let m = mail.clone();
                    let cid = m.id.clone();
                    let initials: String = m
                        .name
                        .split_whitespace()
                        .filter_map(|w| w.chars().next())
                        .take(2)
                        .collect();
                    let reply_placeholder = format!("Reply to {}...", m.name);

                    rsx! {
                        div { class: "min-h-0 flex-1 overflow-y-auto",
                            div { class: "mx-auto max-w-3xl px-6 py-6",

                                // Sender row
                                div { class: "flex items-start justify-between gap-4 pb-4 border-b border-border/40",
                                    div { class: "flex min-w-0 items-center gap-3",
                                        div { class: "flex size-10 shrink-0 items-center justify-center rounded-xl bg-primary/10 border border-primary/20 text-sm font-bold text-primary",
                                            "{initials}"
                                        }
                                        div { class: "min-w-0",
                                            div { class: "flex items-center gap-2",
                                                span { class: "truncate text-sm font-semibold text-foreground",
                                                    "{m.name}"
                                                }
                                                if m.online {
                                                    span { class: "size-2 shrink-0 rounded-full bg-emerald-500 ring-2 ring-background", title: "Active now" }
                                                }
                                            }
                                            span { class: "block truncate text-xs text-muted-foreground",
                                                "{m.email}"
                                            }
                                        }
                                    }
                                    span { class: "shrink-0 text-xs text-muted-foreground",
                                        "{m.date}"
                                    }
                                }

                                // Subject
                                h1 { class: "mt-5 text-lg font-bold text-foreground tracking-tight",
                                    "{m.subject}"
                                }

                                // Labels badges
                                if !m.labels.is_empty() {
                                    div { class: "mt-2 flex flex-wrap gap-1.5",
                                        for label in m.labels.iter() {
                                            span {
                                                key: "{label.name}",
                                                class: "rounded-md bg-muted px-2 py-0.5 text-xs font-medium text-muted-foreground",
                                                "🏷️ {label.name}"
                                            }
                                        }
                                    }
                                }

                                // Body paragraphs
                                div { class: "mt-5 space-y-4 text-xs sm:text-sm leading-relaxed text-foreground/90",
                                    for paragraph in m.body.iter() {
                                        p { class: "p-3 rounded-xl bg-card/40 border border-border/30",
                                            "{paragraph}"
                                        }
                                    }
                                }

                                // Reply box
                                div { class: "mt-8 rounded-2xl border border-border/60 bg-card p-4 shadow-sm",
                                    div { class: "flex items-center gap-2 pb-2 mb-2 border-b border-border/40 text-xs font-semibold text-foreground",
                                        svg { class: "size-3.5 text-primary", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                                            polyline { points: "9 17 4 12 9 7" }
                                            path { d: "M20 18v-2a4 4 0 0 0-4-4H4" }
                                        }
                                        span { "Quick Reply" }
                                    }

                                    textarea {
                                        class: "w-full resize-none bg-transparent text-xs sm:text-sm text-foreground placeholder:text-muted-foreground/60 focus:outline-none",
                                        rows: "4",
                                        placeholder: "{reply_placeholder}",
                                        value: reply_text(),
                                        oninput: move |e| reply_text.set(e.value()),
                                    }

                                    div { class: "mt-3 flex items-center justify-between pt-2 border-t border-border/30",
                                        div { class: "flex items-center gap-1",
                                            button {
                                                r#type: "button",
                                                class: "p-1.5 text-muted-foreground hover:text-foreground rounded-lg hover:bg-muted/60 transition-colors cursor-pointer",
                                                title: "Attach file",
                                                svg { class: "size-3.5", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                                                    path { d: "m21.44 11.05-9.19 9.19a6 6 0 0 1-8.49-8.49l8.57-8.57A4 4 0 1 1 18 8.84l-8.59 8.57a2 2 0 0 1-2.83-2.83l8.49-8.48" }
                                                }
                                            }
                                            button {
                                                r#type: "button",
                                                class: "p-1.5 text-muted-foreground hover:text-foreground rounded-lg hover:bg-muted/60 transition-colors cursor-pointer font-bold text-xs",
                                                title: "Bold",
                                                "B"
                                            }
                                            button {
                                                r#type: "button",
                                                class: "p-1.5 text-muted-foreground hover:text-foreground rounded-lg hover:bg-muted/60 transition-colors cursor-pointer italic text-xs",
                                                title: "Italic",
                                                "I"
                                            }
                                        }

                                        div { class: "flex items-center gap-2",
                                            if !reply_text.read().is_empty() {
                                                button {
                                                    r#type: "button",
                                                    class: "rounded-lg border border-border px-3 py-1.5 text-xs font-medium text-foreground hover:bg-muted cursor-pointer",
                                                    onclick: move |_| reply_text.set(String::new()),
                                                    "Clear"
                                                }
                                            }
                                            button {
                                                r#type: "button",
                                                class: "inline-flex items-center gap-1.5 rounded-xl bg-primary px-4 py-1.5 text-xs font-semibold text-primary-foreground hover:bg-primary/90 transition-all active:scale-98 cursor-pointer shadow-xs",
                                                onclick: move |_| {
                                                    let text = reply_text.read().trim().to_string();
                                                    if !text.is_empty() {
                                                        if let Some(h) = &on_reply {
                                                            h.call((cid.clone(), text));
                                                            reply_text.set(String::new());
                                                            set_toast(toast_message, "Reply sent successfully");
                                                        }
                                                    }
                                                },
                                                svg { class: "size-3.5", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                                                    line { x1: "22", y1: "2", x2: "11", y2: "13" }
                                                    polygon { points: "22 2 15 22 11 13 2 9 22 2" }
                                                }
                                                "Send Reply"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                // Empty state
                div { class: "flex min-h-0 flex-1 flex-col items-center justify-center gap-4 select-none text-center p-8",
                    div { class: "size-16 rounded-2xl bg-muted/60 border border-border/50 flex items-center justify-center text-muted-foreground/60 shadow-xs",
                        svg {
                            class: "size-8",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "1.5",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "m3 3 7.07 16.97 2.51-7.39 7.39-2.51z" }
                            path { d: "M7.2 2.2 8 5.1" }
                            path { d: "m5.1 7.2-2.9-.8" }
                            path { d: "m18.5 3 .5 1.5 1.5.5-1.5.5-.5 1.5-.5-1.5-1.5-.5 1.5-.5z" }
                        }
                    }
                    div { class: "space-y-1",
                        h3 { class: "text-base font-bold text-foreground", "No Message Selected" }
                        p { class: "text-xs text-muted-foreground max-w-xs",
                            "Select an email from the list to view the message thread and reply."
                        }
                    }
                }
            }
        }
    }
}

fn set_toast(mut sig: Signal<String>, msg: &str) {
    sig.set(msg.to_string());
}
