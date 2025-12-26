use crate::constants::{DAY_LONG, DAY_SHORT, EXCLAMATIONS, SEASON_LONG, SEASON_SHORT};
use crate::disc_time::DiscTime;
use crate::random::SimpleRng;

/// Ordinal suffixes lookup table
const ENDINGS: [&str; 10] = ["th", "st", "nd", "rd", "th", "th", "th", "th", "th", "th"];

/// Get ordinal suffix for a number (1st, 2nd, 3rd, 4th, etc.)
#[inline]
fn ending(i: i32) -> &'static str {
    if (11..=13).contains(&(i % 100)) {
        "th"
    } else {
        ENDINGS[(i % 10) as usize]
    }
}

/// Check if a year is a leap year using the C code's formula
/// C: (!(DY(i)%4))&&((DY(i)%100)||(!(DY(i)%400)))
/// where DY(y) = y + 1166
/// Note: This replicates the C code's behavior exactly, including when
/// passed a Discordian year (which adds 1166 again)
#[cfg(feature = "kill_bob")]
fn leapp(i: i32) -> bool {
    let dy = i + 1166;
    (dy % 4 == 0) && ((dy % 100 != 0) || (dy % 400 == 0))
}

/// Calculate days until X-Day (Cfn 40, 9827)
/// Matches C code exactly
#[cfg(feature = "kill_bob")]
fn xday_countdown(yday: i32, year: i32) -> i32 {
    let mut year = year;
    let mut r = (185 - yday) + if yday < 59 && leapp(year) { 1 } else { 0 };

    while year < 9827 {
        year += 1;
        r += if leapp(year) { 366 } else { 365 };
    }
    while year > 9827 {
        r -= if leapp(year) { 366 } else { 365 };
        year -= 1;
    }

    r
}

/// Format a Discordian date according to the format string
/// Matches C code's format() function exactly
pub fn format(fmt: &str, dt: DiscTime, rng: &mut SimpleRng) -> String {
    let bytes = fmt.as_bytes();
    let fmtlen = bytes.len();
    let mut output = String::with_capacity(64);

    // First pass: find extents of St. Tib's Day region
    // C code uses tib_start=-1 and checks tib_start>0, meaning position 0
    // is treated specially (won't update tib_end if tib_start==0)
    let mut tib_start: i32 = -1;
    let mut tib_end: usize = 0;

    let mut i = 0;
    while i < fmtlen {
        if bytes[i] == b'%' && i + 1 < fmtlen {
            match bytes[i + 1] {
                b'A' | b'a' | b'd' | b'e' => {
                    if tib_start > 0 {
                        tib_end = i + 1;
                    } else {
                        tib_start = i as i32;
                    }
                }
                b'{' => tib_start = i as i32,
                b'}' => tib_end = i + 1,
                _ => {}
            }
        }
        i += 1;
    }

    // Second pass: format output
    // Precompute values used multiple times
    let day_idx = (dt.yday % 5) as usize;
    let season_idx = dt.season as usize;
    let day_num = dt.day + 1;

    let mut i = 0;
    while i < fmtlen {
        // Handle St. Tib's Day replacement
        if i as i32 == tib_start && dt.is_tibs_day() {
            output.push_str("St. Tib's Day");
            i = tib_end + 1;
            continue;
        }

        if bytes[i] == b'%' && i + 1 < fmtlen {
            i += 1;
            match bytes[i] {
                b'A' => output.push_str(DAY_LONG[day_idx]),
                b'a' => output.push_str(DAY_SHORT[day_idx]),
                b'B' => output.push_str(SEASON_LONG[season_idx]),
                b'b' => output.push_str(SEASON_SHORT[season_idx]),
                b'd' => itoa(&mut output, day_num),
                b'e' => {
                    itoa(&mut output, day_num);
                    output.push_str(ending(day_num));
                }
                b'H' => {
                    if let Some(name) = dt.holyday_name() {
                        output.push_str(name);
                    }
                }
                b'N' => {
                    // Early exit if not a holyday (goto eschaton in C)
                    if dt.day != 4 && dt.day != 49 {
                        return output;
                    }
                }
                b'n' => output.push('\n'),
                b't' => output.push('\t'),
                b'Y' => itoa(&mut output, dt.year),
                #[allow(clippy::explicit_auto_deref)]
                b'.' => output.push_str(*rng.select(EXCLAMATIONS)),
                #[cfg(feature = "kill_bob")]
                b'X' => itoa(&mut output, xday_countdown(dt.yday, dt.year)),
                b'{' | b'}' => {}
                _ => {}
            }
        } else {
            output.push(bytes[i] as char);
        }
        i += 1;
    }

    output
}

/// Fast integer to string, writes directly to output
#[inline]
fn itoa(output: &mut String, mut n: i32) {
    if n < 0 {
        output.push('-');
        n = -n;
    }
    if n == 0 {
        output.push('0');
        return;
    }
    let start = output.len();
    while n > 0 {
        output.push((b'0' + (n % 10) as u8) as char);
        n /= 10;
    }
    // Reverse the digits we just pushed
    let bytes = unsafe { output.as_bytes_mut() };
    bytes[start..].reverse();
}
