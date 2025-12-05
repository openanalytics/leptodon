use chrono::Datelike;
use chrono::Local;
use chrono::Month;
use chrono::Months;
use chrono::Weekday;
use chrono::format::ParseErrorKind;
use leptos::html::Div;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::AddAnyAttr;
use leptos::prelude::CollectView;
use leptos::prelude::Effect;
use leptos::prelude::IntoAny;
use leptos::prelude::Memo;
use leptos::prelude::NodeRef;
use leptos::prelude::NodeRefAttribute;
use leptos::prelude::Update;
use leptos_use::CalendarDate;
use leptos_use::UseCalendarReturn;
use leptos_use::on_click_outside;
use leptos_use::use_calendar;
use num_traits::FromPrimitive;
use web_sys::KeyboardEvent;
use std::str::FromStr;
use web_sys::FocusEvent;

use crate::button::Button;
use crate::button::ButtonAppearance;
use crate::button::ControllButton;
use crate::class_list;
use crate::icon;
use crate::input::GenericInput;
use crate::util::callback::ArcOneCallback;
use crate::util::callback::BoxOneCallback;
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

const MILLENIUM_IN_MONTHS: Months = Months::new(12 * 100);
const DECENIA_IN_MONTHS: Months = Months::new(12 * 10);
const YEAR_IN_MONTHS: Months = Months::new(12);

/// Elements refer to the date-picker elements like individual days, months, years.
const SELECTED_ELEM_CLASSES: &str = "!hover:bg-oa-blue bg-oa-blue text-white";
const ELEM_CLASSES: &str = "datepicker-cell hover:bg-oa-gray block flex-1 leading-9 border-0 rounded-lg cursor-pointer text-center text-body font-medium text-sme";

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
    DayPicker,
    MonthPicker,
    YearPicker,
    DeceniaPicker,
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
        <ControllButton icon=icon::PreviousIcon() on_click=move |_| { previous_month() }></ControllButton>
        <Button appearance=ButtonAppearance::Transparent on_click=move |_| {
            picker_state.update(|state| state.set_menu(DatePickerMenu::MonthPicker))
        }>{ move || current_month_year.get() }</Button>
        <ControllButton icon=icon::NextIcon() on_click=move |_| { next_month() }></ControllButton>
    }
}

// Day picking section of the date-picker.
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
                                    ELEM_CLASSES,
                                    (SELECTED_ELEM_CLASSES, move || is_selected()),
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
        <ControllButton icon=icon::PreviousIcon() on_click=move |_| { prev_year() }></ControllButton>
        <Button appearance=ButtonAppearance::Transparent on_click=move |_| {
            picker_state.update(|state| state.set_menu(DatePickerMenu::YearPicker))
        }>{ move || current_date.get().format("%Y").to_string() }</Button>
        <ControllButton icon=icon::NextIcon() on_click=move |_| { next_year() }></ControllButton>
    }
}

#[component]
fn MonthPickerMenu<MonthByDateFn>(
    month_by_date: MonthByDateFn,
    current_date: Memo<NaiveDate>,
    picker_state: RwSignal<DatePickerState>,
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
                                    (SELECTED_ELEM_CLASSES, current_date.get().month() == month.number_from_month()),
                                    ELEM_CLASSES
                                )
                                on:click={
                                    let month_by_date = month_by_date.clone();
                                    move |_| {
                                        month_by_date(
                                            &current_date.get()
                                                .with_month(month.number_from_month())
                                                .unwrap_or(NaiveDate::MIN)
                                        );
                                        picker_state.update(|state| state.set_menu(DatePickerMenu::DayPicker))
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
        <ControllButton icon=icon::PreviousIcon() on_click=move |_| { prev_decenia() }></ControllButton>
        <Button appearance=ButtonAppearance::Transparent on_click=move |_| {
            picker_state.update(|state| state.set_menu(DatePickerMenu::DeceniaPicker))
        }>
        { move || {
            let current_year = current_date.get().year();
            let current_decenia = current_year - current_year % 10;
            let decenia_end = current_decenia+9;
            format!("{} - {}", current_decenia, decenia_end)
        }}</Button>
        <ControllButton icon=icon::NextIcon() on_click=move |_| { next_decenia() }></ControllButton>
    }
}

#[component]
fn YearPickerMenu<MonthByDateFn>(
    month_by_date: MonthByDateFn,
    current_date: Memo<NaiveDate>,
    picker_state: RwSignal<DatePickerState>,
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
                        view! {
                            <div
                                class=class_list!(
                                    (SELECTED_ELEM_CLASSES, current_date.get().year() == year),
                                    ELEM_CLASSES
                                )
                                on:click={
                                    let month_by_date = month_by_date.clone();
                                    move |_| {
                                        month_by_date(
                                            &current_date.get()
                                                .with_year(year)
                                                .unwrap_or(NaiveDate::MIN)
                                        );
                                        picker_state.update(|state| state.set_menu(DatePickerMenu::MonthPicker))
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
    current_date: Memo<NaiveDate>
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
        <ControllButton icon=icon::PreviousIcon() on_click=move |_| { prev_millenium() }></ControllButton>
        <Button appearance=ButtonAppearance::Transparent>
        { move || {
            let current_year = current_date.get().year();
            let current_millenia = current_year - current_year % 100;
            let millenia_end = current_millenia + 90;
            format!("{} - {}", current_millenia, millenia_end)
        }}</Button>
        <ControllButton icon=icon::NextIcon() on_click=move |_| { next_millenium() }></ControllButton>
    }
}

#[component]
fn DeceniumPickerMenu<MonthByDateFn>(
    month_by_date: MonthByDateFn,
    current_date: Memo<NaiveDate>,
    picker_state: RwSignal<DatePickerState>,
) -> impl IntoView
where
    MonthByDateFn: Fn(&NaiveDate) + Clone + Send + Sync + 'static,
{
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
                        view! {
                            <div
                                class=class_list!(
                                    (SELECTED_ELEM_CLASSES, decenium_from_naive_date(&current_date.get()) == decenium),
                                    ELEM_CLASSES
                                )
                                on:click={
                                    let month_by_date = month_by_date.clone();
                                    move |_| {
                                        month_by_date(
                                            &naive_date_with_decenium(&current_date.get(), decenium)
                                        );
                                        picker_state.update(|state| state.set_menu(DatePickerMenu::YearPicker))
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


#[component]
pub fn DatePicker(
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(default = "yyyy-mm-dd".into(), into)] placeholder: MaybeProp<String>,

    #[prop(into)] value: RwSignal<NaiveDate>,
    #[prop(optional, into)] label: MaybeProp<String>,
) -> impl IntoView {
    // Extra internal state for hiding and which menu is active.
    let picker_state = RwSignal::new(DatePickerState::default());

    // Input parser
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
    // Input formatter
    let format = Some(BoxOneCallback::new(|date: NaiveDate| date.to_string()));

    // Calendar helper object which backs the calendar view
    let UseCalendarReturn {
        dates,
        weekdays,

        // vv Functions to mutate the above 2 signals.
        previous_month,
        today,
        month_by_date,
        next_month,
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
            move |new, old, _| {
                if Some(new) != old {
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
        move || match picker_state.get().menu {
            DatePickerMenu::DayPicker => view! { <DayPickerMenu weekdays dates value/> }.into_any(),
            DatePickerMenu::MonthPicker => view! {
                <MonthPickerMenu
                    month_by_date=month_by_date.clone()
                    current_date
                    picker_state
                />
            }
            .into_any(),
            DatePickerMenu::YearPicker => view! {
                <YearPickerMenu
                    month_by_date=month_by_date.clone()
                    current_date
                    picker_state
                />
            }
            .into_any(),
            DatePickerMenu::DeceniaPicker => view! {
                <DeceniumPickerMenu
                    month_by_date=month_by_date.clone()
                    current_date
                    picker_state
                />
            }
            .into_any(),
        }
    };

    // Presents the navigation the user can user depending on which menu is active.
    let nav_picker = move || match picker_state.get().menu {
        DatePickerMenu::DayPicker => view! {
            <DayPickerMenuNav
                previous_month=previous_month.clone()
                next_month=next_month.clone()
                current_date
                picker_state
            />
        }
        .into_any(),
        DatePickerMenu::MonthPicker => view! {
            <MonthPickerMenuNav month_by_date=month_by_date.clone()
                current_date
                picker_state />
        }
        .into_any(),
        DatePickerMenu::YearPicker => view! {
            <YearPickerMenuNav month_by_date=month_by_date.clone()
                current_date
                picker_state />
        }
        .into_any(),
        DatePickerMenu::DeceniaPicker => view! {
            <DeceniumPickerMenuNav month_by_date=month_by_date.clone()
                current_date />
        }
        .into_any(),
    };
    
    let target = NodeRef::<Div>::new();
    
    // Magic
    on_click_outside(target, move |event| {
        picker_state.update(|state| state.hide());
    });

    view! {
        <div node_ref=target>
            <GenericInput<NaiveDate, String> name class placeholder label parser format value 
                on:focus=move |_| {
                    picker_state.update(|state| state.show());
                }
                on:keydown=move |key: KeyboardEvent| {
                    console_log(key.code().as_str());
                    if key.code() == "Escape" {
                        picker_state.update(|state| state.hide());
                    }
                    if key.code() == "Enter" {
                        picker_state.update(|state| state.toggle_visibility());
                    }
                }
                {..}
                role="combobox" // Makes vimium like plugins pass specia keys through
            />
            // Picker-Dropdown
            <div class=class_list!(
                ("hidden", move || !picker_state.get().visible),
                "absolute bg-white z-50 ml-2 mt-px active block"
            )>
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
        </div>
    }
}
