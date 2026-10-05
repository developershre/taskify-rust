use dioxus::prelude::*;

use crate::components::ui::{SidebarProvider, TooltipProvider};
use crate::components::{AppSidebar, CalendarSidebar};
use crate::state::use_app_state;

#[component]
pub fn Home() -> Element {
    let mut app_state = use_app_state();

    rsx! {
        TooltipProvider {
            SidebarProvider {
                open: app_state.sidebar_open,
                default_open: true,
                class: "flex h-full w-full p-2 sm:p-3 gap-2 sm:gap-3 bg-background overflow-hidden",


                AppSidebar {}

                // ============================================================
                // Column 2: Middle Area - Dashboard Card Panel (Image 1)
                // ============================================================
                main { class: "flex-1 rounded-2xl border border-border/40 bg-card p-6 flex flex-col h-full overflow-y-auto shadow-xs transition-colors duration-200",

                    // Top Bar: Panel toggle icon + Dashboard label
                    div { class: "flex items-center justify-between pb-2",

                        div { class: "flex items-center gap-2",
                            button {
                                class: "inline-flex size-6 items-center justify-center rounded text-muted-foreground hover:text-foreground transition-colors cursor-pointer",
                                title: "Toggle Sidebar (Ctrl+B)",
                                onclick: move |_| {
                                    app_state.toggle_sidebar();
                                },
                                svg {
                                    class: "size-4 text-muted-foreground/80",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    rect {
                                        width: "18",
                                        height: "18",
                                        x: "3",
                                        y: "3",
                                        rx: "2",
                                    }
                                    path { d: "M9 3v18" }
                                }
                            }
                            span { class: "text-muted-foreground text-xs font-medium",
                                "Dashboard"
                            }
                        }

                        div { class: "flex items-center gap-1.5",
                            // Toggle Right Calendar Sidebar
                            button {
                                class: "inline-flex size-7 items-center justify-center rounded-md text-muted-foreground hover:text-foreground transition-colors cursor-pointer",
                                title: "Toggle Calendar Sidebar",
                                onclick: move |_| {
                                    app_state.toggle_calendar_sidebar();
                                },
                                svg {
                                    class: "size-3.5",
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
                                    path { d: "M15 3v18" }
                                }
                            }
                        }
                    }

                    // Welcome Header from Image 1
                    h1 { class: "text-2xl sm:text-3xl font-bold tracking-tight text-foreground mt-4",
                        "Welcome Back"
                    }
                }

                // ============================================================
                // Column 3: Right Area - Custom Calendar & Notes Card Panel (Image 1 & 3)
                // ============================================================
                CalendarSidebar {}
            }
        }
    }
}
