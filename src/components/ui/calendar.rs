use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CalendarProps {
    #[props(default = 2026)]
    pub default_year: u32,

    #[props(default = 10)]
    pub default_month: u32,

    #[props(default)]
    pub selected: Option<Signal<Option<(u32, u32, u32)>>>,

    #[props(default)]
    pub onselect: Option<EventHandler<(u32, u32, u32)>>,

    #[props(default)]
    pub class: String,
}

#[allow(dead_code)]
fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn day_of_week(mut y: u32, mut m: u32, d: u32) -> usize {
    if m < 3 {
        m += 12;
        y -= 1;
    }
    let k = y % 100;
    let j = y / 100;
    let h = (d + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;
    ((h + 6) % 7) as usize
}

#[allow(dead_code)]
const MONTH_NAMES: [&str; 12] = [
    "January", "February", "March", "April", "May", "June", "July", "August", "September",
    "October", "November", "December",
];

#[allow(dead_code)]
const WEEKDAY_NAMES: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

#[component]
pub fn Calendar(props: CalendarProps) -> Element {
    let initial_y = props.default_year;
    let initial_m = props.default_month;

    let mut current_year = use_signal(move || initial_y);
    let mut current_month = use_signal(move || initial_m);

    let local_selected = use_signal(|| None);
    let mut selected = props.selected.unwrap_or(local_selected);

    let year = *current_year.read();
    let month = *current_month.read();

    let total_days = days_in_month(year, month);
    let start_weekday = day_of_week(year, month, 1);

    // Days from previous month to fill the first row
    let prev_month = if month == 1 { 12 } else { month - 1 };
    let prev_year = if month == 1 { year - 1 } else { year };
    let prev_days_count = days_in_month(prev_year, prev_month);

    let next_month = if month == 12 { 1 } else { month + 1 };
    let next_year = if month == 12 { year + 1 } else { year };

    let class = format!(
        "p-3 rounded-xl border border-border bg-card text-card-foreground shadow-xs w-[280px] {}",
        props.class
    );

    rsx! {
        div {
            "data-slot": "calendar",
            class: class,

            // Header: Month Year + Prev/Next buttons
            div {
                class: "flex items-center justify-between pb-4 pt-1 px-1",

                button {
                    r#type: "button",
                    class: "size-7 inline-flex items-center justify-center rounded-md border border-input \
                            bg-transparent text-muted-foreground hover:bg-accent hover:text-accent-foreground \
                            transition-colors cursor-pointer",
                    onclick: move |_| {
                        let m = current_month();
                        let y = current_year();
                        if m == 1 {
                            current_month.set(12);
                            current_year.set(y - 1);
                        } else {
                            current_month.set(m - 1);
                        }
                    },
                    svg {
                        class: "size-4",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path { d: "M15 18l-6-6 6-6" }
                    }
                    span { class: "sr-only", "Previous month" }
                }

                div {
                    class: "text-sm font-semibold text-foreground select-none",
                    "{MONTH_NAMES[(month - 1) as usize]} {year}"
                }

                button {
                    r#type: "button",
                    class: "size-7 inline-flex items-center justify-center rounded-md border border-input \
                            bg-transparent text-muted-foreground hover:bg-accent hover:text-accent-foreground \
                            transition-colors cursor-pointer",
                    onclick: move |_| {
                        let m = current_month();
                        let y = current_year();
                        if m == 12 {
                            current_month.set(1);
                            current_year.set(y + 1);
                        } else {
                            current_month.set(m + 1);
                        }
                    },
                    svg {
                        class: "size-4",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path { d: "M9 18l6-6-6-6" }
                    }
                    span { class: "sr-only", "Next month" }
                }
            }

            // Weekday Headers
            div {
                class: "grid grid-cols-7 gap-1 text-center mb-1",
                for day_name in WEEKDAY_NAMES {
                    div {
                        key: "{day_name}",
                        class: "text-[0.8rem] font-medium text-muted-foreground py-1 select-none",
                        "{day_name}"
                    }
                }
            }

            // Days Grid
            div {
                class: "grid grid-cols-7 gap-1 text-center",

                // Previous month padding days
                for i in 0..start_weekday {
                    {
                        let day_num = prev_days_count - (start_weekday as u32 - 1) + (i as u32);
                        rsx! {
                            button {
                                key: "prev-{day_num}",
                                r#type: "button",
                                class: "size-8 text-xs font-normal text-muted-foreground/40 rounded-md \
                                        flex items-center justify-center cursor-pointer hover:bg-accent/40",
                                onclick: move |_| {
                                    selected.set(Some((prev_year, prev_month, day_num)));
                                    if let Some(handler) = &props.onselect {
                                        handler.call((prev_year, prev_month, day_num));
                                    }
                                },
                                "{day_num}"
                            }
                        }
                    }
                }

                // Current month days
                for day in 1..=total_days {
                    {
                        let is_selected = match *selected.read() {
                            Some((y, m, d)) => y == year && m == month && d == day,
                            None => false,
                        };

                        let day_class = if is_selected {
                            "bg-primary text-primary-foreground font-semibold hover:bg-primary"
                        } else {
                            "text-foreground hover:bg-accent hover:text-accent-foreground"
                        };

                        rsx! {
                            button {
                                key: "current-{day}",
                                r#type: "button",
                                class: "size-8 text-xs font-normal rounded-md flex items-center justify-center \
                                        transition-colors cursor-pointer {day_class}",
                                onclick: move |_| {
                                    selected.set(Some((year, month, day)));
                                    if let Some(handler) = &props.onselect {
                                        handler.call((year, month, day));
                                    }
                                },
                                "{day}"
                            }
                        }
                    }
                }

                // Next month trailing days
                {
                    let total_rendered = start_weekday as u32 + total_days;
                    let remaining = (7 - (total_rendered % 7)) % 7;
                    rsx! {
                        for next_d in 1..=remaining {
                            button {
                                key: "next-{next_d}",
                                r#type: "button",
                                class: "size-8 text-xs font-normal text-muted-foreground/40 rounded-md \
                                        flex items-center justify-center cursor-pointer hover:bg-accent/40",
                                onclick: move |_| {
                                    selected.set(Some((next_year, next_month, next_d)));
                                    if let Some(handler) = &props.onselect {
                                        handler.call((next_year, next_month, next_d));
                                    }
                                },
                                "{next_d}"
                            }
                        }
                    }
                }
            }
        }
    }
}
