use dioxus::prelude::*;

// ============================================================
// Theme Mode
// ============================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ThemeMode {
    #[default]
    Dark,
    Light,
    System,
}

impl ThemeMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Dark => "dark",
            Self::Light => "light",
            Self::System => "system",
        }
    }

    pub fn from_str_loose(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "light" => Self::Light,
            "system" => Self::System,
            _ => Self::Dark,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Dark => "Dark",
            Self::Light => "Light",
            Self::System => "System",
        }
    }

    pub fn is_dark(&self) -> bool {
        match self {
            Self::Dark => true,
            Self::Light => false,
            Self::System => true, // Default system fallback to dark in this app
        }
    }
}

// ============================================================
// Task Models & Priorities
// ============================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TaskPriority {
    Low,
    #[default]
    Medium,
    High,
    Urgent,
}

impl TaskPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Urgent => "Urgent",
        }
    }

    pub fn badge_variant(&self) -> &'static str {
        match self {
            Self::Low => "outline",
            Self::Medium => "secondary",
            Self::High => "default",
            Self::Urgent => "destructive",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TaskItem {
    pub id: String,
    pub title: String,
    pub description: String,
    pub project: String,
    pub priority: TaskPriority,
    pub completed: bool,
    pub due_date: Option<(u32, u32, u32)>,
}

// ============================================================
// Global Application State
// ============================================================

#[derive(Clone, Copy, PartialEq)]
pub struct AppState {
    pub theme: Signal<ThemeMode>,
    pub active_nav: Signal<String>,
    pub search_open: Signal<bool>,
    pub tasks: Signal<Vec<TaskItem>>,
    pub selected_project: Signal<String>,
    pub show_completed: Signal<bool>,
    pub auto_sync: Signal<bool>,
    pub selected_date: Signal<Option<(u32, u32, u32)>>,
}

impl AppState {
    pub fn is_dark(&self) -> bool {
        self.theme.read().is_dark()
    }

    pub fn toggle_theme(&mut self) {
        let next = if self.is_dark() {
            ThemeMode::Light
        } else {
            ThemeMode::Dark
        };
        self.set_theme(next);
    }

    pub fn set_theme(&mut self, mode: ThemeMode) {
        self.theme.set(mode);

        let theme_str = mode.as_str();
        let is_dark = mode.is_dark();

        // Sync with webview localStorage & documentElement.classList
        let js = format!(
            r#"
            try {{
                localStorage.setItem('taskify-theme', '{theme_str}');
                if ({is_dark}) {{
                    document.documentElement.classList.add('dark');
                }} else {{
                    document.documentElement.classList.remove('dark');
                }}
            }} catch (e) {{}}
            "#
        );
        let _ = document::eval(&js);
    }

    pub fn add_task(
        &mut self,
        title: String,
        description: String,
        project: String,
        priority: TaskPriority,
        due_date: Option<(u32, u32, u32)>,
    ) {
        if title.trim().is_empty() {
            return;
        }

        let new_id = format!("task-{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis());

        self.tasks.write().insert(0, TaskItem {
            id: new_id,
            title,
            description,
            project,
            priority,
            completed: false,
            due_date,
        });
    }

    pub fn toggle_task(&mut self, id: &str) {
        let mut list = self.tasks.write();
        if let Some(task) = list.iter_mut().find(|t| t.id == id) {
            task.completed = !task.completed;
        }
    }

    pub fn delete_task(&mut self, id: &str) {
        self.tasks.write().retain(|t| t.id != id);
    }

    pub fn total_tasks_count(&self) -> usize {
        self.tasks.read().len()
    }

    pub fn completed_tasks_count(&self) -> usize {
        self.tasks.read().iter().filter(|t| t.completed).count()
    }

    pub fn pending_tasks_count(&self) -> usize {
        self.tasks.read().iter().filter(|t| !t.completed).count()
    }

    pub fn filtered_tasks(&self) -> Vec<TaskItem> {
        let tasks = self.tasks.read();
        let proj = self.selected_project.read().clone();
        let show_comp = *self.show_completed.read();

        tasks
            .iter()
            .filter(|t| {
                if !show_comp && t.completed {
                    return false;
                }
                if proj != "All Projects" && t.project != proj {
                    return false;
                }
                true
            })
            .cloned()
            .collect()
    }
}

// Initial sample tasks
fn initial_tasks() -> Vec<TaskItem> {
    vec![
        TaskItem {
            id: "task-1".to_string(),
            title: "Command palette & global shortcuts".to_string(),
            description: "Fixed title search positioning, keyboard event propagation, and outside click dismissal.".to_string(),
            project: "Rust Desktop".to_string(),
            priority: TaskPriority::High,
            completed: true,
            due_date: Some((2026, 10, 5)),
        },
        TaskItem {
            id: "task-2".to_string(),
            title: "Replicate 10 shadcn/ui components".to_string(),
            description: "Built Dialog, Sidebar, Avatar, Calendar, Select, Item, Tooltip, Switch, Breadcrumb, Badge.".to_string(),
            project: "UI Components".to_string(),
            priority: TaskPriority::Urgent,
            completed: true,
            due_date: Some((2026, 10, 5)),
        },
        TaskItem {
            id: "task-3".to_string(),
            title: "Collapsible icon sidebar with tooltips".to_string(),
            description: "Shrink sidebar to 48px, hide labels & badges, and show rich hover tooltips.".to_string(),
            project: "UI Components".to_string(),
            priority: TaskPriority::High,
            completed: true,
            due_date: Some((2026, 10, 6)),
        },
        TaskItem {
            id: "task-4".to_string(),
            title: "Release v1.0 desktop binary".to_string(),
            description: "Package production application with custom frameless window and system tray.".to_string(),
            project: "Release v1.0".to_string(),
            priority: TaskPriority::Medium,
            completed: false,
            due_date: Some((2026, 10, 15)),
        },
    ]
}

// ============================================================
// Initialization Hook
// ============================================================

pub fn use_init_app_state() -> AppState {
    let mut theme = use_signal(|| ThemeMode::Dark);
    let active_nav = use_signal(|| "dashboard".to_string());
    let search_open = use_signal(|| false);
    let tasks = use_signal(initial_tasks);
    let selected_project = use_signal(|| "All Projects".to_string());
    let show_completed = use_signal(|| false);
    let auto_sync = use_signal(|| true);
    let selected_date = use_signal(|| Some((2026, 10, 5)));

    let state = AppState {
        theme,
        active_nav,
        search_open,
        tasks,
        selected_project,
        show_completed,
        auto_sync,
        selected_date,
    };

    use_context_provider(|| state);

    // Initial theme restoration from localStorage
    use_effect(move || {
        let mut eval = document::eval(
            r#"
            try {
                const saved = localStorage.getItem('taskify-theme') || 'dark';
                if (saved === 'dark') {
                    document.documentElement.classList.add('dark');
                } else {
                    document.documentElement.classList.remove('dark');
                }
                dioxus.send(saved);
            } catch (e) {
                dioxus.send('dark');
            }
            "#
        );

        spawn(async move {
            if let Ok(saved_theme) = eval.recv::<String>().await {
                let parsed = ThemeMode::from_str_loose(&saved_theme);
                theme.set(parsed);
            }
        });
    });

    state
}

pub fn use_app_state() -> AppState {
    use_context::<AppState>()
}

pub fn use_theme() -> Signal<ThemeMode> {
    use_app_state().theme
}
