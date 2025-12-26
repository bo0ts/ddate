use crate::constants::HOLYDAYS;

/// Days in each Gregorian month (non-leap year)
const CALENDAR: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// Represents a Discordian date
#[derive(Debug, Clone, Copy, Default)]
pub struct DiscTime {
    pub season: i32, // 0-4, or -1 for invalid
    pub day: i32,    // 0-72, or -1 for St. Tib's Day
    pub yday: i32,   // 0-365 (day of Discordian year)
    pub year: i32,   // Discordian year (Gregorian + 1166)
}

impl DiscTime {
    /// Check if this is St. Tib's Day
    pub fn is_tibs_day(&self) -> bool {
        self.day == -1
    }

    /// Get the holyday name if applicable
    pub fn holyday_name(&self) -> Option<&'static str> {
        if self.day == 4 {
            Some(HOLYDAYS[self.season as usize][0])
        } else if self.day == 49 {
            Some(HOLYDAYS[self.season as usize][1])
        } else {
            None
        }
    }
}

/// Convert day-of-year and year (as tm_yday, tm_year from localtime) to DiscTime
///
/// nday: days since January 1 (0-365)
/// nyear: years since 1900
pub fn convert(nday: i32, nyear: i32) -> DiscTime {
    // nyear is years since 1900, add 3066 to get Discordian year
    // 1900 + 1166 = 3066
    let year = nyear + 3066;
    let mut day = nday;

    // Handle leap year St. Tib's Day
    // Discordian leap years have year % 4 == 2 (because 1166 % 4 == 2)
    if year % 4 == 2 {
        if day == 59 {
            // February 29 (0-indexed day 59) is St. Tib's Day
            day = -1;
        } else if day > 59 {
            // Days after St. Tib's Day are shifted back by one
            day -= 1;
        }
    }

    let yday = day;

    // Calculate season (each season has 73 days)
    let (season, day) = if day >= 0 {
        (day / 73, day % 73)
    } else {
        (0, day) // St. Tib's Day
    };

    DiscTime {
        season,
        day,
        yday,
        year,
    }
}

/// Convert Gregorian date to DiscTime
///
/// Arguments order depends on us_format feature:
/// - us_format disabled (default): arg1=day, arg2=month
/// - us_format enabled: arg1=month, arg2=day
pub fn makeday(arg1: i32, arg2: i32, iyear: i32) -> DiscTime {
    #[cfg(feature = "us_format")]
    let (imonth, iday) = (arg1, arg2);
    #[cfg(not(feature = "us_format"))]
    let (iday, imonth) = (arg1, arg2);

    // Validate month (1-12) and year (not 0)
    if !(1..=12).contains(&imonth) || iyear == 0 {
        return DiscTime {
            season: -1,
            ..Default::default()
        };
    }

    // Validate day
    let month_idx = (imonth - 1) as usize;
    let max_day = CALENDAR[month_idx];

    if iday < 1 || iday > max_day {
        // Special case: Feb 29 on leap year
        let is_leap = iyear % 4 == 0 && (iyear % 100 != 0 || iyear % 400 == 0);
        if !(imonth == 2 && iday == 29 && is_leap) {
            return DiscTime {
                season: -1,
                ..Default::default()
            };
        }
    }

    // Calculate Discordian year
    // Note: Gregorian year 0 doesn't exist, so add 1 for negative years
    let year = iyear + 1166 + if iyear < 0 { 1 } else { 0 };

    // Calculate day of year (0-indexed)
    let days_past: i32 = CALENDAR.iter().take(month_idx).sum();
    let mut day = days_past + iday - 1;

    // Handle St. Tib's Day
    // Discordian leap years have year % 4 == 2
    if year % 4 == 2 && day == 59 && iday == 29 {
        day = -1;
    }

    let yday = day;

    // Calculate season (each season has 73 days)
    let (season, day) = if day >= 0 {
        (day / 73, day % 73)
    } else {
        (0, day) // St. Tib's Day
    };

    DiscTime {
        season,
        day,
        yday,
        year,
    }
}
