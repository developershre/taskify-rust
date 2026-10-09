use dioxus::prelude::*;

use crate::components::ui::{
    Collapsible, CollapsibleContent, CollapsibleContext, Dialog, DialogContent, DialogDescription,
    DialogFooter, DialogHeader, DialogTitle, DialogTrigger, Sidebar, SidebarContent, SidebarFooter,
    SidebarGroup, SidebarGroupContent, SidebarMenu, SidebarMenuButton, SidebarMenuItem,
    SidebarMenuSub, SidebarMenuSubButton, SidebarMenuSubItem, SidebarRail,
};
use crate::state::use_app_state;

use crate::components::sidebar::SidebarHeaderComponent;
use crate::icons::{CodeIcon, GamepadIcon, HelpIcon, LayoutIcon, ZapIcon};

use crate::Route;

/// Toggle hotspot of a collapsible sidebar row (the leading icon and the
/// chevron). Clicking it toggles the collapsible; when the sidebar is
/// collapsed to icons the click falls through to the row's navigation
/// instead (sub-items are hidden in that mode anyway).
#[component]
fn CollapsibleToggle(class: String, children: Element) -> Element {
    let mut ctx = use_context::<CollapsibleContext>();
    let app_state = use_app_state();

    rsx! {
        span {
            class: "flex shrink-0 items-center justify-center {class}",
            onclick: move |e| {
                if *app_state.sidebar_open.read() {
                    e.stop_propagation();
                    ctx.toggle();
                }
            },
            {children}
        }
    }
}

#[component]
pub fn AppSidebar() -> Element {
    let mut app_state = use_app_state();
    let router = router();
    let mut upgrade_open = use_signal(|| false);

    let current_route = router.current::<Route>();

    let is_dashboard = matches!(current_route, Route::Home {});

    let is_tasks_overview = matches!(current_route, Route::Tasks {});
    let is_tasks_all = matches!(current_route, Route::TasksAll {});
    let is_tasks_archived = matches!(current_route, Route::TasksArchived {});
    let is_tasks_urgent = matches!(current_route, Route::TasksUrgent {});
    let is_tasks_group = is_tasks_overview || is_tasks_all || is_tasks_archived || is_tasks_urgent;
    let is_tasks_trigger_active =
        is_tasks_overview || (is_tasks_group && !*app_state.tasks_expanded.read());

    let is_projects_overview = matches!(current_route, Route::Projects {});
    let is_projects_personal = matches!(current_route, Route::ProjectsPersonal {});
    let is_projects_github = matches!(current_route, Route::ProjectsGithub {});
    let is_projects_group =
        is_projects_overview || is_projects_personal || is_projects_github;
    let is_projects_trigger_active =
        is_projects_overview || (is_projects_group && !*app_state.projects_expanded.read());

    let is_calendar = matches!(current_route, Route::Calendar {});

    let is_chat_overview = matches!(current_route, Route::Chat {});
    let is_chat_messaging = matches!(current_route, Route::ChatMessaging {});
    let is_chat_mail = matches!(current_route, Route::ChatMail {});
    let is_chat_issues = matches!(current_route, Route::ChatIssues {});
    let is_chat_group =
        is_chat_overview || is_chat_messaging || is_chat_mail || is_chat_issues;
    let is_chat_trigger_active =
        is_chat_overview || (is_chat_group && !*app_state.chat_expanded.read());

    let is_analytics = matches!(current_route, Route::Analytics {});
    let is_mcp = matches!(current_route, Route::Mcp {});

    rsx! {
        Sidebar { class: "border-0 shrink-0 bg-transparent",

            // 1. Team Switcher Header: Acme Inc / Enterprise
            SidebarHeaderComponent {}

            // 2. Main Navigation Menu
            SidebarContent { class: "gap-1 pt-1",
                SidebarGroup { class: "p-0",
                    SidebarGroupContent {
                        SidebarMenu {
                            // 2.1 Dashboard
                            SidebarMenuItem {
                                SidebarMenuButton {
                                    tooltip: "Dashboard".to_string(),
                                    active: is_dashboard,
                                    onclick: move |_| {
                                        app_state.active_nav.set("dashboard".to_string());
                                        app_state.selected_project.set("All Projects".to_string());
                                        let _ = router.push(Route::Home {});
                                    },
                                    LayoutIcon { class: "size-4 text-muted-foreground" },
                                    span { "Dashboard" }
                                }
                            }

                            // 2.2 Tasks (Collapsible; parent navigates to /tasks & expands, chevron toggles)
                            SidebarMenuItem {
                                Collapsible {
                                    open: app_state.tasks_expanded,
                                    class: "group/tasks w-full",

                                    SidebarMenuButton {
                                        tooltip: "Tasks".to_string(),
                                        active: is_tasks_trigger_active,
                                        onclick: move |_| {
                                            app_state.tasks_expanded.set(true);
                                            app_state.active_nav.set("tasks".to_string());
                                            app_state.selected_project.set("".to_string());
                                            let _ = router.push(Route::Tasks {});
                                        },
                                        CodeIcon { class: "sidebar-icon size-4" }
                                        span { "Tasks" }
                                        CollapsibleToggle {
                                            class: "ml-auto group-data-[state=open]/tasks:rotate-90 group-data-[collapsible=icon]:hidden transition-transform duration-200",
                                            svg {
                                                class: "size-3.5 text-muted-foreground/70 hover:text-foreground",
                                                view_box: "0 0 24 24",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "2",
                                                path { d: "m9 18 6-6-6-6" }
                                            }
                                        }
                                    }
                                    CollapsibleContent {
                                        SidebarMenuSub { class: "border-l border-zinc-700/60 dark:border-zinc-700/60 light:border-zinc-300 ml-4.5 pl-3 py-1 flex flex-col gap-1",
                                            SidebarMenuSubItem {
                                                SidebarMenuSubButton {
                                                    class: "w-full",
                                                    active: is_tasks_all,
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("tasks".to_string());
                                                        app_state.selected_project.set("All Tasks".to_string());
                                                        let _ = router.push(Route::TasksAll {});
                                                    },
                                                    span { "All Tasks" }
                                                }
                                            }
                                            SidebarMenuSubItem {
                                                SidebarMenuSubButton {
                                                    class: "w-full",
                                                    active: is_tasks_archived,
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("tasks".to_string());
                                                        app_state.selected_project.set("Archived".to_string());
                                                        let _ = router.push(Route::TasksArchived {});
                                                    },
                                                    span { "Archived" }
                                                }
                                            }
                                            SidebarMenuSubItem {
                                                SidebarMenuSubButton {
                                                    class: "w-full",
                                                    active: is_tasks_urgent,
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("tasks".to_string());
                                                        app_state.selected_project.set("Urgent".to_string());
                                                        let _ = router.push(Route::TasksUrgent {});
                                                    },
                                                    span { "Urgent" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // 2.3 Projects (Collapsible; parent navigates to /projects & expands, chevron toggles)
                            SidebarMenuItem {
                                Collapsible {
                                    open: app_state.projects_expanded,
                                    class: "group/projects w-full",

                                    SidebarMenuButton {
                                        tooltip: "Projects".to_string(),
                                        active: is_projects_trigger_active,
                                        onclick: move |_| {
                                            app_state.projects_expanded.set(true);
                                            app_state.active_nav.set("projects".to_string());
                                            app_state.selected_project.set("All Projects".to_string());
                                            let _ = router.push(Route::Projects {});
                                        },
                                        svg {
                                            class: "sidebar-icon size-4 text-muted-foreground",
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            stroke_width: "2",
                                            path { d: "M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1-2.5-2.5Z" }
                                            path { d: "M6 6h10M6 10h10" }
                                        }
                                        span { "Projects" }
                                        CollapsibleToggle {
                                            class: "ml-auto group-data-[state=open]/projects:rotate-90 group-data-[collapsible=icon]:hidden transition-transform duration-200",
                                            svg {
                                                class: "size-3.5 text-muted-foreground/70 hover:text-foreground",
                                                view_box: "0 0 24 24",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "2",
                                                path { d: "m9 18 6-6-6-6" }
                                            }
                                        }
                                    }
                                    CollapsibleContent {
                                        SidebarMenuSub { class: "border-l border-zinc-700/60 dark:border-zinc-700/60 light:border-zinc-300 ml-4.5 pl-3 py-1 flex flex-col gap-1",
                                            SidebarMenuSubItem {
                                                SidebarMenuSubButton {
                                                    class: "w-full",
                                                    active: is_projects_overview,
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("projects".to_string());
                                                        app_state.selected_project.set("All Projects".to_string());
                                                        let _ = router.push(Route::Projects {});
                                                    },
                                                    span { "All Projects" }
                                                }
                                            }
                                            SidebarMenuSubItem {
                                                SidebarMenuSubButton {
                                                    class: "w-full",
                                                    active: is_projects_personal,
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("projects".to_string());
                                                        app_state.selected_project.set("Personal".to_string());
                                                        let _ = router.push(Route::ProjectsPersonal {});
                                                    },
                                                    span { "Personal" }
                                                }
                                            }
                                            SidebarMenuSubItem {
                                                SidebarMenuSubButton {
                                                    class: "w-full",
                                                    active: is_projects_github,
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("projects".to_string());
                                                        app_state.selected_project.set("Github Repos".to_string());
                                                        let _ = router.push(Route::ProjectsGithub {});
                                                    },
                                                    span { "Github Repos" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // 2.4 Calendar
                            SidebarMenuItem {
                                SidebarMenuButton {
                                    tooltip: "Calendar".to_string(),
                                    active: is_calendar,
                                    onclick: move |_| {
                                        app_state.active_nav.set("calendar".to_string());
                                        let _ = router.push(Route::Calendar {});
                                    },
                                    svg {
                                        class: "size-4 text-muted-foreground",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        rect {
                                            width: "18",
                                            height: "18",
                                            x: "3",
                                            y: "4",
                                            rx: "2",
                                        }
                                        path { d: "M16 2v4M8 2v4M3 10h18" }
                                    }
                                    span { "Calendar" }
                                }
                            }

                            // 2.5 Chat (Collapsible; parent navigates to /chat & expands, chevron toggles)
                            SidebarMenuItem {
                                Collapsible {
                                    open: app_state.chat_expanded,
                                    class: "group/chat w-full",

                                    SidebarMenuButton {
                                        tooltip: "Chat".to_string(),
                                        active: is_chat_trigger_active,
                                        onclick: move |_| {
                                            app_state.chat_expanded.set(true);
                                            app_state.active_nav.set("chat".to_string());
                                            app_state.selected_project.set("".to_string());
                                            let _ = router.push(Route::Chat {});
                                        },
                                        svg {
                                            class: "sidebar-icon size-4 text-muted-foreground",
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            stroke_width: "2",
                                            path { d: "M7.9 20A9 9 0 1 0 4 16.1L2 22Z" }
                                        }
                                        span { "Chat" }
                                        CollapsibleToggle {
                                            class: "ml-auto group-data-[state=open]/chat:rotate-90 group-data-[collapsible=icon]:hidden transition-transform duration-200",
                                            svg {
                                                class: "size-3.5 text-muted-foreground/70 hover:text-foreground",
                                                view_box: "0 0 24 24",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "2",
                                                path { d: "m9 18 6-6-6-6" }
                                            }
                                        }
                                    }
                                    CollapsibleContent {
                                        SidebarMenuSub { class: "border-l border-zinc-700/60 dark:border-zinc-700/60 light:border-zinc-300 ml-4.5 pl-3 py-1 flex flex-col gap-1",
                                            SidebarMenuSubItem {
                                                SidebarMenuSubButton {
                                                    class: "w-full",
                                                    active: is_chat_messaging,
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("chat".to_string());
                                                        app_state.selected_project.set("Messaging".to_string());
                                                        let _ = router.push(Route::ChatMessaging {});
                                                    },
                                                    span { "Messaging" }
                                                }
                                            }
                                            SidebarMenuSubItem {
                                                SidebarMenuSubButton {
                                                    class: "w-full",
                                                    active: is_chat_mail,
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("chat".to_string());
                                                        app_state.selected_project.set("Mail".to_string());
                                                        let _ = router.push(Route::ChatMail {});
                                                    },
                                                    span { "Mail" }
                                                }
                                            }
                                            SidebarMenuSubItem {
                                                SidebarMenuSubButton {
                                                    class: "w-full",
                                                    active: is_chat_issues,
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("chat".to_string());
                                                        app_state.selected_project.set("issues".to_string());
                                                        let _ = router.push(Route::ChatIssues {});
                                                    },
                                                    span { "issues" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // 2.6 Analytics
                            SidebarMenuItem {
                                SidebarMenuButton {
                                    tooltip: "Analytics".to_string(),
                                    active: is_analytics,
                                    onclick: move |_| {
                                        app_state.active_nav.set("analytics".to_string());
                                        let _ = router.push(Route::Analytics {});
                                    },
                                    svg {
                                        class: "size-4 text-muted-foreground",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        path { d: "M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" }
                                        path { d: "M14 2v6h6M8 13h8M8 17h5" }
                                    }
                                    span { "Analytics" }
                                }
                            }

                            // 2.7 Mcp
                            SidebarMenuItem {
                                SidebarMenuButton {
                                    tooltip: "Mcp".to_string(),
                                    active: is_mcp,
                                    onclick: move |_| {
                                        app_state.active_nav.set("mcp".to_string());
                                        let _ = router.push(Route::Mcp {});
                                    },
                                    svg {
                                        class: "size-4 text-muted-foreground",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        circle { cx: "18", cy: "18", r: "3" }
                                        circle { cx: "6", cy: "6", r: "3" }
                                        path { d: "M13 6h3a2 2 0 0 1 2 2v7M6 9v12" }
                                    }
                                    span { "Mcp" }
                                }
                            }
                        }
                    }
                }

                // 3. Secondary Navigation Items near bottom
                div { class: "mt-auto pt-4 flex flex-col gap-1",
                    SidebarGroup { class: "p-0",
                        SidebarGroupContent {
                            SidebarMenu {
                                SidebarMenuItem {
                                    SidebarMenuButton { tooltip: "Entertainment".to_string(),
                                        GamepadIcon{},
                                        span { "Entertainment" }
                                    }
                                }
                                SidebarMenuItem {
                                    SidebarMenuButton { tooltip: "Help & Support".to_string(),
                                        HelpIcon{},
                                        span { "Help & Support" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            SidebarRail {}
            SidebarFooter{
                SidebarContent {
                    Dialog {
                        open: upgrade_open,
                        DialogTrigger {
                            class: "w-full group-data-[collapsible=icon]:hidden",
                            div {
                                class: "w-full grid grid-cols-1 gap-2.5 rounded-xl border border-primary/25 \
                                        bg-linear-to-br from-primary/15 via-primary/5 to-transparent \
                                        p-2.5 text-left shadow-xs",
                                div {
                                    class: "flex items-center gap-2.5",
                                    span {
                                        class: "size-8 shrink-0 grid place-items-center rounded-lg bg-primary/15 text-primary",
                                        ZapIcon { class: "size-4" }
                                    }
                                    span { class: "flex min-w-0 flex-1 flex-col",
                                        span { class: "text-xs font-semibold text-foreground leading-tight",
                                            "Upgrade to Pro"
                                        }
                                        span { class: "text-[10px] text-muted-foreground truncate",
                                            "Unlimited projects & reports"
                                        }
                                    }
                                }
                                span {
                                    class: "shrink-0 rounded-md bg-primary px-2 py-1 text-[10px] font-semibold text-primary-foreground grid place-items-center py-2",
                                    "Upgrade"
                                }
                            }
                        }
                        DialogContent {
                            DialogHeader {
                                DialogTitle { "Upgrade to Pro" }
                                DialogDescription { "Unlock the full potential of Taskify." }
                            }

                            div { class: "flex flex-col gap-2.5",
                                for benefit in [
                                    "Unlimited projects and tasks",
                                    "Advanced analytics and reports",
                                    "Priority support",
                                ] {
                                    div { key: "{benefit}", class: "flex items-start gap-2.5 text-sm text-foreground",
                                        span {
                                            class: "mt-0.5 flex size-4 shrink-0 items-center justify-center rounded-full bg-primary/15 text-primary",
                                            svg {
                                                class: "size-2.5",
                                                view_box: "0 0 24 24",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "3",
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                path { d: "M20 6 9 17l-5-5" }
                                            }
                                        }
                                        span { "{benefit}" }
                                    }
                                }
                                p { class: "pt-1 text-xs text-muted-foreground",
                                    "Starting at $9/month \u{b7} Cancel anytime"
                                }
                            }

                            DialogFooter {
                                button {
                                    r#type: "button",
                                    class: "h-8 rounded-md border border-border/40 px-3 text-xs font-medium text-foreground hover:bg-muted transition-colors cursor-pointer",
                                    onclick: move |_| upgrade_open.set(false),
                                    "Maybe later"
                                }
                                button {
                                    r#type: "button",
                                    class: "h-8 rounded-md bg-primary px-3 text-xs font-medium text-primary-foreground hover:opacity-90 transition-opacity cursor-pointer",
                                    onclick: move |_| upgrade_open.set(false),
                                    "Upgrade Now"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
