use dioxus::prelude::*;

pub mod tasks_state;

pub use tasks_state::{TaskItem, TaskPriority, TaskStatus};
use tasks_state::initial_tasks;

// ============================================================
// Theme Mode
// ============================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ThemeMode {
    #[default]
    System,
    Dark,
    Light,
}

impl ThemeMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Dark => "dark",
            Self::Light => "light",
        }
    }

    pub fn from_str_loose(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "light" => Self::Light,
            "dark" => Self::Dark,
            _ => Self::System,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::System => "System",
            Self::Dark => "Dark",
            Self::Light => "Light",
        }
    }
}

// ============================================================
// Note Model (for Calendar / Notes Sidebar)
// ============================================================

#[derive(Clone, Debug, PartialEq)]
pub struct NoteItem {
    pub id: String,
    pub title: String,
    pub description: String,
}

// ============================================================
// Global Application State
// ============================================================

#[derive(Clone, Copy, PartialEq)]
pub struct AppState {
    pub theme: Signal<ThemeMode>,
    pub system_is_dark: Signal<bool>,
    pub active_nav: Signal<String>,
    pub search_open: Signal<bool>,
    pub tasks: Signal<Vec<TaskItem>>,
    pub notes: Signal<Vec<NoteItem>>,
    pub selected_project: Signal<String>,
    pub show_completed: Signal<bool>,
    pub auto_sync: Signal<bool>,
    pub selected_date: Signal<Option<(u32, u32, u32)>>,
    pub real_today: Signal<(u32, u32, u32)>,
    pub sidebar_open: Signal<bool>,
    pub calendar_sidebar_open: Signal<bool>,
}

impl AppState {
    pub fn is_dark(&self) -> bool {
        match *self.theme.read() {
            ThemeMode::Dark => true,
            ThemeMode::Light => false,
            ThemeMode::System => *self.system_is_dark.read(),
        }
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

        let is_dark = match mode {
            ThemeMode::Dark => true,
            ThemeMode::Light => false,
            ThemeMode::System => *self.system_is_dark.read(),
        };
        let theme_str = mode.as_str();

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

    pub fn toggle_sidebar(&mut self) {
        let cur = *self.sidebar_open.read();
        self.sidebar_open.set(!cur);
    }

    pub fn toggle_calendar_sidebar(&mut self) {
        let cur = *self.calendar_sidebar_open.read();
        self.calendar_sidebar_open.set(!cur);
    }

    // Notes management
    pub fn add_note(&mut self, title: String, description: String) {
        if title.trim().is_empty() {
            return;
        }
        let id = format!(
            "note-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()
        );
        self.notes.write().insert(
            0,
            NoteItem {
                id,
                title,
                description,
            },
        );
    }

    pub fn delete_note(&mut self, id: &str) {
        self.notes.write().retain(|n| n.id != id);
    }
}

// ============================================================
// Default Initial Data
// ============================================================

fn initial_notes() -> Vec<NoteItem> {
    vec![
        NoteItem {
            id: "note-1".to_string(),
            title: "New Tasks".to_string(),
            description: "A simple item with title and description.".to_string(),
        },
        NoteItem {
            id: "note-2".to_string(),
            title: "New Tasks".to_string(),
            description: "A simple item with title and description.".to_string(),
        },
        NoteItem {
            id: "note-3".to_string(),
            title: "New Tasks".to_string(),
            description: "A simple item with title and description.".to_string(),
        },
    ]
}

// ============================================================
// Real Date Calculation
// ============================================================

pub fn get_real_current_date() -> (u32, u32, u32) {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let days = (secs / 86400) as i64;
    let z = days + 719468;
    let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
    let doe = (z - era * 146097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = (yoe as i64 + era * 400) as u32;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let final_y = if m <= 2 { y + 1 } else { y };
    (final_y, m, d)
}

// ============================================================
// Initialization Hook
// ============================================================

pub fn use_init_app_state() -> AppState {
    let real_now = get_real_current_date();
    let mut theme = use_signal(|| ThemeMode::System);
    let mut system_is_dark = use_signal(|| false);
    let active_nav = use_signal(|| "dashboard".to_string());
    let search_open = use_signal(|| false);
    let tasks = use_signal(initial_tasks);
    let notes = use_signal(initial_notes);
    let selected_project = use_signal(|| "All Projects".to_string());
    let show_completed = use_signal(|| false);
    let auto_sync = use_signal(|| true);
    let mut selected_date = use_signal(move || Some(real_now));
    let mut real_today = use_signal(move || real_now);
    let sidebar_open = use_signal(|| true);
    let calendar_sidebar_open = use_signal(|| true);

    let state = AppState {
        theme,
        system_is_dark,
        active_nav,
        search_open,
        tasks,
        notes,
        selected_project,
        show_completed,
        auto_sync,
        selected_date,
        real_today,
        sidebar_open,
        calendar_sidebar_open,
    };

    use_context_provider(|| state);

    // Initial theme restoration and system color-scheme detection
    use_effect(move || {
        let mut eval = document::eval(
            r#"
            try {
                const mq = window.matchMedia('(prefers-color-scheme: dark)');
                const systemDark = mq.matches;
                const saved = localStorage.getItem('taskify-theme');

                let mode = saved || 'system';
                let effectiveDark = (mode === 'dark') || (mode === 'system' && systemDark);

                if (effectiveDark) {
                    document.documentElement.classList.add('dark');
                } else {
                    document.documentElement.classList.remove('dark');
                }

                dioxus.send(JSON.stringify({ mode: mode, system_dark: systemDark }));

                mq.addEventListener('change', (e) => {
                    const currentSaved = localStorage.getItem('taskify-theme') || 'system';
                    if (currentSaved === 'system') {
                        if (e.matches) {
                            document.documentElement.classList.add('dark');
                        } else {
                            document.documentElement.classList.remove('dark');
                        }
                    }
                    dioxus.send(JSON.stringify({ mode: 'change', system_dark: e.matches }));
                });
            } catch (e) {
                dioxus.send(JSON.stringify({ mode: 'system', system_dark: false }));
            }
            "#,
        );

        spawn(async move {
            while let Ok(msg) = eval.recv::<String>().await {
                let is_sys_dark = msg.contains("\"system_dark\":true");
                system_is_dark.set(is_sys_dark);

                if msg.contains("\"mode\":\"dark\"") {
                    theme.set(ThemeMode::Dark);
                } else if msg.contains("\"mode\":\"light\"") {
                    theme.set(ThemeMode::Light);
                } else if msg.contains("\"mode\":\"system\"") {
                    theme.set(ThemeMode::System);
                }
            }
        });

        let mut date_eval = document::eval(
            r#"
            try {
                const now = new Date();
                dioxus.send(`${now.getFullYear()}-${now.getMonth() + 1}-${now.getDate()}`);
            } catch (e) {}
            "#,
        );

        spawn(async move {
            if let Ok(date_str) = date_eval.recv::<String>().await {
                let parts: Vec<&str> = date_str.split('-').collect();
                if parts.len() == 3 {
                    if let (Ok(y), Ok(m), Ok(d)) = (
                        parts[0].parse::<u32>(),
                        parts[1].parse::<u32>(),
                        parts[2].parse::<u32>(),
                    ) {
                        real_today.set((y, m, d));
                        selected_date.set(Some((y, m, d)));
                    }
                }
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
