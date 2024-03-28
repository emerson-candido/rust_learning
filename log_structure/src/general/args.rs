use clap::Parser;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Parser, Debug)]
pub enum Commands {
    /// Define log settings
    Logging(LoggingCommands),
}

#[derive(Parser, Debug)]
pub struct LoggingCommands {
    /// Set log level
    #[clap(short, long, default_value = "Info")]
    pub log_level: String,
}

pub fn get_args() -> HashMap<String, String> {
    let args: Vec<String> = std::env::args().collect();
    let mut args_values: HashMap<String, String> = HashMap::new();

    // Unittest filter arguments
    let is_testing: bool = std::env::var("RUST_TEST_TASK").is_ok();
    let args_to_parse: Vec<String> = if is_testing {
        args.into_iter()
            .filter(|arg| !arg.eq("--nocapture"))
            .filter(|arg| !arg.starts_with("--format"))
            .filter(|arg| !arg.eq("-Z"))
            .filter(|arg| !arg.eq("unstable-options"))
            .filter(|arg| !arg.eq("--show-output"))
            .collect()
    } else {
        args
    };

    let cli: Cli = Cli::parse_from(args_to_parse);

    let log_level = match &cli.command {
        Some(Commands::Logging(logging_commands)) => {
            logging_commands.log_level.clone()
        }
        _ => "Info".to_string(),
    };

    args_values.insert("log_level".to_string(), log_level);
    args_values
}


#[cfg(test)]
mod tests_get_args {
    use super::*;

    #[test]
    fn test_get_args_with_default_log_level() {
        std::env::set_var("RUST_TEST_TASK", "true");
        let args_values = get_args();
        assert_eq!(args_values["log_level"], "Info");
    }
}
