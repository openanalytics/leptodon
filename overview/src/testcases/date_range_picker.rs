use chrono::NaiveDate;
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
use leptodon::button::Button;
use leptodon::date_picker::DateRangePicker;
use leptodon::paragraph::Paragraph;
use leptos::prelude::Get;
use leptos::prelude::Set;
use leptos::{IntoView, component, prelude::RwSignal, view};
use leptos_meta::Title;

#[component]
pub fn TestDateRangePicker() -> impl IntoView {
    let start_date = RwSignal::new(NaiveDate::from_ymd_opt(2025, 10, 10));
    let end_date = RwSignal::new(NaiveDate::from_ymd_opt(2025, 11, 15));
    view! {
        <Title text="Test DateRangePicker"/>
        <Paragraph>
            {move || format!("{:?} - {:?}", start_date.get(), end_date.get())}
        </Paragraph>
        <DateRangePicker
            min_date=NaiveDate::from_ymd_opt(2020, 10, 10).unwrap()
            max_date=NaiveDate::from_ymd_opt(2030, 10, 10).unwrap()
            start_date
            end_date
        />
        <Button id="set-none" on_click=move |_e| {
            start_date.set(None);
            end_date.set(None);
        }>"Clear"</Button>
        <Button id="set-start-2010" on_click=move |_e| {
            start_date.set(NaiveDate::from_ymd_opt(2010, 10, 10));
        }>"Set 2010 as start_date"</Button>
        <Button id="set-start-2025" on_click=move |_e| {
            start_date.set(NaiveDate::from_ymd_opt(2025, 10, 10));
        }>"Set 2025 as start_date"</Button>
        <Button id="set-end-2026" on_click=move |_e| {
            end_date.set(NaiveDate::from_ymd_opt(2026, 5, 10));
        }>"Set 2026 as end_date"</Button>
        <Button id="set-end-2035" on_click=move |_e| {
            end_date.set(NaiveDate::from_ymd_opt(2035, 5, 10));
        }>"Set 2035 as end_date"</Button>
    }
}
