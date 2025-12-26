pub const DAY_LONG: [&str; 5] = [
    "Sweetmorn",
    "Boomtime",
    "Pungenday",
    "Prickle-Prickle",
    "Setting Orange",
];

pub const DAY_SHORT: [&str; 5] = ["SM", "BT", "PD", "PP", "SO"];

pub const SEASON_LONG: [&str; 5] = [
    "Chaos",
    "Discord",
    "Confusion",
    "Bureaucracy",
    "The Aftermath",
];

pub const SEASON_SHORT: [&str; 5] = ["Chs", "Dsc", "Cfn", "Bcy", "Afm"];

pub const HOLYDAYS: [[&str; 2]; 5] = [
    ["Mungday", "Chaoflux"],
    ["Mojoday", "Discoflux"],
    ["Syaday", "Confuflux"],
    ["Zaraday", "Bureflux"],
    ["Maladay", "Afflux"],
];

/// Static exclamations array matching C's exact order
#[cfg(all(not(feature = "praise_bob"), target_os = "linux"))]
pub const EXCLAMATIONS: &[&str] = &[
    "Hail Eris!", "All Hail Discordia!", "Kallisti!", "Fnord.", "Or not.",
    "Wibble.", "Pzat!", "P'tang!", "Frink!",
    "Grudnuk demand sustenance!", "Keep the Lasagna flying!", "You are what you see.",
    "Or is it?", "This statement is false.", "Lies and slander, sire!", "Hee hee hee!",
    "Hail Eris, Hack Linux!", "",
];

#[cfg(all(not(feature = "praise_bob"), target_os = "macos"))]
pub const EXCLAMATIONS: &[&str] = &[
    "Hail Eris!", "All Hail Discordia!", "Kallisti!", "Fnord.", "Or not.",
    "Wibble.", "Pzat!", "P'tang!", "Frink!",
    "Grudnuk demand sustenance!", "Keep the Lasagna flying!", "You are what you see.",
    "Or is it?", "This statement is false.", "Lies and slander, sire!", "Hee hee hee!",
    "This Fruit is not the True Fruit of Discord.", "",
];

#[cfg(all(not(feature = "praise_bob"), not(target_os = "linux"), not(target_os = "macos")))]
pub const EXCLAMATIONS: &[&str] = &[
    "Hail Eris!", "All Hail Discordia!", "Kallisti!", "Fnord.", "Or not.",
    "Wibble.", "Pzat!", "P'tang!", "Frink!",
    "Grudnuk demand sustenance!", "Keep the Lasagna flying!", "You are what you see.",
    "Or is it?", "This statement is false.", "Lies and slander, sire!", "Hee hee hee!",
    "",
];

#[cfg(all(feature = "praise_bob", target_os = "linux"))]
pub const EXCLAMATIONS: &[&str] = &[
    "Hail Eris!", "All Hail Discordia!", "Kallisti!", "Fnord.", "Or not.",
    "Wibble.", "Pzat!", "P'tang!", "Frink!",
    "Slack!", "Praise \"Bob\"!", "Or kill me.",
    "Grudnuk demand sustenance!", "Keep the Lasagna flying!", "You are what you see.",
    "Or is it?", "This statement is false.", "Lies and slander, sire!", "Hee hee hee!",
    "Hail Eris, Hack Linux!", "",
];

#[cfg(all(feature = "praise_bob", target_os = "macos"))]
pub const EXCLAMATIONS: &[&str] = &[
    "Hail Eris!", "All Hail Discordia!", "Kallisti!", "Fnord.", "Or not.",
    "Wibble.", "Pzat!", "P'tang!", "Frink!",
    "Slack!", "Praise \"Bob\"!", "Or kill me.",
    "Grudnuk demand sustenance!", "Keep the Lasagna flying!", "You are what you see.",
    "Or is it?", "This statement is false.", "Lies and slander, sire!", "Hee hee hee!",
    "This Fruit is not the True Fruit of Discord.", "",
];

#[cfg(all(feature = "praise_bob", not(target_os = "linux"), not(target_os = "macos")))]
pub const EXCLAMATIONS: &[&str] = &[
    "Hail Eris!", "All Hail Discordia!", "Kallisti!", "Fnord.", "Or not.",
    "Wibble.", "Pzat!", "P'tang!", "Frink!",
    "Slack!", "Praise \"Bob\"!", "Or kill me.",
    "Grudnuk demand sustenance!", "Keep the Lasagna flying!", "You are what you see.",
    "Or is it?", "This statement is false.", "Lies and slander, sire!", "Hee hee hee!",
    "",
];

#[cfg(feature = "old_immediate_fmt")]
pub const DEFAULT_IMMEDIATE_FMT: &str =
    "Today is %{%A, the %e day of %B%} in the YOLD %Y%N%nCelebrate %H";

#[cfg(not(feature = "old_immediate_fmt"))]
pub const DEFAULT_IMMEDIATE_FMT: &str = "%{%A, %B %d%}, %Y YOLD";

pub const DEFAULT_FMT: &str = "%{%A, %B %d%}, %Y YOLD";
