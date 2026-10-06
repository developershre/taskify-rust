use dioxus::prelude::*;

use crate::state::use_app_state;
use crate::views::PageHeader;

#[component]
pub fn Projects() -> Element {
    let app_state = use_app_state();
    let router = router();

    let mut map: std::collections::BTreeMap<String, (usize, usize)> = std::collections::BTreeMap::new();
    for task in app_state.tasks.read().iter() {
        let entry = map.entry(task.project.clone()).or_insert((0, 0));
        entry.0 += 1;
        if task.completed {
            entry.1 += 1;
        }
    }

    let projects: Vec<(String, usize, usize, usize, String)> = map
        .into_iter()
        .map(|(name, (total, done))| {
            let pct = (done * 100).checked_div(total).unwrap_or(0);
            let label = if total == 1 {
                "1 task".to_string()
            } else {
                format!("{total} tasks")
            };
            (name, total, done, pct, label)
        })
        .collect();
    let count = projects.len();

    rsx! {
        div { class: "flex flex-col",
            PageHeader {
                title: "Projects",
                description: if count == 1 {
                    "1 project in this workspace".to_string()
                } else {
                    format!("{count} projects in this workspace")
                },
            }

            div { class: "grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3",
                for (name, total, done, pct, label) in projects {
                    {
                        let target = match name.as_str() {
                            "Personal" => "/projects/personal",
                            "Github Repos" => "/projects/github",
                            _ => "/tasks/all",
                        };

                        rsx! {
                            div {
                                key: "{name}",
                                class: "flex flex-col gap-3 rounded-xl border border-border/40 bg-card p-4 shadow-xs cursor-pointer hover:bg-accent/50 transition-colors",
                                onclick: move |_| {
                                    let _ = router.push(target);
                                },
                                div { class: "flex items-center gap-3",
                                    div { class: "flex size-9 shrink-0 items-center justify-center rounded-lg bg-secondary text-foreground",
                                        svg {
                                            class: "size-4",
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            stroke_width: "2",
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            path { d: "M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z" }
                                        }
                                    }
                                    div { class: "flex min-w-0 flex-1 flex-col",
                                        span { class: "truncate text-sm font-semibold text-foreground",
                                            "{name}"
                                        }
                                        span { class: "text-xs text-muted-foreground",
                                            "{label}"
                                        }
                                    }
                                    svg {
                                        class: "size-4 shrink-0 text-muted-foreground/60",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        path { d: "m9 18 6-6-6-6" }
                                    }
                                }
                                div { class: "flex flex-col gap-1.5",
                                    div { class: "flex items-center justify-between text-xs text-muted-foreground",
                                        span { "Progress" }
                                        span { "{done} of {total} completed" }
                                    }
                                    div { class: "h-1.5 w-full overflow-hidden rounded-full bg-muted",
                                        div { class: "h-full rounded-full bg-primary transition-all",
                                            style: "width: {pct}%",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
