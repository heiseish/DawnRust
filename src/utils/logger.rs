//! Print logging text with date and current file
//! Different log levels info, warn, error
//!
pub const GREEN_COLOR: &str = "\x1b[6;92m";
pub const YELLOW_COLOR: &str = "\x1b[6;93m";
pub const CYAN_COLOR: &str = "\x1b[6;96m";
pub const MAGNETA_COLOR: &str = "\x1b[6;95m";
pub const RESET_SEQ: &str = "\x1b[0m";
pub const RED_COLOR: &str = "\x1b[6;91m";

mod logger {

    #[macro_export]
    macro_rules! info {
        ($ ($ arg : tt) *) => {
            let now = crate::Utc::now().format("%Y-%m-%d %H:%M:%S");
            let file_name = file!().split("/").last().unwrap_or("unknown");
            println!(
                "{}{}{} - {}[  INFO  ]{} - {}{}{} - {} - {}",
                crate::utils::logger::MAGNETA_COLOR,
                now,
                crate::utils::logger::RESET_SEQ,
                crate::utils::logger::GREEN_COLOR,
                crate::utils::logger::RESET_SEQ,
                crate::utils::logger::CYAN_COLOR,
                file_name,
                crate::utils::logger::RESET_SEQ,
                line!(),
                format!($ ($ arg) *)
            );
        };
    }
    #[macro_export]
    macro_rules! warn {
        ($ ($ arg : tt) *) => {
            let now = crate::Utc::now().format("%Y-%m-%d %H:%M:%S");
            let file_name = file!().split("/").last().unwrap_or("unknown");
            println!(
                "{}{}{} - {}[  WARN  ]{} - {}{}{} - {} - {}",
                crate::utils::logger::MAGNETA_COLOR,
                now,
                crate::utils::logger::RESET_SEQ,
                crate::utils::logger::YELLOW_COLOR,
                crate::utils::logger::RESET_SEQ,
                crate::utils::logger::CYAN_COLOR,
                file_name,
                crate::utils::logger::RESET_SEQ,
                line!(),
                format!($ ($ arg) *)
            );
        };
    }
    #[macro_export]
    macro_rules! error {
        ($ ($ arg : tt) *) => {
            let now = crate::Utc::now().format("%Y-%m-%d %H:%M:%S");
            let file_name = file!().split("/").last().unwrap_or("unknown");
            println!(
                "{}{}{} - {}[  ERROR ]{}  - {}{}{} - {} - {}",
                crate::utils::logger::MAGNETA_COLOR,
                now,
                crate::utils::logger::RESET_SEQ,
                crate::utils::logger::RED_COLOR,
                crate::utils::logger::RESET_SEQ,
                crate::utils::logger::CYAN_COLOR,
                file_name,
                crate::utils::logger::RESET_SEQ,
                line!(),
                format!($ ($ arg) *)
            );
        };
    }
}
