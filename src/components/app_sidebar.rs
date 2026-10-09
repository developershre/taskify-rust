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

struct NavItem {
    name: &'static str,
    icon: fn() -> Element,
    path: &'static str,
    default_open: bool,
    children: &'static [NavChild],
}

struct NavChild {
    name: &'static str,
    path: &'static str,
}

const NAVIGATION_ITEMS: &[NavItem] = &[
    NavItem {
        name: "Dashboard",
        icon: icon_dashboard,
        path: "/",
        default_open: false,
        children: &[],
    },
    NavItem {
        name: "Tasks",
        icon: icon_tasks,
        path: "/tasks",
        default_open: true,
        children: &[
            NavChild {
                name: "All Tasks",
                path: "/tasks/all",
            },
            NavChild {
                name: "Archived",
                path: "/tasks/archived",
            },
            NavChild {
                name: "Urgent",
                path: "/tasks/urgent",
            },
        ],
    },
    NavItem {
        name: "Projects",
        icon: icon_projects,
        path: "/projects",
        default_open: false,
        children: &[
            NavChild {
                name: "All Projects",
                path: "/projects",
            },
            NavChild {
                name: "Personal",
                path: "/projects/personal",
            },
            NavChild {
                name: "Github Repos",
                path: "/projects/github",
            },
        ],
    },
    NavItem {
        name: "Calendar",
        icon: icon_calendar,
        path: "/calendar",
        default_open: false,
        children: &[],
    },
    NavItem {
        name: "Chat",
        icon: icon_chat,
        path: "/chat",
        default_open: false,
        children: &[
            NavChild {
                name: "Messaging",
                path: "/chat/messaging",
            },
            NavChild {
                name: "Mail",
                path: "/chat/mail",
            },
            NavChild {
                name: "issues",
                path: "/chat/issues",
            },
        ],
    },
    NavItem {
        name: "Analytics",
        icon: icon_analytics,
        path: "/analytics",
        default_open: false,
        children: &[],
    },
    NavItem {
        name: "Mcp",
        icon: icon_mcp,
        path: "/mcp",
        default_open: false,
        children: &[],
    },
];

fn icon_dashboard() -> Element {
    rsx! { LayoutIcon { class: "size-4 text-muted-foreground" } }
}

fn icon_tasks() -> Element {
    rsx! { CodeIcon {} }
}

fn icon_projects() -> Element {
    rsx! {
        svg {
            class: "size-4 text-muted-foreground",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            path { d: "M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1-2.5-2.5Z" }
            path { d: "M6 6h10M6 10h10" }
        }
    }
}

fn icon_calendar() -> Element {
    rsx! {
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
    }
}

fn icon_chat() -> Element {
    rsx! {
        svg {
            class: "size-4 text-muted-foreground",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            path { d: "M7.9 20A9 9 0 1 0 4 16.1L2 22Z" }
        }
    }
}

fn icon_analytics() -> Element {
    rsx! {
        svg {
            class: "size-4 text-muted-foreground",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            path { d: "M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" }
            path { d: "M14 2v6h6M8 13h8M8 17h5" }
        }
    }
}

fn icon_mcp() -> Element {
    rsx! {
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
    }
}

fn chevron_class(name: &str) -> String {
    match name {
        "Tasks" => {
            "ml-auto group-data-[state=open]/tasks:rotate-90 group-data-[collapsible=icon]:hidden"
                .to_string()
        }
        "Projects" => {
            "ml-auto group-data-[state=open]/projects:rotate-90 group-data-[collapsible=icon]:hidden"
                .to_string()
        }
        "Chat" => {
            "ml-auto group-data-[state=open]/chat:rotate-90 group-data-[collapsible=icon]:hidden"
                .to_string()
        }
        _ => "ml-auto group-data-[collapsible=icon]:hidden".to_string(),
    }
}

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

    rsx! {
        Sidebar { class: "border-0 shrink-0 bg-transparent",

            // 1. Team Switcher Header: Acme Inc / Enterprise
            SidebarHeaderComponent {  }

            // 2. Main Navigation Menu (data-driven from NAVIGATION_ITEMS)
            SidebarContent { class: "gap-1 pt-1",
                SidebarGroup { class: "p-0",
                    SidebarGroupContent {
                        SidebarMenu {
                            for item in NAVIGATION_ITEMS {
                                {
                                    let name = item.name;
                                    let id = name.to_lowercase();
                                    let icon = (item.icon)();
                                    if item.children.is_empty() {
                                        rsx! {
                                            SidebarMenuItem { key: "{id}",
                                                SidebarMenuButton {
                                                    tooltip: name.to_string(),
                                                    active: *app_state.active_nav.read() == id,
                                                    onclick: move |_| {
                                                        app_state.active_nav.set(item.name.to_lowercase());
                                                        let _ = router.push(item.path);
                                                    },
                                                    {icon}
                                                    span { "{name}" }
                                                }
                                            }
                                        }
                                    } else {
                                        rsx! {
                                            SidebarMenuItem { key: "{id}",
                                                Collapsible {
                                                    default_open: item.default_open,
                                                    class: "group/{id} w-full",

                                                    SidebarMenuButton { tooltip: name.to_string(),
                                                        active: *app_state.active_nav.read() == id,
                                                        onclick: move |_| {
                                                            app_state.active_nav.set(item.name.to_lowercase());
                                                            let _ = router.push(item.path);
                                                        },
                                                        CollapsibleToggle { class: "sidebar-icon".to_string(), {icon} }
                                                        span { "{name}" }
                                                        CollapsibleToggle {
                                                            class: chevron_class(name),
                                                            svg {
                                                                class: "size-3.5 text-muted-foreground/60",
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
                                                            for child in item.children {
                                                                {
                                                                    let child_name = child.name;
                                                                    rsx! {
                                                                        SidebarMenuSubItem { key: "{child.path}",
                                                                            SidebarMenuSubButton {
                                                                                class: "w-full",
                                                                                active: *app_state.active_nav.read()
                                                                                    == item.name.to_lowercase()
                                                                                    && *app_state.selected_project.read()
                                                                                        == child_name,
                                                                                onclick: move |_| {
                                                                                    app_state.active_nav.set(item.name.to_lowercase());
                                                                                    app_state.selected_project.set(child.name.to_string());
                                                                                    let _ = router.push(child.path);
                                                                                },
                                                                                span { "{child_name}" }
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
