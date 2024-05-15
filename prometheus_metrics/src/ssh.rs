use std::collections::HashMap;
use crate::common;

pub fn ssh_connections() -> HashMap<String, String> {
    let mut connections: HashMap<String, String> = HashMap::new();

    let ssh_processes :String = common::commands::execute("ps", &["-aux"]);
    for ssh_process in ssh_processes.lines() {
        let ssh_process_columns: Vec<&str> = ssh_process.split_whitespace().collect();
        if ssh_process_columns[10].contains("sshd") && ssh_process_columns[11].contains("@") {
            let username :&str = ssh_process_columns[11].split_once("@").unwrap().0;
            let pid :&str = ssh_process_columns[1];

            let open_files :String = common::commands::execute("lsof", &["-p", pid]);
            for open_file in open_files.lines() {
                let open_file_columns: Vec<&str> = open_file.split_whitespace().collect();
                if open_file_columns[0].contains("sshd") && open_file_columns.len() > 9 && open_file_columns[9].contains("ESTABLISHED") {
                    let ssh_established_connection: Vec<&str> = open_file_columns[8].split(":").collect();
                    if let Some(local_ssh_port) = ssh_established_connection.get(2) {
                        let local_ssh_connections :String = common::commands::execute(
                            "ss",
                            &["-o", "state", "established", "sport", "=", ":ssh" ]
                        );
                        for local_ssh_connection in local_ssh_connections.lines() {
                            let local_ssh_connection_columns: Vec<&str> = local_ssh_connection.split_whitespace().collect();
                            if local_ssh_connection_columns[4].contains(local_ssh_port) {
                                let remote_ip :String = local_ssh_connection_columns[4].split_once(":").unwrap().0.to_string();
                                connections.insert(String::from(username), String::from(remote_ip));
                            }
                        }

                    }
                    else {
                        panic!("Port not found")
                    }
                }
            }
        }
    }
    connections
}
