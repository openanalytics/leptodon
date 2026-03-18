// Leptodon
//
// Copyright (C) 2025-2026 Open Analytics NV
//
// ===========================================================================
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the Apache License as published by The Apache Software
// Foundation, either version 2 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the Apache License for more details.
//
// You should have received a copy of the Apache License along with this program.
// If not, see <http://www.apache.org/licenses/>
use crate::{
    badge::{Badge, BadgePostfix, BadgePrefix, BadgeSize, BadgeTheme},
    button::{Button, ButtonAppearance},
    button_group::{ButtonGroup, First, Last},
    class_list,
    class_list::reactive_class::MaybeReactiveClass,
    icon::{self, NextIcon, PreviousIcon},
    popover::{Popover, PopoverAnchor, PopoverTrigger},
    util::{callback::ArcOneCallback, option_comp::OptionComp},
};
use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveTime, Weekday, WeekdaySet};
use leptodon_proc_macros::generate_docs;
use leptos::{
    either::Either,
    logging::{error, warn},
    prelude::*,
    tachys::view::any_view::AnyView,
};
use leptos_use::{CalendarDate, UseCalendarOptions, UseCalendarReturn, use_calendar_with_options};
use std::{fmt, iter, ops::Deref};

mod year;
pub use year::*;

const WORK_WEEK: WeekdaySet = WeekdaySet::from_array([
    Weekday::Mon,
    Weekday::Tue,
    Weekday::Wed,
    Weekday::Thu,
    Weekday::Fri,
]);

/// Will show weeks where at least 1 day is visible of the current month.
///
/// [date] date for which this week is checked, contains info on whether this date is of the current month.
/// [show_days] days visible in the calendar, non visible days are skipped
///
/// Returns whether the week should be shown.
fn should_show_week(date: &CalendarDate, show_days: WeekdaySet) -> bool {
    if !date.is_other_month() {
        return true;
    }

    let wrong_month = date.month();
    let Some(monday) = date.week(Weekday::Mon).checked_first_day() else {
        return false;
    };
    let mut checking_day = monday;
    for _ in iter::repeat_n(0, 7) {
        if checking_day.month() != wrong_month && show_days.contains(checking_day.weekday()) {
            return true;
        }
        if let Some(next_day) = checking_day.succ_opt() {
            checking_day = next_day;
        }
    }
    false
}

#[component]
pub fn MonthCalendarNav(
    current_calendar_date: Memo<NaiveDate>,
    local_date_time: Signal<DateTime<Local>>,
    goto_today: impl Fn() + Clone + Send + Sync + 'static,
    previous_month: impl Fn() + Clone + Send + Sync + 'static,
    next_month: impl Fn() + Clone + Send + Sync + 'static,
) -> impl IntoView {
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
        out
    });

    view! {
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
                    let badge_label = format!("back to {}", local_date_time.format("%B %Y"));
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
    }
}

#[component]
pub fn MonthCalendarUncontrolled(
    /// Days to show, acts as a filter over [dates]
    #[prop(default = RwSignal::new(WORK_WEEK), into)]
    show_days: RwSignal<WeekdaySet>,
    /// Day highlighter, allows you to provide classes for various days.
    day_highlighter: Option<
        ArcOneCallback<(NaiveDate, YearCalendarLayout), MaybeReactiveClass>,
    >,
    /// First day of the week.
    #[prop(default = Weekday::Mon)]
    start_of_week: Weekday,
    /// Provide content inside the day cell
    children: Option<CalendarChildrenFn>,
    /// Days in this month.
    dates: Signal<Vec<CalendarDate>>,
) -> impl IntoView {
    // Explicity is needed for tailwind's css generation.
    let grid_col_class = move || match show_days.get().len() {
        1 => "grid-cols-1",
        2 => "grid-cols-2",
        3 => "grid-cols-3",
        4 => "grid-cols-4",
        5 => "grid-cols-5",
        6 => "grid-cols-6",
        7 => "grid-cols-7",
        _ => "",
    };
    view! {
        <div class=move || class_list!(grid_col_class(), "grid border border-oa-gray-mid dark:border-gray-700 bg-oa-gray-mid dark:bg-gray-600 gap-px grid-rows-[1lh_minmax(0,_1fr)] rounded-lg shadow-sm overflow-auto")>
            <For each=move || show_days.get().iter(Weekday::Mon) key=|idx| *idx let:idx>
                <div class="text-right bg-oa-gray dark:bg-gray-700 font-bold h-fit">
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
                        if !show_days.contains(date.weekday()) {
                            return false;
                        }
                        // Decide if this week should be shown at all.
                        should_show_week(date, show_days)
                    })
                    .map(|date| {
                        view! {
                            <CalendarDay
                                date
                                start_of_week=start_of_week
                                weekdays=show_days.get()
                                day_highlighter=day_highlighter.clone()
                                children=children.clone()
                            />
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}

#[generate_docs]
/// Calendar component.
///   Displays a full-month view, shown days can be configured.
///   Custom content is possible via [children]
#[component]
pub fn Calendar(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Get notified of the visible calendar month.
    #[prop(optional)]
    presented_month_writer: Option<WriteSignal<NaiveDate>>,
    #[prop(default = Signal::derive(|| Local::now()), into)] local_date_time: Signal<
        DateTime<Local>,
    >,
    #[prop(default = Weekday::Mon)] start_of_week: Weekday,
    #[prop(default = RwSignal::new(WORK_WEEK))] show_days: RwSignal<WeekdaySet>,
    #[prop(default = local_date_time.get().date_naive(), into)] initial_date: NaiveDate,
    /// Day highlighter, allows you to provide classes for various days.
    #[prop(optional)]
    day_highlighter: MaybeProp<
        ArcOneCallback<(NaiveDate, YearCalendarLayout), MaybeReactiveClass>,
    >,
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

    // Notify the outside of a change of calendar month.
    Effect::new(move || {
        presented_month_writer.inspect(|writer| writer.set(current_calendar_date.get()));
    });

    view! {
        <div class=class_list!["flex flex-col h-[810px]", class]>
            <MonthCalendarNav current_calendar_date local_date_time goto_today previous_month next_month />
            <MonthCalendarUncontrolled start_of_week show_days children day_highlighter=day_highlighter.get() dates />
        </div>
    }
}

// requires: [date](CalendarDate#weekday()) ∈ [weekdays]
// this is a specific component for grid based calendar, uses col and row to position days in combination with [weekdays].
#[component]
fn CalendarDay(
    date: CalendarDate,
    start_of_week: Weekday,
    /// weekdays of the week being shown
    weekdays: WeekdaySet,
    /// Day highlighter, allows you to provide classes for various days.
    day_highlighter: Option<
        ArcOneCallback<(NaiveDate, YearCalendarLayout), MaybeReactiveClass>,
    >,
    children: Option<CalendarChildrenFn>,
) -> impl IntoView {
    // Some months start in the middle of a week, we then need to manually align calendar items at the correct column.
    let col_idx = weekdays
        .iter(start_of_week)
        .position(|s| s == date.weekday())
        .unwrap_or_else(|| {
            warn!("CalendarDay constructed with a date not present in their weekdays set.");
            0
        });

    let col_class = [
        "col-[1]", "col-[2]", "col-[3]", "col-[4]", "col-[5]", "col-[6]", "col-[7]",
    ];
    if date.is_other_month() {
        view! {
            <div class="bg-gray-100 dark:bg-[#030712] w-full h-full" />
        }
        .into_any()
    } else {
        let mut cell_class_list = class_list!(
            col_class[col_idx],
            "flex flex-col pointer hover:bg-oa-gray bg-white hover:dark:bg-gray-800 dark:bg-gray-900 overflow-auto h-[6lh]"
        );

        cell_class_list = if let Some(day_highlighter) = day_highlighter.clone() {
            let day_highlight = day_highlighter((*date, YearCalendarLayout::Year));
            cell_class_list.add_class(day_highlight)
        } else {
            cell_class_list
        };

        view! {
            <div
                class=cell_class_list
                class=("text-oa-gray", date.is_other_month())
            >
                <div class="self-end">
                    <span class="flex justify-center items-center mr-2 text-oa-gray-darker font-bold">{date.day()}</span>
                </div>
                {if let Some(children) = children {
                    let children = children.clone();
                    Either::Left(move || (*children).get()(*date))
                } else {
                    Either::Right(())
                }}
            </div>
        }.into_any()
    }
}

#[generate_docs]
/// A rectangular calendar-event made to fit inside a calendar-day, shows more details on hover.
#[component]
pub fn CalendarEvent(
    /// Start time, shown next to the event
    #[prop(optional, into)]
    start_time: Option<NaiveTime>,
    /// Event end time, shown in the popover
    #[prop(optional, into)]
    end_time: Option<NaiveTime>,
    /// Event summary, shown in the calendar
    #[prop(optional, into)]
    summary: Option<String>,
    /// Title of the event-popup
    #[prop(optional, into)]
    popup_title: Option<String>,
    /// Content of the event-popup
    #[prop(optional, into)]
    popup_desc: Option<String>,
) -> impl IntoView {
    view! {
        <Popover preferred_pos=PopoverAnchor::Right>
            <PopoverTrigger slot>
                <div class=class_list!("self-stretch p-0.5 bg-teal-100 dark:bg-teal-900 m-0.5 shadow-sm text-xs md:text-sm line-clamp-3 shrink-0", ("grow", start_time.is_none()))>
                    <OptionComp value=start_time let:start_time>
                        <strong class="mr-[0.5ch] font-mono">
                            {start_time.format("%H:%M").to_string()}
                        </strong>
                    </OptionComp>
                    <OptionComp value=summary.clone() let:summary>
                        <span>
                            {summary.to_string()}
                        </span>
                    </OptionComp>
                </div>
            </PopoverTrigger>
            <div>
                <OptionComp value=start_time let:start_time>
                    <OptionComp value=end_time let:end_time>
                        <strong class="mr-[0.5ch] font-mono">
                            {start_time.format("%H:%M").to_string()} - {end_time.format("%H:%M").to_string()}
                        </strong>
                    </OptionComp>
                </OptionComp>
                <OptionComp value=popup_title let:title>
                    <p><strong>
                        {title.to_string()}
                    </strong></p>
                </OptionComp>
                <OptionComp value=popup_desc let:desc>
                    <p>
                        {desc.to_string()}
                    </p>
                </OptionComp>
            </div>
        </Popover>
    }
}

#[derive(Clone)]
pub struct CalendarChildrenFn(RwSignal<ArcOneCallback<NaiveDate, AnyView>>);

impl Deref for CalendarChildrenFn {
    type Target = RwSignal<ArcOneCallback<NaiveDate, AnyView>>;

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
        Self(RwSignal::new(ArcOneCallback::new(move |date| {
            f(date).into_any()
        })))
    }
}
