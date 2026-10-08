use dioxus::prelude::*;

use crate::components::TaskDetailContext;
use crate::components::ui::Badge;
use crate::icons::{CalendarIcon, CloseIcon, TrashIcon};
use crate::state::{use_app_state, AppState, TaskItem, TaskStatus};

use super::shared::{format_due, TaskCheck};

/// Movement (in pixels) required before a mousedown turns into a real drag.
const DRAG_THRESHOLD: f64 = 3.0;

/// Map a board column name to the matching task status.
fn column_status(name: &str) -> TaskStatus {
    match name {
        "Backlog" => TaskStatus::Backlog,
        "In Progress" => TaskStatus::InProgress,
        "Completed" => TaskStatus::Completed,
        other => TaskStatus::Custom(other.to_string()),
    }
}

/// Validate + commit a new column name, then reset the composer.
fn commit_column(mut app_state: AppState, mut new_col: Signal<String>, mut adding: Signal<bool>) {
    let name = new_col.read().trim().to_string();
    if name.is_empty() {
        return;
    }
    app_state.add_board_column(name);
    new_col.set(String::new());
    adding.set(false);
}

/// Move the subtask input into the staged list.
fn stage_subtask(mut subs: Signal<Vec<String>>, mut input: Signal<String>) {
    let title = input.read().trim().to_string();
    if title.is_empty() {
        return;
    }
    subs.write().push(title);
    input.set(String::new());
}

/// Create the task in the target column and close the composer.
fn create_column_task(
    mut app_state: AppState,
    column: String,
    title: Signal<String>,
    subs: Signal<Vec<String>>,
    sub_input: Signal<String>,
    adding: Signal<Option<String>>,
) {
    let name = title.read().trim().to_string();
    if name.is_empty() {
        return;
    }
    let subtasks = subs.read().clone();
    app_state.add_column_task(name, column_status(&column), subtasks);
    reset_task_composer(title, subs, sub_input, adding);
}

/// Clear every composer signal and close it.
fn reset_task_composer(
    mut title: Signal<String>,
    mut subs: Signal<Vec<String>>,
    mut sub_input: Signal<String>,
    mut adding: Signal<Option<String>>,
) {
    title.set(String::new());
    subs.set(Vec::new());
    sub_input.set(String::new());
    adding.set(None);
}

/// Live state of the card currently being dragged.
#[derive(Clone, PartialEq)]
struct DragInfo {
    id: String,
    title: String,
    x: f64,
    y: f64,
    start_x: f64,
    start_y: f64,
    moved: bool,
    hovered: Option<TaskStatus>,
}

/// Live state of the column being re-ordered via its header drag.
#[derive(Clone, PartialEq)]
struct ColDrag {
    index: usize,
    name: String,
    x: f64,
    y: f64,
    start_x: f64,
    start_y: f64,
    moved: bool,
    over: Option<usize>,
}

// ============================================================
// Card (drag source)
// ============================================================

#[component]
fn KanbanCard(
    task: TaskItem,
    mut drag: Signal<Option<DragInfo>>,
    mut col_drag: Signal<Option<ColDrag>>,
    mut suppress: Signal<bool>,
) -> Element {
    let mut app_state = use_app_state();
    let detail = use_context::<TaskDetailContext>();

    let title = task.title.clone();
    let desc = task.description.clone();
    let project = task.project.clone();
    let id = task.id.clone();
    let open_id = task.id.clone();
    let drag_id = task.id.clone();
    let drag_title = task.title.clone();
    let completed = task.completed;
    let priority = task.priority.as_str();
    let prio_variant = task.priority.badge_variant().to_string();
    let due_label = task.due_date.map(format_due);
    let sub_total = task.subtasks.len();
    let sub_done = task.subtasks.iter().filter(|s| s.completed).count();
    let has_subtasks = sub_total > 0;
    let is_dragged = drag
        .read()
        .as_ref()
        .is_some_and(|d| d.moved && d.id == id);

    rsx! {
        div { class: "relative",

            // Stacked "back cards" peek out behind the card when the task has subtasks.
            if has_subtasks {
                div { class: "pointer-events-none absolute inset-x-3 top-2 h-full rounded-lg border border-border/30 bg-muted/50" }
                div { class: "pointer-events-none absolute inset-x-1.5 top-1 h-full rounded-lg border border-border/40 bg-muted/70 shadow-xs" }
            }

            div {
                class: format!(
                    "relative {}",
                    if is_dragged {
                        "group cursor-grabbing rounded-lg border border-dashed border-primary/50 bg-card p-3 shadow-xs opacity-40"
                    } else {
                        "group cursor-grab rounded-lg border border-border/50 bg-card p-3 shadow-xs transition-colors hover:border-border"
                    },
                ),
            onmousedown: move |e| {
                suppress.set(false);
                // A card press cancels any in-flight column drag.
                col_drag.set(None);
                let p = e.client_coordinates();
                drag.set(Some(DragInfo {
                    id: drag_id.clone(),
                    title: drag_title.clone(),
                    x: p.x,
                    y: p.y,
                    start_x: p.x,
                    start_y: p.y,
                    moved: false,
                    hovered: None,
                }));
            },
            // Open the detail dialog on click, unless this click ends a drag.
            onclick: move |_| {
                if !*suppress.read() {
                    detail.show(open_id.clone());
                }
            },

            div { class: "flex items-start gap-2.5",
                TaskCheck { id: id.clone(), completed }
                div { class: "min-w-0 flex-1",
                    p { class: if completed {
                            "text-xs font-medium text-foreground line-through"
                        } else {
                            "text-xs font-medium text-foreground"
                        },
                        "{title}"
                    }
                    if !desc.is_empty() {
                        p { class: "mt-1 line-clamp-2 text-[11px] leading-relaxed text-muted-foreground",
                            "{desc}"
                        }
                    }
                }
            }

            if let Some(due) = &due_label {
                div { class: "mt-2 flex items-center gap-1.5 text-[10px] text-muted-foreground",
                    CalendarIcon { class: "size-3" }
                    "{due}"
                }
            }

            if has_subtasks {
                div { class: "mt-1.5 flex items-center gap-1.5 text-[10px] text-muted-foreground",
                    svg {
                        class: "size-3",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "m3 17 2 2 4-4" }
                        path { d: "m3 7 2 2 4-4" }
                        path { d: "M13 6h8" }
                        path { d: "M13 12h8" }
                        path { d: "M13 18h8" }
                    }
                    span { "{sub_done}/{sub_total} subtasks" }
                }
            }

            div { class: "mt-3 flex items-center justify-between gap-2",
                div { class: "flex min-w-0 items-center gap-1.5",
                    Badge { variant: prio_variant, size: "sm".to_string(), "{priority}" }
                    span { class: "truncate text-[10px] text-muted-foreground", "{project}" }
                }

                button {
                    r#type: "button",
                    class: "flex size-5 cursor-pointer items-center justify-center rounded text-muted-foreground opacity-0 transition-all hover:bg-destructive/10 hover:text-destructive group-hover:opacity-100",
                    title: "Delete task",
                    onclick: move |e| {
                        e.stop_propagation();
                        app_state.delete_task(&id);
                    },
                    TrashIcon { class: "size-3.5" }
                }
            }
        }
    }
    }
}

// ============================================================
// Board (drag target: Backlog / In Progress / Completed columns)
// ============================================================

#[component]
pub fn KanbanBoard(tasks: Vec<TaskItem>) -> Element {
    let mut app_state = use_app_state();
    let mut drag: Signal<Option<DragInfo>> = use_signal(|| None);
    let mut col_drag: Signal<Option<ColDrag>> = use_signal(|| None);
    // Set after a real drag ends so the follow-up click doesn't open the detail dialog.
    let mut suppress = use_signal(|| false);
    // New-column composer state.
    let mut adding = use_signal(|| false);
    let mut new_col = use_signal(String::new);
    // Task composer state (one open at a time, keyed by column name).
    let mut adding_task: Signal<Option<String>> = use_signal(|| None);
    let mut nt_title = use_signal(String::new);
    let mut nt_subs = use_signal(Vec::<String>::new);
    let mut nt_sub_input = use_signal(String::new);

    let column_names: Vec<String> = app_state.board_columns.read().clone();
    let columns: Vec<(String, TaskStatus, Vec<TaskItem>, usize)> = column_names
        .iter()
        .enumerate()
        .map(|(idx, name)| {
            let status = column_status(name);
            let column: Vec<TaskItem> = tasks
                .iter()
                .filter(|t| t.status == status)
                .cloned()
                .collect();
            (name.clone(), status, column, idx)
        })
        .collect();

    let any_dragging = drag.read().is_some() || col_drag.read().is_some();
    let ghost = drag.read().clone().and_then(|d| {
        if d.moved {
            Some((
                d.title,
                d.x,
                d.y,
                d.hovered.map(|h| h.as_str().to_string()),
            ))
        } else {
            None
        }
    });
    let col_ghost_pos = col_drag
        .read()
        .as_ref()
        .filter(|c| c.moved)
        .map(|c| (c.x, c.y, c.name.clone()));

    rsx! {
        div {
            class: format!(
                "flex min-h-0 flex-1 gap-4 overflow-x-auto select-none {}",
                if any_dragging { "cursor-grabbing" } else { "" },
            ),

            // Track the cursor while a drag is active.
            onmousemove: move |e| {
                let maybe_cd = col_drag.read().clone();
                if let Some(mut cd) = maybe_cd {
                    let p = e.client_coordinates();
                    cd.moved = cd.moved
                        || (p.x - cd.start_x).abs() > DRAG_THRESHOLD
                        || (p.y - cd.start_y).abs() > DRAG_THRESHOLD;
                    cd.x = p.x;
                    cd.y = p.y;
                    col_drag.set(Some(cd));
                }
                let maybe_info = drag.read().clone();
                if let Some(mut info) = maybe_info {
                    let p = e.client_coordinates();
                    info.moved = info.moved
                        || (p.x - info.start_x).abs() > DRAG_THRESHOLD
                        || (p.y - info.start_y).abs() > DRAG_THRESHOLD;
                    info.x = p.x;
                    info.y = p.y;
                    drag.set(Some(info));
                }
            },

            // Released over the board background (not on a column).
            onmouseup: move |_| {
                if drag.read().is_some() {
                    drag.set(None);
                }
                if col_drag.read().is_some() {
                    col_drag.set(None);
                }
            },

            for (col_name, status, column, idx) in columns {
                {
                    let count = column.len();
                    let dot_class = match &status {
                        TaskStatus::Backlog => "bg-zinc-400",
                        TaskStatus::InProgress => "bg-blue-500",
                        TaskStatus::Completed => "bg-emerald-500",
                        TaskStatus::Custom(_) => "bg-violet-500",
                    };
                    let is_target =
                        drag.read().as_ref().and_then(|d| d.hovered.clone())
                            == Some(status.clone());
                    let is_composing =
                        adding_task.read().as_deref() == Some(col_name.as_str());
                    let cd_state = col_drag.read().clone();
                    let is_col_dragged = cd_state
                        .as_ref()
                        .is_some_and(|c| c.moved && c.index == idx);
                    let is_col_over =
                        cd_state.as_ref().is_some_and(|c| c.moved && c.over == Some(idx));
                    let cd_name = col_name.clone();
                    let open_col = col_name.clone();
                    let ck_enter = col_name.clone();
                    let ck_btn = col_name.clone();
                    let s_move = status.clone();
                    let s_leave = status.clone();
                    let s_up = status;

                    rsx! {
                        div { key: "{col_name}", class: format!(
                                "flex w-72 min-h-0 shrink-0 flex-col gap-3 {}",
                                if is_col_dragged {
                                    "opacity-40"
                                } else if is_col_over {
                                    "rounded-xl ring-2 ring-inset ring-primary/50"
                                } else {
                                    ""
                                },
                            ),

                            // This column is a drop target while dragging a task,
                            // and a re-order target while dragging a column.
                            onmousemove: move |e| {
                                let maybe_cd = col_drag.read().clone();
                                if let Some(mut cd) = maybe_cd {
                                    let p = e.client_coordinates();
                                    cd.moved = cd.moved
                                        || (p.x - cd.start_x).abs() > DRAG_THRESHOLD
                                        || (p.y - cd.start_y).abs() > DRAG_THRESHOLD;
                                    cd.x = p.x;
                                    cd.y = p.y;
                                    cd.over = Some(idx);
                                    col_drag.set(Some(cd));
                                }
                                let maybe_info = drag.read().clone();
                                if let Some(mut info) = maybe_info {
                                    if info.moved && info.hovered.as_ref() != Some(&s_move) {
                                        info.hovered = Some(s_move.clone());
                                        drag.set(Some(info));
                                    }
                                }
                            },
                            onmouseleave: move |_| {
                                let maybe_cd = col_drag.read().clone();
                                if let Some(mut cd) = maybe_cd {
                                    if cd.over == Some(idx) {
                                        cd.over = None;
                                        col_drag.set(Some(cd));
                                    }
                                }
                                let maybe_info = drag.read().clone();
                                if let Some(mut info) = maybe_info {
                                    if info.hovered.as_ref() == Some(&s_leave) {
                                        info.hovered = None;
                                        drag.set(Some(info));
                                    }
                                }
                            },
                            onmouseup: move |_| {
                                let maybe_cd = col_drag.read().clone();
                                if let Some(cd) = maybe_cd {
                                    if cd.moved {
                                        app_state.move_board_column(
                                            cd.index,
                                            cd.over.unwrap_or(cd.index),
                                        );
                                        suppress.set(true);
                                    }
                                    col_drag.set(None);
                                }
                                let maybe_info = drag.read().clone();
                                if let Some(info) = maybe_info {
                                    app_state.set_task_status(&info.id, s_up.clone());
                                    if info.moved {
                                        suppress.set(true);
                                    }
                                    drag.set(None);
                                }
                            },

                            div {
                                class: "flex cursor-grab select-none items-center gap-2 px-1",
                                // Grabbing the header starts a column re-order drag.
                                onmousedown: move |e| {
                                    drag.set(None);
                                    let p = e.client_coordinates();
                                    col_drag.set(Some(ColDrag {
                                        index: idx,
                                        name: cd_name.clone(),
                                        x: p.x,
                                        y: p.y,
                                        start_x: p.x,
                                        start_y: p.y,
                                        moved: false,
                                        over: Some(idx),
                                    }));
                                },
                                span { class: "size-2 shrink-0 rounded-full {dot_class}" }
                                span { class: "text-sm font-semibold text-foreground", "{col_name}" }
                                span { class: "rounded-full bg-muted px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground",
                                    "{count}"
                                }
                            }

                            div { class: format!(
                                    "flex min-h-28 flex-1 flex-col gap-2.5 overflow-y-auto rounded-xl border border-dashed p-2 pb-3 transition-colors {}",
                                    if is_target {
                                        "border-primary bg-primary/5"
                                    } else {
                                        "border-border/50 bg-muted/20"
                                    },
                                ),
                                for task in column {
                                    KanbanCard { key: "{task.id}", task, drag, col_drag, suppress }
                                }
                                if count == 0 {
                                    p { class: "px-2 py-4 text-center text-[11px] text-muted-foreground",
                                        if is_target {
                                            "Drop task here"
                                        } else {
                                            "No tasks"
                                        }
                                    }
                                }
                            }

                            // Quick-add composer / add-task button.
                            if is_composing {
                                {
                                    let staged: Vec<String> = nt_subs.read().clone();
                                    rsx! {
                                        div { class: "flex flex-col gap-2 rounded-xl border border-primary/50 bg-muted/30 p-2",
                                            input {
                                                r#type: "text",
                                                class: "h-8 w-full rounded-md border border-input bg-background px-2.5 text-xs text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring",
                                                placeholder: "Task title...",
                                                value: "{nt_title}",
                                                autofocus: true,
                                                oninput: move |e| nt_title.set(e.value()),
                                                onkeydown: move |e: KeyboardEvent| {
                                                    match e.key() {
                                                        Key::Enter => create_column_task(
                                                            app_state,
                                                            ck_enter.clone(),
                                                            nt_title,
                                                            nt_subs,
                                                            nt_sub_input,
                                                            adding_task,
                                                        ),
                                                        Key::Escape => reset_task_composer(
                                                            nt_title,
                                                            nt_subs,
                                                            nt_sub_input,
                                                            adding_task,
                                                        ),
                                                        _ => {}
                                                    }
                                                },
                                            }

                                            if !staged.is_empty() {
                                                div { class: "flex flex-col gap-1",
                                                    for (i, sub) in staged.iter().enumerate() {
                                                        {
                                                            let text = sub.clone();
                                                            rsx! {
                                                                div { key: "{i}",
                                                                    class: "group/sub flex items-center gap-2 rounded-md border border-border/50 bg-background px-2 py-1",
                                                                    span { class: "size-1.5 shrink-0 rounded-full bg-muted-foreground/60" }
                                                                    span { class: "min-w-0 flex-1 truncate text-[11px] text-foreground",
                                                                        "{text}"
                                                                    }
                                                                    button {
                                                                        r#type: "button",
                                                                        class: "flex size-4 shrink-0 cursor-pointer items-center justify-center rounded text-muted-foreground transition-colors hover:text-destructive",
                                                                        title: "Remove subtask",
                                                                        onclick: move |e| {
                                                                            e.stop_propagation();
                                                                            let mut list = nt_subs.write();
                                                                            if i < list.len() {
                                                                                list.remove(i);
                                                                            }
                                                                        },
                                                                        CloseIcon { class: "size-2.5" }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                            input {
                                                r#type: "text",
                                                class: "h-7 w-full rounded-md border border-input bg-background px-2.5 text-[11px] text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring",
                                                placeholder: "Add a subtask, press Enter...",
                                                value: "{nt_sub_input}",
                                                oninput: move |e| nt_sub_input.set(e.value()),
                                                onkeydown: move |e: KeyboardEvent| {
                                                    match e.key() {
                                                        Key::Enter => stage_subtask(nt_subs, nt_sub_input),
                                                        Key::Escape => reset_task_composer(
                                                            nt_title,
                                                            nt_subs,
                                                            nt_sub_input,
                                                            adding_task,
                                                        ),
                                                        _ => {}
                                                    }
                                                },
                                            }

                                            div { class: "flex gap-2",
                                                button {
                                                    r#type: "button",
                                                    class: "inline-flex h-7 flex-1 cursor-pointer items-center justify-center rounded-md bg-primary text-xs font-medium text-primary-foreground transition-opacity hover:opacity-90",
                                                    onclick: move |_| create_column_task(
                                                        app_state,
                                                        ck_btn.clone(),
                                                        nt_title,
                                                        nt_subs,
                                                        nt_sub_input,
                                                        adding_task,
                                                    ),
                                                    "Add task"
                                                }
                                                button {
                                                    r#type: "button",
                                                    class: "inline-flex h-7 flex-1 cursor-pointer items-center justify-center rounded-md border border-border text-xs font-medium text-foreground transition-colors hover:bg-muted",
                                                    onclick: move |_| reset_task_composer(
                                                        nt_title,
                                                        nt_subs,
                                                        nt_sub_input,
                                                        adding_task,
                                                    ),
                                                    "Cancel"
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                button {
                                    r#type: "button",
                                    class: "flex w-full cursor-pointer items-center gap-1.5 rounded-lg border border-dashed border-border/50 bg-muted/20 px-2.5 py-2 text-[11px] text-muted-foreground transition-colors hover:border-primary hover:text-primary",
                                    onclick: move |_| adding_task.set(Some(open_col.clone())),
                                    svg {
                                        class: "size-3",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        path { d: "M12 5v14" }
                                        path { d: "M5 12h14" }
                                    }
                                    span { "Add task" }
                                }
                            }
                        }
                    }
                }
            }

            // Create a new board column.
            div { class: "w-64 shrink-0",
                if *adding.read() {
                    div { class: "flex min-h-28 flex-col gap-2 rounded-xl border border-primary/50 bg-muted/30 p-2",
                        input {
                            r#type: "text",
                            class: "h-8 w-full rounded-md border border-input bg-background px-2.5 text-xs text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring",
                            placeholder: "Column name...",
                            value: "{new_col}",
                            autofocus: true,
                            oninput: move |e| new_col.set(e.value()),
                            onkeydown: move |e: KeyboardEvent| {
                                match e.key() {
                                    Key::Enter => commit_column(app_state, new_col, adding),
                                    Key::Escape => {
                                        new_col.set(String::new());
                                        adding.set(false);
                                    }
                                    _ => {}
                                }
                            },
                        }
                        div { class: "flex gap-2",
                            button {
                                r#type: "button",
                                class: "inline-flex h-7 flex-1 cursor-pointer items-center justify-center rounded-md bg-primary text-xs font-medium text-primary-foreground transition-opacity hover:opacity-90",
                                onclick: move |_| commit_column(app_state, new_col, adding),
                                "Add"
                            }
                            button {
                                r#type: "button",
                                class: "inline-flex h-7 flex-1 cursor-pointer items-center justify-center rounded-md border border-border text-xs font-medium text-foreground transition-colors hover:bg-muted",
                                onclick: move |_| {
                                    new_col.set(String::new());
                                    adding.set(false);
                                },
                                "Cancel"
                            }
                        }
                    }
                } else {
                    button {
                        r#type: "button",
                        class: "flex min-h-28 w-full flex-col items-center justify-center gap-1.5 cursor-pointer rounded-xl border border-dashed border-border/50 bg-muted/20 text-muted-foreground transition-colors hover:border-primary hover:text-primary",
                        onclick: move |_| adding.set(true),
                        svg {
                            class: "size-4",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M12 5v14" }
                            path { d: "M5 12h14" }
                        }
                        span { class: "text-xs font-medium", "Add column" }
                    }
                }
            }

            // Floating ghost that follows the cursor.
            if let Some((gtitle, gx, gy, glabel)) = &ghost {
                div {
                    class: "pointer-events-none fixed z-10 w-56 -translate-x-1/2 -translate-y-1/2 rotate-2 rounded-lg border border-primary/40 bg-card p-3 shadow-lg",
                    style: format!("left: {gx}px; top: {gy}px"),
                    p { class: "truncate text-xs font-medium text-foreground", "{gtitle}" }
                    if let Some(label) = glabel {
                        p { class: "mt-1 text-[10px] font-medium text-primary",
                            "Drop into {label}"
                        }
                    }
                }
            }

            // Floating ghost for a column being re-ordered.
            if let Some((gx, gy, gname)) = &col_ghost_pos {
                div {
                    class: "pointer-events-none fixed z-10 w-64 -translate-x-1/2 -translate-y-1/2 rounded-lg border border-primary/40 bg-card p-3 shadow-lg",
                    style: format!("left: {gx}px; top: {gy}px"),
                    p { class: "truncate text-xs font-medium text-foreground", "{gname}" }
                    p { class: "mt-1 text-[10px] font-medium text-primary",
                        "Move column"
                    }
                }
            }
        }
    }
}
