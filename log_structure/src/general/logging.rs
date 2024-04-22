use std::cmp::PartialEq;
use std::collections::HashMap;
use serde::{Serialize};
use chrono::{Utc, DateTime};
use std::env;

use crate::general;

#[derive(Serialize)]
struct LogMessage {
    timestamp: String,
    level: String,
    msg: String,
}

#[derive(Debug)]
enum LogEventLevel {
    Debug,
    Info,
    Warning,
    Error
}

impl LogEventLevel {
    fn from_string(level: &str) -> Self {
        match level {
            "Debug" => LogEventLevel::Debug,
            "Info" => LogEventLevel::Info,
            "Warning" => LogEventLevel::Warning,
            "Error" => LogEventLevel::Error,
            _ => panic!("Invalid log or event level '{}'. Possible values are 'Debug', 'Info', 'Warning' or 'Error'", level),
        }
    }
}

pub fn event(
    event_level: &str,
    message: &str
) -> () {
    let formatted_timestamp:String = timestamp();

    let log_level :LogEventLevel = get_log_level();
    println!("{:?}", log_level);

    let event_level:LogEventLevel = LogEventLevel::from_string(&event_level);

    let show_event :bool = filter_event(log_level, event_level);
    println!("{:?}", show_event);
    //if show_event {
    //    let log_message = LogMessage {
    //        timestamp: formatted_timestamp,
    //        level: event_level.to_string(),
    //        msg: message.to_string(),
    //    };

    //    let log_message: String = message_format(log_message);
    //    println!("{}", log_message)
    //}
}

impl PartialEq for LogEventLevel {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

fn filter_event(log_level: LogEventLevel, event_level: LogEventLevel) -> bool {
    match log_level {
        LogEventLevel::Debug => true,
        LogEventLevel::Info => event_level != LogEventLevel::Debug,
        LogEventLevel::Warning => event_level != LogEventLevel::Debug && event_level != LogEventLevel::Info,
        LogEventLevel::Error => event_level == LogEventLevel::Error
    }
}

fn get_log_level() -> LogEventLevel{
    let log_level_str :String = env::var("LOG_LEVEL").unwrap_or(String::from("Info"));
    let log_level:LogEventLevel = LogEventLevel::from_string(&log_level_str);
    log_level
}

fn old_get_log_level() -> String{
    let args_values:HashMap<String, String> = general::args::get_args();
    let log_level:String = args_values.get("log_level").unwrap().to_string();
    log_level
}

fn message_format(log_message: LogMessage) -> String {
    format!("{}", serde_json::to_string(&log_message).unwrap())
}

fn timestamp() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn validate_args(
    log_level: String
) -> (){
    if log_level == "Debug" || log_level == "Info" || log_level == "Warning" || log_level == "Error" {
        let log_message: String = format!("log_level set as {log_level}");
        event("Info", log_message.as_str())
    }
    else {
        panic!("The specified log_level specified as '{log_level}', that is not an acceptable value, please set as 'Debug', 'Info', 'Warning' or 'Error'")
    }
}


//#[cfg(test)]
//mod tests_filter_event {
//    use super::*;
//
//    #[test]
//    fn test_filter_event_debug() {
//        // Test when log_level is "Debug"
//        assert_eq!(filter_event("Debug", "Info"), true);
//        assert_eq!(filter_event("Debug", "Warning"), true);
//        assert_eq!(filter_event("Debug", "Error"), true);
//    }
//
//    #[test]
//    fn test_filter_event_info() {
//        // Test when log_level is "Info"
//        assert_eq!(filter_event("Info", "Debug"), false);
//        assert_eq!(filter_event("Info", "Info"), true);
//        assert_eq!(filter_event("Info", "Warning"), true);
//        assert_eq!(filter_event("Info", "Error"), true);
//    }
//
//    #[test]
//    fn test_filter_event_warning() {
//        // Test when log_level is "Warning"
//        assert_eq!(filter_event("Warning", "Debug"), false);
//        assert_eq!(filter_event("Warning", "Info"), false);
//        assert_eq!(filter_event("Warning", "Warning"), true);
//        assert_eq!(filter_event("Warning", "Error"), true);
//    }
//
//    #[test]
//    fn test_filter_event_error() {
//        // Test when log_level is "Error"
//        assert_eq!(filter_event("Error", "Debug"), false);
//        assert_eq!(filter_event("Error", "Info"), false);
//        assert_eq!(filter_event("Error", "Warning"), false);
//        assert_eq!(filter_event("Error", "Error"), true);
//    }
//
//    #[test]
//    fn test_filter_event_invalid() {
//        // Test with invalid log_level values
//        assert_eq!(filter_event("Invalid", "Info"), false);
//        assert_eq!(filter_event("Invalid", "Error"), false);
//    }
//}
//
//#[test]
//fn test_get_log_level_without_log_level_arg() {
//    let log_level = get_log_level();
//    assert_eq!(log_level, "Info".to_string())
//}
//
//#[cfg(test)]
//mod tests_message_format {
//    use super::*;
//
//    #[test]
//    fn test_message_format() -> () {
//
//        let formatted_timestamp: String = timestamp();
//        let message_level :String = "Info".to_string();
//        let message_content :String = "Sample message".to_string();
//
//        let log_message = LogMessage {
//            timestamp: formatted_timestamp.clone(),
//            level: message_level.clone(),
//            msg: message_content.clone()
//        };
//
//        let formatted_message: String = message_format(log_message);
//
//        let expected_formatted_message = LogMessage {
//            timestamp: formatted_timestamp,
//            level: message_level,
//            msg: message_content
//        };
//
//        let formatted_expected_message: String = format!("{}", serde_json::to_string(&expected_formatted_message).unwrap());
//
//        assert_eq!(formatted_message, formatted_expected_message)
//    }
//}
//
//#[cfg(test)]
//mod tests_timestamp {
//    use super::*;
//
//    #[test]
//    fn test_timestamp() -> () {
//        let result: String = timestamp();
//        let now: DateTime<Utc> = Utc::now();
//        let expect_result: String = now.format("%Y-%m-%d %H:%M:%S").to_string();
//        assert_eq!(result, expect_result);
//    }
//}
//
//#[test]
//fn test_validate_args_with_valid_log_level() {
//    let valid_log_levels = vec!["Debug", "Info", "Warning", "Error"];
//
//    for log_level in valid_log_levels {
//        validate_args(log_level.to_string());
//    }
//}
