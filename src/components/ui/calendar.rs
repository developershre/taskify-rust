use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CalendarProps {
    #[props(default)]
    pub default_year: Option<u32>,

    #[props(default)]
    pub default_month: Option<u32>,

    #[props(default)]
    pub selected: Option<Signal<Option<(u32, u32, u32)>>>,

    #[props(default)]
    pub onselect: Option<EventHandler<(u32, u32, u32)>>,

    #[props(default)]
    pub class: String,
}

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

// Sakamoto's algorithm: Day of week for 1st of month (0 = Sunday, 1 = Monday, ..., 6 = Saturday)
fn first_day_of_month(year: u32, month: u32) -> usize {
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut y = year;
    if month < 3 {
        y -= 1;
    }
    ((y + y / 4 - y / 100 + y / 400 + t[(month - 1) as usize] + 1) % 7) as usize
}

const MONTH_NAMES: [&str; 12] = [
    "January", "February", "March", "April", "May", "June", "July", "August", "September",
    "October", "November", "December",
];

const WEEKDAY_NAMES: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

#[component]
pub fn Calendar(props: CalendarProps) -> Element {
    let real_now = crate::state::get_real_current_date();
    let initial_y = props.default_year.unwrap_or(real_now.0);
    let initial_m = props.default_month.unwrap_or(real_now.1);

    let mut current_year = use_signal(move || initial_y);
    let mut current_month = use_signal(move || initial_m);

    let local_selected = use_signal(move || Some(real_now));
    let mut selected = props.selected.unwrap_or(local_selected);

    let year = current_year();
    let month = current_month();

    let total_days = days_in_month(year, month);
    let start_weekday = first_day_of_month(year, month);

    // Days from previous month to fill the first row
    let prev_month = if month == 1 { 12 } else { month - 1 };
    let prev_year = if month == 1 { year - 1 } else { year };
    let prev_days_count = days_in_month(prev_year, prev_month);

    let next_month = if month == 12 { 1 } else { month + 1 };
    let next_year = if month == 12 { year + 1 } else { year };

    let total_cells = start_weekday as u32 + total_days;
    let grid_cells = if total_cells <= 35 { 35 } else { 42 };
    let trailing_count = grid_cells - total_cells;

    let class = format!(
        "w-full flex flex-col gap-4 select-none {}",
        props.class
    );

    rsx! {
        div {
            "data-slot": "calendar",
            class: class,

            // Header: Month Year + Prev/Next buttons
            div {
                class: "flex items-center justify-between px-1",

                // Previous Month Button
                button {
                    r#type: "button",
                    class: "size-8 rounded-lg bg-secondary/80 hover:bg-secondary text-foreground flex items-center justify-center transition-colors cursor-pointer border border-border/40 shadow-xs",
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
                        path { d: "m15 18-6-6 6-6" }
                    }
                    span { class: "sr-only", "Previous month" }
                }

                // Month & Year Label
                div {
                    class: "text-sm font-semibold tracking-tight text-foreground select-none",
                    "{MONTH_NAMES[(month - 1) as usize]} {year}"
                }

                // Next Month Button
                button {
                    r#type: "button",
                    class: "size-8 rounded-lg bg-secondary/80 hover:bg-secondary text-foreground flex items-center justify-center transition-colors cursor-pointer border border-border/40 shadow-xs",
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
                        path { d: "m9 18 6-6-6-6" }
                    }
                    span { class: "sr-only", "Next month" }
                }
            }

            // Weekday Headers: Su Mo Tu We Th Fr Sa
            div {
                class: "grid grid-cols-7 gap-1 text-center text-xs font-medium text-muted-foreground pt-1",
                for day_name in WEEKDAY_NAMES {
                    div {
                        key: "{day_name}",
                        class: "select-none",
                        "{day_name}"
                    }
                }
            }

            // Days Grid
            div {
                class: "grid grid-cols-7 gap-y-2 gap-x-1 text-center text-sm",

                // Previous month padding days
                for i in 0..start_weekday {
                    {
                        let day_num = prev_days_count - (start_weekday as u32 - 1) + (i as u32);
                        rsx! {
                            button {
                                key: "prev-{day_num}",
                                r#type: "button",
                                class: "size-8 mx-auto flex items-center justify-center text-xs text-muted-foreground/35 select-none font-normal rounded-lg hover:bg-muted/40 cursor-pointer",
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

                        let is_today = real_now.0 == year && real_now.1 == month && real_now.2 == day;

                        let day_class = if is_selected {
                            "size-8 mx-auto flex items-center justify-center rounded-lg bg-zinc-700 text-white dark:bg-zinc-700 dark:text-white font-semibold text-xs shadow-xs transition-colors cursor-pointer"
                        } else if is_today {
                            "size-8 mx-auto flex items-center justify-center rounded-lg border border-primary/50 text-foreground font-semibold text-xs hover:bg-muted transition-colors cursor-pointer"
                        } else {
                            "size-8 mx-auto flex items-center justify-center rounded-lg text-foreground hover:bg-muted text-xs font-normal transition-colors cursor-pointer"
                        };

                        rsx! {
                            button {
                                key: "current-{day}",
                                r#type: "button",
                                class: day_class,
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
                for next_d in 1..=trailing_count {
                    button {
                        key: "next-{next_d}",
                        r#type: "button",
                        class: "size-8 mx-auto flex items-center justify-center text-xs text-muted-foreground/35 select-none font-normal rounded-lg hover:bg-muted/40 cursor-pointer",
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
