use std::process::{Command, Output};

pub fn execute(command: &str, args: &[&str]) -> String {
    let command_output:Output = Command::new(&command)
        .args(args)
        .output()
        .expect(format!("Failed to execute command {}", &command).as_str());

    if command_output.status.success() {
        return String::from_utf8_lossy(&command_output.stdout).to_string();
    }
    else {
        panic!("Command failed with exit code {:?}", command_output.status.code());
    }
}
