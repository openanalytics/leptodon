use chrono::{Local, NaiveDate, TimeZone};
use leptodon::{
    button::Button,
    calendar::{YearCalendar, YearCalendarLayout},
};
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
use leptos::{
    IntoView, component,
    prelude::{AddAnyAttr, RwSignal, Set},
    view,
};
use leptos_meta::Title;

#[component]
pub fn TestYearCalendar() -> impl IntoView {
    let test_date = Local.from_utc_datetime(
        &NaiveDate::from_ymd_opt(2026, 2, 10)
            .expect("real date")
            .and_hms_opt(1, 1, 1)
            .expect("real time"),
    );
    let layout = RwSignal::new(YearCalendarLayout::Year);
    view! {
        <Title text="Test Year Calendar"/>
        <YearCalendar local_date_time=test_date current_year=RwSignal::new(2026) layout/>
        <Button on_click=move |_| {
            layout.set(YearCalendarLayout::Year);
        } attr:data-testid="set_layout_year">"Set layout -> YearCalendar"</Button>
        <Button on_click=move |_| {
            layout.set(YearCalendarLayout::TwelveMonths);
        } attr:data-testid="set_layout_twelve_months">"Set layout -> TwelveMonths"</Button>
    }
}
