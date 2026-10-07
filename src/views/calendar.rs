use dioxus::prelude::*;

use crate::components::month_calendar::{
    is_event_on, shift_event, CalEvent, CalendarToolbar, DayEventsDialog, EventColor, MonthGrid,
    NewEventDialog, WeekdayHeader, MONTH_NAMES,
};
use crate::state::{use_app_state, OverlayState};

pub fn sample_events() -> Vec<CalEvent> {
    vec![
        CalEvent {
            id: "ev-1".to_string(),
            title: "Project Deadline".to_string(),
            time: Some("1pm".to_string()),
            date: (2026, 9, 28),
            end_date: None,
            color: EventColor::Amber,
            completed: true,
        },
        CalEvent {
            id: "ev-2".to_string(),
            title: "Team Meeting".to_string(),
            time: Some("10am".to_string()),
            date: (2026, 10, 7),
            end_date: None,
            color: EventColor::Sky,
            completed: false,
        },
        CalEvent {
            id: "ev-3".to_string(),
            title: "Lunch with Client".to_string(),
            time: Some("12pm".to_string()),
            date: (2026, 10, 8),
            end_date: None,
            color: EventColor::Emerald,
            completed: false,
        },
        CalEvent {
            id: "ev-4".to_string(),
            title: "Product Launch".to_string(),
            time: None,
            date: (2026, 10, 10),
            end_date: Some((2026, 10, 13)),
            color: EventColor::Violet,
            completed: false,
        },
        CalEvent {
            id: "ev-5".to_string(),
            title: "Sales Conference".to_string(),
            time: Some("2:30pm".to_string()),
            date: (2026, 10, 11),
            end_date: None,
            color: EventColor::Rose,
            completed: false,
        },
        CalEvent {
            id: "ev-6".to_string(),
            title: "Sprint Planning".to_string(),
            time: None,
            date: (2026, 10, 12),
            end_date: None,
            color: EventColor::Pink,
            completed: false,
        },
        CalEvent {
            id: "ev-7".to_string(),
            title: "Team Meeting".to_string(),
            time: Some("9am".to_string()),
            date: (2026, 10, 12),
            end_date: None,
            color: EventColor::Orange,
            completed: false,
        },
        CalEvent {
            id: "ev-8".to_string(),
            title: "Design Sync".to_string(),
            time: Some("11am".to_string()),
            date: (2026, 10, 12),
            end_date: None,
            color: EventColor::Sky,
            completed: false,
        },
        CalEvent {
            id: "ev-9".to_string(),
            title: "Budget Review".to_string(),
            time: Some("3pm".to_string()),
            date: (2026, 10, 12),
            end_date: None,
            color: EventColor::Amber,
            completed: false,
        },
        CalEvent {
            id: "ev-10".to_string(),
            title: "Marketing Strategy Session".to_string(),
            time: Some("10am".to_string()),
            date: (2026, 10, 16),
            end_date: None,
            color: EventColor::Emerald,
            completed: false,
        },
        CalEvent {
            id: "ev-11".to_string(),
            title: "Annual Shareholders Meeting".to_string(),
            time: None,
            date: (2026, 10, 24),
            end_date: None,
            color: EventColor::Sky,
            completed: false,
        },
    ]
}

#[component]
pub fn Calendar() -> Element {
    let today = crate::state::get_real_current_date();

    let mut year = use_signal(move || today.0);
    let mut month = use_signal(move || today.1);
    let mut view = use_signal(|| "Month".to_string());

    let mut clicked_date = use_signal(|| None::<(u32, u32, u32)>);
    let mut day_dialog_open = use_signal(|| false);
    let mut add_dialog_open = use_signal(|| false);

    let app_state = use_app_state();
    let mut events = app_state.calendar_events;
    let overlay = use_context::<OverlayState>();
    use_effect(move || {
        let open = *day_dialog_open.read() || *add_dialog_open.read();
        overlay.set_dialog_open(open);
    });
    use_drop(move || {
        overlay.set_dialog_open(false);
    });

    // Keep the page in sync with the sidebar mini-calendar selection
    use_effect(move || {
        if let Some((y, m, _)) = *app_state.selected_date.read() {
            year.set(y);
            month.set(m);
        }
    });

    let month_label = format!("{} {}", MONTH_NAMES[(month() - 1) as usize], year());

    rsx! {
        div { class: "flex h-full min-h-0 flex-col",
            div { class: "flex-1 min-h-0 flex flex-col overflow-hidden rounded-2xl border border-border/40 bg-card shadow-xs",

            CalendarToolbar {
                month_label: month_label,
                view_label: view(),
                on_prev: move |_| {
                    let m = month();
                    let y = year();
                    if m == 1 {
                        month.set(12);
                        year.set(y - 1);
                    } else {
                        month.set(m - 1);
                    }
                },
                on_next: move |_| {
                    let m = month();
                    let y = year();
                    if m == 12 {
                        month.set(1);
                        year.set(y + 1);
                    } else {
                        month.set(m + 1);
                    }
                },
                on_today: move |_| {
                    year.set(today.0);
                    month.set(today.1);
                },
                on_view_change: move |new_view| view.set(new_view),
                on_new_event: move |_| {
                    clicked_date.set(Some(today));
                    add_dialog_open.set(true);
                },
            }

            WeekdayHeader {}

            div { class: "flex-1 min-h-0 flex flex-col overflow-hidden",
            MonthGrid {
                year: year(),
                month: month(),
                today: Some(today),
                events: events(),
                on_event_move: move |(id, new_date)| {
                    let mut list = events.write();
                    if let Some(event) = list.iter_mut().find(|e| e.id == id) {
                        shift_event(event, new_date);
                    }
                },
                on_day_click: move |date| {
                    clicked_date.set(Some(date));
                    let has_events = events.read().iter().any(|e| is_event_on(e, date));
                    if has_events {
                        day_dialog_open.set(true);
                    } else {
                        add_dialog_open.set(true);
                    }
                },
            }
            }
            }
        }

        DayEventsDialog {
            open: day_dialog_open,
            date: clicked_date().unwrap_or(today),
            events: events(),
            on_add_event: move |_| add_dialog_open.set(true),
        }

        NewEventDialog {
            open: add_dialog_open,
            date: clicked_date().unwrap_or(today),
            on_submit: move |event: CalEvent| {
                events.write().push(event);
            },
        }
    }
}
