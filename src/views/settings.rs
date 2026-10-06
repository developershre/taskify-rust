use dioxus::prelude::*;

use crate::components::ui::{Avatar, AvatarFallback, AvatarImage, Badge, Switch};
use crate::state::use_app_state;
use crate::state::ThemeMode;
use crate::views::PageHeader;

#[component]
pub fn Settings() -> Element {
    let mut app_state = use_app_state();
    let current_theme = *app_state.theme.read();

    rsx! {
        div { class: "flex max-w-3xl flex-col gap-6",
            PageHeader {
                title: "Settings",
                description: "Manage your account, appearance and notifications.",
            }

            // Profile
            section { class: "flex flex-col gap-4 rounded-xl border border-border/40 bg-card p-5 shadow-xs",
                div { class: "flex flex-col gap-1",
                    h2 { class: "text-sm font-semibold text-foreground", "Profile" }
                    p { class: "text-xs text-muted-foreground",
                        "Your public profile information across Taskify."
                    }
                }
                div { class: "flex items-center gap-4",
                    Avatar { class: "size-14 rounded-xl overflow-hidden border border-border/40 shrink-0",
                        AvatarImage {
                            src: "https://ui.shadcn.com/avatars/shadcn.jpg".to_string(),
                            alt: "Test User".to_string(),
                        }
                        AvatarFallback { "TU" }
                    }
                    div { class: "flex min-w-0 flex-col gap-1",
                        div { class: "flex items-center gap-2",
                            span { class: "truncate text-sm font-semibold text-foreground",
                                "Test User"
                            }
                            Badge { variant: "secondary", size: "sm", "Free plan" }
                        }
                        span { class: "truncate text-xs text-muted-foreground",
                            "test@taskify.app"
                        }
                    }
                    button {
                        r#type: "button",
                        class: "ml-auto inline-flex h-8 shrink-0 items-center rounded-md border border-border bg-transparent px-3 text-xs font-medium text-foreground hover:bg-muted transition-colors cursor-pointer",
                        onclick: move |_| println!("Settings > Edit profile"),
                        "Edit profile"
                    }
                }
            }

            // Appearance
            section { class: "flex flex-col gap-4 rounded-xl border border-border/40 bg-card p-5 shadow-xs",
                div { class: "flex flex-col gap-1",
                    h2 { class: "text-sm font-semibold text-foreground", "Appearance" }
                    p { class: "text-xs text-muted-foreground",
                        "Choose how Taskify looks on your device."
                    }
                }
                div { class: "flex items-center gap-2",
                    for mode in [ThemeMode::System, ThemeMode::Light, ThemeMode::Dark] {
                        {
                            let is_active = current_theme == mode;
                            let active_class = if is_active {
                                "border-primary bg-secondary text-foreground"
                            } else {
                                "border-border text-muted-foreground hover:bg-muted hover:text-foreground"
                            };
                            let label = mode.label();
                            rsx! {
                                button {
                                    key: "{label}",
                                    r#type: "button",
                                    class: "inline-flex h-8 items-center rounded-md border px-3 text-xs font-medium transition-colors cursor-pointer {active_class}",
                                    onclick: move |_| {
                                        app_state.set_theme(mode);
                                    },
                                    "{label}"
                                }
                            }
                            }
                        }
                    }
                }

            // Notifications
            section { class: "flex flex-col gap-4 rounded-xl border border-border/40 bg-card p-5 shadow-xs",
                div { class: "flex flex-col gap-1",
                    h2 { class: "text-sm font-semibold text-foreground", "Notifications" }
                    p { class: "text-xs text-muted-foreground",
                        "Decide what lands in your inbox."
                    }
                }
                div { class: "flex flex-col gap-3",
                    for (label, description, checked) in [
                        ("Task assignments", "Get notified when a task is assigned to you", true),
                        ("Daily digest", "A morning summary of due and overdue tasks", true),
                        ("Mentions", "When a teammate mentions you in a comment", false),
                        ("Product updates", "News about Taskify features and releases", false),
                    ] {
                        div { key: "{label}", class: "flex items-center justify-between gap-4",
                            div { class: "flex min-w-0 flex-col",
                                span { class: "text-xs font-medium text-foreground",
                                    "{label}"
                                }
                                span { class: "text-[11px] text-muted-foreground",
                                    "{description}"
                                }
                            }
                            Switch { default_checked: checked }
                        }
                    }
                }
            }

            // Danger zone
            section { class: "flex flex-col gap-4 rounded-xl border border-destructive/40 bg-card p-5 shadow-xs",
                div { class: "flex flex-col gap-1",
                    h2 { class: "text-sm font-semibold text-destructive", "Danger zone" }
                    p { class: "text-xs text-muted-foreground",
                        "Permanent actions for your account."
                    }
                }
                div { class: "flex items-center justify-between gap-4 rounded-lg border border-border/40 p-3",
                    div { class: "flex min-w-0 flex-col",
                        span { class: "text-xs font-medium text-foreground", "Delete account" }
                        span { class: "text-[11px] text-muted-foreground",
                            "Remove your account and all associated data."
                        }
                    }
                    button {
                        r#type: "button",
                        class: "inline-flex h-8 shrink-0 items-center rounded-md bg-destructive px-3 text-xs font-medium text-destructive-foreground shadow-xs hover:bg-destructive/90 transition-colors cursor-pointer",
                        onclick: move |_| println!("Settings > Delete account"),
                        "Delete account"
                    }
                }
            }
        }
    }
}
