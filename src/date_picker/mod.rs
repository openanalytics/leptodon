use chrono::Datelike;
use chrono::Local;
use chrono::Month;
use chrono::Weekday;
use chrono::format::ParseErrorKind;
use leptos::prelude::CollectView;
use leptos::prelude::IntoAny;
use leptos::prelude::Memo;
use leptos_use::UseCalendarReturn;
use leptos_use::use_calendar;
use num_traits::FromPrimitive;
use std::str::FromStr;

use crate::button::Button;
use crate::button::ButtonAppearance;
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

#[component]
pub fn DatePicker(
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(default = "yyyy-mm-dd".into(), into)] placeholder: MaybeProp<String>,

    #[prop(into)] value: RwSignal<NaiveDate>,
    #[prop(optional, into)] label: MaybeProp<String>,
) -> impl IntoView {
    let input_ref = NodeRef::<html::element::Input>::new();

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
    
    let current_month_year = Memo::new(move |_| {
        let current = dates
            .get()
            .into_iter()
            .find_map(|date| {
                if !date.is_other_month() && date.is_first_day_of_month() {
                    Some(*date)
                } else {
                    None
                }
            })
            .unwrap_or(Local::now().date_naive());
        format!(
            "{} {}",
            Month::from_u32(current.month()).unwrap().name(),
            current.year(),
        )
    });

    let format = Some(BoxOneCallback::new(|date: NaiveDate| date.to_string()));

    view! {
        <GenericInput<NaiveDate, String> name class placeholder label parser format value />
        <div class="relative top-0 left-0 z-50 pt-2 active block">
        <div class="inline-block rounded-b-lg border border-oa-gray p-4">

        <div class="datepicker-header">
            <div class="flex justify-between mb-2">
                <Button icon=icon::PreviousIcon() on_click=move |_| { previous_month() } appearance=ButtonAppearance::Transparent></Button>
                <Button appearance=ButtonAppearance::Transparent>{ move || current_month_year.get() }</Button>
                <Button icon=icon::NextIcon() on_click=move |_| { next_month() } appearance=ButtonAppearance::Transparent></Button>
            </div>
        </div>

        <div class="datepicker-main p-1">

        <div class="datepicker-view flex">
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
                                    class="datepicker-cell hover:bg-neutral-tertiary-medium block flex-1 leading-9 border-0 rounded-base cursor-pointer text-center text-body font-medium text-sm day"
                                    class:text-red-500=date.is_today()
                                    class:text-gray-500=date.is_other_month()
                                    class:border-red-500=move || is_selected()
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
