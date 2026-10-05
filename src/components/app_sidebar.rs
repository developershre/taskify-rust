use dioxus::prelude::*;

use crate::components::ui::{
    Avatar, AvatarFallback, AvatarImage, Collapsible, CollapsibleContent, CollapsibleTrigger,
    Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent, SidebarHeader,
    SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarMenuSub, SidebarMenuSubButton,
    SidebarMenuSubItem, SidebarRail,
};
use crate::state::use_app_state;

#[component]
pub fn AppSidebar() -> Element {
    let mut app_state = use_app_state();

    rsx! {
        Sidebar { class: "border-0 bg-transparent shrink-0",

            // 1. Team Switcher Header: Acme Inc / Enterprise
            SidebarHeader { class: "p-0 pb-2",
                div { class: "w-full flex items-center gap-3 p-1.5 rounded-lg hover:bg-sidebar-accent/50 cursor-pointer transition-colors group-data-[collapsible=icon]:justify-center group-data-[collapsible=icon]:p-0",
                    // Squircle Icon with drawer/box
                    div { class: "flex size-9 items-center justify-center rounded-xl bg-zinc-950 dark:bg-zinc-900 text-white shrink-0 shadow-sm border border-zinc-800",
                        svg {
                            class: "size-4 text-zinc-200",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            rect {
                                width: "18",
                                height: "18",
                                x: "3",
                                y: "3",
                                rx: "3",
                            }
                            path { d: "M7 8h10M7 12h10M7 16h10" }
                        }
                    }
                    div { class: "sidebar-text flex flex-col min-w-0 flex-1 text-left leading-tight",
                        span { class: "font-semibold text-xs text-foreground truncate tracking-tight",
                            "Acme Inc"
                        }
                        span { class: "text-[10px] text-muted-foreground truncate", "Enterprise" }
                    }
                    // Chevrons Up/Down
                    svg {
                        class: "sidebar-text size-3.5 text-muted-foreground/60 shrink-0 ml-auto",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path { d: "m7 15 5 5 5-5M7 9l5-5 5 5" }
                    }
                }
            }

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
                                    },
                                    svg {
                                        class: "size-4 text-muted-foreground",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        rect {
                                            width: "7",
                                            height: "7",
                                            x: "3",
                                            y: "3",
                                            rx: "1.5",
                                        }
                                        rect {
                                            width: "7",
                                            height: "7",
                                            x: "14",
                                            y: "3",
                                            rx: "1.5",
                                        }
                                        rect {
                                            width: "7",
                                            height: "7",
                                            x: "14",
                                            y: "14",
                                            rx: "1.5",
                                        }
                                        rect {
                                            width: "7",
                                            height: "7",
                                            x: "3",
                                            y: "14",
                                            rx: "1.5",
                                        }
                                    }
                                    span { "Dashboard" }
                                }
                            }

                            // 2.2 Tasks (Collapsible)
                            Collapsible {
                                default_open: true,
                                class: "group/tasks w-full",
                                SidebarMenuItem {
                                    CollapsibleTrigger { class: "w-full",
                                        SidebarMenuButton { tooltip: "Tasks".to_string(),
                                            svg {
                                                class: "w-full size-4 text-muted-foreground",
                                                view_box: "0 0 24 24",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "2",
                                                rect {
                                                    width: "18",
                                                    height: "18",
                                                    x: "3",
                                                    y: "3",
                                                    rx: "2",
                                                }
                                                path { d: "M8 12h8" }
                                            }
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
                                        SidebarMenuSub { class: "w-full border-l border-zinc-700/60 dark:border-zinc-700/60 light:border-zinc-300 ml-4.5 pl-3 py-1 flex flex-col gap-1",
                                            SidebarMenuSubItem {
                                                SidebarMenuSubButton {
                                                    class: "w-full",
                                                    active: *app_state.selected_project.read() == "All Tasks",
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("tasks".to_string());
                                                        app_state.selected_project.set("All Tasks".to_string());
                                                    },
                                                    span { "All Tasks" }
                                                }
                                            }
                                            SidebarMenuSubItem {
                                                SidebarMenuSubButton {
                                                    active: *app_state.selected_project.read() == "Archived",
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("tasks".to_string());
                                                        app_state.selected_project.set("Archived".to_string());
                                                    },
                                                    span { "Archived" }
                                                }
                                            }
                                            SidebarMenuSubItem {
                                                SidebarMenuSubButton {
                                                    active: *app_state.selected_project.read() == "Urgent",
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("tasks".to_string());
                                                        app_state.selected_project.set("Urgent".to_string());
                                                    },
                                                    span { "Urgent" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // 2.3 Projects (Collapsible)
                            Collapsible {
                                default_open: true,
                                class: "group/projects w-full",
                                SidebarMenuItem {
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
                                                    active: *app_state.selected_project.read() == "All Projects",
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("dashboard".to_string());
                                                        app_state.selected_project.set("All Projects".to_string());
                                                    },
                                                    span { "All Porjects" }
                                                }
                                            }
                                            SidebarMenuSubItem {
                                                SidebarMenuSubButton {
                                                    active: *app_state.selected_project.read() == "Personal",
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("tasks".to_string());
                                                        app_state.selected_project.set("Personal".to_string());
                                                    },
                                                    span { "Personal" }
                                                }
                                            }
                                            SidebarMenuSubItem {
                                                SidebarMenuSubButton {
                                                    active: *app_state.selected_project.read() == "Github Repos",
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("tasks".to_string());
                                                        app_state.selected_project.set("Github Repos".to_string());
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
                                        if !*app_state.calendar_sidebar_open.read() {
                                            app_state.calendar_sidebar_open.set(true);
                                        }
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
                            Collapsible {
                                default_open: true,
                                class: "group/chat w-full",
                                SidebarMenuItem {
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
                                                    active: true,
                                                    class: "bg-secondary text-foreground font-medium rounded-lg px-2.5 py-1.5",
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("chat".to_string());
                                                    },
                                                    span { "Messaging" }
                                                }
                                            }
                                            SidebarMenuSubItem {
                                                SidebarMenuSubButton {
                                                    active: false,
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("chat".to_string());
                                                    },
                                                    span { "Mail" }
                                                }
                                            }
                                            SidebarMenuSubItem {
                                                SidebarMenuSubButton {
                                                    active: false,
                                                    onclick: move |_| {
                                                        app_state.active_nav.set("chat".to_string());
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
                                    onclick: move |_| app_state.active_nav.set("analytics".to_string()),
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
                                    onclick: move |_| app_state.active_nav.set("mcp".to_string()),
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
                                        svg {
                                            class: "size-4 text-muted-foreground",
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            stroke_width: "2",
                                            path { d: "M6 12h4m-2-2v4M15 11h.01M18 13h.01" }
                                            rect {
                                                width: "20",
                                                height: "12",
                                                x: "2",
                                                y: "6",
                                                rx: "6",
                                            }
                                        }
                                        span { "Entertainment" }
                                    }
                                }
                                SidebarMenuItem {
                                    SidebarMenuButton { tooltip: "Help & Support".to_string(),
                                        svg {
                                            class: "size-4 text-muted-foreground",
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            stroke_width: "2",
                                            rect {
                                                width: "7",
                                                height: "7",
                                                x: "3",
                                                y: "3",
                                                rx: "1.5",
                                            }
                                            rect {
                                                width: "7",
                                                height: "7",
                                                x: "14",
                                                y: "3",
                                                rx: "1.5",
                                            }
                                            rect {
                                                width: "7",
                                                height: "7",
                                                x: "14",
                                                y: "14",
                                                rx: "1.5",
                                            }
                                            rect {
                                                width: "7",
                                                height: "7",
                                                x: "3",
                                                y: "14",
                                                rx: "1.5",
                                            }
                                        }
                                        span { "Help & Support" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 4. User Profile Footer: shadcn (Image 1 & 2)
            SidebarFooter { class: "p-0 pt-2 border-0",
                div { class: "flex items-center gap-3 p-1.5 rounded-lg hover:bg-sidebar-accent/50 cursor-pointer transition-colors group-data-[collapsible=icon]:justify-center group-data-[collapsible=icon]:p-0",
                    Avatar { class: "size-8 border border-border/40 shrink-0 rounded-lg overflow-hidden",
                        AvatarImage {
                            src: "https://images.unsplash.com/photo-1534528741775-53994a69daeb?w=100&auto=format&fit=crop&q=80"
                                .to_string(),
                            alt: "shadcn".to_string(),
                        }
                        AvatarFallback { "CN" }
                    }
                    div { class: "sidebar-text flex flex-col min-w-0 flex-1 text-left leading-tight",
                        span { class: "font-semibold text-xs text-foreground truncate",
                            "shadcn"
                        }
                        span { class: "text-[10px] text-muted-foreground truncate", "m@example.com" }
                    }
                    svg {
                        class: "sidebar-text size-3.5 text-muted-foreground/60 shrink-0 ml-auto",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path { d: "m7 15 5 5 5-5M7 9l5-5 5 5" }
                    }
                }
            }

            SidebarRail {}
        }
    }
}
