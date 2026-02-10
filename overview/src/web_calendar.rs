use chrono::Datelike;
use chrono::Local;
use leptos::logging::error;
use leptos::prelude::ClassAttribute;
use leptos::prelude::CollectView;
use leptos::prelude::Effect;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::IntoAny;
use leptos::prelude::ServerFnError;
use leptos::prelude::signal;
use leptos::server;
use leptos::server::LocalResource;
use leptos_components::class_list;
use leptos_components::popover::Popover;
use leptos_components::popover::PopoverAnchor;
use leptos_components::popover::PopoverTrigger;
use leptos_components::util::option_comp::OptionComp;

use leptos::{IntoView, component, view};
use leptos_components::calendar::Calendar;

#[server]
pub async fn read_calendar<'a>(
    month: u32,
    year: i32,
) -> Result<Vec<crate::ical_property::Event>, ServerFnError> {
    return ics_events(month, year).ok_or(ServerFnError::ServerError(format!("Parse error")));
}

#[cfg(feature = "ssr")]
fn ics_events(month: u32, year: i32) -> Option<Vec<crate::ical_property::Event>> {
    use std::fs;
    use std::fs::File;
    use std::io::BufReader;

    let entries = fs::read_dir("assets");

    let Ok(entries) = entries else {
        eprintln!("Failed to read assets");
        return None;
    };

    let mut events_buf: Vec<crate::ical_property::Event> = vec![];
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
            let buf = BufReader::new(File::open(path).unwrap());

            let reader = ical::IcalParser::new(buf);

            events_buf.splice(0..0, reader
                .filter_map(|s| s.ok())
                .map(|cal|
                    cal.events
                        .into_iter()
                        .filter_map(|event|
                            crate::ical_property::Event::try_from(&event)
                                .inspect_err(|err| error!("Could not parse calendar event {:?}: {err}. Most likely malformed.", &event))
                                .ok()
                    )
                )
                .flatten()
                .filter(|event| if let Some(start) = &event.start {
                    let naive_event_date = start.as_naive_date();
                    naive_event_date.month() == month && naive_event_date.year() == year
                } else {
                    false
                })
                .collect::<Vec<crate::ical_property::Event>>().into_iter());
        }
    }
    return Some(events_buf);
}

#[component]
pub fn PopulatedCalendar() -> impl IntoView {
    let local_date = Local::now();
    let (presented_month_reader, presented_month_writer) = signal(local_date.date_naive());
    let async_data = LocalResource::new(move || {
        let presented_month = presented_month_reader.get();
        read_calendar(presented_month.month(), presented_month.year())
    });
    Effect::new(move || {
        if let Some(Err(err)) = async_data.get() {
            error!("{}", err);
        }
    });
    let children = move |date| {
        if let Some(events) = async_data.get() {
            if let Ok(events) = events {
                return events
                    .iter()
                    .filter(|event| {
                        event
                            .start
                            .as_ref()
                            .is_some_and(|start| start.as_naive_date() == date)
                    })
                    .map(|event_today| {
                        let tz = chrono::Local::now().timezone();
                        let start_time = match event_today
                            .start
                            .as_ref()
                            .expect("Checked to be some during filter")
                        {
                            crate::ical_property::DateMaybeTime::DateTime(date_time) => {
                                Some(date_time.with_timezone(&tz))
                            }
                            crate::ical_property::DateMaybeTime::Date(_) => None,
                        };
                        let end_time = match event_today
                            .end
                            .as_ref()
                            .expect("Checked to be some during filter")
                        {
                            crate::ical_property::DateMaybeTime::DateTime(date_time) => {
                                Some(date_time.with_timezone(&tz))
                            }
                            crate::ical_property::DateMaybeTime::Date(_) => None,
                        };
                        let title = event_today.summary.as_ref().cloned();
                        let popup_title = title.as_ref().cloned();
                        let popup_desc = event_today.description.as_ref().cloned();

                        view! {
                            <Popover preferred_pos=PopoverAnchor::Right>
                                <PopoverTrigger slot>
                                    <div class=class_list!("self-stretch p-0.5 bg-teal-100 dark:bg-teal-900 m-0.5 shadow-sm text-xs md:text-sm line-clamp-3 shrink-0", ("grow", start_time.is_none()))>
                                        <OptionComp value=start_time let:start_time>
                                            <strong class="mr-[0.5ch] font-mono">
                                                {start_time.time().format("%H:%M").to_string()}
                                            </strong>
                                        </OptionComp>
                                        <OptionComp value=title.clone() let:title>
                                            <span>
                                                {title.to_string()}
                                            </span>
                                        </OptionComp>
                                    </div>
                                </PopoverTrigger>
                                <div>
                                    <OptionComp value=start_time let:start_time>
                                        <OptionComp value=end_time let:end_time>
                                            <strong class="mr-[0.5ch] font-mono">
                                                {start_time.time().format("%H:%M").to_string()} - {end_time.time().format("%H:%M").to_string()}
                                            </strong>
                                        </OptionComp>
                                    </OptionComp>
                                    <OptionComp value=popup_title let:title>
                                        <p><strong>
                                            {title.to_string()}
                                        </strong></p>
                                    </OptionComp>
                                    <OptionComp value=popup_desc let:desc>
                                        <p>
                                            {desc.to_string()}
                                        </p>
                                    </OptionComp>
                                </div>
                            </Popover>
                        }
                        .into_any()
                    })
                    .collect_view();
            }
        }
        vec![
            view! {
                <div class="grow self-stretch">Loading..</div>
            }
            .into_any(),
        ]
    };
    view! {
        <Calendar children presented_month_writer />
    }
}
