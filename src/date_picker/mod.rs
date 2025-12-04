use chrono::Datelike;
use chrono::Local;
use chrono::Month;
use chrono::Months;
use chrono::Weekday;
use chrono::format::ParseErrorKind;
use leptos::prelude::AnyView;
use leptos::prelude::CollectView;
use leptos::prelude::IntoAny;
use leptos::prelude::Memo;
use leptos::prelude::signal;
use leptos_use::CalendarDate;
use leptos_use::UseCalendarReturn;
use leptos_use::use_calendar;
use num_traits::FromPrimitive;
use std::str::FromStr;

use crate::button::Button;
use crate::button::ButtonAppearance;
use crate::button::ControllButton;
use crate::class_list;
use crate::icon;
use crate::input::GenericInput;
use crate::input::Input;
use crate::input::InputType;
use crate::util::callback::ArcOneCallback;
use crate::util::callback::BoxOneCallback;
use chrono::NaiveDate;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::GetUntracked;
use leptos::prelude::NodeRef;
use leptos::prelude::NodeRefAttribute;
use leptos::prelude::OnAttribute;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::tachys::html;
use leptos::{
    IntoView, component,
    prelude::{MaybeProp, Signal},
    view,
};

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

#[derive(Default, Clone)]
pub(crate) struct DatePickerState {
    pub visible: bool,
    pub menu_state: DatePickerMenu,
}

impl DatePickerState {
    fn shown(&self) -> Self {
        DatePickerState {
            visible: true,
            menu_state: self.menu_state,
        }
    }
    fn hidden(&self) -> Self {
        DatePickerState {
            visible: false,
            menu_state: self.menu_state,
        }
    }
    fn with_menu(&self, menu: DatePickerMenu) -> Self {
        DatePickerState {
            visible: self.visible,
            menu_state: menu,
        }
    }
}

#[derive(Default, Copy, Clone)]
pub(crate) enum DatePickerMenu {
    DayPicker,
    #[default]
    MonthPicker,
    YearPicker,
    MilleniaPicker,
}

#[component]
fn DayPickerMenuNav<PreviousMonthFn, NextMonthFn>(
    previous_month: PreviousMonthFn,
    next_month: NextMonthFn,
    current_month_year: Memo<String>,
    state: RwSignal<DatePickerState>
) -> impl IntoView
where
    PreviousMonthFn: Fn() + Clone + Send + Sync + 'static,
    NextMonthFn: Fn() + Clone + Send + Sync + 'static,
{
    view! {
        <ControllButton icon=icon::PreviousIcon() on_click=move |_| { previous_month() }></ControllButton>
        <Button appearance=ButtonAppearance::Transparent on_click=move |_| {
            state.set(state.get().with_menu(DatePickerMenu::MonthPicker))
        }>{ move || current_month_year.get() }</Button>
        <ControllButton icon=icon::NextIcon() on_click=move |_| { next_month() }></ControllButton>
    }
}

#[component]
fn MonthPickerMenuNav<MonthByDateFn>(
    month_by_date: MonthByDateFn,
    current_date: Memo<NaiveDate>,
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
        <ControllButton icon=icon::PreviousIcon() on_click=move |_| { prev_year() }></ControllButton>
        <Button appearance=ButtonAppearance::Transparent>{ move || current_date.get().format("%Y").to_string() }</Button>
        <ControllButton icon=icon::NextIcon() on_click=move |_| { next_year() }></ControllButton>
    }
}

#[component]
fn DayPickerMenu(
    weekdays: Signal<Vec<usize>>,
    dates: Signal<Vec<CalendarDate>>,
    value: RwSignal<NaiveDate>,
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
                        let is_selected = move || { *date == value.get() };
                        view! {
                            <div
                                class=class_list!(
                                    "datepicker-cell hover:bg-oa-gray block flex-1 leading-9 border-0 rounded-lg cursor-pointer text-center text-body font-medium text-sm",
                                    ("hover:bg-oa-blue bg-oa-blue text-white", move || is_selected()),
                                    ("text-gray-500", date.is_other_month()),
                                    ("text-oa-blue", move || date.is_today())
                                )

                                class:border-transparent=move || !is_selected()
                                on:click=move |_| value.set(*date)
                            >

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

#[component]
fn MonthPickerMenu<MonthByDateFn>(
    month_by_date: MonthByDateFn,
    current_date: Memo<NaiveDate>,
    state: RwSignal<DatePickerState>
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
                        view! {
                            <div
                                class=class_list!(
                                    "datepicker-cell hover:bg-oa-gray block flex-1 leading-9 border-0 rounded-lg cursor-pointer text-center text-body font-medium text-sm"
                                )
                                on:click={
                                    let month_by_date = month_by_date.clone();
                                    move |_| {
                                        month_by_date(
                                            &current_date.get()
                                                .with_month(month.number_from_month())
                                                .unwrap_or(NaiveDate::MIN)
                                        );
                                        state.set(state.get().with_menu(DatePickerMenu::DayPicker))
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

#[component]
pub fn DatePicker(
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(default = "yyyy-mm-dd".into(), into)] placeholder: MaybeProp<String>,

    #[prop(into)] value: RwSignal<NaiveDate>,
    #[prop(optional, into)] label: MaybeProp<String>,
) -> impl IntoView {
    // let input_ref = NodeRef::<html::element::Input>::new();

    let parser = Some(ArcOneCallback::new(|to_parse: String| {
        NaiveDate::from_str(to_parse.as_str()).map_err(|s| {
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
        })
    }));

    let UseCalendarReturn {
        dates,
        weekdays,

        // vv Functions to mutate the above 2 signals.
        previous_month,
        today,
        month_by_date,
        next_month,
    } = use_calendar();

    let date_picker_state = RwSignal::new(DatePickerState::default());

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

    let current_month_year = Memo::new(move |_| {
        let current = current_date.get();
        format!(
            "{} {}",
            Month::from_u32(current.month()).unwrap().name(),
            current.year(),
        )
    });

    let format = Some(BoxOneCallback::new(|date: NaiveDate| date.to_string()));

    let body_picker = {
        let month_by_date = month_by_date.clone();
        move || match date_picker_state.get().menu_state {
            DatePickerMenu::DayPicker => view! { <DayPickerMenu weekdays dates value/> }.into_any(),
            DatePickerMenu::MonthPicker => view! {
                <MonthPickerMenu
                    month_by_date=month_by_date.clone()
                    current_date
                    state=date_picker_state
                />
            }
            .into_any(),
            DatePickerMenu::YearPicker => todo!(),
            DatePickerMenu::MilleniaPicker => todo!(),
        }
    };
    let nav_picker = move || match date_picker_state.get().menu_state {
        DatePickerMenu::DayPicker => view! {
            <DayPickerMenuNav
                previous_month=previous_month.clone()
                next_month=next_month.clone()
                current_month_year
                state=date_picker_state
            />
        }
        .into_any(),
        DatePickerMenu::MonthPicker => view! {
            <MonthPickerMenuNav month_by_date=month_by_date.clone()
            current_date/>
        }
        .into_any(),
        DatePickerMenu::YearPicker => todo!(),
        DatePickerMenu::MilleniaPicker => todo!(),
    };
    view! {
        <GenericInput<NaiveDate, String> name class placeholder label parser format value />
        <div class="relative top-0 left-0 z-50 pt-2 active block">
            <div class="inline-block rounded-b-lg border border-oa-gray p-4">

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

                <div class="datepicker-footer">
                    <div class="datepicker-controls flex space-x-2 rtl:space-x-reverse mt-2">
                        <button type="button" class="button today-btn text-white bg-brand hover:bg-brand-strong focus:ring-4 focus:ring-brand-medium font-medium rounded-base text-sm px-5 py-2 text-center w-1/2 hidden">Today</button>
                        <button type="button" class="button clear-btn text-body bg-neutral-secondary-medium border border-default-medium hover:bg-neutral-tertiary-medium focus:ring-4 focus:ring-neutral-tertiary font-medium rounded-base text-sm px-5 py-2 text-center w-1/2 hidden">Clear</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
