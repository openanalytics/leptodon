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
use chrono::Datelike;
use chrono::NaiveDate;
use chrono::Weekday;
use leptodon::date_picker::DateMenuOption;
use leptodon::date_picker::DatePicker;
use leptodon::date_picker::day_highlighter;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptodon::util::callback::ArcOneCallback;
use leptodon_proc_macros::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::RwSignal;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[generate_codeblock(DatePickerExample)]
#[component]
pub fn DatePickerDemo() -> impl IntoView {
    let value = RwSignal::new(None);
    view! {
        <p>
            {move || format!("{:?}", value.get())}
        </p>
        <DatePicker
            class="my-3"
            min_date=NaiveDate::from_ymd_opt(1900, 1, 1).expect("valid date")
            placeholder="Published at: yyyy-mm-dd"
            value
            label="When was this work published?"
        />
    }
}

#[generate_codeblock(DatePickerHighlighterExample)]
#[component]
pub fn DatePickerHighlighterDemo() -> impl IntoView {
    let value = RwSignal::new(None);
    let weekend_red = ArcOneCallback::new(move |day: DateMenuOption| {
        let base = day_highlighter(value)(day);
        let weekend_red = if let DateMenuOption::Day(calendar_date) = day
            && [Weekday::Sat, Weekday::Sun].contains(&calendar_date.weekday())
            // Don't make the day red when they are selected.
            && !value
                .get()
                .map(|selected_date| calendar_date.is_selected(&selected_date))
                .unwrap_or_default()
        {
            "bg-red-100 dark:bg-red-900"
        } else {
            ""
        };
        format!("{base} {weekend_red}")
    });
    view! {
        <p>
            {move || format!("{:?}", value.get())}
        </p>
        <DatePicker
            class="my-3"
            min_date=NaiveDate::from_ymd_opt(1900, 1, 1).expect("valid date")
            max_date=NaiveDate::from_ymd_opt(2026, 2, 13).expect("valid date")
            placeholder="Published at: yyyy-mm-dd"
            required=true
            highlighter=weekend_red
            value
            label="When was this work published?"
        />
    }
}

#[component]
pub fn DatePickerDemoPage() -> impl IntoView {
    view! {
        <Title text="DatePicker"/>

        <FixedCenterColumn>
            <Heading4 anchor="datepicker">"DatePicker"</Heading4>
            <DatePickerExample />

            <Heading4 anchor="datepicker-highlighter">"DatePicker Highlighter"</Heading4>
            <DatePickerHighlighterExample />

            <leptodon::date_picker::DatePickerDocs />
        </FixedCenterColumn>
    }
}
