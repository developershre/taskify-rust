use dioxus::prelude::*;

use crate::components::ui::{
    Collapsible, CollapsibleContent, CollapsibleTrigger, Dialog, DialogContent, DialogDescription,
    DialogFooter, DialogHeader, DialogTitle, DialogTrigger, Sidebar, SidebarContent, SidebarFooter,
    SidebarGroup, SidebarGroupContent, SidebarMenu, SidebarMenuButton, SidebarMenuItem,
    SidebarMenuSub, SidebarMenuSubButton, SidebarMenuSubItem, SidebarRail,
};
use crate::state::use_app_state;

use crate::components::sidebar::SidebarHeaderComponent;
use crate::icons::{CodeIcon, GamepadIcon, HelpIcon, LayoutIcon, ZapIcon};

#[component]
pub fn AppSidebar() -> Element {
    let mut app_state = use_app_state();
    let router = router();
    let mut upgrade_open = use_signal(|| false);

    rsx! {
        Sidebar { class: "border-0 shrink-0 bg-transparent",

            // 1. Team Switcher Header: Acme Inc / Enterprise
            SidebarHeaderComponent {  }

            // 2. Main Navigation Menu
            SidebarContent { class: "gap-1 pt-1",
                SidebarGroup { class: "p-0",
                    SidebarGroupContent {
                        SidebarMenu {
                            // 2.1 Dashboard
                            SidebarMenuItem {
                                SidebarMenuButton {
                                    tooltip: "Dashboard".to_string(),
                                    active: *app_state.active_nav.read() == "dashboard",
                                    onclick: move |_| {
                                        app_state.active_nav.set("dashboard".to_string());
                                        let _ = router.push("/");
                                    },
                                    LayoutIcon { class: "size-4 text-muted-foreground" },
                                    span { "Dashboard" }
                                }
                            }

                            // 2.2 Tasks (Collapsible)
                            SidebarMenuItem {
                                Collapsible {
                                    default_open: true,
                                    class: "group/tasks w-full",

                                    CollapsibleTrigger { class: "w-full",
                                        SidebarMenuButton { tooltip: "Tasks".to_string(),
                                            CodeIcon{}
                                            span { "Tasks" }
                                            svg {
                                                class: "ml-auto size-3.5 text-muted-foreground/60 group-data-[state=open]/tasks:rotate-90 group-data-[collapsible=icon]:hidden",
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
                                                    active: *app_state.active_nav.read() == "tasks"
                                                        && *app_state.selected_project.read()
                                                            == "All Tasks",
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("tasks".to_string());
                                                        app_state.selected_project.set("All Tasks".to_string());
                                                        let _ = router.push("/tasks/all");
                                                    },
                                                    span { "All Tasks" }
                                                }
                                            }
                                            SidebarMenuSubItem {
                                                SidebarMenuSubButton {
                                                    class: "w-full",
                                                    active: *app_state.active_nav.read() == "tasks"
                                                        && *app_state.selected_project.read()
                                                            == "Archived",
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("tasks".to_string());
                                                        app_state.selected_project.set("Archived".to_string());
                                                        let _ = router.push("/tasks/archived");
                                                    },
                                                    span { "Archived" }
                                                }
                                            }
                                            SidebarMenuSubItem {
                                                SidebarMenuSubButton {
                                                    class: "w-full",
                                                    active: *app_state.active_nav.read() == "tasks"
                                                        && *app_state.selected_project.read()
                                                            == "Urgent",
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("tasks".to_string());
                                                        app_state.selected_project.set("Urgent".to_string());
                                                        let _ = router.push("/tasks/urgent");
                                                    },
                                                    span { "Urgent" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // 2.3 Projects (Collapsible)
                            SidebarMenuItem {
                                Collapsible {
                                    default_open: false,
                                    class: "group/projects w-full",

                                    CollapsibleTrigger { class: "w-full",
                                        SidebarMenuButton { tooltip: "Projects".to_string(),
                                            svg {
                                                class: "size-4 text-muted-foreground",
                                                view_box: "0 0 24 24",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "2",
                                                path { d: "M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1-2.5-2.5Z" }
                                                path { d: "M6 6h10M6 10h10" }
                                            }
                                            span { "Projects" }
                                            svg {
                                                class: "ml-auto size-3.5 text-muted-foreground/60 group-data-[state=open]/projects:rotate-90 group-data-[collapsible=icon]:hidden",
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
                                                    active: *app_state.active_nav.read() == "projects"
                                                        && *app_state.selected_project.read()
                                                            == "All Projects",
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("projects".to_string());
                                                        app_state.selected_project.set("All Projects".to_string());
                                                        let _ = router.push("/projects");
                                                    },
                                                    span { "All Projects" }
                                                }
                                            }
                                            SidebarMenuSubItem {
                                                SidebarMenuSubButton {
                                                    class: "w-full",
                                                    active: *app_state.active_nav.read() == "projects"
                                                        && *app_state.selected_project.read()
                                                            == "Personal",
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("projects".to_string());
                                                        app_state.selected_project.set("Personal".to_string());
                                                        let _ = router.push("/projects/personal");
                                                    },
                                                    span { "Personal" }
                                                }
                                            }
                                            SidebarMenuSubItem {
                                                SidebarMenuSubButton {
                                                    class: "w-full",
                                                    active: *app_state.active_nav.read() == "projects"
                                                        && *app_state.selected_project.read()
                                                            == "Github Repos",
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("projects".to_string());
                                                        app_state.selected_project.set("Github Repos".to_string());
                                                        let _ = router.push("/projects/github");
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
                                    active: *app_state.active_nav.read() == "calendar",
                                    onclick: move |_| {
                                        app_state.active_nav.set("calendar".to_string());
                                        let _ = router.push("/calendar");
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

                            // 2.5 Chat (Collapsible with active Messaging item)
                            SidebarMenuItem {
                                Collapsible {
                                    default_open: false,
                                    class: "group/chat w-full",

                                    CollapsibleTrigger { class: "w-full",
                                        SidebarMenuButton { tooltip: "Chat".to_string(),
                                            svg {
                                                class: "size-4 text-muted-foreground",
                                                view_box: "0 0 24 24",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "2",
                                                path { d: "M7.9 20A9 9 0 1 0 4 16.1L2 22Z" }
                                            }
                                            span { "Chat" }
                                            svg {
                                                class: "ml-auto size-3.5 text-muted-foreground/60 group-data-[state=open]/chat:rotate-90 group-data-[collapsible=icon]:hidden",
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
                                                    class: "w-full bg-secondary text-foreground font-medium rounded-lg px-2.5 py-1.5",
                                                    active: *app_state.active_nav.read() == "chat"
                                                        && *app_state.selected_project.read()
                                                            == "Messaging",
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("chat".to_string());
                                                        app_state.selected_project.set("Messaging".to_string());
                                                        let _ = router.push("/chat/messaging");
                                                    },
                                                    span { "Messaging" }
                                                }
                                            }
                                            SidebarMenuSubItem {
                                                SidebarMenuSubButton {
                                                    class: "w-full",
                                                    active: *app_state.active_nav.read() == "chat"
                                                        && *app_state.selected_project.read() == "Mail",
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("chat".to_string());
                                                        app_state.selected_project.set("Mail".to_string());
                                                        let _ = router.push("/chat/mail");
                                                    },
                                                    span { "Mail" }
                                                }
                                            }
                                            SidebarMenuSubItem {
                                                SidebarMenuSubButton {
                                                    class: "w-full",
                                                    active: *app_state.active_nav.read() == "chat"
                                                        && *app_state.selected_project.read() == "issues",
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("chat".to_string());
                                                        app_state.selected_project.set("issues".to_string());
                                                        let _ = router.push("/chat/issues");
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
                                    active: *app_state.active_nav.read() == "analytics",
                                    onclick: move |_| {
                                        app_state.active_nav.set("analytics".to_string());
                                        let _ = router.push("/analytics");
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
                                    active: *app_state.active_nav.read() == "mcp",
                                    onclick: move |_| {
                                        app_state.active_nav.set("mcp".to_string());
                                        let _ = router.push("/mcp");
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
                                    class: "shrink-0 rounded-md bg-primary px-2 py-1 text-[10px] font-semibold text-primary-foreground",
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
