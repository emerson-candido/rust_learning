use std::collections::HashMap;
use crate::general;

pub fn validate() -> () {
    let args_values:HashMap<String, String> = general::args::get_args();
    let log_level:String = args_values.get("log_level").unwrap().to_string();
    general::logging::validate_args(log_level.to_string());
}
