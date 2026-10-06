use dioxus::prelude::*;

use crate::components::ui::{
    DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuLabel,
    DropdownMenuSeparator, DropdownMenuShortcut, DropdownMenuTrigger, SidebarHeader,
};

use crate::icons::{AudioWaveformIcon, CommandIcon, GalleryVerticalEndIcon, PlusRectIcon};

#[derive(Clone, Copy)]
struct Team {
    name: &'static str,
    plan: &'static str,
    logo: fn() -> Element,
}

#[component]
pub fn SidebarHeaderComponent() -> Element {
    let teams = [
        Team {
            name: "Acme Inc",
            logo: || rsx! { GalleryVerticalEndIcon { class: "size-4" } },
            plan: "Enterprise",
        },
        Team {
            name: "Acme Corp.",
            logo: || rsx! { AudioWaveformIcon { class: "size-4" } },
            plan: "Startup",
        },
        Team {
            name: "Evil Corp.",
            logo: || rsx! { CommandIcon { class: "size-4" } },
            plan: "Free",
        },
    ];

    let mut active = use_signal(|| 0usize);
    let current = teams[active()];

    rsx! {
        SidebarHeader { class: "p-0 pb-2",
            DropdownMenu {
                DropdownMenuTrigger {
                    div { class: "w-56 flex items-center gap-3 p-1.5 rounded-lg hover:bg-sidebar-accent/50 cursor-pointer transition-colors group-data-[collapsible=icon]:justify-center group-data-[collapsible=icon]:p-0",
                        // Squircle Icon with drawer/box
                        div { class: "flex size-9 items-center justify-center rounded-lg bg-zinc-950 dark:bg-zinc-900 text-white shrink-0 shadow-sm border border-zinc-800",
                            {(current.logo)()}
                        }
                        div { class: "sidebar-text flex flex-col min-w-0 flex-1 text-left leading-tight",
                            span { class: "font-semibold text-xs text-foreground truncate tracking-tight",
                                "{current.name}"
                            }
                            span { class: "text-[10px] text-muted-foreground truncate", "{current.plan}" }
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
                DropdownMenuContent {
                    class: "w-60",
                    side: "right",
                    side_offset: 4,
                    DropdownMenuLabel {
                        children: rsx! {
                            span {
                                "Teams"
                            }
                        },
                    }
                    DropdownMenuGroup {
                        children: rsx! {
                            for (i, team) in teams.iter().enumerate() {
                                DropdownMenuItem {
                                    key: "{i}",
                                    onclick: move |_| active.set(i),
                                    children: rsx! {
                                        div { class: "w-full flex items-center gap-3 rounded-lg hover:bg-sidebar-accent/50 cursor-pointer transition-colors group-data-[collapsible=icon]:justify-center group-data-[collapsible=icon]:p-0",
                                            // Squircle Icon with drawer/box
                                            div { class: "flex size-7 p-2 items-center justify-center rounded-sm bg-zinc-200 dark:bg-zinc-900 text-black shrink-0 shadow-sm",
                                                {(team.logo)()}
                                            }
                                            div { class: "sidebar-text flex flex-col min-w-0 flex-1 text-left leading-tight",
                                                span { class: "font-semibold text-xs text-foreground truncate tracking-tight",
                                                    "{team.name}"
                                                }
                                            }
                                        }
                                        DropdownMenuShortcut {
                                            children: rsx! {
                                                span {
                                                    "⌘+{i + 1}"
                                                }
                                            },
                                        }
                                    },
                                }
                            }
                        },
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuItem {
                        children: rsx! {
                            div {
                                class: "w-full flex items-center gap-3 p-1.5 rounded-lg hover:bg-sidebar-accent/50 cursor-pointer transition-colors group-data-[collapsible=icon]:justify-center group-data-[collapsible=icon]:p-0",
                                PlusRectIcon { class: "size-4" },
                                span{
                                    "Add Team"
                                }
                            }
                        },
                    }
                }
            }
        }

    }
}
