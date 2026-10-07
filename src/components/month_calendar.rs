use dioxus::prelude::*;

use crate::components::ui::{
    Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};

pub const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

const WEEKDAY_NAMES: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

const MAX_VISIBLE_EVENTS: usize = 3;

pub type EventMove = (String, (u32, u32, u32));

fn is_leap_year(year: u32) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

// Sakamoto's algorithm: Day of week for 1st of month (0 = Sunday, ..., 6 = Saturday)
fn first_day_of_month(year: u32, month: u32) -> usize {
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut y = year;
    if month < 3 {
        y -= 1;
    }
    ((y + y / 4 - y / 100 + y / 400 + t[(month - 1) as usize] + 1) % 7) as usize
}

// ============================================================
// Event Model
// ============================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventColor {
    Amber,
    Sky,
    Emerald,
    Violet,
    Rose,
    Orange,
    Pink,
}

pub const ALL_EVENT_COLORS: [EventColor; 7] = [
    EventColor::Amber,
    EventColor::Sky,
    EventColor::Emerald,
    EventColor::Violet,
    EventColor::Rose,
    EventColor::Orange,
    EventColor::Pink,
];

impl EventColor {
    pub fn chip_class(&self) -> &'static str {
        match self {
            Self::Amber => "bg-amber-100 text-amber-900 dark:bg-amber-900/50 dark:text-amber-100",
            Self::Sky => "bg-sky-100 text-sky-900 dark:bg-sky-900/50 dark:text-sky-100",
            Self::Emerald => {
                "bg-emerald-100 text-emerald-900 dark:bg-emerald-900/50 dark:text-emerald-100"
            }
            Self::Violet => {
                "bg-violet-100 text-violet-900 dark:bg-violet-900/50 dark:text-violet-100"
            }
            Self::Rose => "bg-rose-100 text-rose-900 dark:bg-rose-900/50 dark:text-rose-100",
            Self::Orange => {
                "bg-orange-100 text-orange-900 dark:bg-orange-900/50 dark:text-orange-100"
            }
            Self::Pink => "bg-pink-100 text-pink-900 dark:bg-pink-900/50 dark:text-pink-100",
        }
    }

    pub fn dot_class(&self) -> &'static str {
        match self {
            Self::Amber => "bg-amber-500",
            Self::Sky => "bg-sky-500",
            Self::Emerald => "bg-emerald-500",
            Self::Violet => "bg-violet-500",
            Self::Rose => "bg-rose-500",
            Self::Orange => "bg-orange-500",
            Self::Pink => "bg-pink-500",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CalEvent {
    pub id: String,
    pub title: String,
    pub time: Option<String>,
    pub date: (u32, u32, u32),
    pub end_date: Option<(u32, u32, u32)>,
    pub color: EventColor,
    pub completed: bool,
}

fn next_day((y, m, d): (u32, u32, u32)) -> (u32, u32, u32) {
    if d < days_in_month(y, m) {
        (y, m, d + 1)
    } else if m < 12 {
        (y, m + 1, 1)
    } else {
        (y + 1, 1, 1)
    }
}

fn days_between(from: (u32, u32, u32), to: (u32, u32, u32)) -> u32 {
    let mut current = from;
    let mut count = 0;
    while current < to && count < 500 {
        current = next_day(current);
        count += 1;
    }
    count
}

fn add_days(date: (u32, u32, u32), days: u32) -> (u32, u32, u32) {
    let mut current = date;
    for _ in 0..days {
        current = next_day(current);
    }
    current
}

pub fn is_event_on(event: &CalEvent, date: (u32, u32, u32)) -> bool {
    let end = event.end_date.unwrap_or(event.date);
    event.date <= date && date <= end
}

pub fn format_date(date: (u32, u32, u32)) -> String {
    format!(
        "{} {}, {}",
        MONTH_NAMES[(date.1 - 1) as usize],
        date.2,
        date.0
    )
}

pub fn parse_time_minutes(time: &str) -> Option<u32> {
    let t = time.trim().to_lowercase();
    let (rest, meridiem) = if let Some(r) = t.strip_suffix("am") {
        (r, Some(false))
    } else if let Some(r) = t.strip_suffix("pm") {
        (r, Some(true))
    } else {
        (t.as_str(), None)
    };
    let rest = rest.trim();
    let (h_str, m_str) = match rest.split_once(':') {
        Some((h, m)) => (h, m),
        None => (rest, "0"),
    };
    let h: u32 = h_str.trim().parse().ok()?;
    let m: u32 = m_str.trim().parse().ok()?;
    if m > 59 {
        return None;
    }
    let h24 = match meridiem {
        None => {
            if h > 23 {
                return None;
            }
            h
        }
        Some(false) => {
            if !(1..=12).contains(&h) {
                return None;
            }
            if h == 12 {
                0
            } else {
                h
            }
        }
        Some(true) => {
            if !(1..=12).contains(&h) {
                return None;
            }
            if h == 12 {
                12
            } else {
                h + 12
            }
        }
    };
    Some(h24 * 60 + m)
}

fn hour_label(hour: u32) -> String {
    let (h12, suffix) = match hour {
        0 => (12, "am"),
        h if h < 12 => (h, "am"),
        12 => (12, "pm"),
        h => (h - 12, "pm"),
    };
    format!("{h12}{suffix}")
}

pub fn shift_event(event: &mut CalEvent, new_date: (u32, u32, u32)) {
    let duration = event
        .end_date
        .map(|end| days_between(event.date, end))
        .unwrap_or(0);
    event.date = new_date;
    event.end_date = event.end_date.map(|_| add_days(new_date, duration));
}

// ============================================================
// EventChip
// ============================================================

#[derive(Props, Clone, PartialEq)]
pub struct EventChipProps {
    pub title: String,

    #[props(default)]
    pub time: Option<String>,

    pub color: EventColor,

    #[props(default)]
    pub completed: bool,

    #[props(default)]
    pub class: String,

    #[props(default)]
    pub event_id: Option<String>,

    #[props(default = true)]
    pub show_title: bool,

    #[props(default)]
    pub dragging: Option<Signal<Option<String>>>,
}

#[component]
pub fn EventChip(props: EventChipProps) -> Element {
    let EventChipProps {
        title,
        time,
        color,
        completed,
        class,
        event_id,
        show_title,
        dragging,
    } = props;

    let chip_class = color.chip_class();
    let strike = if completed { "line-through" } else { "" };

    let is_dragging = match (&dragging, &event_id) {
        (Some(drag), Some(id)) => drag().as_deref() == Some(id.as_str()),
        _ => false,
    };

    let state_class = if is_dragging {
        " opacity-50"
    } else if event_id.is_some() {
        " cursor-grab active:cursor-grabbing"
    } else {
        ""
    };

    rsx! {
        div {
            class: "flex min-h-[22px] min-w-0 items-center gap-1 rounded-md px-1.5 py-[3px] text-xs leading-4 {chip_class}{state_class} {class}",
            draggable: "true",
            ondragstart: move |e| {
                if let (Some(mut drag), Some(id)) = (dragging, &event_id) {
                    let _ = e.data_transfer().set_data("text/plain", id);
                    *drag.write() = Some(id.clone());
                }
            },
            ondragend: move |_| {
                if let Some(mut drag) = dragging {
                    *drag.write() = None;
                }
            },

            if show_title {
                if let Some(time) = time {
                    span { class: "shrink-0 font-medium opacity-70", "{time}" }
                }
                span { class: "min-w-0 truncate font-medium {strike}", "{title}" }
            }
        }
    }
}

// ============================================================
// WeekdayHeader
// ============================================================

#[component]
pub fn WeekdayHeader() -> Element {
    rsx! {
        div { class: "grid grid-cols-7 border-b border-border/40 shrink-0",
            for name in WEEKDAY_NAMES {
                div {
                    key: "{name}",
                    class: "px-2 py-2.5 text-center text-xs font-medium text-muted-foreground",
                    "{name}"
                }
            }
        }
    }
}

// ============================================================
// DayCell
// ============================================================

#[derive(Props, Clone, PartialEq)]
pub struct DayCellProps {
    pub date: (u32, u32, u32),

    #[props(default)]
    pub is_today: bool,

    #[props(default)]
    pub is_other_month: bool,

    #[props(default)]
    pub events: Vec<CalEvent>,

    #[props(default)]
    pub dragging: Option<Signal<Option<String>>>,

    #[props(default)]
    pub on_event_move: Option<EventHandler<EventMove>>,

    #[props(default)]
    pub on_day_click: Option<EventHandler<(u32, u32, u32)>>,
}

#[component]
pub fn DayCell(props: DayCellProps) -> Element {
    let DayCellProps {
        date,
        is_today,
        is_other_month,
        events,
        dragging,
        on_event_move,
        on_day_click,
    } = props;

    let day = date.2;
    let hidden_count = events.len().saturating_sub(MAX_VISIBLE_EVENTS);

    let number_class = if is_today {
        "size-6 rounded-full bg-primary text-primary-foreground font-semibold"
    } else if is_other_month {
        "text-muted-foreground/40"
    } else {
        "text-muted-foreground"
    };

    let click_class = if on_day_click.is_some() {
        " cursor-pointer"
    } else {
        ""
    };

    rsx! {
        div {
            class: "flex h-full min-h-0 flex-col gap-1 border-b border-r border-border/40 p-1 sm:p-1.5 overflow-hidden{click_class}",
            onclick: move |_| {
                if let Some(handler) = &on_day_click {
                    handler.call(date);
                }
            },

            ondragover: move |e| {
                if let Some(drag) = dragging {
                    if drag().is_some() {
                        e.prevent_default();
                    }
                }
            },
            ondrop: move |e| {
                if let (Some(mut drag), Some(handler)) = (dragging, &on_event_move) {
                    if let Some(id) = drag() {
                        e.prevent_default();
                        handler.call((id, date));
                        *drag.write() = None;
                    }
                }
            },

            div { class: "flex h-6 items-center px-0.5 shrink-0",
                span { class: "inline-flex items-center justify-center text-xs {number_class}", "{day}" }
            }

            div { class: "flex min-w-0 flex-1 flex-col gap-1 overflow-hidden",
                for event in events.iter().take(MAX_VISIBLE_EVENTS) {
                    EventChip {
                        key: "{event.id}",
                        title: event.title.clone(),
                        time: event.time.clone(),
                        color: event.color,
                        completed: event.completed,
                        event_id: Some(event.id.clone()),
                        show_title: event.date == date,
                        dragging: dragging,
                    }
                }

                if hidden_count > 0 {
                    div { class: "px-1.5 text-[11px] font-medium text-muted-foreground shrink-0",
                        "+ {hidden_count} more"
                    }
                }
            }
        }
    }
}

// ============================================================
// CalendarToolbar
// ============================================================

#[derive(Props, Clone, PartialEq)]
pub struct CalendarToolbarProps {
    pub month_label: String,

    pub on_prev: EventHandler<()>,

    pub on_next: EventHandler<()>,

    pub on_today: EventHandler<()>,

    #[props(default = "Month".to_string())]
    pub view_label: String,

    #[props(default)]
    pub on_view_change: Option<EventHandler<String>>,

    #[props(default)]
    pub on_new_event: Option<EventHandler<()>>,
}

#[component]
pub fn CalendarToolbar(props: CalendarToolbarProps) -> Element {
    let CalendarToolbarProps {
        month_label,
        on_prev,
        on_next,
        on_today,
        view_label,
        on_view_change,
        on_new_event,
    } = props;

    let view_items = ["Month", "Week", "Day", "Agenda"].map(|label| {
        let handler = on_view_change;
        rsx! {
            DropdownMenuItem {
                key: "{label}",
                onclick: move |_| {
                    if let Some(handler) = &handler {
                        handler.call(label.to_string());
                    }
                },
                "{label}"
            }
        }
    });

    rsx! {
        div { class: "flex flex-wrap items-center justify-between gap-3 px-3 py-2.5 sm:px-4 sm:py-3 shrink-0",

            div { class: "flex items-center gap-1.5",

                button {
                    r#type: "button",
                    class: "rounded-lg border border-border/40 px-3 py-1.5 text-sm font-medium hover:bg-muted transition-colors cursor-pointer",
                    onclick: move |_| on_today.call(()),
                    "Today"
                }

                button {
                    r#type: "button",
                    class: "size-8 rounded-lg text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer",
                    onclick: move |_| on_prev.call(()),
                    crate::icons::ChevronLeftIcon { class: "size-4" }
                    span { class: "sr-only", "Previous month" }
                }

                button {
                    r#type: "button",
                    class: "size-8 rounded-lg text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer",
                    onclick: move |_| on_next.call(()),
                    crate::icons::ChevronRightIcon { class: "size-4" }
                    span { class: "sr-only", "Next month" }
                }

                h2 { class: "ml-1 text-base sm:text-xl font-semibold tracking-tight text-foreground select-none",
                    "{month_label}"
                }
            }

            div { class: "flex items-center gap-2",

                DropdownMenu {
                    DropdownMenuTrigger {
                        class: "gap-1.5 rounded-lg border border-border/40 px-2.5 py-1.5 hover:bg-muted",
                        "{view_label}"
                        svg {
                            class: "size-3.5 shrink-0",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "m6 9 6 6 6-6" }
                        }
                    }
                    DropdownMenuContent { class: "w-36",
                        {view_items.into_iter()}
                    }
                }

                button {
                    r#type: "button",
                    class: "inline-flex items-center gap-1.5 rounded-lg bg-primary px-3 py-1.5 text-sm font-medium text-primary-foreground hover:opacity-90 transition-opacity cursor-pointer",
                    onclick: move |_| {
                        if let Some(handler) = &on_new_event {
                            handler.call(());
                        }
                    },
                    svg {
                        class: "size-3.5",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M5 12h14" }
                        path { d: "M12 5v14" }
                    }
                    "New event"
                }
            }
        }
    }
}

// ============================================================
// MonthGrid
// ============================================================

#[derive(Props, Clone, PartialEq)]
pub struct MonthGridProps {
    pub year: u32,

    pub month: u32,

    #[props(default)]
    pub today: Option<(u32, u32, u32)>,

    #[props(default)]
    pub events: Vec<CalEvent>,

    #[props(default)]
    pub class: String,

    #[props(default)]
    pub on_event_move: Option<EventHandler<EventMove>>,

    #[props(default)]
    pub on_day_click: Option<EventHandler<(u32, u32, u32)>>,
}

#[component]
pub fn MonthGrid(props: MonthGridProps) -> Element {
    let MonthGridProps {
        year,
        month,
        today,
        events,
        class,
        on_event_move,
        on_day_click,
    } = props;

    let dragging: Signal<Option<String>> = use_signal(|| None);

    let today = today.unwrap_or_else(crate::state::get_real_current_date);

    let total_days = days_in_month(year, month);
    let start_weekday = first_day_of_month(year, month);

    let prev_month = if month == 1 { 12 } else { month - 1 };
    let prev_year = if month == 1 { year - 1 } else { year };
    let prev_days_count = days_in_month(prev_year, prev_month);

    let next_month = if month == 12 { 1 } else { month + 1 };
    let next_year = if month == 12 { year + 1 } else { year };

    let total_cells = start_weekday as u32 + total_days;
    let grid_cells = if total_cells <= 35 { 35 } else { 42 };
    let row_count = grid_cells / 7;
    let trailing_count = grid_cells - total_cells;

    let events_for = |date: (u32, u32, u32)| -> Vec<CalEvent> {
        events
            .iter()
            .filter(|event| is_event_on(event, date))
            .cloned()
            .collect()
    };

    let mut cells: Vec<Element> = Vec::new();

    for i in 0..start_weekday {
        let day = prev_days_count - (start_weekday as u32 - 1) + i as u32;
        let date = (prev_year, prev_month, day);
        cells.push(rsx! {
            DayCell {
                key: "prev-{prev_year}-{prev_month}-{day}",
                date,
                is_other_month: true,
                events: events_for(date),
                dragging: Some(dragging),
                on_event_move: on_event_move,
                on_day_click: on_day_click,
            }
        });
    }

    for day in 1..=total_days {
        let date = (year, month, day);
        cells.push(rsx! {
            DayCell {
                key: "current-{year}-{month}-{day}",
                date,
                is_today: today == date,
                events: events_for(date),
                dragging: Some(dragging),
                on_event_move: on_event_move,
                on_day_click: on_day_click,
            }
        });
    }

    for day in 1..=trailing_count {
        let date = (next_year, next_month, day);
        cells.push(rsx! {
            DayCell {
                key: "next-{next_year}-{next_month}-{day}",
                date,
                is_other_month: true,
                events: events_for(date),
                dragging: Some(dragging),
                on_event_move: on_event_move,
                on_day_click: on_day_click,
            }
        });
    }

    rsx! {
        div {
            class: "grid grid-cols-7 h-full w-full min-h-0 flex-1 [&>*:nth-child(7n)]:border-r-0 [&>*:nth-last-child(-n+7)]:border-b-0 {class}",
            style: "grid-template-rows: repeat({row_count}, minmax(0, 1fr));",
            {cells.into_iter()}
        }
    }
}

// ============================================================
// EventInfoCard
// ============================================================

#[derive(Props, Clone, PartialEq)]
pub struct EventInfoCardProps {
    pub event: CalEvent,
}

#[component]
pub fn EventInfoCard(props: EventInfoCardProps) -> Element {
    let event = props.event;
    let today = crate::state::get_real_current_date();
    let deadline = event.end_date.unwrap_or(event.date);
    let days = if event.end_date.is_some() {
        days_between(event.date, deadline) + 1
    } else {
        1
    };
    let dot = event.color.chip_class();
    let strike = if event.completed { "line-through" } else { "" };
    let time_label = event.time.as_deref().unwrap_or("All day");

    let mut deadline_text = format!("Deadline: {}", format_date(deadline));
    if days > 1 {
        deadline_text.push_str(&format!(" \u{b7} {days}-day event"));
    }

    let status = if event.completed {
        String::new()
    } else if deadline < today {
        "overdue".to_string()
    } else if deadline == today {
        "due today".to_string()
    } else {
        format!("in {} days", days_between(today, deadline))
    };
    let status_class = if status == "overdue" {
        "font-medium text-destructive"
    } else {
        "text-muted-foreground/70"
    };

    rsx! {
        div { class: "flex items-start gap-3 rounded-lg border border-border/60 bg-card px-3 py-2",
            span { class: "mt-1.5 size-2.5 shrink-0 rounded-full {dot}" }
            div { class: "flex min-w-0 flex-1 flex-col gap-0.5",
                div { class: "flex items-baseline gap-2",
                    span { class: "w-14 shrink-0 text-xs font-medium text-muted-foreground tabular-nums",
                        "{time_label}"
                    }
                    span { class: "min-w-0 flex-1 truncate text-sm font-medium text-foreground {strike}",
                        "{event.title}"
                    }
                    if event.completed {
                        span { class: "shrink-0 rounded bg-emerald-100 px-1.5 py-0.5 text-[10px] font-semibold text-emerald-900 dark:bg-emerald-900/50 dark:text-emerald-100",
                            "Done"
                        }
                    }
                }
                div { class: "flex min-w-0 flex-wrap items-baseline gap-x-1 text-[11px] text-muted-foreground",
                    span { "{deadline_text}" }
                    if !status.is_empty() {
                        span { class: "{status_class}", "· {status}" }
                    }
                }
            }
        }
    }
}

// ============================================================
// DayEventsDialog
// ============================================================

#[derive(Props, Clone, PartialEq)]
pub struct DayEventsDialogProps {
    pub open: Signal<bool>,

    pub date: (u32, u32, u32),

    #[props(default)]
    pub events: Vec<CalEvent>,

    #[props(default)]
    pub on_add_event: Option<EventHandler<()>>,
}

#[component]
pub fn DayEventsDialog(props: DayEventsDialogProps) -> Element {
    let DayEventsDialogProps {
        mut open,
        date,
        events,
        on_add_event,
    } = props;

    let day_events: Vec<CalEvent> = events
        .iter()
        .filter(|event| is_event_on(event, date))
        .cloned()
        .collect();

    let mut all_day: Vec<CalEvent> = Vec::new();
    let mut timed: Vec<(u32, CalEvent)> = Vec::new();
    for event in day_events {
        match event.time.as_deref().and_then(parse_time_minutes) {
            Some(mins) => timed.push((mins, event)),
            None => all_day.push(event),
        }
    }
    timed.sort_by_key(|(mins, _)| *mins);

    let mut hours: Vec<(u32, Vec<CalEvent>)> = (0..24).map(|h| (h, Vec::new())).collect();
    for (mins, event) in timed {
        hours[(mins / 60) as usize].1.push(event);
    }

    let count = all_day.len() + hours.iter().map(|row| row.1.len()).sum::<usize>();
    let has_any = count > 0;
    let desc = match count {
        0 => "No events scheduled for this day.".to_string(),
        1 => "1 event · full 24-hour timeline".to_string(),
        n => format!("{n} events · full 24-hour timeline"),
    };

    rsx! {
        Dialog { open: open,
            DialogContent { class: "max-w-md",
                DialogHeader {
                    DialogTitle { "{format_date(date)}" }
                    DialogDescription { "{desc}" }
                }

                if !has_any {
                    div { class: "rounded-lg border border-dashed border-border/60 px-3 py-6 text-center text-sm text-muted-foreground",
                        "Nothing scheduled for this day yet."
                    }
                }

                if has_any {
                    div { class: "flex max-h-96 flex-col overflow-y-auto rounded-lg border border-border/40",
                        if !all_day.is_empty() {
                            div { class: "border-b border-border/40 bg-muted/40 px-2 py-2",
                                div { class: "mb-1.5 text-[10px] font-semibold uppercase tracking-wider text-muted-foreground",
                                    "All day"
                                }
                                div { class: "flex flex-col gap-1.5",
                                    for event in all_day.iter() {
                                        EventInfoCard { key: "{event.id}", event: event.clone() }
                                    }
                                }
                            }
                        }
                        for row in hours.iter() {
                            div { class: "flex border-b border-border/30 last:border-b-0",
                                span { class: "w-14 shrink-0 border-r border-border/30 px-1 pt-2 text-right text-[11px] font-medium text-muted-foreground tabular-nums",
                                    "{hour_label(row.0)}"
                                }
                                div { class: "flex min-w-0 flex-1 flex-col gap-1.5 p-1.5",
                                    for event in row.1.iter() {
                                        EventInfoCard { key: "{event.id}", event: event.clone() }
                                    }
                                }
                            }
                        }
                    }
                }

                DialogFooter {
                    button {
                        r#type: "button",
                        class: "inline-flex h-8 items-center rounded-md border border-border px-3 text-xs font-medium text-foreground hover:bg-muted transition-colors cursor-pointer",
                        onclick: move |_| open.set(false),
                        "Cancel"
                    }
                    button {
                        r#type: "button",
                        class: "inline-flex h-8 items-center gap-1.5 rounded-md bg-primary px-3 text-xs font-medium text-primary-foreground hover:opacity-90 transition-opacity cursor-pointer",
                        onclick: move |_| {
                            open.set(false);
                            if let Some(handler) = &on_add_event {
                                handler.call(());
                            }
                        },
                        "+ Add event"
                    }
                }
            }
        }
    }
}

// ============================================================
// NewEventDialog
// ============================================================

#[derive(Props, Clone, PartialEq)]
pub struct NewEventDialogProps {
    pub open: Signal<bool>,

    pub date: (u32, u32, u32),

    #[props(default)]
    pub on_submit: Option<EventHandler<CalEvent>>,
}

fn submit_new_event(
    title: Signal<String>,
    time: Signal<String>,
    color: Signal<EventColor>,
    mut open: Signal<bool>,
    date: (u32, u32, u32),
    on_submit: Option<EventHandler<CalEvent>>,
) {
    let trimmed = title.read().trim().to_string();
    if trimmed.is_empty() {
        return;
    }
    let time_value = time.read().trim().to_string();
    if let Some(handler) = on_submit {
        handler.call(CalEvent {
            id: format!(
                "ev-{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_millis())
                    .unwrap_or_default()
            ),
            title: trimmed,
            time: if time_value.is_empty() {
                None
            } else {
                Some(time_value)
            },
            date,
            end_date: None,
            color: *color.read(),
            completed: false,
        });
    }
    open.set(false);
}

#[component]
pub fn NewEventDialog(props: NewEventDialogProps) -> Element {
    let NewEventDialogProps {
        mut open,
        date,
        on_submit,
    } = props;

    let mut title = use_signal(String::new);
    let mut time = use_signal(String::new);
    let mut color = use_signal(|| EventColor::Sky);

    use_effect(move || {
        if !*open.read() {
            title.write().clear();
            time.write().clear();
            *color.write() = EventColor::Sky;
        }
    });

    let field_class = "w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring z-1000";

    rsx! {
        Dialog { open: open,
            DialogContent { class: "max-w-md",
                DialogHeader {
                    DialogTitle { "New Event" }
                    DialogDescription { "Add an event on {format_date(date)}." }
                }

                div { class: "flex flex-col gap-4",
                    div { class: "flex flex-col gap-1.5",
                        label { class: "text-xs font-medium text-foreground", "Title" }
                        input {
                            r#type: "text",
                            class: "{field_class}",
                            placeholder: "e.g. Design review",
                            value: "{title}",
                            autofocus: true,
                            oninput: move |e| title.set(e.value()),
                            onkeydown: move |e: KeyboardEvent| {
                                if e.key() == Key::Enter {
                                    submit_new_event(title, time, color, open, date, on_submit);
                                }
                            },
                        }
                    }

                    div { class: "flex flex-col gap-1.5",
                        label { class: "text-xs font-medium text-foreground", "Time" }
                        input {
                            r#type: "text",
                            class: "{field_class}",
                            placeholder: "e.g. 10am or 2:30pm",
                            value: "{time}",
                            oninput: move |e| time.set(e.value()),
                        }
                        span { class: "text-[11px] text-muted-foreground",
                            "Optional. Leave blank to make it an all-day event."
                        }
                    }

                    div { class: "flex flex-col gap-1.5",
                        label { class: "text-xs font-medium text-foreground", "Color" }
                        div { class: "flex items-center gap-2",
                            for c in ALL_EVENT_COLORS {
                                {
                                    let is_active = *color.read() == c;
                                    let swatch = c.chip_class();
                                    let ring_class = if is_active {
                                        "border-foreground ring-2 ring-foreground/30"
                                    } else {
                                        "border-border/40 hover:border-border"
                                    };
                                    rsx! {
                                        button {
                                            key: "{c:?}",
                                            r#type: "button",
                                            class: "size-7 cursor-pointer rounded-md border-2 transition-all {swatch} {ring_class}",
                                            onclick: move |_| color.set(c),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                DialogFooter {
                    button {
                        r#type: "button",
                        class: "inline-flex h-8 items-center rounded-md border border-border px-3 text-xs font-medium text-foreground hover:bg-muted transition-colors cursor-pointer",
                        onclick: move |_| open.set(false),
                        "Cancel"
                    }
                    button {
                        r#type: "button",
                        class: "inline-flex h-8 items-center rounded-md bg-primary px-3 text-xs font-medium text-primary-foreground hover:opacity-90 transition-opacity cursor-pointer disabled:opacity-50",
                        disabled: title.read().trim().is_empty(),
                        onclick: move |_| submit_new_event(title, time, color, open, date, on_submit),
                        "Add Event"
                    }
                }
            }
        }
    }
}
