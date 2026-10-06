use dioxus::prelude::*;

use crate::components::ui::Badge;
use crate::views::PageHeader;

struct Issue {
    number: &'static str,
    title: &'static str,
    label: &'static str,
    variant: &'static str,
    open: bool,
    author: &'static str,
    time: &'static str,
}

#[component]
pub fn ChatIssues() -> Element {
    let issues = [
        Issue {
            number: "#124",
            title: "Popup renders behind dialog when sidebar is transformed",
            label: "Bug",
            variant: "destructive",
            open: true,
            author: "alice",
            time: "2h ago",
        },
        Issue {
            number: "#121",
            title: "Add keyboard navigation to command palette",
            label: "Enhancement",
            variant: "secondary",
            open: true,
            author: "marc",
            time: "5h ago",
        },
        Issue {
            number: "#118",
            title: "Document the popup positioning helper",
            label: "Docs",
            variant: "outline",
            open: false,
            author: "shadcn",
            time: "Yesterday",
        },
        Issue {
            number: "#115",
            title: "Analytics bars overflow on narrow windows",
            label: "Bug",
            variant: "destructive",
            open: true,
            author: "alice",
            time: "2d ago",
        },
        Issue {
            number: "#110",
            title: "Support drag & drop between task columns",
            label: "Feature",
            variant: "default",
            open: false,
            author: "marc",
            time: "4d ago",
        },
    ];

    rsx! {
        div { class: "flex flex-col",
            PageHeader {
                title: "Issues",
                description: "Track bugs and feature requests from your repositories.",
                actions: rsx! {
                    div { class: "flex items-center gap-2 pt-2",
                        Badge { variant: "secondary", "3 open" }
                    }
                },
            }

            div { class: "flex flex-col gap-2",
                for issue in issues.iter() {
                    {
                        let status_class = if issue.open {
                            "text-emerald-500"
                        } else {
                            "text-muted-foreground"
                        };
                        rsx! {
                            div {
                                key: "{issue.number}",
                                class: "flex cursor-pointer items-center gap-3 rounded-xl border border-border/40 bg-card p-3.5 shadow-xs transition-colors hover:bg-accent/50",
                                div { class: "flex size-8 shrink-0 items-center justify-center {status_class}",
                                    if issue.open {
                                        svg {
                                            class: "size-4",
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            stroke_width: "2",
                                            circle { cx: "12", cy: "12", r: "9" }
                                            path { d: "M12 7v5l3 2" }
                                        }
                                    } else {
                                        svg {
                                            class: "size-4",
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            stroke_width: "2",
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            circle { cx: "12", cy: "12", r: "9" }
                                            path { d: "m9 12 2 2 4-4" }
                                        }
                                    }
                                }
                                div { class: "flex min-w-0 flex-1 flex-col gap-1",
                                    div { class: "flex items-center gap-2",
                                        span { class: "text-xs font-medium text-muted-foreground",
                                            "{issue.number}"
                                        }
                                        span { class: "truncate text-sm font-medium text-foreground",
                                            "{issue.title}"
                                        }
                                    }
                                    div { class: "flex items-center gap-2",
                                        Badge {
                                            variant: issue.variant,
                                            size: "sm",
                                            "{issue.label}"
                                        }
                                        span { class: "text-[10px] text-muted-foreground",
                                            "{issue.author} · {issue.time}"
                                        }
                                    }
                                }
                                if issue.open {
                                    Badge { variant: "outline", size: "sm", "Open" }
                                } else {
                                    Badge { variant: "outline", size: "sm", "Closed" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
