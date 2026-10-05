use dioxus::prelude::*;

use crate::components::ui::{
    Avatar, AvatarFallback, AvatarImage, Badge, Breadcrumb, BreadcrumbItem, BreadcrumbLink,
    BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator, Calendar, Collapsible, CollapsibleContent,
    CollapsibleTrigger, Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter,
    DialogHeader, DialogTitle, DialogTrigger, Input, Item, ItemActions, ItemContent,
    ItemDescription, ItemMedia, ItemTitle, Select, SelectContent, SelectItem, SelectTrigger,
    SelectValue, Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent,
    SidebarGroupLabel, SidebarHeader, SidebarMenu, SidebarMenuBadge, SidebarMenuButton,
    SidebarMenuItem, SidebarProvider, SidebarRail, SidebarTrigger, Switch, Tooltip, TooltipContent,
    TooltipProvider, TooltipTrigger,
};
use crate::icons::{MoonIcon, SunIcon};
use crate::state::{use_app_state, TaskPriority, ThemeMode};

#[component]
pub fn Home() -> Element {
    let mut app_state = use_app_state();
    let mut new_task_title = use_signal(String::new);
    let mut new_task_project = use_signal(|| "Rust Desktop".to_string());
    let mut new_task_priority = use_signal(|| TaskPriority::High);

    let is_dark = app_state.is_dark();
    let pending_count = app_state.pending_tasks_count();
    let filtered_tasks = app_state.filtered_tasks();
    let selected_date = app_state.selected_date;

    rsx! {
        TooltipProvider {
            SidebarProvider {
                default_open: true,

                // ============================================================
                // Left Collapsible Sidebar
                // ============================================================
                Sidebar {
                    SidebarHeader {
                        div {
                            class: "flex items-center gap-2 min-w-0 cursor-pointer select-none",
                            img {
                                src: asset!("/assets/logo.png"),
                                class: "size-6 pointer-events-none shrink-0",
                            }
                            span {
                                class: "sidebar-text font-semibold text-sm tracking-tight truncate",
                                "Taskify"
                            }
                        }
                        div {
                            class: "sidebar-text ml-auto",
                            SidebarTrigger {}
                        }
                    }

                    SidebarContent {
                        // Overview Group
                        SidebarGroup {
                            SidebarGroupLabel { "Workspace" }
                            SidebarGroupContent {
                                SidebarMenu {
                                    SidebarMenuItem {
                                        SidebarMenuButton {
                                            tooltip: "Dashboard".to_string(),
                                            active: *app_state.active_nav.read() == "dashboard",
                                            onclick: move |_| {
                                                app_state.active_nav.set("dashboard".to_string());
                                                app_state.selected_project.set("All Projects".to_string());
                                            },
                                            svg {
                                                view_box: "0 0 24 24",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "2",
                                                rect { width: "7", height: "9", x: "3", y: "3", rx: "1" }
                                                rect { width: "7", height: "5", x: "14", y: "3", rx: "1" }
                                                rect { width: "7", height: "9", x: "14", y: "12", rx: "1" }
                                                rect { width: "7", height: "5", x: "3", y: "16", rx: "1" }
                                            }
                                            span { "Dashboard" }
                                        }
                                    }
                                    SidebarMenuItem {
                                        SidebarMenuButton {
                                            tooltip: format!("My Tasks ({pending_count})"),
                                            active: *app_state.active_nav.read() == "tasks",
                                            onclick: move |_| app_state.active_nav.set("tasks".to_string()),
                                            svg {
                                                view_box: "0 0 24 24",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "2",
                                                path { d: "M9 11l3 3L22 4" }
                                                path { d: "M21 12v7a2 2 0 01-2 2H5a2 2 0 01-2-2V5a2 2 0 012-2h11" }
                                            }
                                            span { "My Tasks" }
                                            SidebarMenuBadge { "{pending_count}" }
                                        }
                                    }
                                    SidebarMenuItem {
                                        SidebarMenuButton {
                                            tooltip: "Schedule".to_string(),
                                            active: *app_state.active_nav.read() == "calendar",
                                            onclick: move |_| app_state.active_nav.set("calendar".to_string()),
                                            svg {
                                                view_box: "0 0 24 24",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "2",
                                                rect { width: "18", height: "18", x: "3", y: "4", rx: "2" }
                                                path { d: "M16 2v4M8 2v4M3 10h18" }
                                            }
                                            span { "Schedule" }
                                        }
                                    }
                                }
                            }
                        }

                        // Projects Group
                        SidebarGroup {
                            SidebarGroupLabel { "Projects" }
                            SidebarGroupContent {
                                SidebarMenu {
                                    SidebarMenuItem {
                                        SidebarMenuButton {
                                            tooltip: "Rust Desktop".to_string(),
                                            active: *app_state.selected_project.read() == "Rust Desktop",
                                            onclick: move |_| {
                                                app_state.selected_project.set("Rust Desktop".to_string());
                                                app_state.active_nav.set("tasks".to_string());
                                            },
                                            span { class: "sidebar-icon size-2 rounded-full bg-orange-500 shrink-0" }
                                            span { "Rust Desktop" }
                                        }
                                    }
                                    SidebarMenuItem {
                                        SidebarMenuButton {
                                            tooltip: "UI Components".to_string(),
                                            active: *app_state.selected_project.read() == "UI Components",
                                            onclick: move |_| {
                                                app_state.selected_project.set("UI Components".to_string());
                                                app_state.active_nav.set("tasks".to_string());
                                            },
                                            span { class: "sidebar-icon size-2 rounded-full bg-blue-500 shrink-0" }
                                            span { "UI Components" }
                                        }
                                    }
                                    SidebarMenuItem {
                                        SidebarMenuButton {
                                            tooltip: "Release v1.0".to_string(),
                                            active: *app_state.selected_project.read() == "Release v1.0",
                                            onclick: move |_| {
                                                app_state.selected_project.set("Release v1.0".to_string());
                                                app_state.active_nav.set("tasks".to_string());
                                            },
                                            span { class: "sidebar-icon size-2 rounded-full bg-emerald-500 shrink-0" }
                                            span { "Release v1.0" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // User Profile Footer
                    SidebarFooter {
                        div {
                            class: "flex items-center gap-3 p-1 rounded-lg hover:bg-sidebar-accent/50 cursor-pointer transition-colors group-data-[collapsible=icon]:justify-center",
                            Avatar {
                                class: "size-8 border border-border shrink-0",
                                AvatarImage {
                                    src: "https://images.unsplash.com/photo-1534528741775-53994a69daeb?w=100&auto=format&fit=crop&q=80".to_string(),
                                    alt: "User Avatar".to_string(),
                                }
                                AvatarFallback { "TD" }
                            }
                            div {
                                class: "sidebar-text flex flex-col min-w-0 flex-1",
                                span { class: "text-xs font-medium text-foreground truncate", "Developer" }
                                span { class: "text-[10px] text-muted-foreground truncate", "developer@taskify.rs" }
                            }
                        }
                    }

                    SidebarRail {}
                }

                // ============================================================
                // Main Dashboard Body
                // ============================================================
                div {
                    class: "flex-1 flex flex-col h-full overflow-y-auto bg-background p-6 gap-6",

                    // Top Bar: Breadcrumb + Action Controls + Theme Toggle
                    div {
                        class: "flex flex-wrap items-center justify-between gap-4 pb-4 border-b border-border/40",

                        div {
                            class: "flex items-center gap-2",
                            SidebarTrigger { class: "-ml-1" }
                            div { class: "h-4 w-px bg-border/60 mx-1" }
                            Breadcrumb {
                                BreadcrumbList {
                                    BreadcrumbItem {
                                        BreadcrumbLink {
                                            href: "#",
                                            "Taskify"
                                        }
                                    }
                                    BreadcrumbSeparator {}
                                    BreadcrumbItem {
                                        BreadcrumbLink {
                                            href: "#",
                                            "Workspace"
                                        }
                                    }
                                    BreadcrumbSeparator {}
                                    BreadcrumbItem {
                                        BreadcrumbPage { "{app_state.active_nav.read()}" }
                                    }
                                }
                            }
                        }

                        div {
                            class: "flex items-center gap-2",

                            // Global Theme Quick Toggle
                            Tooltip {
                                TooltipTrigger {
                                    button {
                                        class: "inline-flex size-8 items-center justify-center rounded-lg border border-border bg-card hover:bg-muted text-muted-foreground hover:text-foreground transition-colors cursor-pointer",
                                        onclick: move |_| {
                                            app_state.toggle_theme();
                                        },
                                        if is_dark {
                                            SunIcon { class: "size-4".to_string() }
                                        } else {
                                            MoonIcon { class: "size-4".to_string() }
                                        }
                                    }
                                }
                                TooltipContent {
                                    side: "bottom".to_string(),
                                    if is_dark { "Switch to Light Theme (Ctrl+T)" } else { "Switch to Dark Theme (Ctrl+T)" }
                                }
                            }

                            Tooltip {
                                TooltipTrigger {
                                    Badge {
                                        variant: "outline".to_string(),
                                        class: "text-xs px-2.5 py-1 gap-1.5 cursor-pointer",
                                        span { class: "size-1.5 rounded-full bg-emerald-500" }
                                        "Cloud Sync"
                                    }
                                }
                                TooltipContent {
                                    side: "bottom".to_string(),
                                    "Connected & Synced with Taskify Cloud"
                                }
                            }

                            // New Task Dialog
                            Dialog {
                                DialogTrigger {
                                    class: "bg-primary text-primary-foreground hover:bg-primary/90 px-3.5 py-1.5 gap-1.5 shadow-xs",
                                    svg {
                                        class: "size-4",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        path { d: "M12 5v14M5 12h14" }
                                    }
                                    "New Task"
                                }
                                DialogContent {
                                    DialogHeader {
                                        DialogTitle { "Create New Task" }
                                        DialogDescription {
                                            "Add a new task to your global workspace state. Realtime updates across components."
                                        }
                                    }
                                    div {
                                        class: "flex flex-col gap-4 py-2",
                                        div {
                                            class: "flex flex-col gap-1.5",
                                            label { class: "text-xs font-medium text-foreground", "Task Title" }
                                            Input {
                                                placeholder: "e.g. Implement authentication flow".to_string(),
                                                value: "{new_task_title}",
                                                oninput: move |e: FormEvent| new_task_title.set(e.value()),
                                            }
                                        }
                                        div {
                                            class: "flex flex-col gap-1.5",
                                            label { class: "text-xs font-medium text-foreground", "Project" }
                                            Select {
                                                default_value: "Rust Desktop".to_string(),
                                                onchange: move |val: String| new_task_project.set(val),
                                                SelectTrigger {
                                                    SelectValue {}
                                                }
                                                SelectContent {
                                                    SelectItem { value: "Rust Desktop".to_string(), "Rust Desktop" }
                                                    SelectItem { value: "UI Components".to_string(), "UI Components" }
                                                    SelectItem { value: "Release v1.0".to_string(), "Release v1.0" }
                                                }
                                            }
                                        }
                                        div {
                                            class: "flex flex-col gap-1.5",
                                            label { class: "text-xs font-medium text-foreground", "Priority" }
                                            Select {
                                                default_value: "High".to_string(),
                                                onchange: move |val: String| {
                                                    let p = match val.as_str() {
                                                        "Urgent" => TaskPriority::Urgent,
                                                        "High" => TaskPriority::High,
                                                        "Low" => TaskPriority::Low,
                                                        _ => TaskPriority::Medium,
                                                    };
                                                    new_task_priority.set(p);
                                                },
                                                SelectTrigger {
                                                    SelectValue {}
                                                }
                                                SelectContent {
                                                    SelectItem { value: "Urgent".to_string(), "Urgent" }
                                                    SelectItem { value: "High".to_string(), "High" }
                                                    SelectItem { value: "Medium".to_string(), "Medium" }
                                                    SelectItem { value: "Low".to_string(), "Low" }
                                                }
                                            }
                                        }
                                    }
                                    DialogFooter {
                                        DialogClose {
                                            button {
                                                r#type: "button",
                                                class: "px-4 py-2 text-sm font-medium border border-border rounded-md hover:bg-accent cursor-pointer transition-colors",
                                                "Cancel"
                                            }
                                        }
                                        DialogClose {
                                            button {
                                                r#type: "button",
                                                class: "px-4 py-2 text-sm font-medium bg-primary text-primary-foreground rounded-md hover:bg-primary/90 cursor-pointer shadow-xs transition-colors",
                                                onclick: move |_| {
                                                    let title = new_task_title.read().clone();
                                                    let proj = new_task_project.read().clone();
                                                    let prio = *new_task_priority.read();
                                                    let date = *app_state.selected_date.read();
                                                    if !title.trim().is_empty() {
                                                        app_state.add_task(
                                                            title,
                                                            "Created via global state dialog".to_string(),
                                                            proj,
                                                            prio,
                                                            date,
                                                        );
                                                        new_task_title.write().clear();
                                                    }
                                                },
                                                "Create Task"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Filter & Settings Bar (Select + Switches + Global Theme Switcher)
                    div {
                        class: "flex flex-wrap items-center justify-between gap-4 p-4 rounded-xl border border-border bg-card",

                        div {
                            class: "flex items-center gap-3 w-64",
                            Select {
                                default_value: app_state.selected_project.read().clone(),
                                onchange: move |v: String| app_state.selected_project.set(v),
                                SelectTrigger {
                                    SelectValue {}
                                }
                                SelectContent {
                                    SelectItem { value: "All Projects".to_string(), "All Projects" }
                                    SelectItem { value: "Rust Desktop".to_string(), "Rust Desktop" }
                                    SelectItem { value: "UI Components".to_string(), "UI Components" }
                                    SelectItem { value: "Release v1.0".to_string(), "Release v1.0" }
                                }
                            }
                        }

                        div {
                            class: "flex items-center gap-6",

                            div {
                                class: "flex items-center gap-2",
                                Switch {
                                    checked: app_state.show_completed,
                                    onchange: move |v| app_state.show_completed.set(v),
                                }
                                label { class: "text-xs font-medium text-muted-foreground select-none cursor-pointer", "Show Completed" }
                            }

                            div {
                                class: "flex items-center gap-2",
                                Switch {
                                    checked: app_state.auto_sync,
                                    onchange: move |v| app_state.auto_sync.set(v),
                                }
                                label { class: "text-xs font-medium text-muted-foreground select-none cursor-pointer", "Auto-Sync" }
                            }

                            // Theme Selector
                            div {
                                class: "flex items-center gap-2 border-l border-border/60 pl-4",
                                span { class: "text-xs font-medium text-muted-foreground", "Theme:" }
                                button {
                                    class: format!(
                                        "px-2.5 py-1 text-xs rounded-md font-medium cursor-pointer transition-colors {}",
                                        if is_dark { "bg-primary text-primary-foreground" } else { "text-muted-foreground hover:bg-muted" }
                                    ),
                                    onclick: move |_| app_state.set_theme(ThemeMode::Dark),
                                    "Dark"
                                }
                                button {
                                    class: format!(
                                        "px-2.5 py-1 text-xs rounded-md font-medium cursor-pointer transition-colors {}",
                                        if !is_dark { "bg-primary text-primary-foreground" } else { "text-muted-foreground hover:bg-muted" }
                                    ),
                                    onclick: move |_| app_state.set_theme(ThemeMode::Light),
                                    "Light"
                                }
                            }
                        }
                    }

                    // Collapsible Roadmap & Info Card
                    Collapsible {
                        default_open: true,
                        class: "group/collapsible flex flex-col rounded-xl border border-border bg-card p-4 transition-all shadow-xs",
                        div {
                            class: "flex items-center justify-between",
                            div {
                                class: "flex items-center gap-2",
                                span { class: "font-semibold text-sm text-foreground", "Taskify Global State & Theme Engine" }
                                Badge { variant: "secondary".to_string(), class: "text-[10px]", "Reactive Active" }
                            }
                            CollapsibleTrigger {
                                class: "inline-flex size-7 items-center justify-center rounded-md hover:bg-muted text-muted-foreground transition-transform cursor-pointer",
                                svg {
                                    class: "size-4 transition-transform duration-200 group-data-[state=open]/collapsible:rotate-180",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    path { d: "m6 9 6 6 6-6" }
                                }
                            }
                        }
                        CollapsibleContent {
                            class: "pt-3 text-xs text-muted-foreground leading-relaxed flex flex-col gap-2 border-t border-border/40 mt-3",
                            p { "• Global state management tracks reactive tasks, projects, filtering, and theme preferences across all components." }
                            p { "• Full Light/Dark theming with live document class synchronization, CSS variable cascading, and localStorage persistence." }
                            p { "• Press Ctrl+T anywhere to instantaneously toggle between sleek Dark and crisp Light modes." }
                        }
                    }

                    // Split Section: Dynamic Tasks (using Item) + Interactive Calendar
                    div {
                        class: "grid grid-cols-1 lg:grid-cols-3 gap-6",

                        // Task List (2 cols)
                        div {
                            class: "lg:col-span-2 flex flex-col gap-3",
                            div {
                                class: "flex items-center justify-between",
                                h3 { class: "text-sm font-semibold text-foreground", "Active Tasks" }
                                span { class: "text-xs text-muted-foreground", "{filtered_tasks.len()} tasks shown • {pending_count} pending" }
                            }

                            if filtered_tasks.is_empty() {
                                div {
                                    class: "flex flex-col items-center justify-center p-8 border border-dashed border-border rounded-xl text-muted-foreground text-sm",
                                    "No tasks matching current filter."
                                }
                            }

                            for task in filtered_tasks.iter() {
                                Item {
                                    key: "{task.id}",
                                    ItemMedia {
                                        button {
                                            class: format!(
                                                "size-5 rounded border flex items-center justify-center cursor-pointer transition-colors {}",
                                                if task.completed {
                                                    "bg-primary border-primary text-primary-foreground"
                                                } else {
                                                    "border-muted-foreground/40 hover:border-primary text-transparent"
                                                }
                                            ),
                                            onclick: {
                                                let task_id = task.id.clone();
                                                move |_| {
                                                    app_state.toggle_task(&task_id);
                                                }
                                            },
                                            svg {
                                                class: "size-3",
                                                view_box: "0 0 24 24",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "3",
                                                path { d: "M20 6L9 17l-5-5" }
                                            }
                                        }
                                    }
                                    ItemContent {
                                        ItemTitle {
                                            span {
                                                class: if task.completed { "line-through text-muted-foreground opacity-60" } else { "text-foreground" },
                                                "{task.title}"
                                            }
                                        }
                                        ItemDescription {
                                            "{task.description} • Project: {task.project}"
                                        }
                                    }
                                    ItemActions {
                                        Badge {
                                            variant: task.priority.badge_variant().to_string(),
                                            class: "text-[10px]",
                                            "{task.priority.as_str()}"
                                        }
                                        if task.completed {
                                            Badge {
                                                variant: "default".to_string(),
                                                class: "text-[10px]",
                                                "Done"
                                            }
                                        }
                                        button {
                                            class: "size-6 inline-flex items-center justify-center rounded text-muted-foreground hover:text-destructive hover:bg-destructive/10 transition-colors cursor-pointer ml-1",
                                            title: "Delete task",
                                            onclick: {
                                                let tid = task.id.clone();
                                                move |_| app_state.delete_task(&tid)
                                            },
                                            svg {
                                                class: "size-3.5",
                                                view_box: "0 0 24 24",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "2",
                                                path { d: "M18 6L6 18M6 6l12 12" }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Calendar Widget (1 col)
                        div {
                            class: "flex flex-col gap-3",
                            h3 { class: "text-sm font-semibold text-foreground", "Schedule" }
                            Calendar {
                                default_year: 2026,
                                default_month: 10,
                                selected: selected_date,
                                onselect: move |(y, m, d)| {
                                    app_state.selected_date.set(Some((y, m, d)));
                                },
                            }
                            if let Some((y, m, d)) = *selected_date.read() {
                                div {
                                    class: "text-xs text-muted-foreground p-3 rounded-lg border border-border bg-card",
                                    span { class: "font-medium text-foreground", "Selected Date: " }
                                    "{y}-{m:02}-{d:02}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
