#[cfg(feature = "ssr")]
use anyhow::{anyhow, Error};
use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, Utc};
use ical::parser::ical::component::IcalEvent;
#[cfg(feature = "ssr")]
use regex::Regex;
use rrule::RRuleSet;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

trait OptionVecPush<T> {
    fn push(&mut self, t: T);
}

impl<T> OptionVecPush<T> for Option<Vec<T>> {
    fn push(&mut self, element: T) {
        if self.is_none() {
            let _ = self.insert(vec![element]);
        } else {
            self.as_mut().unwrap().push(element);
        }
    }
}

/// Events can either happen at a date
/// or a date time.
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum DateMaybeTime {
    /// Event with a date and time
    DateTime(DateTime<Utc>),
    /// Event without a time, just a date
    Date(NaiveDate), // without time zone
}
#[cfg(feature = "ssr")]
impl From<NaiveDate> for DateMaybeTime {
    fn from(value: NaiveDate) -> Self {
        Self::Date(value)
    }
}
#[cfg(feature = "ssr")]
impl From<DateTime<Utc>> for DateMaybeTime {
    fn from(value: DateTime<Utc>) -> Self {
        Self::DateTime(value)
    }
}

impl DateMaybeTime {
    /// Ignore potentially exiting time, only take naive date.
    pub fn as_naive_date(&self) -> NaiveDate {
        match self {
            Self::Date(d) => d.clone(),
            Self::DateTime(dt) => dt.date_naive(),
        }
    }
}

/// When inviting others, an
/// Event can be tentative, confirmed or cancelled.
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum EventStatus {
    /// Invite was not confirmed.
    Tentative,
    /// Invite was confirmed.
    Confirmed,
    /// Invite was cancelled.
    Cancelled,
}
#[cfg(feature = "ssr")]
impl FromStr for EventStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "TENTATIVE" => Ok(EventStatus::Tentative),
            "CONFIRMED" => Ok(EventStatus::Confirmed),
            "CANCELLED" => Ok(EventStatus::Cancelled),
            _ => Err(()),
        }
    }
}

/// Whether an event is blocking a time interval
/// in the calender.
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum EventTransparency {
    /// Event block interval.
    Opaque,
    /// Event does not block interval.
    Transparent,
}
#[cfg(feature = "ssr")]
impl FromStr for EventTransparency {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "OPAQUE" => Ok(EventTransparency::Opaque),
            "TRANSPARENT" => Ok(EventTransparency::Transparent),
            _ => Err(()),
        }
    }
}

#[cfg(feature = "ssr")]
fn parse_duration(s: &str) -> Result<Duration, Error> {
    let re = Regex::new(
                r"^P(?:(?P<days>\d+)D)?(?:T(?:(?P<hours>\d+)H)?(?:(?P<minutes>\d+)M)?(?:(?P<seconds>\d+)S)?)?$",
            ).unwrap();

    if let Some(captures) = re.captures(s) {
        let days = captures
            .name("days")
            .map(|m| m.as_str().parse::<i64>().unwrap_or(0))
            .unwrap_or(0);
        let hours = captures
            .name("hours")
            .map(|m| m.as_str().parse::<i64>().unwrap_or(0))
            .unwrap_or(0);
        let minutes = captures
            .name("minutes")
            .map(|m| m.as_str().parse::<i64>().unwrap_or(0))
            .unwrap_or(0);
        let seconds = captures
            .name("seconds")
            .map(|m| m.as_str().parse::<i64>().unwrap_or(0))
            .unwrap_or(0);

        Ok(Duration::days(days)
            + Duration::hours(hours)
            + Duration::minutes(minutes)
            + Duration::seconds(seconds))
    } else {
        Err(anyhow!("Invalid duration format"))
    }
}

#[cfg(feature = "ssr")]
fn parse_datetime(s: &str) -> Result<DateMaybeTime, Error> {
    if let Ok(d) = NaiveDate::parse_from_str(s, "%Y%m%d") {
        return Ok(d.into());
    }
    let naive_datetime_res = NaiveDateTime::parse_from_str(s, "%Y%m%dT%H%M%SZ");
    if let Ok(dt) = naive_datetime_res {
        return Ok(dt.and_utc().into());
    }
    // No DateTime given, assume local
    let naive_datetime_res = NaiveDateTime::parse_from_str(s, "%Y%m%dT%H%M%S");
    if let Ok(dt) = naive_datetime_res {
        // TODO: does this work?
        let dt = dt.and_local_timezone(Local).unwrap();
        return Ok(dt.to_utc().into());
    }

    dateparser::parse(s).map(Into::into)
}

/// Heart of this crate. It is supposed to
/// define an event as described in RFC 5545,
/// but with fitting datatypes.
#[derive(Debug, Default, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Event {
    /// Matches UID.
    pub uid: Option<String>,
    /// Matches CREATED.
    pub created: Option<DateMaybeTime>,
    /// Matches SUMMARY.
    pub summary: Option<String>,
    /// Matches START.
    pub start: Option<DateMaybeTime>,
    /// Matches END.
    pub end: Option<DateMaybeTime>,
    /// Matches DURATION.
    pub duration: Option<Duration>,
    /// Matches LOCATION.
    pub location: Option<String>,
    /// Matches DESCRIPTION.
    pub description: Option<String>,
    /// Matches STATUS.
    pub status: Option<EventStatus>,
    /// Matches TRANSPARENCY.
    pub transparency: Option<EventTransparency>,
    /// Matches CATEGORIES.
    pub categories: Option<Vec<String>>,
    /// Matches ATTENDEES.
    pub attendees: Option<Vec<String>>,
    /// Matches ORGANIZER.
    pub organizer: Option<String>,
    /// Matches PRIORITY.
    pub priority: Option<u8>,
    /// Matches SEQUENCE.
    pub sequence: Option<i32>,
    /// Matches DTSTAMP.
    pub dtstamp: Option<DateMaybeTime>,
    /// Matches RECURRENCE_ID.
    pub recurrence_id: Option<DateMaybeTime>,
    /// Contains information from RRULE, RDATE, EXDATE, EXRULE and DTSTART.
    pub rrule: Option<RRuleSet>,
    /// Matches COMMENT.
    pub comment: Option<String>,
    /// Matches ATTACH.
    pub attach: Option<Vec<String>>,
    /// Matches ALARMS.
    pub alarms: Option<Vec<String>>,
    /// Matches LAST_MODIFIED.
    pub last_modified: Option<DateMaybeTime>,
}
#[cfg(feature = "ssr")]
impl TryFrom<&IcalEvent> for Event {
    type Error = Error;

    fn try_from(value: &IcalEvent) -> Result<Self, Self::Error> {
        map_ical_event(value)
    }
}
#[cfg(feature = "ssr")]
impl TryFrom<IcalEvent> for Event {
    type Error = Error;

    fn try_from(value: IcalEvent) -> Result<Self, Self::Error> {
        map_ical_event(&value)
    }
}
#[cfg(feature = "ssr")]
fn map_ical_event(input: &IcalEvent) -> Result<Event, Error> {
    let mut event = Event::default();
    let mut rrule_lines: Option<Vec<_>> = None;
    let mut has_rrules = false;
    for prop in input.properties.iter() {
        if prop.value.is_none() {
            continue;
        }
        let value = prop.value.as_ref().unwrap();
        let key: &str = &prop.name;
        if ["RDATE", "RRULE", "EXDATE", "EXRULE", "DTSTART"].contains(&key) {
            rrule_lines.push(format!("{}:{}", key, value));
        }
        match key {
            "UID" => event.uid = Some(value.to_string()),
            "SUMMARY" => event.summary = Some(value.to_string()),
            "DTSTART" => event.start = Some(parse_datetime(value.as_str())?),
            "DTEND" => event.end = Some(parse_datetime(value.as_str())?),
            "CREATED" => event.created = Some(parse_datetime(value.as_str())?),
            "DURATION" => event.duration = Some(parse_duration(value)?),
            "LOCATION" => event.location = Some(value.to_string()),
            "DESCRIPTION" => event.description = Some(value.to_string()),
            "STATUS" => event.status = Some(value.parse().map_err(|_| anyhow!("Invalid status"))?),
            "LAST-MODIFIED" => event.last_modified = Some(parse_datetime(value)?),
            "TRANSPARENCY" => {
                event.transparency =
                    Some(value.parse().map_err(|_| anyhow!("Invalid transparency"))?)
            }
            "CATEGORIES" => event.categories.push(value.to_string()), // Push to OptionVector
            "ATTENDEE" => event.attendees.push(value.to_string()),    // Push to OptionVector
            "ORGANIZER" => event.organizer = Some(value.to_string()),
            "PRIORITY" => {
                event.priority = Some(value.parse().map_err(|_| anyhow!("Invalid priority"))?)
            }
            "SEQUENCE" => {
                event.sequence = Some(value.parse().map_err(|_| anyhow!("Invalid sequence"))?)
            }
            "DTSTAMP" => event.dtstamp = Some(parse_datetime(value.as_str())?),
            "RECURRENCE-ID" => event.recurrence_id = Some(parse_datetime(value.as_str())?),
            "RRULE" => has_rrules = true,
            "RDATE" | "EXRULE" | "EXDATE" => (),
            "COMMENT" => event.comment = Some(value.to_string()),
            "ATTACH" => event.attach.push(value.to_string()),
            "ALARM" => event.alarms.push(value.to_string()),
            x if x.starts_with("X-") => (),
            "TRANSP" | "CLASS" | "URL" => (),
            x => return Err(anyhow!("Unknown property key: {}", x)),
        }
    }
    if has_rrules {
        let rrule: RRuleSet = rrule_lines.unwrap().join("\n").parse()?;
        event.rrule = Some(rrule);
    }
    Ok(event)
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    #[test]
    fn it_works() {
        let buf = BufReader::new(File::open("resources/test1.ical").unwrap());

        let reader = ical::IcalParser::new(buf);

        for calendar in reader {
            let cal = calendar.unwrap();
            for event in cal.events {
                let res = map_ical_event(&event);
                let res = res.unwrap();
                if res.summary == Some("Jeden Montag bis Freitag ganztägig".into()) {
                    println!("{:#?}", res);
                    for event in res.rrule.unwrap().into_iter().take(100) {
                        println!("Occurance: {}", event)
                    }
                }
            }
        }
    }
}