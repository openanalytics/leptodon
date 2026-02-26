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
use attr_docgen::generate_docs;
use chrono::Days;
use leptos::logging::debug_log;
use leptos::prelude::GetUntracked;
use leptos::prelude::GlobalAttributes;
// Do not remove until leptos is upgraded above 0.8.14
use chrono::Datelike;
use chrono::Local;
use chrono::Month;
use chrono::Months;
use chrono::Weekday;
use chrono::format::ParseErrorKind;
use leptos::html::Div;
use leptos::prelude::AddAnyAttr;
use leptos::prelude::CollectView;
use leptos::prelude::Effect;
use leptos::prelude::FlattenOptionRefOption;
use leptos::prelude::IntoAny;
#[allow(unused)]
use leptos::prelude::IntoAnyAttribute;
use leptos::prelude::Memo;
use leptos::prelude::NodeRef;
use leptos::prelude::NodeRefAttribute;
use leptos::prelude::Update;
use leptos_use::CalendarDate;
use leptos_use::OnClickOutsideOptions;
use leptos_use::UseCalendarReturn;
use leptos_use::on_click_outside_with_options;
use leptos_use::use_calendar;
use num_traits::FromPrimitive;
use std::cmp::Ordering;
use std::str::FromStr;
use web_sys::KeyboardEvent;

use crate::button::Button;
use crate::button::ButtonAppearance;
use crate::button::ControlButton;
use crate::class_list;
use crate::icon;
use crate::input::GenericInput;
use crate::util::callback::ArcOneCallback;
use crate::util::callback::BoxOneCallback;
use crate::util::shared_id::shared_id;
use chrono::NaiveDate;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::OnAttribute;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::{
    IntoView, component,
    prelude::{MaybeProp, Signal},
    view,
};
pub mod range_picker;

const MILLENIUM_IN_MONTHS: Months = Months::new(12 * 100);
const DECENIA_IN_MONTHS: Months = Months::new(12 * 10);
const YEAR_IN_MONTHS: Months = Months::new(12);

/// Elements refer to the date-picker elements like individual days, months, years.
const SELECTED_ELEM_CLASSES: &str = "hover:!bg-oa-blue-lighter bg-oa-blue text-white ";
const SELECTABLE_ELEM_CLASSES: &str = "hover:bg-oa-gray hover:dark:bg-gray-600  block flex-1 leading-9 border-0 cursor-pointer text-center text-body font-medium text-sme";
const DISABLED_ELEM_CLASSES: &str = "opacity-40 block flex-1 leading-9 border-0 text-center text-body font-medium text-sme cursor-not-allowed";

const MONTHS: [Month; 12] = [
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

// Decenia need to be passed as a year rounded to the tens.
// e.g. 2010, refers to 2010-2019
fn naive_date_with_decenium(date: &NaiveDate, decenium: i32) -> NaiveDate {
    assert!(decenium % 10 == 0);
    let remainder = date.year() % 10;
    date.with_year(decenium.saturating_add(remainder))
        .unwrap_or(NaiveDate::MAX)
}

// Decenia need to be passed as a year rounded to the tens.
// e.g. 2010, refers to 2010-2019
fn decenium_from_naive_date(date: &NaiveDate) -> i32 {
    let remainder = date.year() % 10;
    date.year().saturating_sub(remainder)
}

/// Returns the [menu]-granular date-range containing [date].
///   e.g.
///   * 2022-11-13, DatePickerMenu::Day -> (2022-11-13, 2022-11-13)
///   * 2022-11-13, DatePickerMenu::Month -> (2022-11-1, 2022-11-30)
///   * 2022-11-13, DatePickerMenu::Year -> (2022-1-1, 2022-12-31)
///   * 2022-11-13, DatePickerMenu::Decenium -> (2020-1-1, 2029-12-31)
fn menu_range(date: NaiveDate, menu: DatePickerMenu) -> (NaiveDate, NaiveDate) {
    match menu {
        DatePickerMenu::Day => (date, date),
        DatePickerMenu::Month => {
            let start = date.with_day(1).unwrap_or(NaiveDate::MIN);
            let end = date
                .with_day(1)
                .unwrap_or(NaiveDate::MIN)
                .checked_add_months(Months::new(1))
                .unwrap_or(NaiveDate::MAX)
                .checked_sub_days(Days::new(1))
                .unwrap_or(NaiveDate::MAX);
            (start, end)
        }
        DatePickerMenu::Year => {
            let start = date
                .with_day(1)
                .unwrap_or(NaiveDate::MIN)
                .with_month(1)
                .unwrap_or(NaiveDate::MIN);
            let end = date
                .with_day(1)
                .unwrap_or(NaiveDate::MIN)
                .with_month(12)
                .unwrap_or(NaiveDate::MAX)
                .checked_add_months(Months::new(1))
                .unwrap_or(NaiveDate::MAX)
                .checked_sub_days(Days::new(1))
                .unwrap_or(NaiveDate::MAX);
            (start, end)
        }
        DatePickerMenu::Decenia => {
            let base_year = decenium_from_naive_date(&date);
            let start = NaiveDate::from_ymd_opt(base_year, 1, 1).unwrap_or(NaiveDate::MIN);
            let end = NaiveDate::from_ymd_opt(base_year + 9, 12, 31).unwrap_or(NaiveDate::MAX);
            (start, end)
        }
    }
}

/// Whether menu-granular-[date]-range intersects the (min_date..=max_date) range.
fn menu_item_intersects_range(
    date: NaiveDate,
    menu: DatePickerMenu,
    min_date: MaybeProp<NaiveDate>,
    max_date: MaybeProp<NaiveDate>,
) -> bool {
    let (menu_min, menu_max) = menu_range(date, menu);
    if let Some(min_date) = min_date.get() {
        if let Some(max_date) = max_date.get() {
            menu_max >= min_date && menu_min <= max_date
        } else {
            menu_max > min_date
        }
    } else if let Some(max_date) = max_date.get() {
        menu_min < max_date
    } else {
        true
    }
}

/// Stores visibility and menu state information for a date-picker
#[derive(Default, Clone)]
pub(crate) struct DatePickerState {
    pub visible: bool,
    pub menu: DatePickerMenu,
}

impl DatePickerState {
    /// Creates a copy of [&self] that is visible.
    fn show(&mut self) {
        self.visible = true;
    }
    /// Creates a copy of [&self] that is not visible.
    fn hide(&mut self) {
        self.visible = false;
    }
    /// Creates a copy of [&self] that is not visible.
    fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }
    /// Creates a copy of [&self] with [menu] as menu state.
    fn set_menu(&mut self, menu: DatePickerMenu) {
        self.menu = menu;
    }
}

#[derive(Default, Copy, Clone)]
pub(crate) enum DatePickerMenu {
    #[default]
    Day,
    Month,
    Year,
    Decenia,
}

// Navigation component for the day picking view of the date-picker.
// Looks like: [<] [month] [>]
#[component]
fn DayPickerMenuNav<PreviousMonthFn, NextMonthFn>(
    previous_month: PreviousMonthFn,
    next_month: NextMonthFn,
    current_date: Memo<NaiveDate>,
    picker_state: RwSignal<DatePickerState>,
) -> impl IntoView
where
    PreviousMonthFn: Fn() + Clone + Send + Sync + 'static,
    NextMonthFn: Fn() + Clone + Send + Sync + 'static,
{
    let current_month_year = Memo::new(move |_| {
        let current = current_date.get();
        format!(
            "{} {}",
            Month::from_u32(current.month()).unwrap().name(),
            current.year(),
        )
    });

    view! {
        <ControlButton icon=icon::PreviousIcon() on_click=move |_| { previous_month() } {..} tabindex="-1"></ControlButton>
        <Button appearance=ButtonAppearance::Transparent on_click=move |_| {
            picker_state.update(|state| state.set_menu(DatePickerMenu::Month))
        } {..} tabindex="-1">{ move || current_month_year.get() }</Button>
        <ControlButton icon=icon::NextIcon() on_click=move |_| { next_month() } {..} tabindex="-1"></ControlButton>
    }
}

// Day picking section of the date-picker.
#[component]
fn DayPickerMenu(
    weekdays: Signal<Vec<usize>>,
    #[prop(into)] highlighter: MaybeProp<ArcOneCallback<DateMenuOption, String>>,
    dates: Signal<Vec<CalendarDate>>,
    value: RwSignal<Option<NaiveDate>>,
    min_date: MaybeProp<NaiveDate>,
    max_date: MaybeProp<NaiveDate>,
) -> impl IntoView {
    view! {
        <div class="days">
            <div class="days-of-week grid grid-cols-7 mb-1">
                {move || {
                    weekdays
                        .get()
                        .iter()
                        .map(|weekday| {
                            view! {
                                <span class="dow text-center h-6 leading-6 text-sm font-medium text-body">
                                    {Weekday::try_from(*weekday as u8).unwrap().to_string()}
                                </span>
                            }
                        })
                        .collect_view()
                        .into_any()
                }}
            </div>

            <div class="datepicker-grid w-64 grid grid-cols-7">
            {move || {
                dates
                    .get()
                    .into_iter()
                    .map(|date| {
                        let intersects = menu_item_intersects_range(*date, DatePickerMenu::Day, min_date, max_date);
                        let classes = class_list!(
                            (SELECTABLE_ELEM_CLASSES, intersects),
                            (DISABLED_ELEM_CLASSES, !intersects),
                            highlighter.get().map(|it| it(DateMenuOption::Day(date))).unwrap_or_default()
                        );

                        view! {
                            <div class=classes on:click=move |_| {
                                if intersects {
                                    value.set(Some(*date))
                                }
                            }>
                                {date.day()}
                            </div>
                        }
                    })
                    .collect_view()
                    .into_any()
            }}
            </div>
        </div>
    }
}

// Navigation component for the month picking view of the date-picker.
// Looks like: [<] [year] [>]
#[component]
fn MonthPickerMenuNav<MonthByDateFn>(
    month_by_date: MonthByDateFn,
    current_date: Memo<NaiveDate>,
    picker_state: RwSignal<DatePickerState>,
) -> impl IntoView
where
    MonthByDateFn: Fn(&NaiveDate) + Clone + Send + Sync + 'static,
{
    let prev_year = {
        let month_by_date = month_by_date.clone();
        move || {
            month_by_date(
                &current_date
                    .get()
                    .checked_sub_months(Months::new(12))
                    .unwrap_or(NaiveDate::MIN),
            );
        }
    };
    let next_year = move || {
        month_by_date(
            &current_date
                .get()
                .checked_add_months(Months::new(12))
                .unwrap_or(NaiveDate::MAX),
        );
    };

    view! {
        <ControlButton icon=icon::PreviousIcon() on_click=move |_| { prev_year() } {..} tabindex="-1"></ControlButton>
        <Button appearance=ButtonAppearance::Transparent on_click=move |_| {
            picker_state.update(|state| state.set_menu(DatePickerMenu::Year))
        } {..} tabindex="-1">{ move || current_date.get().format("%Y").to_string() }</Button>
        <ControlButton icon=icon::NextIcon() on_click=move |_| { next_year() } {..} tabindex="-1"></ControlButton>
    }
}

#[component]
fn MonthPickerMenu<MonthByDateFn>(
    month_by_date: MonthByDateFn,
    #[prop(into)] highlighter: MaybeProp<ArcOneCallback<DateMenuOption, String>>,
    current_date: Memo<NaiveDate>,
    picker_state: RwSignal<DatePickerState>,
    min_date: MaybeProp<NaiveDate>,
    max_date: MaybeProp<NaiveDate>,
) -> impl IntoView
where
    MonthByDateFn: Fn(&NaiveDate) + Clone + Send + Sync + 'static,
{
    view! {
        <div class="months">
            <div class="datepicker-grid w-64 grid grid-cols-4">
            {move || {
                MONTHS
                    .iter()
                    .map(|month| {
                        let date = current_date.get()
                            .with_month(month.number_from_month())
                            .unwrap_or(NaiveDate::MIN);
                        let intersects = menu_item_intersects_range(date, DatePickerMenu::Month, min_date, max_date);
                        let classes = class_list!(
                            (SELECTABLE_ELEM_CLASSES, intersects),
                            (DISABLED_ELEM_CLASSES, !intersects),
                            highlighter.get()
                                .map(|it| it(DateMenuOption::Month(month.number_from_month())))
                                .unwrap_or_default()
                        );

                        view! {
                            <div
                                class=classes
                                on:click={
                                    let month_by_date = month_by_date.clone();
                                    move |_| {
                                        if intersects {
                                            month_by_date(&date);
                                            picker_state.update(|state| state.set_menu(DatePickerMenu::Day))
                                        }
                                    }
                                }
                            >
                                {&month.name()[0..3]}
                            </div>
                        }
                    })
                    .collect_view()
                    .into_any()
            }}
            </div>
        </div>
    }
}

// Navigation component for the month picking view of the date-picker.
// Looks like: [<] [Decenium] [>]
#[component]
fn YearPickerMenuNav<MonthByDateFn>(
    month_by_date: MonthByDateFn,
    current_date: Memo<NaiveDate>,
    picker_state: RwSignal<DatePickerState>,
) -> impl IntoView
where
    MonthByDateFn: Fn(&NaiveDate) + Clone + Send + Sync + 'static,
{
    let prev_decenia = {
        let month_by_date = month_by_date.clone();
        move || {
            month_by_date(
                &current_date
                    .get()
                    .checked_sub_months(Months::new(12 * 10))
                    .unwrap_or(NaiveDate::MIN),
            );
        }
    };
    let next_decenia = move || {
        month_by_date(
            &current_date
                .get()
                .checked_add_months(Months::new(12 * 10))
                .unwrap_or(NaiveDate::MAX),
        );
    };

    view! {
        <ControlButton icon=icon::PreviousIcon() on_click=move |_| { prev_decenia() } {..} tabindex="-1"></ControlButton>
        <Button appearance=ButtonAppearance::Transparent on_click=move |_| {
            picker_state.update(|state| state.set_menu(DatePickerMenu::Decenia))
        } {..} tabindex="-1">
        { move || {
            let current_year = current_date.get().year();
            let current_decenia = current_year - current_year % 10;
            let decenia_end = current_decenia+9;
            format!("{} - {}", current_decenia, decenia_end)
        }}</Button>
        <ControlButton icon=icon::NextIcon() on_click=move |_| { next_decenia() } {..} tabindex="-1"></ControlButton>
    }
}

#[component]
fn YearPickerMenu<MonthByDateFn>(
    month_by_date: MonthByDateFn,
    #[prop(into)] highlighter: MaybeProp<ArcOneCallback<DateMenuOption, String>>,
    current_date: Memo<NaiveDate>,
    picker_state: RwSignal<DatePickerState>,
    min_date: MaybeProp<NaiveDate>,
    max_date: MaybeProp<NaiveDate>,
) -> impl IntoView
where
    MonthByDateFn: Fn(&NaiveDate) + Clone + Send + Sync + 'static,
{
    let relevant_years = move || {
        let current_year = current_date.get().year();
        let current_decenia = current_year - current_year % 10;
        // Should be 12 years in this range to fill a 4x3 grid
        current_decenia - 1..=current_decenia + 10
    };
    view! {
        <div class="years">
            <div class="datepicker-grid w-64 grid grid-cols-4">
            {move || {
                relevant_years()
                    .map(|year| {
                        let date = current_date.get()
                            .with_year(year)
                            .unwrap_or(NaiveDate::MIN);
                        let intersects = menu_item_intersects_range(date, DatePickerMenu::Year, min_date, max_date);
                        let classes = class_list!(
                            (SELECTABLE_ELEM_CLASSES, intersects),
                            (DISABLED_ELEM_CLASSES, !intersects),
                            highlighter.get()
                                .map(|it| it(DateMenuOption::Year(year)))
                                .unwrap_or_default()
                        );
                        view! {
                            <div
                                class=classes
                                on:click={
                                    let month_by_date = month_by_date.clone();
                                    move |_| {
                                        if intersects {
                                            month_by_date(
                                                &current_date.get()
                                                    .with_year(year)
                                                    .unwrap_or(NaiveDate::MIN)
                                            );
                                            picker_state.update(|state| state.set_menu(DatePickerMenu::Month))
                                        }
                                    }
                                }
                            >
                                {format!("{year}")}
                            </div>
                        }
                    })
                    .collect_view()
                    .into_any()
            }}
            </div>
        </div>
    }
}

// Navigation component for the month picking view of the date-picker.
// Looks like: [<] [Millenium] [>]
#[component]
fn DeceniumPickerMenuNav<MonthByDateFn>(
    month_by_date: MonthByDateFn,
    current_date: Memo<NaiveDate>,
) -> impl IntoView
where
    MonthByDateFn: Fn(&NaiveDate) + Clone + Send + Sync + 'static,
{
    let prev_millenium = {
        let month_by_date = month_by_date.clone();
        move || {
            month_by_date(
                &current_date
                    .get()
                    .checked_sub_months(MILLENIUM_IN_MONTHS)
                    .unwrap_or(NaiveDate::MIN),
            );
        }
    };
    let next_millenium = move || {
        month_by_date(
            &current_date
                .get()
                .checked_add_months(MILLENIUM_IN_MONTHS)
                .unwrap_or(NaiveDate::MAX),
        );
    };

    view! {
        <ControlButton icon=icon::PreviousIcon() on_click=move |_| { prev_millenium() } {..} tabindex="-1"></ControlButton>
        <Button appearance=ButtonAppearance::Transparent>
        { move || {
            let current_year = current_date.get().year();
            let current_millenia = current_year - current_year % 100;
            let millenia_end = current_millenia + 90;
            format!("{} - {}", current_millenia, millenia_end)
        }}</Button>
        <ControlButton icon=icon::NextIcon() on_click=move |_| { next_millenium() } {..} tabindex="-1"></ControlButton>
    }
}

#[component]
fn DeceniumPickerMenu<MonthByDateFn>(
    month_by_date: MonthByDateFn,
    #[prop(into)] highlighter: MaybeProp<ArcOneCallback<DateMenuOption, String>>,
    current_date: Memo<NaiveDate>,
    picker_state: RwSignal<DatePickerState>,
    min_date: MaybeProp<NaiveDate>,
    max_date: MaybeProp<NaiveDate>,
) -> impl IntoView
where
    MonthByDateFn: Fn(&NaiveDate) + Clone + Send + Sync + 'static,
{
    // Range of the years within relevant decenia.
    let relevant_decenia = move || {
        let current_year = current_date.get().year();
        let current_millenium = current_year - current_year % 100;
        // Should be 12 decenia in this range to fill a 4x3 grid
        current_millenium - 20..=current_millenium + 90
    };
    view! {
        <div class="decenia">
            <div class="datepicker-grid w-64 grid grid-cols-4">
            {move || {
                relevant_decenia()
                    .step_by(10)
                    .map(|decenium| {
                        let date = current_date.get()
                            .with_year(decenium)
                            .unwrap_or(NaiveDate::MIN);
                        let intersects = menu_item_intersects_range(date, DatePickerMenu::Decenia, min_date, max_date);
                        let classes = class_list!(
                            (SELECTABLE_ELEM_CLASSES, intersects),
                            (DISABLED_ELEM_CLASSES, !intersects),
                            highlighter.get()
                                .map(|it| it(DateMenuOption::Decenium(decenium)))
                                .unwrap_or_default()
                        );

                        view! {
                            <div
                                class=classes
                                on:click={
                                    let month_by_date = month_by_date.clone();
                                    move |_| {
                                        if intersects {
                                            month_by_date(
                                                &naive_date_with_decenium(&current_date.get(), decenium)
                                            );
                                            picker_state.update(|state| state.set_menu(DatePickerMenu::Year))
                                        }
                                    }
                                }
                            >
                                {format!("{decenium}")}
                            </div>
                        }
                    })
                    .collect_view()
                    .into_any()
            }}
            </div>
        </div>
    }
}

#[derive(Clone, Copy)]
pub enum DateMenuOption {
    Day(CalendarDate),
    Month(u32),
    Year(i32),
    Decenium(i32),
}

impl DateMenuOption {
    pub fn matches_date(&self, date: NaiveDate) -> bool {
        match self {
            DateMenuOption::Day(calendar_date) => date == **calendar_date,
            DateMenuOption::Month(month) => date.month() == *month,
            DateMenuOption::Year(year) => date.year() == *year,
            DateMenuOption::Decenium(decenium) => date.year() - date.year() % 10 == *decenium,
        }
    }

    /// returns that [self] is %Ordering% than/to [date]
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::cmp::Ordering;
    /// # use chrono::NaiveDate;
    /// # use leptodon::date_picker::DateMenuOption;
    ///
    /// let date = NaiveDate::from_ymd_opt(2025, 11, 01).unwrap();
    ///
    /// let month_option = DateMenuOption::Month(10);
    /// let year_option = DateMenuOption::Year(2025);
    /// let decenium_option = DateMenuOption::Decenium(2030);
    ///
    /// assert_eq!(month_option.compare_against(date), Ordering::Less);
    /// assert_eq!(year_option.compare_against(date), Ordering::Equal);
    /// assert_eq!(decenium_option.compare_against(date), Ordering::Greater);
    /// ```
    pub fn compare_against(&self, date: NaiveDate) -> Ordering {
        match self {
            DateMenuOption::Day(calendar_date) => (**calendar_date).cmp(&date),
            DateMenuOption::Month(month) => month.cmp(&date.month()),
            DateMenuOption::Year(year) => year.cmp(&date.year()),
            DateMenuOption::Decenium(decenium) => decenium.cmp(&(date.year() - date.year() % 10)),
        }
    }
}

// Default single-calendar highlighter.
// Highlights the selected day. Days from other months are grayer.
pub fn day_highlighter(
    value: RwSignal<Option<NaiveDate>>,
) -> ArcOneCallback<DateMenuOption, String> {
    ArcOneCallback::new(move |date: DateMenuOption| {
        if let Some(value) = value.get()
            && date.matches_date(value)
        {
            SELECTED_ELEM_CLASSES.to_string() + " rounded-lg"
        } else if let DateMenuOption::Day(date) = date
            && date.is_other_month()
        {
            "text-gray-500 rounded-lg".to_string()
        } else {
            "rounded-lg".to_string()
        }
    })
}

#[generate_docs]
#[component]
pub fn DatePicker(
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(default = "yyyy-mm-dd".into(), into)] placeholder: MaybeProp<String>,
    /// Asserts that min_date <= value
    #[prop(optional, into)]
    min_date: MaybeProp<NaiveDate>,
    /// Asserts that max_date >= value
    #[prop(optional, into)]
    max_date: MaybeProp<NaiveDate>,

    #[prop(into)] value: RwSignal<Option<NaiveDate>>,
    /// A function which maps a DayMenuOption -> String (css class) to style special days on the date-picker.
    #[prop(default = day_highlighter(*value).into(), into)]
    highlighter: MaybeProp<ArcOneCallback<DateMenuOption, String>>,

    #[prop(optional)] required: bool,
    #[prop(optional, into)] label: MaybeProp<String>,
) -> impl IntoView {
    // Extra internal state for hiding and which menu is active.
    let picker_state = RwSignal::new(DatePickerState::default());
    let date_picker_id = id;

    // Input parser
    let parser = ArcOneCallback::new(move |to_parse: String| {
        if to_parse.is_empty() {
            return Ok(None);
        }

        // Parse date
        let date = NaiveDate::from_str(to_parse.as_str())
            .map(Option::Some)
            .map_err(|s| {
                match s.kind() {
                    ParseErrorKind::OutOfRange => "input is out of range".to_string(),
                    ParseErrorKind::Impossible => "no possible date matching input".to_string(),
                    ParseErrorKind::NotEnough => "input is not enough for unique date".to_string(),
                    // This error can be triggered by one too many 0's
                    ParseErrorKind::Invalid => "try to format as: yyyy-mm-dd".to_string(),
                    ParseErrorKind::TooShort => "too little input".to_string(),
                    ParseErrorKind::TooLong => "too much input".to_string(),
                    ParseErrorKind::BadFormat => "try to format as: yyyy-mm-dd".to_string(),
                    _ => "Unknown error, try to format as: yyyy-mm-dd".to_string(),
                }
            })?;

        // Min max checks
        if let Some(date) = date {
            if let Some(min_date) = min_date.get()
                && min_date > date
            {
                return Err(format!("Enter a date >= {min_date}"));
            }
            if let Some(max_date) = max_date.get()
                && max_date < date
            {
                return Err(format!("Enter a date <= {max_date}"));
            }
        };

        Ok(date)
    });
    // Input formatter
    let format = BoxOneCallback::new(|date: Option<NaiveDate>| {
        if let Some(date) = date {
            date.to_string()
        } else {
            String::new()
        }
    });

    // Calendar helper object which backs the calendar view
    let UseCalendarReturn {
        dates,
        weekdays,

        // vv Functions to mutate the above 2 signals.
        previous_month,
        month_by_date,
        next_month,
        ..
    } = use_calendar();

    // Current date fetched from the calendar helper
    let current_date = Memo::new(move |_| {
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
            .unwrap_or(Local::now().date_naive())
    });

    // Handle changes of the passed [value], can change due to external use or due to a date being picked.
    Effect::watch(
        move || value.get(),
        {
            let month_by_date = month_by_date.clone();
            move |new: &Option<NaiveDate>, old, _| {
                if let Some(new) = new
                    && Some(new) != old.flatten()
                {
                    picker_state.update(|state| state.hide());
                    month_by_date(new);
                }
            }
        },
        false,
    );

    // Presents the items the user can pick depending on which menu is active.
    let body_picker = {
        let month_by_date = month_by_date.clone();
        move || {
            match picker_state.get().menu {
                DatePickerMenu::Day => view! {
                    <DayPickerMenu weekdays dates value highlighter max_date min_date />
                }.into_any(),
                DatePickerMenu::Month => view! {
                    <MonthPickerMenu month_by_date=month_by_date.clone() highlighter current_date picker_state
                    max_date min_date
                    />
                }
                .into_any(),
                DatePickerMenu::Year => view! {
                    <YearPickerMenu month_by_date=month_by_date.clone() highlighter current_date picker_state
                    max_date min_date
                    />
                }
                .into_any(),
                DatePickerMenu::Decenia => view! {
                    <DeceniumPickerMenu month_by_date=month_by_date.clone() highlighter current_date picker_state
                    max_date min_date
                    />
                }
                .into_any(),
            }
        }
    };

    // Presents the navigation the user can user depending on which menu is active.
    let nav_picker = move || match picker_state.get().menu {
        DatePickerMenu::Day => view! {
            <DayPickerMenuNav
                previous_month=previous_month.clone()
                next_month=next_month.clone()
                current_date
                picker_state
            />
        }
        .into_any(),
        DatePickerMenu::Month => view! {
            <MonthPickerMenuNav month_by_date=month_by_date.clone()
                current_date
                picker_state />
        }
        .into_any(),
        DatePickerMenu::Year => view! {
            <YearPickerMenuNav month_by_date=month_by_date.clone()
                current_date
                picker_state />
        }
        .into_any(),
        DatePickerMenu::Decenia => view! {
            <DeceniumPickerMenuNav month_by_date=month_by_date.clone()
                current_date />
        }
        .into_any(),
    };

    let target = NodeRef::<Div>::new();

    // Only attached global click handler when the picker is visible.
    let last_click_to_close_listener = RwSignal::new(None);
    Effect::watch(
        move || picker_state.get().visible,
        move |new, old, _| {
            if old == Some(new) {
                return;
            }
            if *new {
                // Just became visible
                let cancel = on_click_outside_with_options(
                    target,
                    move |_event| {
                        debug_log!("clicked outside date_picker, closing.");
                        picker_state.update(|state| state.hide());
                    },
                    OnClickOutsideOptions::default(),
                );
                last_click_to_close_listener.set(Some(cancel));
            } else {
                // Just hid
                if let Some(cancellable) = last_click_to_close_listener.get_untracked() {
                    cancellable();
                }
            }
        },
        true,
    );

    let id = shared_id();

    let id = Signal::derive(move || {
        let provided_id = date_picker_id.get();
        if let Some(provided_id) = provided_id {
            format!("{}-popup", provided_id)
        } else {
            id.clone()
        }
    });

    type OptDate = Option<NaiveDate>;
    view! {
        <div node_ref=target>
            <GenericInput<OptDate, String> id=date_picker_id.get() name class placeholder label parser format value required
                on_focus=ArcOneCallback::new(move |_| {
                    picker_state.update(|state| state.show());
                })
                on:keydown=move |key: KeyboardEvent| {
                    debug_log!("{}", key.code().as_str());
                    if key.code() == "Escape" || key.code() == "Tab" {
                        picker_state.update(|state| state.hide());
                    }
                    if key.code() == "Enter" {
                        picker_state.update(|state| state.toggle_visibility());
                    }
                }
                {..}
                role="combobox" // Makes vimium like plugins pass special keys through
                aria-expanded=move || picker_state.get().visible
                aria-controls=id
            />
            // Picker-Dropdown
            <div id=id class=class_list!(
                ("hidden", move || !picker_state.get().visible),
                "absolute bg-white dark:bg-gray-800 z-50 ml-2 mt-px active block"
            )>
                <div class="inline-block rounded-b-lg border border-oa-gray dark:border-gray-700 p-4">

                    <div class="datepicker-header">
                        <div class="flex justify-between mb-2">
                            { nav_picker }
                        </div>
                    </div>

                    <div class="datepicker-main p-1">
                        <div class="datepicker-view flex">
                            { body_picker }
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_decenium_from_naive_date() {
        assert_eq!(
            decenium_from_naive_date(&NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()),
            2020
        );
        assert_eq!(
            decenium_from_naive_date(&NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
            2020
        );
        assert_eq!(
            decenium_from_naive_date(&NaiveDate::from_ymd_opt(2029, 12, 31).unwrap()),
            2020
        );
        assert_eq!(
            decenium_from_naive_date(&NaiveDate::from_ymd_opt(2030, 1, 1).unwrap()),
            2030
        );
    }

    #[test]
    fn test_naive_date_with_decenium() {
        assert_eq!(
            naive_date_with_decenium(&NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(), 2020),
            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()
        );
        assert_eq!(
            naive_date_with_decenium(&NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(), 2010),
            NaiveDate::from_ymd_opt(2013, 1, 1).unwrap()
        );
        assert_eq!(
            naive_date_with_decenium(&NaiveDate::from_ymd_opt(2015, 6, 15).unwrap(), 2030),
            NaiveDate::from_ymd_opt(2035, 6, 15).unwrap()
        );
    }

    #[test]
    fn test_menu_range() {
        let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        assert_eq!(menu_range(date, DatePickerMenu::Day), (date, date));

        let date = NaiveDate::from_ymd_opt(2023, 5, 15).unwrap();
        assert_eq!(
            menu_range(date, DatePickerMenu::Month),
            (
                NaiveDate::from_ymd_opt(2023, 5, 1).unwrap(),
                NaiveDate::from_ymd_opt(2023, 5, 31).unwrap()
            )
        );

        let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        assert_eq!(
            menu_range(date, DatePickerMenu::Year),
            (
                NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2023, 12, 31).unwrap()
            )
        );

        let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        assert_eq!(
            menu_range(date, DatePickerMenu::Decenia),
            (
                NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2029, 12, 31).unwrap()
            )
        );
    }

    #[test]
    fn test_menu_item_intersects_range() {
        let min_date = NaiveDate::from_ymd_opt(2023, 5, 1).unwrap();
        let max_date = NaiveDate::from_ymd_opt(2023, 5, 31).unwrap();
        let menu = DatePickerMenu::Month;

        // Date inside range
        assert!(menu_item_intersects_range(
            NaiveDate::from_ymd_opt(2023, 5, 15).unwrap(),
            menu,
            min_date.into(),
            max_date.into()
        ));

        // At start of range
        assert!(menu_item_intersects_range(
            min_date,
            menu,
            min_date.into(),
            max_date.into()
        ));

        // At end of range
        assert!(menu_item_intersects_range(
            max_date,
            menu,
            min_date.into(),
            max_date.into()
        ));

        // Before range
        assert!(!menu_item_intersects_range(
            NaiveDate::from_ymd_opt(2023, 4, 30).unwrap(),
            menu,
            min_date.into(),
            max_date.into()
        ));

        // After range
        assert!(!menu_item_intersects_range(
            NaiveDate::from_ymd_opt(2023, 6, 1).unwrap(),
            menu,
            min_date.into(),
            max_date.into()
        ));
    }
}
