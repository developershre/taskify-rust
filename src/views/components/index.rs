use dioxus::prelude::*;

use crate::components::ui::{
    Avatar, AvatarFallback, Badge, Switch, Tabs, TabsContent, TabsList, TabsTrigger,
};
use crate::components::{EmptyState, PageHeader};
use crate::icons::{CalendarIcon, LayersIcon, ZapIcon};

#[component]
fn ComponentCard(title: String, description: String, children: Element) -> Element {
    rsx! {
        div { class: "flex flex-col gap-3 rounded-xl border border-border/40 bg-card p-5 shadow-xs transition-colors",
            div { class: "flex flex-col gap-0.5",
                h3 { class: "text-sm font-semibold text-foreground", "{title}" }
                p { class: "text-xs text-muted-foreground", "{description}" }
            }
            div { class: "pt-2",
                {children}
            }
        }
    }
}

#[component]
pub fn Components() -> Element {
    let router = router();
    let switch_enabled = use_signal(|| true);

    rsx! {
        div { class: "flex max-w-5xl flex-col gap-8 pb-12",
            PageHeader {
                title: "Components Library".to_string(),
                description: "Design system building blocks, interactive widgets, and compound showcases.".to_string(),
                actions: rsx! {
                    button {
                        r#type: "button",
                        class: "inline-flex h-9 items-center gap-2 rounded-lg bg-primary px-3.5 text-xs font-medium text-primary-foreground shadow-xs hover:opacity-90 transition-opacity cursor-pointer",
                        onclick: move |_| {
                            let _ = router.push("/components/calendar");
                        },
                        CalendarIcon { class: "size-4" }
                        "Calendar Showcase"
                    }
                },
            }

            // Featured Callout for Sub-pages
            div { class: "relative overflow-hidden rounded-2xl border border-primary/20 bg-gradient-to-r from-primary/10 via-primary/5 to-transparent p-6 sm:p-8",
                div { class: "flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4",
                    div { class: "flex items-start gap-4",
                        div { class: "flex size-12 shrink-0 items-center justify-center rounded-xl bg-primary text-primary-foreground shadow-sm",
                            CalendarIcon { class: "size-6" }
                        }
                        div { class: "flex flex-col gap-1",
                            div { class: "flex items-center gap-2",
                                h2 { class: "text-lg font-bold tracking-tight text-foreground",
                                    "Calendar Component Suite"
                                }
                                span { class: "inline-flex items-center rounded-md bg-primary/20 px-2 py-0.5 text-[10px] font-semibold text-primary",
                                    "Dedicated Page"
                                }
                            }
                            p { class: "max-w-xl text-xs sm:text-sm text-muted-foreground",
                                "Full suite of modular calendar components including Toolbar, WeekdayHeader, DayCell, EventChip, and DayEventsDialog."
                            }
                        }
                    }
                    button {
                        r#type: "button",
                        class: "inline-flex h-9 shrink-0 items-center gap-2 rounded-lg border border-border bg-card px-4 text-xs font-semibold text-foreground hover:bg-muted transition-colors cursor-pointer",
                        onclick: move |_| {
                            let _ = router.push("/components/calendar");
                        },
                        "Explore Calendar View →"
                    }
                }
            }

            // Grid of Components
            div { class: "flex flex-col gap-4",
                div { class: "flex items-center gap-2 border-b border-border/40 pb-2",
                    LayersIcon {}
                    h2 { class: "text-sm font-semibold uppercase tracking-wider text-muted-foreground",
                        "Shared Core Components"
                    }
                }

                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    ComponentCard {
                        title: "Empty State".to_string(),
                        description: "Provides helpful fallback states for empty lists, search queries, or blank workspaces.".to_string(),
                        EmptyState {
                            title: "No tasks found".to_string(),
                            description: "Get started by creating your first task or importing from a template.".to_string(),
                            icon: rsx! {
                                ZapIcon { class: "size-5 text-amber-500" }
                            },
                        }
                    }

                    ComponentCard {
                        title: "Interactive Tabs".to_string(),
                        description: "Accessible tabs with animated states and multiple layout variants.".to_string(),
                        Tabs {
                            value: "preview".to_string(),
                            TabsList {
                                TabsTrigger {
                                    value: "preview".to_string(),
                                    "Preview"
                                }
                                TabsTrigger {
                                    value: "code".to_string(),
                                    "Code"
                                }
                                TabsTrigger {
                                    value: "usage".to_string(),
                                    "Usage"
                                }
                            }
                            TabsContent {
                                value: "preview".to_string(),
                                div { class: "rounded-lg border border-border/30 bg-muted/40 p-4 text-xs text-muted-foreground",
                                    "Active tab: Preview. Render your component controls here."
                                }
                            }
                            TabsContent {
                                value: "code".to_string(),
                                div { class: "rounded-lg border border-border/30 bg-muted/40 p-4 font-mono text-xs text-muted-foreground",
                                    "Tabs {{ value: \"preview\", ... }}"
                                }
                            }
                            TabsContent {
                                value: "usage".to_string(),
                                div { class: "rounded-lg border border-border/30 bg-muted/40 p-4 text-xs text-muted-foreground",
                                    "Use for switching between segmented content panels."
                                }
                            }
                        }
                    }

                    ComponentCard {
                        title: "Badges & Toggles".to_string(),
                        description: "Status indicators, tag pills, and interactive switch controls.".to_string(),
                        div { class: "flex flex-col gap-4",
                            div { class: "flex flex-wrap items-center gap-2",
                                Badge { "Default" }
                                Badge { class: "bg-emerald-500/15 text-emerald-600 dark:text-emerald-400 border border-emerald-500/20", "Success" }
                                Badge { class: "bg-amber-500/15 text-amber-600 dark:text-amber-400 border border-amber-500/20", "Pending" }
                                Badge { class: "bg-rose-500/15 text-rose-600 dark:text-rose-400 border border-rose-500/20", "Urgent" }
                            }
                            div { class: "flex items-center justify-between rounded-lg border border-border/30 bg-muted/30 p-3",
                                div { class: "flex flex-col",
                                    span { class: "text-xs font-medium text-foreground", "Enable feature flag" }
                                    span { class: "text-[11px] text-muted-foreground", "Toggle live status in reactive state" }
                                }
                                Switch {
                                    checked: switch_enabled,
                                }
                            }
                        }
                    }

                    ComponentCard {
                        title: "Avatars & User Representation".to_string(),
                        description: "User profile imagery with fallback initials and group clustering.".to_string(),
                        div { class: "flex items-center gap-3",
                            Avatar { class: "size-10",
                                AvatarFallback { "JD" }
                            }
                            Avatar { class: "size-10 bg-primary/10 text-primary border border-primary/20",
                                AvatarFallback { "TK" }
                            }
                            Avatar { class: "size-10 bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 border border-emerald-500/20",
                                AvatarFallback { "AI" }
                            }
                            div { class: "flex flex-col gap-0.5 pl-2",
                                span { class: "text-xs font-semibold text-foreground", "Team Members" }
                                span { class: "text-[11px] text-muted-foreground", "3 active contributors" }
                            }
                        }
                    }
                }
            }
        }
    }
}
