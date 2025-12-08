use std::cmp::Ordering;

use crate::date_picker::DateMenuOption;
use crate::date_picker::DatePicker;
use crate::util::callback::ArcOneCallback;
use chrono::NaiveDate;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos_use::CalendarDate;

use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::{
    IntoView, component,
    prelude::{MaybeProp, Signal},
    view,
};

#[component]
pub fn DateRangePicker(
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(default = "yyyy-mm-dd".into(), into)] placeholder: MaybeProp<String>,

    /// Picked start date of the date-range
    #[prop(default = RwSignal::new(None), into)]
    start_date: RwSignal<Option<NaiveDate>>,

    /// Picked end date of the date-range
    #[prop(default = RwSignal::new(None), into)]
    end_date: RwSignal<Option<NaiveDate>>,

    /// Maps dates to tailwind classes to style the dates.
    /// Can be used to highlight specific days, e.g. festive days and weekends.
    #[prop(optional, into)]
    highlighter: MaybeProp<ArcOneCallback<DateMenuOption, String>>,
    #[prop(optional, into)] label: MaybeProp<String>,
) -> impl IntoView {
    let start_date = RwSignal::new(None);
    let end_date = RwSignal::new(None);

    let range_highlighter = move |date: DateMenuOption| {
        if let Some(start) = start_date.get()
            && let Some(end) = end_date.get()
        {
            if date.matches_date(start) {
                return "rounded-l-lg bg-oa-blue text-white".to_string();
            }   
            if date.matches_date(end) {
                return "rounded-r-lg bg-oa-blue text-white".to_string();
            }
            
            // !These matches_date are fuzzy and can work on any [DateMenuOption] supported granularity.
            // They need to happy before the range check.
            if date.compare_against(end) == Ordering::Less
               && date.compare_against(start) == Ordering::Greater
           {
               return "bg-oa-gray-darker".to_string();
           } else {
               return "rounded-lg".to_string();
           }
        } else {
            if let Some(start) = start_date.get() {
                if date.matches_date(start) {
                    return "rounded-lg bg-oa-blue text-white".to_string();
                }
            }
            
            if let Some(end) = end_date.get() {
                if date.matches_date(end) {
                    return "rounded-lg bg-oa-blue text-white".to_string();
                }
            }
        }
       
        return "rounded-lg".to_string();
    };

    let combined_highlighter = ArcOneCallback::new(move |date: DateMenuOption| {
        let mut all_classes = vec![range_highlighter(date)];
        if let Some(provided_highlighter) = highlighter.get() {
            all_classes.push(provided_highlighter(date));
        }

        // Could be extended to other menus ?
        if let DateMenuOption::Day(date) = date
            && date.is_other_month()
        {
            all_classes.push("text-gray-500".to_string());
        }
        // if date.is_today() {
        //     all_classes += "oa-text-blue";
        // } else if date.is_other_month() {
        //     all_classes += "text-gray-500";
        // }

        // console_log(format!("highlighting as {all_classes}").as_str());
        all_classes.join(" ")
    });

    view! {
        <div class="inline-flex">
            <DatePicker placeholder value=start_date highlighter=combined_highlighter.clone() />
            "Until"
            <DatePicker placeholder value=end_date highlighter=combined_highlighter/>
        </div>
    }
}
