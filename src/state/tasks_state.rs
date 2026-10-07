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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TaskStatus {
    #[default]
    Backlog,
    InProgress,
    Completed,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Backlog => "Backlog",
            Self::InProgress => "In Progress",
            Self::Completed => "Completed",
        }
    }

    pub fn badge_variant(&self) -> &'static str {
        match self {
            Self::Backlog => "outline",
            Self::InProgress => "secondary",
            Self::Completed => "default",
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
    pub status: TaskStatus,
    pub due_date: Option<(u32, u32, u32)>,
}

impl AppState {
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

        let new_id = format!(
            "task-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()
        );

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
            task.status = status;
            task.completed = status == TaskStatus::Completed;
        }
    }

    pub fn delete_task(&mut self, id: &str) {
        self.tasks.write().retain(|t| t.id != id);
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
        },
    ]
}
