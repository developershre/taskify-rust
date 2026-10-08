use dioxus::prelude::*;

use super::AppState;

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

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum TaskStatus {
    #[default]
    Backlog,
    InProgress,
    Completed,
    /// User-created board column.
    Custom(String),
}

impl TaskStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Backlog => "Backlog",
            Self::InProgress => "In Progress",
            Self::Completed => "Completed",
            Self::Custom(name) => name,
        }
    }

    pub fn badge_variant(&self) -> &'static str {
        match self {
            Self::Backlog => "outline",
            Self::InProgress => "secondary",
            Self::Completed => "default",
            Self::Custom(_) => "outline",
        }
    }
}

/// A single sub-task belonging to a parent task.
#[derive(Clone, Debug, PartialEq)]
pub struct SubTask {
    pub title: String,
    pub completed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TaskItem {
    pub id: String,
    pub title: String,
    pub description: String,
    pub project: String,
    pub priority: TaskPriority,
    pub completed: bool,
    pub status: TaskStatus,
    pub due_date: Option<(u32, u32, u32)>,
    pub subtasks: Vec<SubTask>,
}

impl AppState {
    /// Timestamp-based id for a freshly created task.
    fn new_task_id() -> String {
        format!(
            "task-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()
        )
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

        let new_id = Self::new_task_id();

        self.tasks.write().insert(
            0,
            TaskItem {
                id: new_id,
                title,
                description,
                project,
                priority,
                completed: false,
                status: TaskStatus::Backlog,
                due_date,
                subtasks: Vec::new(),
            },
        );
    }

    /// Quick-add a task directly into a board column, with optional subtasks.
    pub fn add_column_task(&mut self, title: String, status: TaskStatus, subtasks: Vec<String>) {
        let title = title.trim().to_string();
        if title.is_empty() {
            return;
        }
        let new_id = Self::new_task_id();
        let completed = status == TaskStatus::Completed;

        self.tasks.write().insert(
            0,
            TaskItem {
                id: new_id,
                title,
                description: String::new(),
                project: "General".to_string(),
                priority: TaskPriority::Medium,
                completed,
                status,
                due_date: None,
                subtasks: subtasks
                    .into_iter()
                    .map(|t| SubTask {
                        title: t,
                        completed: false,
                    })
                    .collect(),
            },
        );
    }

    pub fn toggle_task(&mut self, id: &str) {
        let mut list = self.tasks.write();
        if let Some(task) = list.iter_mut().find(|t| t.id == id) {
            task.completed = !task.completed;
            task.status = if task.completed {
                TaskStatus::Completed
            } else {
                TaskStatus::Backlog
            };
        }
    }

    pub fn set_task_status(&mut self, id: &str, status: TaskStatus) {
        let mut list = self.tasks.write();
        if let Some(task) = list.iter_mut().find(|t| t.id == id) {
            task.completed = status == TaskStatus::Completed;
            task.status = status;
        }
    }

    pub fn delete_task(&mut self, id: &str) {
        self.tasks.write().retain(|t| t.id != id);
    }

    pub fn add_subtask(&mut self, task_id: &str, title: String) {
        let title = title.trim().to_string();
        if title.is_empty() {
            return;
        }
        if let Some(task) = self
            .tasks
            .write()
            .iter_mut()
            .find(|t| t.id == task_id)
        {
            task.subtasks.push(SubTask {
                title,
                completed: false,
            });
        }
    }

    pub fn toggle_subtask(&mut self, task_id: &str, index: usize) {
        if let Some(task) = self.tasks.write().iter_mut().find(|t| t.id == task_id) {
            if let Some(sub) = task.subtasks.get_mut(index) {
                sub.completed = !sub.completed;
            }
        }
    }

    pub fn remove_subtask(&mut self, task_id: &str, index: usize) {
        if let Some(task) = self.tasks.write().iter_mut().find(|t| t.id == task_id) {
            if index < task.subtasks.len() {
                task.subtasks.remove(index);
            }
        }
    }

    /// Append a new board column (case-insensitive duplicates rejected).
    pub fn add_board_column(&mut self, name: String) {
        let name = name.trim().to_string();
        if name.is_empty() {
            return;
        }
        let mut cols = self.board_columns.write();
        if cols.iter().any(|c| c.eq_ignore_ascii_case(&name)) {
            return;
        }
        cols.push(name);
    }

    /// Move a board column from index `from` to index `to` (insert-at-index).
    pub fn move_board_column(&mut self, from: usize, to: usize) {
        if from == to {
            return;
        }
        let mut cols = self.board_columns.write();
        if from >= cols.len() {
            return;
        }
        let name = cols.remove(from);
        let to = to.min(cols.len());
        cols.insert(to, name);
    }

    pub fn total_tasks_count(&self) -> usize {
        self.tasks.read().len()
    }

    pub fn pending_tasks_count(&self) -> usize {
        self.tasks.read().iter().filter(|t| !t.completed).count()
    }

    pub fn completed_tasks_count(&self) -> usize {
        self.tasks.read().iter().filter(|t| t.completed).count()
    }

    pub fn filtered_tasks(&self) -> Vec<TaskItem> {
        let list = self.tasks.read();
        let proj = self.selected_project.read();
        let show_all = *self.show_completed.read();

        list.iter()
            .filter(|t| {
                let project_match = proj.as_str() == "All Projects"
                    || proj.as_str() == "All Tasks"
                    || t.project == *proj;
                let completion_match = show_all || !t.completed;
                project_match && completion_match
            })
            .cloned()
            .collect()
    }
}

// ============================================================
// Default Initial Data
// ============================================================

pub fn initial_tasks() -> Vec<TaskItem> {
    vec![
        TaskItem {
            id: "task-1".to_string(),
            title: "Replicate shadcn sidebar & calendar".to_string(),
            description:
                "Build custom calendar sidebar, collapsible menus with tree guides and actions."
                    .to_string(),
            project: "UI Components".to_string(),
            priority: TaskPriority::Urgent,
            completed: false,
            status: TaskStatus::InProgress,
            due_date: Some((2024, 9, 10)),
            subtasks: vec![
                SubTask {
                    title: "Sidebar shell + collapse logic".to_string(),
                    completed: true,
                },
                SubTask {
                    title: "Tree guide styling".to_string(),
                    completed: true,
                },
                SubTask {
                    title: "Action menu on hover".to_string(),
                    completed: false,
                },
                SubTask {
                    title: "Rich hover tooltips".to_string(),
                    completed: false,
                },
            ],
        },
        TaskItem {
            id: "task-2".to_string(),
            title: "Follow system theme dynamically".to_string(),
            description:
                "Default to system theme without forcing black theme, and allow user customization."
                    .to_string(),
            project: "Rust Desktop".to_string(),
            priority: TaskPriority::High,
            completed: true,
            status: TaskStatus::Completed,
            due_date: Some((2024, 9, 10)),
            subtasks: vec![
                SubTask {
                    title: "Detect system theme".to_string(),
                    completed: true,
                },
                SubTask {
                    title: "User override setting".to_string(),
                    completed: true,
                },
            ],
        },
        TaskItem {
            id: "task-3".to_string(),
            title: "Collapsible icon sidebar with tooltips".to_string(),
            description:
                "Shrink sidebar to 48px, hide labels & badges, and show rich hover tooltips."
                    .to_string(),
            project: "UI Components".to_string(),
            priority: TaskPriority::High,
            completed: true,
            status: TaskStatus::Completed,
            due_date: Some((2024, 9, 11)),
            subtasks: Vec::new(),
        },
        TaskItem {
            id: "task-4".to_string(),
            title: "Release v1.0 desktop binary".to_string(),
            description:
                "Package production application with custom frameless window and system tray."
                    .to_string(),
            project: "Release v1.0".to_string(),
            priority: TaskPriority::Medium,
            completed: false,
            status: TaskStatus::Backlog,
            due_date: Some((2024, 9, 15)),
            subtasks: vec![
                SubTask {
                    title: "Set up CI pipeline".to_string(),
                    completed: false,
                },
                SubTask {
                    title: "Code signing".to_string(),
                    completed: false,
                },
                SubTask {
                    title: "Write changelog".to_string(),
                    completed: false,
                },
            ],
        },
    ]
}
