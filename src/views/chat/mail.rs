use dioxus::prelude::*;

use crate::views::PageHeader;

struct Mail {
    sender: &'static str,
    subject: &'static str,
    preview: &'static str,
    time: &'static str,
    unread: bool,
}

#[component]
pub fn ChatMail() -> Element {
    let mut selected = use_signal(|| 0usize);

    let mails = [
        Mail {
            sender: "GitHub",
            subject: "[taskify] PR #42 was merged",
            preview: "Your pull request \"Fix popup positioning\" was merged into main…",
            time: "09:51",
            unread: true,
        },
        Mail {
            sender: "Dioxus Team",
            subject: "Release notes for 0.7.10",
            preview: "This release includes fixes for router layouts and hot reload…",
            time: "08:30",
            unread: true,
        },
        Mail {
            sender: "Linear",
            subject: "5 issues assigned to you",
            preview: "TASK-101, TASK-98, TASK-95 and 2 more need your attention…",
            time: "Yesterday",
            unread: false,
        },
        Mail {
            sender: "Vercel",
            subject: "Your preview deployment is ready",
            preview: "taskify-web-git-fix-sidebar was deployed successfully…",
            time: "Yesterday",
            unread: false,
        },
        Mail {
            sender: "Marc Rivera",
            subject: "Re: Calendar sync design",
            preview: "Looks good to me — I left two small comments on the doc…",
            time: "Monday",
            unread: false,
        },
    ];

    rsx! {
        div { class: "flex flex-col",
            PageHeader {
                title: "Mail",
                description: "Your unified inbox for workspace notifications.",
            }

            div { class: "flex flex-col gap-2",
                for (i, mail) in mails.iter().enumerate() {
                    {
                        let is_active = selected() == i;
                        let row_class = if is_active {
                            "flex cursor-pointer items-start gap-3 rounded-xl border p-3.5 shadow-xs transition-colors bg-accent border-accent-foreground/20"
                        } else {
                            "flex cursor-pointer items-start gap-3 rounded-xl border p-3.5 shadow-xs transition-colors bg-card border-border/40 hover:bg-accent/50"
                        };
                        rsx! {
                            div {
                                key: "{mail.subject}",
                                class: row_class,
                                onclick: move |_| selected.set(i),
                                div { class: "flex size-9 shrink-0 items-center justify-center rounded-lg bg-secondary text-muted-foreground",
                                    svg {
                                        class: "size-4",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        rect {
                                            width: "20",
                                            height: "16",
                                            x: "2",
                                            y: "4",
                                            rx: "2",
                                        }
                                        path { d: "m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" }
                                    }
                                }
                                div { class: "flex min-w-0 flex-1 flex-col gap-1",
                                    div { class: "flex items-center justify-between gap-2",
                                        span { class: if mail.unread {
                                            "truncate text-sm font-semibold text-foreground"
                                        } else {
                                            "truncate text-sm font-medium text-muted-foreground"
                                        },
                                            "{mail.sender}"
                                        }
                                        span { class: "shrink-0 text-[10px] text-muted-foreground",
                                            "{mail.time}"
                                        }
                                    }
                                    span { class: "truncate text-xs font-medium text-foreground",
                                        "{mail.subject}"
                                    }
                                    p { class: "truncate text-xs text-muted-foreground",
                                        "{mail.preview}"
                                    }
                                }
                                if mail.unread {
                                    span { class: "mt-1.5 size-2 shrink-0 rounded-full bg-primary" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
