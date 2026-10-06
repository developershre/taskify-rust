use dioxus::prelude::*;

use crate::components::ui::{
    Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator,
    DropdownMenuShortcut, DropdownMenuTrigger, Switch,
};
use crate::components::user::User;
use crate::components::{SearchContext, TitleSearch};
use crate::icons::{CloseIcon, MaximizeIcon, MinimizeIcon, MoonIcon, SunIcon};
use crate::state::{get_real_current_date, use_app_state, AppState, TaskPriority, ThemeMode};

/// Shared state for the New Task dialog (titlebar provides, command palette consumes).
#[derive(Clone, Copy, PartialEq)]
pub struct NewTaskForm {
    pub open: Signal<bool>,
    pub title: Signal<String>,
    pub description: Signal<String>,
    pub project: Signal<String>,
    pub priority: Signal<TaskPriority>,
    pub due_today: Signal<bool>,
}

fn submit_new_task(mut app_state: AppState, mut form: NewTaskForm) {
    let title = form.title.read().trim().to_string();
    if title.is_empty() {
        return;
    }
    let description = form.description.read().trim().to_string();
    let project = form.project.read().trim().to_string();
    let priority = *form.priority.read();
    let due_date = if *form.due_today.read() {
        Some(get_real_current_date())
    } else {
        None
    };

    app_state.add_task(
        title,
        description,
        if project.is_empty() {
            "General".to_string()
        } else {
            project
        },
        priority,
        due_date,
    );

    form.title.set(String::new());
    form.description.set(String::new());
    form.project.set(String::new());
    form.priority.set(TaskPriority::Medium);
    form.due_today.set(false);
    form.open.set(false);
}

fn set_zoom(mut zoom: Signal<u32>, pct: u32) {
    let pct = pct.clamp(50, 200);
    zoom.set(pct);
    dioxus::desktop::window().set_zoom_level(pct as f64 / 100.0);
}

#[component]
pub fn TitleBar() -> Element {
    let mut app_state = use_app_state();
    let mut search_open = app_state.search_open;
    use_context_provider(|| SearchContext { open: search_open });

    let mut new_task = NewTaskForm {
        open: use_signal(|| false),
        title: use_signal(String::new),
        description: use_signal(String::new),
        project: use_signal(String::new),
        priority: use_signal(|| TaskPriority::Medium),
        due_today: use_signal(|| false),
    };
    use_context_provider(|| new_task);

    let mut show_shortcuts = use_signal(|| false);
    let mut show_about = use_signal(|| false);
    let zoom = use_signal(|| 100u32);

    let router = router();

    // Register all application keyboard shortcuts
    use_effect(move || {
        let mut eval = document::eval(
            r#"
            window.addEventListener('keydown', (e) => {
                const isInput = e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA' || e.target.isContentEditable;
                const isCtrlOrMeta = e.ctrlKey || e.metaKey;

                // Ctrl+K / Cmd+K: Always toggle search palette
                if (isCtrlOrMeta && e.key.toLowerCase() === 'k') {
                    e.preventDefault();
                    dioxus.send('search.toggle');
                    return;
                }

                // Escape: Always close palettes/menus/dialogs
                if (e.key === 'Escape') {
                    dioxus.send('escape');
                    return;
                }

                // F11: Toggle maximize
                if (e.key === 'F11') {
                    e.preventDefault();
                    dioxus.send('window.toggle_maximize');
                    return;
                }

                // Ctrl+Q: Exit app
                if (isCtrlOrMeta && e.key.toLowerCase() === 'q') {
                    e.preventDefault();
                    dioxus.send('window.close');
                    return;
                }

                // Other shortcuts when not typing in an input
                if (isCtrlOrMeta && !isInput) {
                    const key = e.key.toLowerCase();
                    if (e.shiftKey && key === 'n') {
                        e.preventDefault();
                        dioxus.send('notes.toggle');
                    } else if (key === 'n') {
                        e.preventDefault();
                        dioxus.send('task.new');
                    } else if (key === 't') {
                        e.preventDefault();
                        dioxus.send('theme.toggle');
                    } else if (key === 'b') {
                        e.preventDefault();
                        dioxus.send('sidebar.toggle');
                    } else if (e.key === '=' || e.key === '+') {
                        e.preventDefault();
                        dioxus.send('zoom.in');
                    } else if (e.key === '-') {
                        e.preventDefault();
                        dioxus.send('zoom.out');
                    } else if (e.key === '0') {
                        e.preventDefault();
                        dioxus.send('zoom.reset');
                    }
                }
            });
            "#,
        );

        spawn(async move {
            let win = dioxus::desktop::window();
            while let Ok(msg) = eval.recv::<String>().await {
                match msg.as_str() {
                    "search.toggle" => {
                        let current = *search_open.read();
                        search_open.set(!current);
                    }
                    "escape" => {
                        search_open.set(false);
                        new_task.open.set(false);
                        show_shortcuts.set(false);
                        show_about.set(false);
                    }
                    "task.new" => {
                        new_task.open.set(true);
                    }
                    "notes.toggle" => {
                        app_state.toggle_calendar_sidebar();
                    }
                    "theme.toggle" => {
                        app_state.toggle_theme();
                    }
                    "sidebar.toggle" => {
                        app_state.toggle_sidebar();
                    }
                    "zoom.in" => {
                        let z = *zoom.read();
                        set_zoom(zoom, z + 10);
                    }
                    "zoom.out" => {
                        let z = *zoom.read();
                        set_zoom(zoom, z - 10);
                    }
                    "zoom.reset" => {
                        set_zoom(zoom, 100);
                    }
                    "window.close" => {
                        win.close();
                    }
                    "window.toggle_maximize" => {
                        win.toggle_maximized();
                    }
                    _ => {}
                }
            }
        });
    });

    let is_dark = app_state.is_dark();
    let theme_root_class = if is_dark { "dark" } else { "" };

    rsx! {
            div {
                class: "{theme_root_class} fixed top-0 left-0 flex flex-col h-screen w-screen overflow-hidden bg-background text-foreground select-none",

                header {
                    class: "w-full flex items-center p-2 bg-background border-b border-border/40 shrink-0",
                // ============================================================
                // Logo
                // ============================================================
                div {
                    class: "
                    flex
                    items-center
                    gap-2
                    px-2
                    cursor-default
                    shrink-0
                ",

                    img {
                        src: asset!("/assets/logo.png"),
                        class: "size-6 pointer-events-none",
                    }
                }

                // ============================================================
                // Titlebar Menus (File, Task, View, Go, Help)
                // ============================================================
                div {
                    class: "flex items-center gap-0.5 shrink-0",

                    // File
                    DropdownMenu {
                        DropdownMenuTrigger {
                            class: "px-2 py-1 text-xs text-muted-foreground hover:text-foreground hover:bg-muted/80 rounded transition-colors cursor-pointer",
                            "File"
                        }
                        DropdownMenuContent {
                            class: "w-56",
                            DropdownMenuItem {
                                onclick: move |_| new_task.open.set(true),
                                span { "New Task..." }
                                DropdownMenuShortcut { "Ctrl+N" }
                            }
                            DropdownMenuItem {
                                onclick: move |_| {
                                    app_state.add_note(
                                        "Untitled note".to_string(),
                                        "Write something worth keeping.".to_string(),
                                    );
                                    let notes_open = *app_state.calendar_sidebar_open.read();
                                    if !notes_open {
                                        app_state.toggle_calendar_sidebar();
                                    }
                                },
                                span { "New Note" }
                            }
                            DropdownMenuSeparator {}
                            DropdownMenuItem {
                                onclick: move |_| {
                                    let _ = router.push("/settings");
                                },
                                span { "Settings" }
                            }
                            DropdownMenuSeparator {}
                            DropdownMenuItem {
                                variant: "destructive".to_string(),
                                onclick: move |_| dioxus::desktop::window().close(),
                                span { "Exit" }
                                DropdownMenuShortcut { "Ctrl+Q" }
                            }
                        }
                    }

                    // Task
                    DropdownMenu {
                        DropdownMenuTrigger {
                            class: "px-2 py-1 text-xs text-muted-foreground hover:text-foreground hover:bg-muted/80 rounded transition-colors cursor-pointer",
                            "Task"
                        }
                        DropdownMenuContent {
                            class: "w-56",
                            DropdownMenuItem {
                                onclick: move |_| {
                                    let ids: Vec<String> = app_state
                                        .tasks
                                        .read()
                                        .iter()
                                        .filter(|t| !t.completed)
                                        .map(|t| t.id.clone())
                                        .collect();
                                    for id in ids {
                                        app_state.toggle_task(&id);
                                    }
                                },
                                span { "Complete All Tasks" }
                            }
                            DropdownMenuItem {
                                onclick: move |_| {
                                    let ids: Vec<String> = app_state
                                        .tasks
                                        .read()
                                        .iter()
                                        .filter(|t| t.completed)
                                        .map(|t| t.id.clone())
                                        .collect();
                                    for id in ids {
                                        app_state.delete_task(&id);
                                    }
                                },
                                span { "Clear Completed Tasks" }
                            }
                            DropdownMenuSeparator {}
                            DropdownMenuItem {
                                onclick: move |_| {
                                    let next = !*app_state.show_completed.read();
                                    app_state.show_completed.set(next);
                                },
                                span { "Show Completed" }
                                if *app_state.show_completed.read() {
                                    span { class: "ml-auto text-xs text-primary font-bold", "✓" }
                                }
                            }
                        }
                    }

                    // View
                    DropdownMenu {
                        DropdownMenuTrigger {
                            class: "px-2 py-1 text-xs text-muted-foreground hover:text-foreground hover:bg-muted/80 rounded transition-colors cursor-pointer",
                            "View"
                        }
                        DropdownMenuContent {
                            class: "w-60",
                            DropdownMenuItem {
                                onclick: move |_| app_state.toggle_sidebar(),
                                span { "Toggle Sidebar" }
                                DropdownMenuShortcut { "Ctrl+B" }
                            }
                            DropdownMenuItem {
                                onclick: move |_| app_state.toggle_calendar_sidebar(),
                                span { "Toggle Notes Panel" }
                                DropdownMenuShortcut { "Ctrl+Shift+N" }
                            }
                            DropdownMenuSeparator {}
                            DropdownMenuItem {
                                onclick: move |_| {
                                    let z = *zoom.read();
                                    set_zoom(zoom, z + 10);
                                },
                                span { "Zoom In" }
                                DropdownMenuShortcut { "Ctrl+=" }
                            }
                            DropdownMenuItem {
                                onclick: move |_| {
                                    let z = *zoom.read();
                                    set_zoom(zoom, z - 10);
                                },
                                span { "Zoom Out" }
                                DropdownMenuShortcut { "Ctrl+-" }
                            }
                            DropdownMenuItem {
                                disabled: *zoom.read() == 100,
                                onclick: move |_| set_zoom(zoom, 100),
                                span { "Reset Zoom ({zoom}%)" }
                                DropdownMenuShortcut { "Ctrl+0" }
                            }
                            DropdownMenuSeparator {}
                            DropdownMenuItem {
                                onclick: move |_| app_state.set_theme(ThemeMode::Dark),
                                span { "Dark" }
                                if is_dark {
                                    span { class: "ml-auto text-xs text-primary font-bold", "✓" }
                                }
                            }
                            DropdownMenuItem {
                                onclick: move |_| app_state.set_theme(ThemeMode::Light),
                                span { "Light" }
                                if !is_dark {
                                    span { class: "ml-auto text-xs text-primary font-bold", "✓" }
                                }
                            }
                            DropdownMenuItem {
                                onclick: move |_| app_state.toggle_theme(),
                                span { "Toggle Theme" }
                                DropdownMenuShortcut { "Ctrl+T" }
                            }
                            DropdownMenuSeparator {}
                            DropdownMenuItem {
                                onclick: move |_| dioxus::desktop::window().toggle_maximized(),
                                span { "Toggle Maximize" }
                                DropdownMenuShortcut { "F11" }
                            }
                        }
                    }

                    // Go
                    DropdownMenu {
                        DropdownMenuTrigger {
                            class: "px-2 py-1 text-xs text-muted-foreground hover:text-foreground hover:bg-muted/80 rounded transition-colors cursor-pointer",
                            "Go"
                        }
                        DropdownMenuContent {
                            class: "w-48",
                            DropdownMenuItem {
                                onclick: move |_| {
                                    let _ = router.push("/");
                                },
                                span { "Dashboard" }
                            }
                            DropdownMenuItem {
                                onclick: move |_| {
                                    let _ = router.push("/tasks/all");
                                },
                                span { "All Tasks" }
                            }
                            DropdownMenuItem {
                                onclick: move |_| {
                                    let _ = router.push("/tasks/urgent");
                                },
                                span { "Urgent Tasks" }
                            }
                            DropdownMenuSeparator {}
                            DropdownMenuItem {
                                onclick: move |_| {
                                    let _ = router.push("/projects");
                                },
                                span { "Projects" }
                            }
                            DropdownMenuItem {
                                onclick: move |_| {
                                    let _ = router.push("/calendar");
                                },
                                span { "Calendar" }
                            }
                            DropdownMenuItem {
                                onclick: move |_| {
                                    let _ = router.push("/analytics");
                                },
                                span { "Analytics" }
                            }
                        }
                    }

                    // Help
                    DropdownMenu {
                        DropdownMenuTrigger {
                            class: "px-2 py-1 text-xs text-muted-foreground hover:text-foreground hover:bg-muted/80 rounded transition-colors cursor-pointer",
                            "Help"
                        }
                        DropdownMenuContent {
                            class: "w-52",
                            DropdownMenuItem {
                                onclick: move |_| show_shortcuts.set(true),
                                span { "Keyboard Shortcuts..." }
                            }
                            DropdownMenuSeparator {}
                            DropdownMenuItem {
                                onclick: move |_| show_about.set(true),
                                span { "About Taskify" }
                            }
                        }
                    }
                }
                // ============================================================
                // Draggable Area
                // ============================================================
                div {
                    class: "
                    flex-1
                    h-full
                    flex
                    items-center
                    justify-center
                    cursor-default
                ",

                    onmousedown: move |_| {
                        dioxus::desktop::window().drag();
                    },

                    TitleSearch{},
                }

                // ============================================================
                // Window Controls & Theme Toggle
                // ============================================================
                div {
                    class: "flex items-center gap-1.5",

                    div{
                        class: "flex items-center justify-center gap-1.5",
                        User{},

                        // Theme Quick Toggle
                        button {
                            class: "p-1.5 hover:bg-accent text-muted-foreground hover:text-foreground rounded-md transition-colors cursor-pointer mr-0.5",
                            title: if is_dark { "Switch to Light Theme (Ctrl+T)" } else { "Switch to Dark Theme (Ctrl+T)" },
                            onclick: move |_| {
                                app_state.toggle_theme();
                            },
                            if is_dark {
                                SunIcon { class: "size-3.5".to_string() }
                            } else {
                                MoonIcon { class: "size-3.5".to_string() }
                            }
                        }
                    }
                    // Minimize
                    button {
                        class: "p-2 hover:bg-accent text-muted-foreground hover:text-foreground rounded-sm cursor-pointer transition-colors",

                        onclick: move |_| {
                            dioxus::desktop::window().window.set_minimized(true);
                        },

                        MinimizeIcon {
                            class: "size-3".to_string(),
                        }
                    }

                    // Maximize
                    button {
                        class: "p-2 hover:bg-accent text-muted-foreground hover:text-foreground rounded-sm cursor-pointer transition-colors",
                        onclick: move |_| {
                            dioxus::desktop::window().toggle_maximized();
                        },

                        MaximizeIcon {
                            class: "size-3".to_string(),
                        }
                    }

                    // Close
                    button {
                        class: "p-2 hover:bg-destructive hover:text-white rounded-sm cursor-pointer transition-colors",

                        onclick: move |_| {
                            dioxus::desktop::window().close();
                        },

                        CloseIcon {
                            class: "size-3".to_string(),
                        }
                    }
            }

            }

            main {
                class: "flex-1 overflow-hidden flex select-text",
                Outlet::<crate::Route> {}
            }

            // ============================================================
            // Dialogs (New Task, Shortcuts, About)
            // ============================================================
            Dialog { open: new_task.open,
                DialogContent {
                    DialogHeader {
                        DialogTitle { "New Task" }
                        DialogDescription { "Add a task to your workspace. It shows up in your lists right away." }
                    }

                    div { class: "flex flex-col gap-4",
                        div { class: "flex flex-col gap-1.5",
                            label { class: "text-xs font-medium text-foreground", "Title" }
                            input {
                                r#type: "text",
                                class: "w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring",
                                placeholder: "e.g. Design the onboarding flow",
                                value: "{new_task.title}",
                                autofocus: true,
                                oninput: move |e| new_task.title.set(e.value()),
                                onkeydown: move |e: KeyboardEvent| {
                                    if e.key() == Key::Enter {
                                        submit_new_task(app_state, new_task);
                                    }
                                },
                            }
                        }

                        div { class: "flex flex-col gap-1.5",
                            label { class: "text-xs font-medium text-foreground", "Description" }
                            textarea {
                                class: "min-h-16 w-full resize-none rounded-md border border-input bg-transparent px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring",
                                placeholder: "Optional details...",
                                value: "{new_task.description}",
                                oninput: move |e| new_task.description.set(e.value()),
                            }
                        }

                        div { class: "flex flex-col gap-1.5",
                            label { class: "text-xs font-medium text-foreground", "Project" }
                            input {
                                r#type: "text",
                                class: "w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring",
                                placeholder: "General",
                                value: "{new_task.project}",
                                oninput: move |e| new_task.project.set(e.value()),
                            }
                        }

                        div { class: "flex flex-col gap-1.5",
                            label { class: "text-xs font-medium text-foreground", "Priority" }
                            div { class: "flex items-center gap-2",
                                for p in [TaskPriority::Low, TaskPriority::Medium, TaskPriority::High, TaskPriority::Urgent] {
                                    {
                                        let is_active = *new_task.priority.read() == p;
                                        let active_class = if is_active {
                                            "border-primary bg-secondary text-foreground"
                                        } else {
                                            "border-border text-muted-foreground hover:bg-muted hover:text-foreground"
                                        };
                                        let label = p.as_str();
                                        rsx! {
                                            button {
                                                key: "{label}",
                                                r#type: "button",
                                                class: "inline-flex h-8 items-center rounded-md border px-3 text-xs font-medium transition-colors cursor-pointer {active_class}",
                                                onclick: move |_| {
                                                    new_task.priority.set(p);
                                                },
                                                "{label}"
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "flex items-center justify-between gap-4",
                            div { class: "flex flex-col",
                                span { class: "text-xs font-medium text-foreground", "Due today" }
                                span { class: "text-[11px] text-muted-foreground", "Sets the due date to today." }
                            }
                            Switch {
                                checked: new_task.due_today,
                                onchange: move |v| new_task.due_today.set(v),
                            }
                        }
                    }

                    DialogFooter {
                        button {
                            r#type: "button",
                            class: "inline-flex h-8 items-center rounded-md border border-border px-3 text-xs font-medium text-foreground hover:bg-muted transition-colors cursor-pointer",
                            onclick: move |_| new_task.open.set(false),
                            "Cancel"
                        }
                        button {
                            r#type: "button",
                            class: "inline-flex h-8 items-center rounded-md bg-primary px-3 text-xs font-medium text-primary-foreground hover:opacity-90 transition-opacity cursor-pointer disabled:opacity-50",
                            disabled: new_task.title.read().trim().is_empty(),
                            onclick: move |_| submit_new_task(app_state, new_task),
                            "Add Task"
                        }
                    }
                }
            }

            Dialog { open: show_shortcuts,
                DialogContent { class: "max-w-md",
                    DialogHeader {
                        DialogTitle { "Keyboard Shortcuts" }
                        DialogDescription { "Speed up your workflow with these shortcuts." }
                    }
                    div { class: "flex flex-col gap-2",
                        for (label, keys) in [
                            ("New task", "Ctrl+N"),
                            ("Command palette", "Ctrl+K"),
                            ("Toggle sidebar", "Ctrl+B"),
                            ("Toggle notes panel", "Ctrl+Shift+N"),
                            ("Toggle theme", "Ctrl+T"),
                            ("Zoom in", "Ctrl+="),
                            ("Zoom out", "Ctrl+-"),
                            ("Reset zoom", "Ctrl+0"),
                            ("Toggle maximize", "F11"),
                            ("Exit", "Ctrl+Q"),
                        ] {
                            div { key: "{label}", class: "flex items-center justify-between gap-4",
                                span { class: "text-xs text-foreground", "{label}" }
                                kbd { class: "pointer-events-none inline-flex h-5 select-none items-center rounded border bg-muted px-1.5 font-mono text-[10px] font-medium text-muted-foreground",
                                    "{keys}"
                                }
                            }
                        }
                    }
                }
            }

            Dialog { open: show_about,
                DialogContent { class: "max-w-sm",
                    DialogHeader {
                        DialogTitle { "About Taskify" }
                        DialogDescription { "Task management, minus the clutter." }
                    }
                    div { class: "flex flex-col gap-2.5",
                        div { class: "flex items-center justify-between gap-4",
                            span { class: "text-xs text-muted-foreground", "Version" }
                            span { class: "text-xs font-medium text-foreground", "0.1.0" }
                        }
                        div { class: "flex items-center justify-between gap-4",
                            span { class: "text-xs text-muted-foreground", "Built with" }
                            span { class: "text-xs font-medium text-foreground", "Dioxus 0.7 · Rust" }
                        }
                        div { class: "flex items-center justify-between gap-4",
                            span { class: "text-xs text-muted-foreground", "Platform" }
                            span { class: "text-xs font-medium text-foreground", "Windows · WebView2" }
                        }
                    }
                    DialogFooter {
                        button {
                            r#type: "button",
                            class: "inline-flex h-8 items-center rounded-md bg-primary px-3 text-xs font-medium text-primary-foreground hover:opacity-90 transition-opacity cursor-pointer",
                            onclick: move |_| show_about.set(false),
                            "Close"
                        }
                    }
                }
            }
        }
    }
}
