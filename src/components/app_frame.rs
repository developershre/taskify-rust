use dioxus::prelude::*;

use crate::components::ui::{SidebarProvider, TooltipProvider};
use crate::components::{AppSidebar, BreadcrumbComponent, CalendarSidebar, NewNoteDialog};
use crate::state::{use_app_state, OverlayState};
use crate::Route;

#[component]
pub fn AppFrame() -> Element {
    let mut app_state = use_app_state();
    let overlay = use_context::<OverlayState>();

    use_effect(move || {
        let route = router().current::<Route>();
        let (nav, project) = match &route {
            Route::Home {} => ("dashboard", "All Projects"),
            Route::Tasks {} => ("tasks", ""),
            Route::TasksAll {} => ("tasks", "All Tasks"),
            Route::TasksArchived {} => ("tasks", "Archived"),
            Route::TasksUrgent {} => ("tasks", "Urgent"),
            Route::Projects {} => ("projects", ""),
            Route::ProjectsPersonal {} => ("projects", "Personal"),
            Route::ProjectsGithub {} => ("projects", "Github Repos"),
            Route::Calendar {} => ("calendar", ""),
            Route::Chat {} => ("chat", ""),
            Route::ChatMessaging {} => ("chat", "Messaging"),
            Route::ChatMail {} => ("chat", "Mail"),
            Route::ChatIssues {} => ("chat", "issues"),
            Route::Analytics {} => ("analytics", ""),
            Route::Mcp {} => ("mcp", ""),
            Route::Settings {} => ("settings", ""),
            Route::Components {} => ("components", ""),
            Route::ComponentsCalendar {} => ("components", "Calendar"),
        };
        app_state.active_nav.set(nav.to_string());
        app_state.selected_project.set(project.to_string());
    });

    let main_dialog_class = if overlay.is_dialog_open() {
        " relative z-50"
    } else {
        ""
    };

    rsx! {
        TooltipProvider {
            SidebarProvider {
                open: app_state.sidebar_open,
                default_open: true,
                class: "flex h-full w-full p-2 sm:p-3 gap-2 sm:gap-3 bg-background overflow-hidden",

                AppSidebar {}

                // ============================================================
                // Column 2: Middle Area - Card Panel (shared across pages)
                // ============================================================
                main { class: format!("flex-1 min-w-0 rounded-2xl border border-border/40 bg-card p-3 sm:p-6 flex flex-col h-full overflow-y-auto shadow-xs transition-colors duration-200{}", main_dialog_class),

                    // Top Bar: Panel toggle icon + Breadcrumb
                    div { class: "flex items-center justify-between pb-2 shrink-0",

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
                            BreadcrumbComponent {}
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
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
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

                    // Page content (swaps per route)
                    div { class: "flex-1 min-h-0 flex flex-col",
                        Outlet::<Route> {}
                    }
                }

                // ============================================================
                // Mobile drawer backdrops
                // ============================================================
                if *app_state.sidebar_open.read() {
                    div {
                        class: "fixed inset-0 z-30 bg-black/50 md:hidden",
                        onclick: move |_| app_state.sidebar_open.set(false),
                    }
                }

                if *app_state.calendar_sidebar_open.read() {
                    div {
                        class: "fixed inset-0 z-30 bg-black/50 xl:hidden",
                        onclick: move |_| app_state.calendar_sidebar_open.set(false),
                    }
                }

                // ============================================================
                // Column 3: Right Area - Custom Calendar & Notes Card Panel
                // ============================================================
                CalendarSidebar {}

                NewNoteDialog {}
            }
        }
    }
}
