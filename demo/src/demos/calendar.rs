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
use attr_docgen::generate_codeblock;
use chrono::Datelike;
use chrono::Local;
use chrono::NaiveDate;
use chrono::NaiveTime;
use chrono::Weekday;
use leptodon::calendar::Calendar;
use leptodon::calendar::CalendarEvent;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::IntoAny;
use leptos::prelude::RwSignal;
use leptos::prelude::signal;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[generate_codeblock(CalendarExample)]
#[component]
pub fn CalendarDemo() -> impl IntoView {
    let local_date = Local::now();
    let (_presented_month_reader, presented_month_writer) = signal(local_date.date_naive());
    let children = move |date: NaiveDate| {
        if date.day().is_multiple_of(5) {
            view! {
                <CalendarEvent
                    start_time=NaiveTime::from_hms_opt(5, 0, 0).unwrap()
                    end_time=NaiveTime::from_hms_opt(17, 0, 0).unwrap()
                    summary="A multiple of 5!"
                    popup_desc="This day is a multiple of 5." />
            }
            .into_any()
        } else {
            ().into_any()
        }
    };

    view! {
        <p>"You can use "<code>presented_month_reader</code>" to send new web requests and on-response update the calendar-events."</p>
        <Calendar
            children
            presented_month_writer
            show_days=RwSignal::new(Box::new([Weekday::Mon, Weekday::Tue, Weekday::Thu, Weekday::Fri].as_ref()))
            />
    }
}

#[component]
pub fn CalendarDemoPage() -> impl IntoView {
    view! {
        <Title text="Calendar Component"/>

        <FixedCenterColumn>
            <Heading4 anchor="calendar">"Calendar"</Heading4>
            <CalendarExample />

            <leptodon::calendar::CalendarDocs />
            <leptodon::calendar::CalendarEventDocs />
        </FixedCenterColumn>
    }
}
