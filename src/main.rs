mod constants;
mod disc_time;
mod format;
mod random;

use std::env;
use std::process;

use constants::{DEFAULT_FMT, DEFAULT_IMMEDIATE_FMT};
use disc_time::{convert, makeday, DiscTime};
use format::format;
use random::SimpleRng;

const PACKAGE_STRING: &str = "Stand Alone";

fn main() {
    let args: Vec<String> = env::args().collect();

    // Get program name (basename)
    let progname = args[0].rsplit('/').next().unwrap_or(&args[0]);

    let mut rng = SimpleRng::new();
    let mut format_str: Option<&str> = None;
    let mut pi = 1;

    // Parse arguments matching C's loop behavior exactly
    while pi < args.len() {
        let arg = args[pi].as_bytes();
        if arg.is_empty() {
            break;
        }

        match arg[0] {
            b'+' => {
                format_str = Some(&args[pi][1..]);
                pi += 1;
            }
            b'-' => {
                if arg.len() > 1 && arg[1] == b'V' {
                    println!("{} ({})", progname, PACKAGE_STRING);
                }
                usage(&args[0]);
            }
            _ => break,
        }
    }

    // After loop: check remaining arguments
    let remaining = args.len() - pi;

    let (dt, default_format): (DiscTime, &str) = if remaining == 3 {
        // Explicit date provided: day month year (or month day year with us_format)
        let arg1: i32 = args[pi].parse().unwrap_or(0);
        let arg2: i32 = args[pi + 1].parse().unwrap_or(0);
        let year: i32 = args[pi + 2].parse().unwrap_or(0);

        let dt = makeday(arg1, arg2, year);

        if dt.season == -1 {
            println!("Invalid date -- out of range");
            process::exit(-1);
        }

        (dt, DEFAULT_FMT)
    } else if remaining == 0 {
        // Current date
        let (yday, year) = get_current_date();
        let dt = convert(yday, year);
        (dt, DEFAULT_IMMEDIATE_FMT)
    } else {
        // Wrong number of arguments
        usage(&args[0]);
    };

    let fmt = format_str.unwrap_or(default_format);
    let output = format(fmt, dt, &mut rng);
    println!("{}", output);
}

fn usage(progname: &str) -> ! {
    eprintln!("usage: {} [+format] [day month year]", progname);
    process::exit(1);
}

/// Get current date as (tm_yday, tm_year) matching C's localtime()
/// tm_yday: days since January 1 (0-365)
/// tm_year: years since 1900
fn get_current_date() -> (i32, i32) {
    #[cfg(unix)]
    {
        use std::ffi::c_long;

        #[repr(C)]
        struct Tm {
            tm_sec: i32,
            tm_min: i32,
            tm_hour: i32,
            tm_mday: i32,
            tm_mon: i32,
            tm_year: i32,
            tm_wday: i32,
            tm_yday: i32,
            tm_isdst: i32,
            // Additional fields on some platforms, but we only need up to tm_yday
        }

        extern "C" {
            fn time(tloc: *mut c_long) -> c_long;
            fn localtime(timep: *const c_long) -> *const Tm;
        }

        unsafe {
            let t = time(std::ptr::null_mut());
            let tm = localtime(&t);
            ((*tm).tm_yday, (*tm).tm_year)
        }
    }

    #[cfg(not(unix))]
    {
        // Fallback for non-Unix: calculate from SystemTime (UTC only)
        use std::time::{SystemTime, UNIX_EPOCH};

        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        calculate_utc_date(secs)
    }
}

/// Calculate yday and year from Unix timestamp (UTC only, fallback)
#[cfg(not(unix))]
fn calculate_utc_date(secs: u64) -> (i32, i32) {
    // Days since Unix epoch (Jan 1, 1970)
    let days = (secs / 86400) as i32;

    // Calculate year
    let mut year = 1970;
    let mut remaining_days = days;

    loop {
        let days_in_year = if is_leap_year_greg(year) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }

    // remaining_days is now yday (0-indexed)
    (remaining_days, year - 1900)
}

#[cfg(not(unix))]
fn is_leap_year_greg(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
