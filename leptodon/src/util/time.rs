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
use chrono::{Duration, NaiveDate};

/// Saturates NaiveDate when subtraacting the *duration* would underflow the *date*
///
/// <div class="warning">
/// Don't use with *duration* exceeding 290181997741 days.
/// This function will ceil duration at the MAX representable TimeDelta time as it should be enough for normal use-cases.
/// This number comes from the internal chrono::TimeDelta limit: ((i64::MAX/1000) seconds -> days)
/// </div>
pub fn date_saturating_sub(date: NaiveDate, delta: Duration) -> NaiveDate {
    let opt_result = date.checked_sub_signed(delta);
    opt_result.unwrap_or(NaiveDate::MIN)
}

/// Saturates NaiveDate when adding the *duration* would overflow the *date*
///
/// <div class="warning">
/// Don't use with *duration* exceeding 290181997741 days.
/// This function will ceil duration at the MAX representable TimeDelta time as it should be enough for normal use-cases.
/// This number comes from the internal chrono::TimeDelta limit: ((i64::MAX/1000) seconds -> days)
/// </div>
pub fn date_saturating_add(date: NaiveDate, delta: Duration) -> NaiveDate {
    let opt_result = date.checked_add_signed(delta);
    opt_result.unwrap_or(NaiveDate::MAX)
}

#[cfg(test)]
mod test {
    use crate::util::time::{date_saturating_add, date_saturating_sub};
    use chrono::Duration;
    use chrono::NaiveDate;

    #[test]
    fn test_date_saturating_sub() {
        let mut date = NaiveDate::from_ymd_opt(2026, 5, 4).unwrap();

        date = date_saturating_sub(date, Duration::hours(5));
        assert_eq!(date, NaiveDate::from_ymd_opt(2026, 5, 4).unwrap());

        date = date_saturating_sub(date, Duration::days(5));
        assert_eq!(date, NaiveDate::from_ymd_opt(2026, 4, 29).unwrap());

        date = date_saturating_sub(date, Duration::MAX);
        assert_eq!(date, NaiveDate::MIN);
    }

    #[test]
    fn test_date_saturating_add() {
        let mut date = NaiveDate::from_ymd_opt(2026, 5, 4).unwrap();

        // Adding less than a day shoult not change the result
        date = date_saturating_add(date, Duration::hours(23));
        assert_eq!(date, NaiveDate::from_ymd_opt(2026, 5, 4).unwrap());

        // Add 1 month.
        date = date_saturating_add(date, Duration::days(31));
        assert_eq!(date, NaiveDate::from_ymd_opt(2026, 6, 4).unwrap());

        // Overflow
        date = date_saturating_add(date, Duration::MAX);
        assert_eq!(date, NaiveDate::MAX);
    }
}
