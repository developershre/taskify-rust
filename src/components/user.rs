use dioxus::prelude::*;

use crate::components::ui::{
    Avatar, AvatarFallback, AvatarImage, Badge, Dialog, DialogContent, DialogDescription,
    DialogFooter, DialogHeader, DialogTitle, DropdownMenu, DropdownMenuContent, DropdownMenuItem,
    DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuShortcut, DropdownMenuTrigger,
};
use crate::icons::{CommandIcon, MoonIcon, SunIcon};
use crate::state::use_app_state;

/// Open state for the notifications dialog (provided by TitleBar, consumed here).
#[derive(Clone, Copy, PartialEq)]
pub struct NotificationsContext {
    pub open: Signal<bool>,
}

#[derive(Clone, PartialEq)]
struct NotifItem {
    letter: &'static str,
    title: &'static str,
    body: &'static str,
    time: &'static str,
    unread: bool,
}

fn sample_notifications() -> Vec<NotifItem> {
    vec![
        NotifItem {
            letter: "A",
            title: "Task assigned",
            body: "Alice assigned you to \"Fix login bug\"",
            time: "2 min ago",
            unread: true,
        },
        NotifItem {
            letter: "B",
            title: "New comment",
            body: "Bob commented on \"Q4 roadmap\"",
            time: "1 hour ago",
            unread: true,
        },
        NotifItem {
            letter: "D",
            title: "Due today",
            body: "\"Design review\" is due today at 5:00 PM",
            time: "3 hours ago",
            unread: true,
        },
        NotifItem {
            letter: "W",
            title: "Weekly report ready",
            body: "Your weekly productivity report is available",
            time: "Yesterday",
            unread: false,
        },
        NotifItem {
            letter: "E",
            title: "You were mentioned",
            body: "Emily mentioned you in \"API docs\"",
            time: "2 days ago",
            unread: false,
        },
    ]
}

#[component]
pub fn User() -> Element {
    let mut app_state = use_app_state();
    let router = router();
    let is_dark = app_state.is_dark();
    let mut notifications = use_context::<NotificationsContext>();
    let mut items = use_signal(sample_notifications);
    let unread_count = items.read().iter().filter(|n| n.unread).count();

    rsx! {
        DropdownMenu {
            DropdownMenuTrigger {
                class: "rounded-full outline-none cursor-pointer",
                Avatar { class: "size-4 rounded-full overflow-hidden border border-border/60",
                    AvatarImage {
                        src: "https://ui.shadcn.com/avatars/shadcn.jpg".to_string(),
                        alt: "Test User".to_string(),
                    }
                    AvatarFallback { "TU" }
                }
            }

            DropdownMenuContent {
                class: "w-64",
                align: "end",
                side_offset: 10,

                // Account header
                DropdownMenuLabel {
                    div { class: "flex items-center gap-3",
                        Avatar { class: "size-10 rounded-lg overflow-hidden border border-border/60 shrink-0",
                            AvatarImage {
                                src: "https://ui.shadcn.com/avatars/shadcn.jpg".to_string(),
                                alt: "Test User".to_string(),
                            }
                            AvatarFallback { "TU" }
                        }
                        div { class: "flex min-w-0 flex-col gap-1",
                            div { class: "flex items-center gap-1.5",
                                span { class: "truncate text-sm font-semibold text-foreground",
                                    "Test User"
                                }
                                Badge { variant: "secondary", size: "sm", "Free" }
                            }
                            span { class: "truncate text-xs text-muted-foreground",
                                "test@taskify.app"
                            }
                        }
                    }
                }

                DropdownMenuSeparator {}

                // Account
                DropdownMenuItem {
                    onclick: move |_| {
                        let _ = router.push("/settings");
                    },
                    svg {
                        class: "size-4 shrink-0 text-muted-foreground",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2" }
                        circle { cx: "12", cy: "7", r: "4" }
                    }
                    span { "Profile" }
                }
                DropdownMenuItem {
                    onclick: move |_| {
                        let _ = router.push("/settings");
                    },
                    svg {
                        class: "size-4 shrink-0 text-muted-foreground",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path {
                            d: "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z",
                        }
                        circle { cx: "12", cy: "12", r: "3" }
                    }
                    span { "Settings" }
                }
                DropdownMenuItem {
                    onclick: move |_| {
                        notifications.open.set(true);
                    },
                    svg {
                        class: "size-4 shrink-0 text-muted-foreground",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9" }
                        path { d: "M10.3 21a1.94 1.94 0 0 0 3.4 0" }
                    }
                    span { "Notifications" }
                    if unread_count > 0 {
                        DropdownMenuShortcut { "{unread_count}" }
                    }
                }

                DropdownMenuSeparator {}

                // Quick actions
                DropdownMenuItem {
                    onclick: move |_| {
                        app_state.search_open.set(true);
                    },
                    CommandIcon { class: "size-4 shrink-0 text-muted-foreground" }
                    span { "Command palette" }
                    DropdownMenuShortcut { "Ctrl K" }
                }
                DropdownMenuItem {
                    onclick: move |_| {
                        app_state.toggle_theme();
                    },
                    if is_dark {
                        SunIcon { class: "size-4 shrink-0 text-muted-foreground" }
                    } else {
                        MoonIcon { class: "size-4 shrink-0 text-muted-foreground" }
                    }
                    span { if is_dark { "Light mode" } else { "Dark mode" } }
                    DropdownMenuShortcut { "Ctrl T" }
                }

                DropdownMenuSeparator {}

                // Plan
                DropdownMenuItem {
                    onclick: move |_| {
                        let _ = router.push("/settings");
                    },
                    svg {
                        class: "size-4 shrink-0 text-muted-foreground",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        rect {
                            width: "20",
                            height: "14",
                            x: "2",
                            y: "5",
                            rx: "2",
                        }
                        path { d: "M2 10h20" }
                    }
                    span { "Billing & plan" }
                    DropdownMenuShortcut {
                        children: rsx! {
                            Badge { variant: "outline", size: "sm", "Free" }
                        },
                    }
                }
                DropdownMenuItem {
                    variant: "destructive",
                    onclick: move |_| {
                        println!("User > Sign out");
                    },
                    svg {
                        class: "size-4 shrink-0",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" }
                        path { d: "m16 17 5-5-5-5" }
                        path { d: "M21 12H9" }
                    }
                    span { "Sign out" }
                }
            }
        }

        // ============================================================
        // Notifications dialog (opened from the menu item above)
        // ============================================================
        Dialog {
            open: notifications.open,
            DialogContent { class: "max-w-lg",
                DialogHeader {
                    DialogTitle { "Notifications" }
                    DialogDescription {
                        if unread_count > 0 {
                            "You have {unread_count} unread notifications."
                        } else {
                            "You're all caught up."
                        }
                    }
                }

                div { class: "flex max-h-80 flex-col gap-1 overflow-y-auto pr-1",
                    for item in items.read().iter() {
                        div {
                            key: "{item.title}",
                            class: "flex items-start gap-3 rounded-lg p-2.5 transition-colors hover:bg-muted",

                            div { class: "mt-0.5 flex size-8 shrink-0 items-center justify-center rounded-full bg-primary/10 text-xs font-semibold text-primary",
                                "{item.letter}"
                            }

                            div { class: "min-w-0 flex-1",
                                div { class: "flex items-center gap-2",
                                    span { class: "truncate text-sm font-medium text-foreground",
                                        "{item.title}"
                                    }
                                    if item.unread {
                                        span { class: "size-1.5 shrink-0 rounded-full bg-blue-500" }
                                    }
                                    span { class: "ml-auto shrink-0 text-[11px] text-muted-foreground",
                                        "{item.time}"
                                    }
                                }
                                p { class: "line-clamp-2 text-xs text-muted-foreground",
                                    "{item.body}"
                                }
                            }
                        }
                    }
                }

                DialogFooter {
                    button {
                        r#type: "button",
                        class: "inline-flex h-8 items-center rounded-md border border-border px-3 text-xs font-medium text-foreground hover:bg-muted transition-colors cursor-pointer disabled:opacity-50",
                        disabled: unread_count == 0,
                        onclick: move |_| {
                            items.write().iter_mut().for_each(|n| n.unread = false);
                        },
                        "Mark all as read"
                    }
                    button {
                        r#type: "button",
                        class: "inline-flex h-8 items-center rounded-md bg-primary px-3 text-xs font-medium text-primary-foreground hover:opacity-90 transition-opacity cursor-pointer",
                        onclick: move |_| notifications.open.set(false),
                        "Close"
                    }
                }
            }
        }
    }
}
