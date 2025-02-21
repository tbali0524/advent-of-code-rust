//! ANSI color codes for terminal output.

pub const ANSI_RESET: &str = "\x1b[0m";
pub const ANSI_GREEN: &str = "\x1b[32m";
pub const ANSI_GREEN_INV: &str = "\x1b[1;37;42m";
pub const ANSI_RED: &str = "\x1b[31m";
pub const ANSI_RED_INV: &str = "\x1b[1;37;41m";
pub const ANSI_YELLOW: &str = "\x1b[33m";
pub const ANSI_YELLOW_INV: &str = "\x1b[1;37;43m";
