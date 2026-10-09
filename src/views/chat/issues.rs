use dioxus::prelude::*;
use crate::components::ui::{
    Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator, DropdownMenuTrigger,
};
use crate::icons::{PlusIcon, SearchIcon, TrashIcon};

#[derive(Clone, PartialEq, Debug)]
pub enum IssueStatus {
    Open,
    InProgress,
    InReview,
    Done,
}

impl IssueStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            IssueStatus::Open => "Open",
            IssueStatus::InProgress => "In Progress",
            IssueStatus::InReview => "In Review",
            IssueStatus::Done => "Done",
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum IssuePriority {
    Urgent,
    High,
    Medium,
    Low,
}

impl IssuePriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            IssuePriority::Urgent => "Urgent",
            IssuePriority::High => "High",
            IssuePriority::Medium => "Medium",
            IssuePriority::Low => "Low",
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct IssueItem {
    pub id: String,
    pub key: String,
    pub title: String,
    pub description: String,
    pub status: IssueStatus,
    pub priority: IssuePriority,
    pub assignee_name: String,
    pub assignee_initials: String,
    pub comments_count: usize,
    pub label: String,
    pub created_at: String,
}

fn sample_issues() -> Vec<IssueItem> {
    vec![
        IssueItem {
            id: "iss-1".to_string(),
            key: "TSK-201".to_string(),
            title: "Fix WebSocket reconnect jitter on unstable network connections".to_string(),
            description: "When the client loses connectivity momentarily, the retry backoff causes unnecessary socket reconnect spam.".to_string(),
            status: IssueStatus::InProgress,
            priority: IssuePriority::Urgent,
            assignee_name: "Jacquenetta Slowgrave".to_string(),
            assignee_initials: "JS".to_string(),
            comments_count: 5,
            label: "Bug".to_string(),
            created_at: "2 hours ago".to_string(),
        },
        IssueItem {
            id: "iss-2".to_string(),
            key: "TSK-198".to_string(),
            title: "Design and implement Dark Mode contrast tokens for chat bubbles".to_string(),
            description: "Ensure that text contrast complies with WCAG AAA standards across all theme modes.".to_string(),
            status: IssueStatus::InReview,
            priority: IssuePriority::High,
            assignee_name: "Nickola Peever".to_string(),
            assignee_initials: "NP".to_string(),
            comments_count: 12,
            label: "UI/UX".to_string(),
            created_at: "Yesterday".to_string(),
        },
        IssueItem {
            id: "iss-3".to_string(),
            key: "TSK-194".to_string(),
            title: "Add offline cache persistence with IndexedDB for instant reload".to_string(),
            description: "Store recent chat channels and emails in local storage so users see immediate data without waiting for network fetches.".to_string(),
            status: IssueStatus::Open,
            priority: IssuePriority::High,
            assignee_name: "Farand Hume".to_string(),
            assignee_initials: "FH".to_string(),
            comments_count: 3,
            label: "Feature".to_string(),
            created_at: "3 days ago".to_string(),
        },
        IssueItem {
            id: "iss-4".to_string(),
            key: "TSK-189".to_string(),
            title: "Optimize SVG icon bundle size and remove redundant lucide imports".to_string(),
            description: "Bundle analysis showed unused icon variants inflating the binary footprint.".to_string(),
            status: IssueStatus::Done,
            priority: IssuePriority::Medium,
            assignee_name: "Ossie Peasey".to_string(),
            assignee_initials: "OP".to_string(),
            comments_count: 2,
            label: "Performance".to_string(),
            created_at: "5 days ago".to_string(),
        },
        IssueItem {
            id: "iss-5".to_string(),
            key: "TSK-176".to_string(),
            title: "Add audio recording waveform visualization in chat composer".to_string(),
            description: "Give real-time visual feedback while the user is recording a voice note memo.".to_string(),
            status: IssueStatus::Open,
            priority: IssuePriority::Low,
            assignee_name: "Hall Negri".to_string(),
            assignee_initials: "HN".to_string(),
            comments_count: 0,
            label: "UI/UX".to_string(),
            created_at: "1 week ago".to_string(),
        },
        IssueItem {
            id: "iss-6".to_string(),
            key: "TSK-162".to_string(),
            title: "Support multi-file drag-and-drop uploads in email composition".to_string(),
            description: "Allow users to drop attachments directly anywhere in the email reading pane or compose modal.".to_string(),
            status: IssueStatus::Done,
            priority: IssuePriority::Medium,
            assignee_name: "You".to_string(),
            assignee_initials: "ME".to_string(),
            comments_count: 8,
            label: "Feature".to_string(),
            created_at: "2 weeks ago".to_string(),
        },
    ]
}

#[component]
pub fn ChatIssues() -> Element {
    let mut issues = use_signal(sample_issues);
    let mut search_query = use_signal(String::new);
    let mut status_filter = use_signal(|| "all".to_string());
    let mut priority_filter = use_signal(|| "all".to_string());
    let mut new_issue_open = use_signal(|| false);
    let mut selected_issue_id = use_signal(|| Option::<String>::None);

    // New issue form signals
    let mut form_title = use_signal(String::new);
    let mut form_desc = use_signal(String::new);
    let mut form_label = use_signal(|| "Feature".to_string());
    let mut form_priority = use_signal(|| "High".to_string());

    let query = search_query.read().to_lowercase();
    let current_status = status_filter();
    let current_priority = priority_filter();

    let filtered_issues: Vec<IssueItem> = issues
        .read()
        .iter()
        .filter(|item| {
            let matches_status = match current_status.as_str() {
                "open" => item.status == IssueStatus::Open,
                "in_progress" => item.status == IssueStatus::InProgress,
                "in_review" => item.status == IssueStatus::InReview,
                "done" => item.status == IssueStatus::Done,
                _ => true,
            };

            let matches_priority = match current_priority.as_str() {
                "urgent" => item.priority == IssuePriority::Urgent,
                "high" => item.priority == IssuePriority::High,
                "medium" => item.priority == IssuePriority::Medium,
                "low" => item.priority == IssuePriority::Low,
                _ => true,
            };

            let matches_search = query.is_empty()
                || item.title.to_lowercase().contains(&query)
                || item.key.to_lowercase().contains(&query)
                || item.assignee_name.to_lowercase().contains(&query)
                || item.description.to_lowercase().contains(&query);

            matches_status && matches_priority && matches_search
        })
        .cloned()
        .collect();

    let open_count = issues.read().iter().filter(|i| i.status == IssueStatus::Open).count();
    let in_progress_count = issues.read().iter().filter(|i| i.status == IssueStatus::InProgress).count();
    let in_review_count = issues.read().iter().filter(|i| i.status == IssueStatus::InReview).count();
    let done_count = issues.read().iter().filter(|i| i.status == IssueStatus::Done).count();

    let selected_issue = selected_issue_id()
        .and_then(|id| issues.read().iter().find(|i| i.id == id).cloned());

    rsx! {
        div { class: "flex h-full w-full flex-col overflow-hidden bg-background select-none",

            // Top Header: Title + Stats + New Issue
            div { class: "flex flex-col gap-4 border-b border-border/40 p-4 sm:p-6 bg-card/20 shrink-0",
                div { class: "flex flex-wrap items-center justify-between gap-4",
                    div { class: "flex items-center gap-3",
                        div { class: "flex size-10 items-center justify-center rounded-xl bg-primary/10 border border-primary/20 text-primary shadow-xs",
                            svg { class: "size-5", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                circle { cx: "12", cy: "12", r: "10" }
                                line { x1: "12", y1: "8", x2: "12", y2: "12" }
                                line { x1: "12", y1: "16", x2: "12.01", y2: "16" }
                            }
                        }
                        div {
                            h1 { class: "text-lg sm:text-xl font-bold tracking-tight text-foreground", "Issue Tracker" }
                            p { class: "text-xs text-muted-foreground", "Track, prioritize, and resolve team tickets and bugs" }
                        }
                    }

                    button {
                        r#type: "button",
                        class: "inline-flex items-center gap-2 rounded-xl bg-primary px-4 py-2 text-xs font-semibold text-primary-foreground shadow-xs hover:bg-primary/90 active:scale-98 transition-all cursor-pointer",
                        onclick: move |_| new_issue_open.set(true),
                        PlusIcon { class: "size-3.5" }
                        span { "New Issue" }
                    }
                }

                // Controls bar: Search + Status Tabs + Priority Filter
                div { class: "flex flex-wrap items-center justify-between gap-3 pt-1",
                    // Status filter tabs
                    div { class: "flex items-center rounded-xl bg-muted/80 p-1 gap-1 text-xs font-medium overflow-x-auto",
                        StatusTab {
                            label: "All",
                            count: issues.read().len(),
                            is_active: status_filter() == "all",
                            onclick: move |_| status_filter.set("all".to_string()),
                        }
                        StatusTab {
                            label: "Open",
                            count: open_count,
                            is_active: status_filter() == "open",
                            onclick: move |_| status_filter.set("open".to_string()),
                        }
                        StatusTab {
                            label: "In Progress",
                            count: in_progress_count,
                            is_active: status_filter() == "in_progress",
                            onclick: move |_| status_filter.set("in_progress".to_string()),
                        }
                        StatusTab {
                            label: "In Review",
                            count: in_review_count,
                            is_active: status_filter() == "in_review",
                            onclick: move |_| status_filter.set("in_review".to_string()),
                        }
                        StatusTab {
                            label: "Done",
                            count: done_count,
                            is_active: status_filter() == "done",
                            onclick: move |_| status_filter.set("done".to_string()),
                        }
                    }

                    // Search input & Priority filter
                    div { class: "flex items-center gap-2 min-w-0 flex-1 sm:flex-initial justify-end",
                        div { class: "relative min-w-44 sm:min-w-64",
                            SearchIcon { class: "absolute left-3 top-1/2 size-3.5 -translate-y-1/2 text-muted-foreground/60 pointer-events-none" }
                            input {
                                r#type: "text",
                                class: "w-full rounded-xl border border-border/50 bg-background/70 pl-8.5 pr-3 py-1.5 text-xs text-foreground placeholder:text-muted-foreground/60 focus:outline-none focus:ring-1 focus:ring-primary/40 transition-all",
                                placeholder: "Filter issues by title or #tag...",
                                value: search_query(),
                                oninput: move |e| search_query.set(e.value()),
                            }
                        }

                        select {
                            class: "rounded-xl border border-border/50 bg-background/70 px-3 py-1.5 text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-primary/40 cursor-pointer",
                            value: priority_filter(),
                            onchange: move |e| priority_filter.set(e.value()),
                            option { value: "all", "All Priorities" }
                            option { value: "urgent", "🔥 Urgent" }
                            option { value: "high", "⚡ High" }
                            option { value: "medium", "Medium" }
                            option { value: "low", "Low" }
                        }
                    }
                }
            }

            // Main Content: Issues List + Detail Panel
            div { class: "flex-1 min-h-0 flex overflow-hidden",

                // Issue Rows List
                div { class: "flex-1 min-w-0 overflow-y-auto p-4 sm:p-6 space-y-2.5",
                    if filtered_issues.is_empty() {
                        div { class: "py-20 flex flex-col items-center justify-center gap-3 text-center text-muted-foreground",
                            div { class: "size-12 rounded-2xl bg-muted/60 flex items-center justify-center text-muted-foreground/50",
                                svg { class: "size-6", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                                    circle { cx: "12", cy: "12", r: "10" }
                                    path { d: "m9 12 2 2 4-4" }
                                }
                            }
                            div { class: "space-y-1",
                                h4 { class: "text-sm font-semibold text-foreground", "No issues match your filter" }
                                p { class: "text-xs text-muted-foreground", "Try clearing your search query or selecting a different status tab." }
                            }
                        }
                    }

                    for issue in filtered_issues {
                        {
                            let iss = issue.clone();
                            let is_selected = selected_issue_id().as_deref() == Some(iss.id.as_str());

                            rsx! {
                                IssueRow {
                                    key: "{iss.id}",
                                    issue: iss,
                                    is_selected: is_selected,
                                    on_select: move |id: String| {
                                        selected_issue_id.set(Some(id));
                                    },
                                    on_toggle_done: move |id: String| {
                                        let mut list = issues.write();
                                        if let Some(item) = list.iter_mut().find(|i| i.id == id) {
                                            item.status = if item.status == IssueStatus::Done {
                                                IssueStatus::Open
                                            } else {
                                                IssueStatus::Done
                                            };
                                        }
                                    },
                                    on_delete: move |id: String| {
                                        issues.write().retain(|i| i.id != id);
                                        if selected_issue_id() == Some(id) {
                                            selected_issue_id.set(None);
                                        }
                                    },
                                }
                            }
                        }
                    }
                }

                // Issue Detail Drawer (if selected)
                if let Some(detail) = selected_issue {
                    {
                        let detail_id_toggle = detail.id.clone();
                        let detail_id_delete = detail.id.clone();

                        rsx! {
                            div { class: "w-80 sm:w-96 border-l border-border/40 bg-card/50 backdrop-blur p-5 overflow-y-auto flex flex-col gap-5 shrink-0 animate-in slide-in-from-right-4 duration-150",
                                div { class: "flex items-center justify-between pb-3 border-b border-border/40",
                                    span { class: "font-mono text-xs font-bold text-primary", "#{detail.key}" }
                                    button {
                                        r#type: "button",
                                        class: "text-muted-foreground hover:text-foreground text-xs cursor-pointer",
                                        onclick: move |_| selected_issue_id.set(None),
                                        "✕"
                                    }
                                }

                                div { class: "space-y-2",
                                    h3 { class: "text-base font-bold text-foreground leading-snug", "{detail.title}" }
                                    p { class: "text-xs text-muted-foreground leading-relaxed", "{detail.description}" }
                                }

                                div { class: "space-y-3 pt-3 border-t border-border/40 text-xs",
                                    div { class: "flex justify-between items-center",
                                        span { class: "text-muted-foreground", "Status" }
                                        span { class: "font-semibold text-foreground px-2 py-0.5 rounded-md bg-muted", "{detail.status.as_str()}" }
                                    }
                                    div { class: "flex justify-between items-center",
                                        span { class: "text-muted-foreground", "Priority" }
                                        span { class: "font-semibold text-foreground", "{detail.priority.as_str()}" }
                                    }
                                    div { class: "flex justify-between items-center",
                                        span { class: "text-muted-foreground", "Assignee" }
                                        span { class: "font-semibold text-foreground", "{detail.assignee_name}" }
                                    }
                                    div { class: "flex justify-between items-center",
                                        span { class: "text-muted-foreground", "Label" }
                                        span { class: "font-semibold text-primary", "🏷️ {detail.label}" }
                                    }
                                    div { class: "flex justify-between items-center",
                                        span { class: "text-muted-foreground", "Created" }
                                        span { class: "text-muted-foreground", "{detail.created_at}" }
                                    }
                                }

                                div { class: "pt-4 border-t border-border/40 flex items-center justify-between",
                                    button {
                                        r#type: "button",
                                        class: "rounded-xl border border-border px-3 py-1.5 text-xs font-medium text-foreground hover:bg-muted cursor-pointer",
                                        onclick: move |_| {
                                            let mut list = issues.write();
                                            if let Some(item) = list.iter_mut().find(|i| i.id == detail_id_toggle) {
                                                item.status = if item.status == IssueStatus::Done {
                                                    IssueStatus::Open
                                                } else {
                                                    IssueStatus::Done
                                                };
                                            }
                                        },
                                        if detail.status == IssueStatus::Done { "Reopen Issue" } else { "Mark as Done" }
                                    }

                                    button {
                                        r#type: "button",
                                        class: "rounded-xl border border-destructive/40 bg-destructive/10 px-3 py-1.5 text-xs font-medium text-destructive hover:bg-destructive/20 cursor-pointer",
                                        onclick: move |_| {
                                            issues.write().retain(|i| i.id != detail_id_delete);
                                            selected_issue_id.set(None);
                                        },
                                        "Delete Issue"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // New Issue Modal
            Dialog {
                open: new_issue_open,
                DialogContent {
                    DialogHeader {
                        DialogTitle { "Create New Issue" }
                        DialogDescription { "Create a new bug report, feature request, or task for the team." }
                    }

                    div { class: "py-3 space-y-3",
                        div { class: "space-y-1",
                            label { class: "text-xs font-semibold text-foreground", "Title" }
                            input {
                                r#type: "text",
                                class: "w-full rounded-xl border border-border bg-background px-3 py-1.5 text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-primary",
                                placeholder: "e.g. Memory leak in background worker thread",
                                value: form_title(),
                                oninput: move |e| form_title.set(e.value()),
                            }
                        }

                        div { class: "grid grid-cols-2 gap-3",
                            div { class: "space-y-1",
                                label { class: "text-xs font-semibold text-foreground", "Label" }
                                select {
                                    class: "w-full rounded-xl border border-border bg-background px-3 py-1.5 text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-primary",
                                    value: form_label(),
                                    onchange: move |e| form_label.set(e.value()),
                                    option { "Bug" }
                                    option { "Feature" }
                                    option { "UI/UX" }
                                    option { "Performance" }
                                    option { "Refactor" }
                                }
                            }

                            div { class: "space-y-1",
                                label { class: "text-xs font-semibold text-foreground", "Priority" }
                                select {
                                    class: "w-full rounded-xl border border-border bg-background px-3 py-1.5 text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-primary",
                                    value: form_priority(),
                                    onchange: move |e| form_priority.set(e.value()),
                                    option { "Urgent" }
                                    option { "High" }
                                    option { "Medium" }
                                    option { "Low" }
                                }
                            }
                        }

                        div { class: "space-y-1",
                            label { class: "text-xs font-semibold text-foreground", "Description" }
                            textarea {
                                class: "w-full rounded-xl border border-border bg-background p-3 text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-primary resize-none",
                                rows: "4",
                                placeholder: "Add details, steps to reproduce, or acceptance criteria...",
                                value: form_desc(),
                                oninput: move |e| form_desc.set(e.value()),
                            }
                        }
                    }

                    DialogFooter {
                        button {
                            r#type: "button",
                            class: "h-8 rounded-lg border border-border px-3 text-xs font-medium text-foreground hover:bg-muted cursor-pointer",
                            onclick: move |_| new_issue_open.set(false),
                            "Cancel"
                        }
                        button {
                            r#type: "button",
                            class: "h-8 rounded-lg bg-primary px-4 text-xs font-medium text-primary-foreground hover:opacity-90 cursor-pointer shadow-xs",
                            onclick: move |_| {
                                let title = form_title.read().trim().to_string();
                                let desc = form_desc.read().trim().to_string();
                                if !title.is_empty() {
                                    let prio = match form_priority.read().as_str() {
                                        "Urgent" => IssuePriority::Urgent,
                                        "High" => IssuePriority::High,
                                        "Low" => IssuePriority::Low,
                                        _ => IssuePriority::Medium,
                                    };
                                    let next_num = issues.read().len() + 205;
                                    let new_item = IssueItem {
                                        id: format!("iss-{}", next_num),
                                        key: format!("TSK-{}", next_num),
                                        title,
                                        description: if desc.is_empty() { "No description provided.".to_string() } else { desc },
                                        status: IssueStatus::Open,
                                        priority: prio,
                                        assignee_name: "You".to_string(),
                                        assignee_initials: "ME".to_string(),
                                        comments_count: 0,
                                        label: form_label.read().clone(),
                                        created_at: "Just now".to_string(),
                                    };
                                    issues.write().insert(0, new_item);
                                    form_title.set(String::new());
                                    form_desc.set(String::new());
                                    new_issue_open.set(false);
                                }
                            },
                            "Create Issue"
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct StatusTabProps {
    label: &'static str,
    count: usize,
    is_active: bool,
    onclick: EventHandler<()>,
}

#[component]
fn StatusTab(props: StatusTabProps) -> Element {
    let active_class = if props.is_active {
        "bg-background text-foreground shadow-xs font-semibold"
    } else {
        "text-muted-foreground hover:text-foreground hover:bg-muted/60"
    };

    rsx! {
        button {
            r#type: "button",
            class: "flex items-center gap-1.5 px-3 py-1.5 rounded-lg transition-all cursor-pointer {active_class}",
            onclick: move |_| props.onclick.call(()),
            span { "{props.label}" }
            span { class: "rounded-full bg-muted/80 px-1.5 py-0.2 text-[10px] text-muted-foreground font-semibold",
                "{props.count}"
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct IssueRowProps {
    issue: IssueItem,
    is_selected: bool,
    on_select: EventHandler<String>,
    on_toggle_done: EventHandler<String>,
    on_delete: EventHandler<String>,
}

#[component]
fn IssueRow(props: IssueRowProps) -> Element {
    let issue = props.issue;
    let id = issue.id.clone();
    let id_toggle = issue.id.clone();
    let id_menu_toggle = issue.id.clone();
    let id_menu_del = issue.id.clone();
    let on_select = props.on_select;
    let on_toggle_done = props.on_toggle_done;
    let on_delete = props.on_delete;

    let is_done = issue.status == IssueStatus::Done;

    let row_class = if props.is_selected {
        "flex items-center justify-between gap-3 p-3.5 rounded-2xl border border-primary/40 bg-primary/5 shadow-xs transition-all cursor-pointer"
    } else {
        "flex items-center justify-between gap-3 p-3.5 rounded-2xl border border-border/50 bg-card/70 hover:bg-muted/40 hover:border-border transition-all cursor-pointer"
    };

    let priority_badge = match issue.priority {
        IssuePriority::Urgent => rsx! {
            span { class: "rounded-md bg-destructive/15 border border-destructive/30 px-2 py-0.5 text-[10px] font-bold text-destructive",
                "🔥 Urgent"
            }
        },
        IssuePriority::High => rsx! {
            span { class: "rounded-md bg-amber-500/15 border border-amber-500/30 px-2 py-0.5 text-[10px] font-bold text-amber-500",
                "⚡ High"
            }
        },
        IssuePriority::Medium => rsx! {
            span { class: "rounded-md bg-blue-500/15 border border-blue-500/30 px-2 py-0.5 text-[10px] font-semibold text-blue-500",
                "Medium"
            }
        },
        IssuePriority::Low => rsx! {
            span { class: "rounded-md bg-muted border border-border/50 px-2 py-0.5 text-[10px] font-medium text-muted-foreground",
                "Low"
            }
        },
    };

    let status_icon = match issue.status {
        IssueStatus::Done => rsx! {
            span { class: "flex size-5 items-center justify-center rounded-full bg-emerald-500 text-black shadow-xs",
                svg { class: "size-3.5", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "3",
                    path { d: "M20 6 9 17l-5-5" }
                }
            }
        },
        IssueStatus::InProgress => rsx! {
            span { class: "flex size-5 items-center justify-center rounded-full bg-amber-500/20 text-amber-500",
                svg { class: "size-3 animate-spin", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.5",
                    path { d: "M21 12a9 9 0 1 1-6.219-8.56" }
                }
            }
        },
        IssueStatus::InReview => rsx! {
            span { class: "flex size-5 items-center justify-center rounded-full bg-purple-500/20 text-purple-500",
                svg { class: "size-3", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.5",
                    circle { cx: "12", cy: "12", r: "8" }
                }
            }
        },
        IssueStatus::Open => rsx! {
            span { class: "flex size-5 items-center justify-center rounded-full border-2 border-muted-foreground/40 hover:border-primary transition-colors",
            }
        },
    };

    rsx! {
        div {
            class: row_class,
            onclick: move |_| on_select.call(id.clone()),

            // Left: Status check button + Key + Title
            div { class: "flex items-center gap-3 min-w-0 flex-1",
                button {
                    r#type: "button",
                    class: "shrink-0 cursor-pointer",
                    title: if is_done { "Reopen" } else { "Mark as Done" },
                    onclick: move |e| {
                        e.stop_propagation();
                        on_toggle_done.call(id_toggle.clone());
                    },
                    {status_icon}
                }

                div { class: "flex flex-col min-w-0 gap-0.5",
                    div { class: "flex items-center gap-2 min-w-0",
                        span { class: "font-mono text-xs font-bold text-muted-foreground shrink-0", "#{issue.key}" }
                        span {
                            class: format!(
                                "truncate text-xs sm:text-sm font-medium {}",
                                if is_done { "line-through text-muted-foreground/70" } else { "text-foreground" }
                            ),
                            "{issue.title}"
                        }
                    }
                    div { class: "flex items-center gap-2 text-[11px] text-muted-foreground",
                        span { class: "rounded-md bg-primary/10 px-1.5 py-0.2 text-primary font-medium", "🏷️ {issue.label}" }
                        span { "•" }
                        span { "{issue.created_at}" }
                        if issue.comments_count > 0 {
                            span { "•" }
                            span { class: "flex items-center gap-0.5",
                                svg { class: "size-3", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                                    path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
                                }
                                "{issue.comments_count}"
                            }
                        }
                    }
                }
            }

            // Right: Priority + Assignee + Options
            div { class: "flex items-center gap-2.5 shrink-0",
                {priority_badge}

                div {
                    class: "flex size-7 items-center justify-center rounded-full bg-muted border border-border/50 text-xs font-bold text-foreground",
                    title: "Assigned to {issue.assignee_name}",
                    "{issue.assignee_initials}"
                }

                DropdownMenu {
                    DropdownMenuTrigger {
                        class: "inline-flex size-7 items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer",
                        svg { class: "size-3.5", view_box: "0 0 24 24", fill: "currentColor",
                            circle { cx: "12", cy: "5", r: "1.5" }
                            circle { cx: "12", cy: "12", r: "1.5" }
                            circle { cx: "12", cy: "19", r: "1.5" }
                        }
                    }
                    DropdownMenuContent {
                        align: "end",
                        side_offset: 6,
                        DropdownMenuItem {
                            onclick: move |_| on_toggle_done.call(id_menu_toggle.clone()),
                            if is_done { "Mark as In Progress" } else { "Mark as Done" }
                        }
                        DropdownMenuSeparator {}
                        DropdownMenuItem {
                            variant: "destructive",
                            onclick: move |_| on_delete.call(id_menu_del.clone()),
                            TrashIcon { class: "size-3.5" }
                            span { "Delete issue" }
                        }
                    }
                }
            }
        }
    }
}
