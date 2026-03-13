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
use std::fmt::Display;

use crate::calendar::CalendarChildrenFn;
use crate::calendar::MonthCalendarUncontrolled;
use crate::heading::Heading3;
use crate::radio::RadioOption;
use crate::{
    badge::{Badge, BadgePostfix, BadgePrefix, BadgeSize, BadgeTheme},
    button::{Button, ButtonAppearance},
    button_group::{ButtonGroup, First, Last},
    class_list,
    class_list::reactive_class::MaybeReactiveClass,
    icon::{self, NextIcon, PreviousIcon},
    util::callback::ArcOneCallback,
};
use chrono::DateTime;
use chrono::Datelike;
use chrono::Local;
use chrono::Month;
use chrono::NaiveDate;
use chrono::Weekday;
use chrono::WeekdaySet;
use leptos::prelude::*;
use leptos_use::CalendarDate;
use leptos_use::UseCalendarOptions;
use leptos_use::UseCalendarReturn;
use leptos_use::use_calendar_with_options;

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub enum YearCalendarLayout {
    #[default]
    Year,
    TwelveMonths,
}

impl Display for YearCalendarLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YearCalendarLayout::Year => write!(f, "Year"),
            YearCalendarLayout::TwelveMonths => write!(f, "12 Months"),
        }
    }
}

impl RadioOption for YearCalendarLayout {
    fn value(&self) -> Oco<'static, str> {
        match self {
            YearCalendarLayout::Year => "year".into(),
            YearCalendarLayout::TwelveMonths => "twelve_months".into(),
        }
    }
}

const MAX_DAYS_IN_MONTH: u8 = 31;
const TABLE_HEADER_CLASS: &str = "!bg-oa-gray text-center p-0.5 md:p-1 2xl:p-2";
const TABLE_CELL_CLASS: &str = "bg-white dark:bg-black hover:!bg-hover-light";
const MONTHS_IN_YEAR: [Month; 12] = [
    Month::January,
    Month::February,
    Month::March,
    Month::April,
    Month::May,
    Month::June,
    Month::July,
    Month::August,
    Month::September,
    Month::October,
    Month::November,
    Month::December,
];

pub fn dates_from_month(year: i32, month: Month) -> Vec<CalendarDate> {
    let Some(date_this_month) = NaiveDate::from_ymd_opt(year, month.number_from_month(), 1) else {
        return vec![];
    };
    // Calendar helper object which backs the calendar view
    let UseCalendarReturn { dates, .. } = use_calendar_with_options(UseCalendarOptions {
        initial_date: Some(date_this_month).into(),
        ..Default::default()
    });
    dates.get_untracked()
}

/// A grid containing every month's calendar.
#[component]
pub fn YearCalendarTwelveMonthLayout(
    current_year: RwSignal<i32>,
    /// Events to show on each day
    children: Option<CalendarChildrenFn>,
    /// Day highlighter, allows you to provide classes for various days.
    day_highlighter: Option<
        ArcOneCallback<(NaiveDate, YearCalendarLayout), MaybeReactiveClass>,
    >,
) -> impl IntoView {
    let show_days: RwSignal<WeekdaySet> = RwSignal::new(WeekdaySet::ALL);

    // TODO: Fix laggy rendering
    // Either render adjacent years async in the background so we can simply do an await -> DOM swap when navigating.
    // use Signals to update each calendar sequentially, first one would update with 1/12th the current delay.
    let calendars = MONTHS_IN_YEAR
        .map(|month| {
            let dates = Signal::from(dates_from_month(current_year.get(), month));
            view! {
                <div>
                    <div class="flex w-full justify-center">
                        <Heading3 anchor={month.name().to_lowercase()}>
                            {month.name()}
                        </Heading3>
                    </div>
                    <MonthCalendarUncontrolled show_days day_highlighter=day_highlighter.clone() children=children.clone() dates />
                </div>
            }
            .into_any()
        })
        .collect_view()
        .into_any();

    view! {
        <div class="grid grid-cols-1 lg:grid-cols-2 2xl:grid-cols-3 gap-6">
            {calendars}
        </div>
    }
    .into_any()
}

/// A single grid containing a full year of days.
#[component]
pub fn YearCalendarYearLayout(
    current_year: RwSignal<i32>,
    /// Events to show on each day
    children: Option<CalendarChildrenFn>,
    /// Day highlighter, allows you to provide classes for various days.
    day_highlighter: Option<
        ArcOneCallback<(NaiveDate, YearCalendarLayout), MaybeReactiveClass>,
    >,
) -> impl IntoView {
    let hovered_elem: RwSignal<Option<(Month, u8)>> = RwSignal::new(None);

    // [1][2]...[31]
    let day_cells = (1..=MAX_DAYS_IN_MONTH)
        .map(|day_in_month| {
            view! {
                <div class=class_list!(
                    ("!bg-hover-light", move || match hovered_elem.get() {
                        Some((_, hover_day)) => hover_day == day_in_month,
                        _ => false
                    }),
                    TABLE_HEADER_CLASS, TABLE_CELL_CLASS, "content-center"
                )>
                    {day_in_month}
                </div>
            }
        })
        .collect_view();

    // [Jan]|[]...[]
    // ⋮           ⋮
    // [Dec]|[]...[]
    let content_cells = MONTHS_IN_YEAR.map(|month| view! {
        // Month cells
        // [Jan]
        <div
            class=class_list!(
                ("!bg-hover-light", move || match hovered_elem.get() {
                    Some((hover_month, _)) => hover_month == month,
                    _ => false
                }),
                TABLE_HEADER_CLASS, TABLE_CELL_CLASS,
                "content-center text-start capitalize vertical-lr text-vertical
                md:max-lg:horizontal-tb md:max-lg:text-upright md:max-lg:text-center
                xl:horizontal-tb xl:text-upright"
            )
            on:mouseenter=move |_| {
                hovered_elem.set(None);
            }
        >
            <div class="rotate-180 md:max-lg:rotate-0 xl:rotate-0">
            {month.name()[0..3].to_string()}
            </div>
        </div>

        // []...[]
        {
            let children = children.clone();
            let day_highlighter = day_highlighter.clone();
            move || (1..=MAX_DAYS_IN_MONTH).map(|day_in_month| {
                let children = children.clone();
                let date = NaiveDate::from_ymd_opt(current_year.get(), month.number_from_month(), day_in_month as u32);
                if let Some(date) = date {
                    let mut cell_class_list = class_list!(TABLE_CELL_CLASS);

                    cell_class_list = if let Some(day_highlighter) = day_highlighter.clone() {
                        let day_highlight = day_highlighter((date, YearCalendarLayout::Year));
                        cell_class_list.add_class(day_highlight)
                    } else {
                        cell_class_list
                    };

                    view!{
                        <div
                            class=cell_class_list
                            on:mouseenter=move |_| {
                                hovered_elem.set(Some((month, day_in_month)));
                            }>
                            {children.map(|children| children.get()(date))}
                        </div>
                    }.into_any()
                } else {
                    // day_in_month is likely out of range
                    view!{
                        <div class=class_list!(TABLE_CELL_CLASS, "!bg-oa-gray-darker")></div>
                    }.into_any()
                }
            }).collect_view()
        }
    }.into_any()).collect_view().into_any();

    // Produces vvv (On small screens it will be transposed).
    //
    // [   ][1]...[31]
    // [Jan][ ]...[  ]
    // ⋮
    // [Dec][ ]...[  ]
    view! {
        <div class="grid gap-px
            grid-cols-[repeat(13,_minmax(0,_1fr))] grid-rows-[repeat(32,_minmax(0,_1fr))] grid-flow-col
            lg:grid-cols-[repeat(32,_minmax(0,_1fr))] lg:grid-rows-[repeat(13,_minmax(0,_1fr))] lg:grid-flow-row
            bg-oa-gray-darker border border-oa-gray-darker rounded-lg overflow-auto text-sm lg:text-base"
            on:mouseleave=move |_| {
                hovered_elem.set(None);
            }>
            // Header
            <div class=class_list!(TABLE_HEADER_CLASS, TABLE_CELL_CLASS)>""</div>
            {day_cells}

            // Content
            {content_cells}
        </div>
    }.into_any()
}

// (current_year <- back to local_year) ... <  >
#[component]
pub fn YearCalendarNavbar(
    current_year: RwSignal<i32>,
    local_date_time: Signal<DateTime<Local>>,
) -> impl IntoView {
    let is_current_year =
        Signal::derive(move || current_year.get() == local_date_time.get().year());

    view! {
        <div class="flex items-center justify-between pb-4">
            <span class="text-lg inline-flex">
                <span class="w-[5ch] text-right">
                    {move || { current_year.get() }}
                </span>

                // Badge: "Current Year" | "<- back to local_year" | "back to local_year ->"
                {move || if is_current_year.get() {
                    view! {
                        <Badge class="ml-2" theme=BadgeTheme::Success size=BadgeSize::Large border=true>
                            "Current Year"
                        </Badge>
                    }.into_any()
                } else {
                    let local_date_time = local_date_time.get();
                    let badge_label = format!("back to {}", local_date_time.format("%Y"));

                    let jump_badge = if current_year.get() < local_date_time.year() {
                        // Viewing the past
                        view! {
                            <Badge
                                theme=BadgeTheme::Warning
                                postfix=BadgePostfix::Icon(icon::RightArrowIcon())
                                size=BadgeSize::Large
                                border=true
                            >{badge_label}</Badge>
                        }.into_any()
                    } else {
                        // Viewing into the future
                        view! {
                            <Badge
                                theme=BadgeTheme::Warning
                                prefix=BadgePrefix::Icon(icon::LeftArrowIcon())
                                size=BadgeSize::Large
                                border=true
                            >{badge_label}</Badge>
                        }.into_any()
                    };

                    view! {
                        <Button
                            class="ml-2"
                            appearance=ButtonAppearance::Minimal
                            on_click=move |_| current_year.set(local_date_time.year())
                        >{jump_badge}</Button>
                    }.into_any()
                }}
            </span>

            // Nav buttons <  >
            <span>
                <ButtonGroup>
                    <First slot:first>
                        <Button icon=PreviousIcon() on_click=move |_|
                            current_year.update(|current_year| *current_year -= 1)
                        />
                    </First>
                    <Last slot:last>
                        <Button icon=NextIcon() on_click=move |_|
                            current_year.update(|current_year| *current_year += 1)
                        />
                    </Last>
                </ButtonGroup>
            </span>
        </div>
    }
}

pub fn calendar_weekend_highlighter(date: NaiveDate) -> MaybeReactiveClass {
    if date.weekday().number_from_monday() > Weekday::Fri.number_from_monday() {
        class_list!("!bg-oa-gray")
    } else {
        class_list!("")
    }
    .into()
}

/// Year-Calendar component.
///   Displays a full-year view, supports a single-grid full year layout or 12 month-calendars.
///   Custom content is possible via [children]
#[component]
pub fn YearCalendar(
    /// Real local time, calendar shows a jump button when viewing other years.
    #[prop(default = Signal::derive(|| Local::now()), into)]
    local_date_time: Signal<DateTime<Local>>,
    /// The year that is being shown.
    #[prop(default = RwSignal::new(local_date_time.get().year()))]
    current_year: RwSignal<i32>,
    /// Day highlighter, allows you to provide classes for various days.
    #[prop(default = ArcOneCallback::new(|(date, _)| calendar_weekend_highlighter(date)))]
    day_highlighter: ArcOneCallback<(NaiveDate, YearCalendarLayout), MaybeReactiveClass>,
    /// Yearcalendar layout.
    #[prop(optional, into)]
    layout: Signal<YearCalendarLayout>,
    /// Events to show on each day
    #[prop(optional, into)]
    children: Option<CalendarChildrenFn>,
) -> impl IntoView {
    view! {
        <YearCalendarNavbar current_year local_date_time />

        // Main calendar content
        {move || match layout.get() {
            YearCalendarLayout::TwelveMonths => view! {
                <YearCalendarTwelveMonthLayout current_year
                    day_highlighter=Some(day_highlighter.clone())
                    children=children.clone()
                />
            }.into_any(),
            YearCalendarLayout::Year => view! {
                <YearCalendarYearLayout current_year
                    day_highlighter=Some(day_highlighter.clone())
                    children=children.clone()
                />
            }.into_any()
        }}
    }
}
