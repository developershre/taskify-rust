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
}

#[component]
pub fn MailRead(props: MailReadProps) -> Element {
    rsx! {
        section { class: "flex min-w-0 flex-1 flex-col overflow-hidden bg-background",

            // Toolbar
            div { class: "flex shrink-0 items-center justify-between border-b border-border/40 px-3 py-2",

                div { class: "flex items-center gap-1",
                    { tool_button(
                        "Archive",
                        rsx! {
                            svg {
                                class: "size-4",
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
                    ) }
                    { tool_button(
                        "Move to inbox",
                        rsx! {
                            svg {
                                class: "size-4",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.75",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                rect { width: "20", height: "5", x: "2", y: "3", rx: "1" }
                                path { d: "M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8" }
                                path { d: "M12 11v5" }
                                path { d: "m9.5 13.5 2.5 2.5 2.5-2.5" }
                            }
                        },
                    ) }
                    { tool_button(
                        "Delete",
                        rsx! {
                            svg {
                                class: "size-4",
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
                    ) }
                    { tool_button(
                        "Snooze",
                        rsx! {
                            svg {
                                class: "size-4",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.75",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                circle { cx: "12", cy: "12", r: "10" }
                                path { d: "M12 6v6l4 2" }
                            }
                        },
                    ) }
                }

                div { class: "flex items-center gap-1",
                    { tool_button(
                        "Reply",
                        rsx! {
                            svg {
                                class: "size-4",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.75",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                polyline { points: "9 17 4 12 9 7" }
                                path { d: "M20 18v-2a4 4 0 0 0-4-4H4" }
                            }
                        },
                    ) }
                    { tool_button(
                        "Reply all",
                        rsx! {
                            svg {
                                class: "size-4",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.75",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                polyline { points: "7 17 2 12 7 7" }
                                polyline { points: "12 17 7 12 12 7" }
                                path { d: "M22 18v-2a4 4 0 0 0-4-4H6" }
                            }
                        },
                    ) }
                    { tool_button(
                        "Forward",
                        rsx! {
                            svg {
                                class: "size-4",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.75",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                polyline { points: "15 17 20 12 15 7" }
                                path { d: "M4 18v-2a4 4 0 0 1 4-4h12" }
                            }
                        },
                    ) }

                    div { class: "mx-1 h-5 w-px bg-border" }

                    // More options
                    DropdownMenu {
                        DropdownMenuTrigger {
                            class: "rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted/60",
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
                            DropdownMenuItem { "Mark as unread" }
                            DropdownMenuItem { "Mute conversation" }
                            DropdownMenuSeparator {}
                            DropdownMenuItem { variant: "destructive", "Delete" }
                        }
                    }
                }
            }

            if let Some(mail) = &props.mail {
                { mail_content(mail) }
            } else {
                // Empty state
                div { class: "flex min-h-0 flex-1 flex-col items-center justify-center gap-4 select-none",
                    svg {
                        class: "size-9 text-muted-foreground/70",
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
                    p { class: "text-lg text-muted-foreground",
                        "No message selected"
                    }
                }
            }
        }
    }
}

fn tool_button(title: &str, icon: Element) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: TOOL_CLASS,
            title: "{title}",
            {icon}
        }
    }
}

fn mail_content(mail: &Mail) -> Element {
    let initials: String = mail
        .name
        .split_whitespace()
        .filter_map(|w| w.chars().next())
        .take(2)
        .collect();
    let reply_placeholder = format!("Reply to {}...", mail.name);

    rsx! {
        div { class: "min-h-0 flex-1 overflow-y-auto",
            div { class: "mx-auto max-w-3xl px-6 py-6",

                // Sender row
                div { class: "flex items-start justify-between gap-4",
                    div { class: "flex min-w-0 items-center gap-3",
                        div { class: "flex size-10 shrink-0 items-center justify-center rounded-full bg-primary text-sm font-semibold text-primary-foreground",
                            "{initials}"
                        }
                        div { class: "min-w-0",
                            div { class: "flex items-center gap-2",
                                span { class: "truncate text-sm font-semibold text-foreground",
                                    "{mail.name}"
                                }
                                if mail.online {
                                    span { class: "size-2 shrink-0 rounded-full bg-blue-500" }
                                }
                            }
                            span { class: "block truncate text-xs text-muted-foreground",
                                "{mail.email}"
                            }
                        }
                    }
                    span { class: "shrink-0 text-xs text-muted-foreground",
                        "{mail.date}"
                    }
                }

                // Subject
                h1 { class: "mt-5 text-lg font-semibold text-foreground",
                    "{mail.subject}"
                }

                // Body
                div { class: "mt-4 space-y-4",
                    for paragraph in mail.body.iter() {
                        p { class: "text-sm leading-relaxed text-muted-foreground",
                            "{paragraph}"
                        }
                    }
                }

                // Reply box
                div { class: "mt-8 rounded-lg border border-border/60 bg-card p-3",
                    textarea {
                        class: "w-full resize-none bg-transparent text-sm text-foreground placeholder:text-muted-foreground/70 focus:outline-none",
                        rows: "3",
                        placeholder: "{reply_placeholder}",
                    }
                    div { class: "mt-2 flex justify-end",
                        button {
                            r#type: "button",
                            class: "rounded-md bg-primary px-3.5 py-1.5 text-sm font-medium text-primary-foreground hover:bg-primary/90 transition-colors cursor-pointer",
                            onclick: move |_| {},
                            "Send"
                        }
                    }
                }
            }
        }
    }
}
