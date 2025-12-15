use crate::{
    badge::{Badge, BadgePostfix, BadgePrefix, BadgeSize, BadgeTheme},
    button::{Button, ButtonAppearance},
    button_group::{ButtonGroup, First, Last},
    class_list,
    icon::{self, NextIcon, PreviousIcon},
    util::signals::{OptionModel, OptionModelWithValue},
};
use chrono::{DateTime, Datelike, Days, Local, Month, Months, NaiveDate, NaiveTime, Weekday};
use const_str::join;
use leptos::{
    logging::{debug_error, error},
    prelude::*,
    tachys::view::any_view::AnyView,
};
use leptos_use::{
    CalendarDate, UseCalendarOptions, UseCalendarReturn, use_calendar, use_calendar_with_options,
};
use num_traits::FromPrimitive;
use std::{fmt, iter, ops::Deref, sync::Arc};

const WORK_WEEK: [Weekday; 5] = [
    Weekday::Mon,
    Weekday::Tue,
    Weekday::Wed,
    Weekday::Thu,
    Weekday::Fri,
];

/// Will show weeks where at least 1 day is visible of the current month.
/// 
/// [date] date for which this week is checked, contains info on whether this date is of the current month.
/// [show_days] days visible in the calendar, non visible days are skipped
/// 
/// Returns whether the week should be shown.
fn should_show_week(date: &CalendarDate, show_days: &[Weekday]) -> bool {
    if !date.is_other_month() {
        return true;
    }
    
    let wrong_month = date.month();
    let Some(monday) = date.week(Weekday::Mon).checked_first_day() else {
        return false;
    };
    let mut checking_day = monday;
    for _ in iter::repeat_n(0, 7) {
        if checking_day.month() != wrong_month && show_days.contains(&checking_day.weekday()) {
            return true;
        }
        if let Some(next_day) = checking_day.succ_opt() {
            checking_day = next_day;
        }
    }
    return false;
}

#[component]
pub fn Calendar(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// selected date.
    #[prop(optional, into)]
    value: OptionModel<NaiveDate>,
    #[prop(default = Signal::derive(|| Local::now()), into)] local_date_time: Signal<
        DateTime<Local>,
    >,
    #[prop(default = RwSignal::new(Box::new(& WORK_WEEK)), into)] show_days: RwSignal<
        Box<&'static [Weekday]>,
    >,
    #[prop(default = Local::now().date_naive(), into)] initial_date: NaiveDate,
    #[prop(optional, into)] children: Option<CalendarChildrenFn>,
) -> impl IntoView {
    // Calendar helper object which backs the calendar view
    let UseCalendarReturn {
        dates,

        // vv Functions to mutate the above 2 signals.
        previous_month,
        next_month,
        today: goto_today,
        ..
    } = use_calendar_with_options(UseCalendarOptions {
        initial_date: Some(initial_date).into(),
        ..Default::default()
    });

    // Current date on the calendar.
    let current_calendar_date = Memo::new(move |_| {
        dates
            .get()
            .into_iter()
            .find_map(|date| {
                if !date.is_other_month() && date.is_first_day_of_month() {
                    Some(*date)
                } else {
                    None
                }
            })
            .unwrap_or(local_date_time.get().date_naive())
    });

    let is_current_month = Memo::new(move |_| {
        let local_date_time = local_date_time.get();
        current_calendar_date.get().month() == local_date_time.month()
            && current_calendar_date.get().year() == local_date_time.year()
    });

    let current_month_year = Memo::new(move |_| {
        let current = current_calendar_date.get();
        let mut out = String::new();
        if let Err(fmt::Error) = current.format("%B %Y").write_to(&mut out) {
            error!("%B %Y is no longer a valid chronos format string");
        }
        return out;
    });

    view! {
        <div class=class_list!["flex flex-col h-[720px]", class]>
            <div class="flex items-center justify-between pb-4">
                <span class="text-lg inline-flex">
                    <span class="w-[13ch] text-right">
                        {move || { current_month_year.get() }}
                    </span>
                    {move || if is_current_month.get() {
                        view! {
                            <Badge class="ml-2" theme=BadgeTheme::Success size=BadgeSize::Large border=true>Current Month</Badge>
                        }.into_any()
                    } else {
                        let local_date_time = local_date_time.get();
                        let badge_label = format!("back to {}", local_date_time.format("%B %Y").to_string());
                        if current_calendar_date.get() < local_date_time.date_naive() {
                            // Viewing the past
                            view! {
                                <Button class="ml-2" appearance=ButtonAppearance::Minimal
                                    on_click={ let goto_today = goto_today.clone(); move |_| goto_today() }
                                >
                                    <Badge
                                        theme=BadgeTheme::Warning
                                        postfix=BadgePostfix::Icon(icon::RightArrowIcon())
                                        size=BadgeSize::Large
                                        border=true
                                    >{badge_label}</Badge>
                                </Button>
                            }.into_any()
                        } else {
                            // Viewing into the future
                            view! {
                                <Button class="ml-2" appearance=ButtonAppearance::Minimal
                                    on_click={ let goto_today = goto_today.clone(); move |_| goto_today() }
                                >
                                    <Badge
                                        theme=BadgeTheme::Warning
                                        prefix=BadgePrefix::Icon(icon::LeftArrowIcon())
                                        size=BadgeSize::Large
                                        border=true
                                    >{badge_label}</Badge>
                                </Button>
                            }.into_any()
                        }
                    }}
                </span>
                <span>
                    <ButtonGroup>
                        <First slot:first><Button icon=PreviousIcon() on_click={ let previous_month = previous_month.clone(); move |_| previous_month() }/></First>
                        <Last slot:last><Button icon=NextIcon() on_click={ let next_month = next_month.clone(); move |_| next_month() }/></Last>
                    </ButtonGroup>
                </span>
            </div>
            <div class="flex grid border border-oa-gray-mid bg-oa-gray-mid gap-px grid-cols-5 grid-rows-[1lh_minmax(0,_1fr)]">
                <For each=move || show_days.get().iter() key=|idx| *idx let:idx>
                    <div class="text-right bg-oa-gray font-bold h-fit">
                        <span class="mr-2">
                        {idx.to_string()}
                        </span>
                    </div>
                </For>
                {move || {
                    dates
                        .get()
                        .into_iter()
                        .filter(|date| {
                            let show_days = show_days.get();
                            // Non visible dates are removed from the iterator.
                            if !show_days.contains(&date.weekday()) {
                                return false;
                            }
                            // Decide if this week should be shown at all.
                            should_show_week(date, *show_days)
                        })
                        .enumerate()
                        .map(|(index, date)| {
                            view! {
                                <CalendarItem
                                    value
                                    index=index
                                    date=date
                                    children=children.clone()
                                />
                            }
                        })
                        .collect_view()
                }}

            </div>
        </div>
    }
}

#[component]
fn CalendarItem(
    value: OptionModel<NaiveDate>,
    index: usize,
    date: CalendarDate,
    children: Option<CalendarChildrenFn>,
) -> impl IntoView {
    let is_selected = Memo::new({
        let date = date.clone();
        move |_| {
            value.with(|value_date| match value_date {
                OptionModelWithValue::T(v) => v == date.deref(),
                OptionModelWithValue::Option(v) => v.as_ref() == Some(date.deref()),
            })
        }
    });
    let on_click = {
        let date = date.clone();
        move |_| {
            value.set(Some(*date.deref()));
        }
    };

    // Some months start in the middle of a week, we then need to manually align calendar items at the correct column.
    let col_idx = date.weekday() as usize; // 0-indexed
    let col_class = [
        "col-[1]", "col-[2]", "col-[3]", "col-[4]", "col-[5]", "col-[6]", "col-[7]",
    ];
    if date.is_other_month() {
        view! {
            <div class="bg-gray-100 w-full h-full" />
        }.into_any()
    } else {
        view! {
            <div
                class=class_list!("flex flex-col pointer hover:bg-oa-gray bg-white overflow-auto", col_class[col_idx])
                class=("text-oa-gray", date.is_other_month())
                on:click=on_click
            >
                <div class="self-end">
                    <span class="flex justify-center items-center mr-2 text-oa-gray-darker font-bold">{date.day()}</span>
                </div>
                {children.map(|c| c(*date))}
            </div>
        }.into_any()
    }

}

struct CalendarDay {
    events: Vec<CalendarEvent>
}

struct CalendarEvent {
    start_time: NaiveTime,
    end_time: NaiveTime,
    title: String,
    desc: Option<String>,
    location: Option<String>,
}

#[derive(Clone)]
pub struct CalendarChildrenFn(Arc<dyn Fn(NaiveDate) -> AnyView + Send + Sync>);

impl Deref for CalendarChildrenFn {
    type Target = Arc<dyn Fn(NaiveDate) -> AnyView + Send + Sync>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F, C> From<F> for CalendarChildrenFn
where
    F: Fn(NaiveDate) -> C + Send + Sync + 'static,
    C: RenderHtml + Send + 'static,
{
    fn from(f: F) -> Self {
        Self(Arc::new(move |date| f(date).into_any()))
    }
}
