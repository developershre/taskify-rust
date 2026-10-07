use dioxus::prelude::*;

use crate::components::month_calendar::{
    CalendarToolbar, DayCell, DayEventsDialog, EventChip, EventColor, EventInfoCard, MonthGrid,
    NewEventDialog, WeekdayHeader,
};

#[derive(Props, Clone, PartialEq)]
struct SectionProps {
    pub title: String,
    pub desc: String,
    pub children: Element,
}

#[component]
fn Section(props: SectionProps) -> Element {
    rsx! {
        div { class: "flex flex-col gap-3",
            div { class: "flex flex-col gap-1",
                h2 { class: "text-base font-semibold tracking-tight text-foreground", "{props.title}" }
                p { class: "text-xs text-muted-foreground", "{props.desc}" }
            }
            div { class: "rounded-xl border border-border/40 bg-card p-4 shadow-xs",
                {props.children}
            }
        }
    }
}

#[component]
pub fn ComponentsCalendar() -> Element {
    let today = crate::state::get_real_current_date();
    let events = crate::views::calendar::sample_events();

    let mut demo_day_open = use_signal(|| false);
    let mut demo_add_open = use_signal(|| false);

    let overlay = use_context::<crate::state::OverlayState>();
    use_effect(move || {
        let open = *demo_day_open.read() || *demo_add_open.read();
        overlay.set_dialog_open(open);
    });
    use_drop(move || {
        overlay.set_dialog_open(false);
    });

    let chip_demos = [
        ("Project Deadline", Some("1pm"), EventColor::Amber, true),
        ("Team Meeting", Some("10am"), EventColor::Sky, false),
        (
            "Lunch with Client",
            Some("12pm"),
            EventColor::Emerald,
            false,
        ),
        ("Product Launch", None, EventColor::Violet, false),
        ("Sales Conference", Some("2:30pm"), EventColor::Rose, false),
        ("Team Meeting", Some("9am"), EventColor::Orange, false),
        ("Sprint Planning", None, EventColor::Pink, false),
    ];

    rsx! {
        div { class: "flex flex-col gap-8",

            div { class: "flex flex-col gap-1",
                h1 { class: "text-2xl font-bold tracking-tight text-foreground", "Calendar" }
                p { class: "text-sm text-muted-foreground",
                    "Building blocks used to compose the calendar month view."
                }
            }

            Section {
                title: "CalendarToolbar".to_string(),
                desc: "Month label, prev/next navigation, view switcher and new event action.".to_string(),
                div { class: "rounded-xl border border-border/40 overflow-hidden",
                    CalendarToolbar {
                        month_label: "October 2026".to_string(),
                        on_prev: move |_| {},
                        on_next: move |_| {},
                        on_today: move |_| {},
                    }
                }
            }

            Section {
                title: "WeekdayHeader".to_string(),
                desc: "Column labels for the seven day-of-week columns.".to_string(),
                div { class: "rounded-xl border border-border/40 overflow-hidden",
                    WeekdayHeader {}
                }
            }

            Section {
                title: "EventChip".to_string(),
                desc: "Color variants, optional time prefix and completed strikethrough.".to_string(),
                div { class: "flex flex-wrap items-center gap-2",
                    for demo in chip_demos {
                        EventChip {
                            key: "{demo.0}-{demo.2:?}",
                            title: demo.0.to_string(),
                            time: demo.1.map(|t| t.to_string()),
                            color: demo.2,
                            completed: demo.3,
                        }
                    }
                }
            }

            Section {
                title: "DayCell".to_string(),
                desc: "Single day column: today highlight, other-month days, event stack and overflow.".to_string(),
                div { class: "grid grid-cols-2 sm:grid-cols-4 border-t border-l border-border/40 min-h-28",
                    DayCell { date: (2026, 10, 6), events: events.iter().filter(|e| e.date == (2026, 10, 6)).cloned().collect() }
                    DayCell { date: (2026, 10, 7), is_today: true, events: events.iter().filter(|e| e.date == (2026, 10, 7)).cloned().collect() }
                    DayCell { date: (2026, 10, 12), events: events.iter().filter(|e| e.date == (2026, 10, 12)).cloned().collect() }
                    DayCell { date: (2026, 9, 30), is_other_month: true, events: events.iter().filter(|e| e.date == (2026, 9, 30)).cloned().collect() }
                }
            }

            Section {
                title: "MonthGrid".to_string(),
                desc: "Full month grid: leading/trailing days, today detection and per-day event filtering.".to_string(),
                div { class: "rounded-xl border border-border/40 overflow-hidden h-[600px]",
                    MonthGrid {
                        year: 2026,
                        month: 10,
                        today: Some(today),
                        events: events.clone(),
                    }
                }
            }

            Section {
                title: "EventInfoCard".to_string(),
                desc: "Rich event row: color, time, title, done badge and deadline with remaining status.".to_string(),
                div { class: "flex max-w-md flex-col gap-2",
                    EventInfoCard { event: events[3].clone() }
                    EventInfoCard { event: events[0].clone() }
                }
            }

            Section {
                title: "DayEventsDialog".to_string(),
                desc: "Full 24-hour day timeline: all-day section on top, hour rows, deadline and status per event, with an add action.".to_string(),
                div { class: "flex flex-col items-start gap-3",
                    button {
                        r#type: "button",
                        class: "inline-flex h-8 items-center rounded-md border border-border px-3 text-xs font-medium text-foreground hover:bg-muted transition-colors cursor-pointer",
                        onclick: move |_| demo_day_open.set(true),
                        "Open day dialog (Oct 12)"
                    }
                    DayEventsDialog {
                        open: demo_day_open,
                        date: (2026, 10, 12),
                        events: events.clone(),
                        on_add_event: move |_| demo_add_open.set(true),
                    }
                }
            }

            Section {
                title: "NewEventDialog".to_string(),
                desc: "Create form: title, optional time and color swatch picker for the target date.".to_string(),
                div { class: "flex flex-col items-start gap-3",
                    button {
                        r#type: "button",
                        class: "inline-flex h-8 items-center rounded-md border border-border px-3 text-xs font-medium text-foreground hover:bg-muted transition-colors cursor-pointer",
                        onclick: move |_| demo_add_open.set(true),
                        "Open new event dialog"
                    }
                    NewEventDialog {
                        open: demo_add_open,
                        date: (2026, 10, 8),
                    }
                }
            }
        }
    }
}
