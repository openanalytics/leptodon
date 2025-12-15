use chrono::Datelike;
use chrono::Month;
use ical::parser::ical::component::IcalCalendar;
use icalendar::Calendar;
use icalendar::parser::Calendar;
use leptos::attr::title;
use leptos::logging::log;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::ServerFnError;
use leptos::server;
use std::fs;
use std::fs::File;
use std::io::BufReader;

use leptos::{IntoView, component, view};
use leptos_components::calendar::Calendar;

#[server]
pub async fn  read_calendar<'a>(month: u32, year: i32) -> Result<Vec<Calendar<'static>>, ServerFnError> {
    return ics_events(month, year).ok_or(ServerFnError::ServerError(format!("Parse error")))
}

fn ics_events(month: u32, year: i32) -> Option<Vec<Calendar<'static>>> {
    let entries = fs::read_dir("assets");

    let Ok(entries) = entries else {
        eprintln!("Failed to read assets");
        return None;
    };

    let mut events_buf: Vec<Calendar<'static>> = vec![];
    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Failed to read entry: {}", e);
                continue;
            }
        };
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("ics") {
            let Ok(file) = std::fs::read_to_string(path) else {
                return None;
            };
            let parsed = file.parse::<Calendar>();
      
            events_buf.splice(0..0, parsed
                .filter_map(|s| s.ok())
                .map(|cal|
                    cal.events
                        .into_iter()
                        .filter_map(|event|
                            ical_property::Event::try_from(&event)
                                .inspect_err(|err| log!("Could not parse calendar event {:?}: {err}. Most likely malformed.", &event))
                                .ok()
                    )
                )
                .flatten()
                .filter(|event| if let Some(start) = &event.start {
                    let naive_event_date = start.as_naive_date();
                    naive_event_date.month() == month && naive_event_date.year() == year
                }else {
                    false
                })
                .collect::<Vec<ical_property::Event>>().into_iter());
        }
    }
    return None;
}

#[component]
pub fn PopulatedCalendar() -> impl IntoView {
    let children = move |date| {
        view! {
            <div class="grow self-stretch">Slot1</div>
            <div class="grow self-stretch">Slot2</div>
        }
    };
    view! {
        <Calendar children />
    }
}
