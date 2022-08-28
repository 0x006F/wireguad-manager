use crate::utils::generate_wg_keys;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, read_to_string, File},
    io::{BufRead, BufReader},
    process::exit,
};

const WIREGUARD_PATH: &str = "/home/giri/wireguard_mg";

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerProfile {
    pub public_key: String,
    private_key: String,
    pub public_ip: String,
    pub private_ip: String,
    pub port: i32,
    pub wan_interface: String,
    pub dns: Option<String>,
    #[serde(skip_deserializing)]
    pub clients: Vec<String>,
}

impl ServerProfile {
    fn generate(
        public_ip: String,
        private_ip: String,
        wan_interface: String,
        port: Option<i32>,
    ) -> ServerProfile {
        let (private_key, public_key) = generate_wg_keys();
        return {
            ServerProfile {
                public_key,
                private_key,
                public_ip,
                private_ip,
                port: port.unwrap_or(6412),
                wan_interface,
                dns: None,
                clients: vec!["".to_owned()],
            }
        };
    }

    fn persist(&self) {
        let json_string = serde_json::to_string_pretty(&self).unwrap();
        let file_write_result = fs::write(WIREGUARD_PATH.to_owned() + "/server.json", json_string);

        if file_write_result.is_err() {
            println!("Failed to write server config. Will exit");
            println!("{}", file_write_result.err().unwrap());
        }
    }

    fn rotate(&mut self) {
        let new_keys = generate_wg_keys();
        self.private_key = new_keys.0;
        self.public_key = new_keys.1;
        self.persist();
    }

    pub fn read_from_config() -> Option<ServerProfile> {
        let contents = std::fs::read_to_string(WIREGUARD_PATH.to_owned() + "/server.json");

        match contents {
            Ok(content) => {
                let mut profile: ServerProfile = serde_json::from_str(&content).unwrap();
                profile.extract_clients();
                return Some(profile);
            }
            Err(_) => {
                return None;
            }
        }
    }

    fn extract_clients(&mut self) {
        let client_identifier = "# client_id:";
        let mut config_file_path = String::new();
        config_file_path.push_str(&WIREGUARD_PATH);
        config_file_path.push_str("/");
        config_file_path.push_str(&self.wan_interface);
        config_file_path.push_str(".conf");

        let config_file = File::open(&config_file_path);
        match config_file {
            Err(_) => {
                println!("Could not read config file.");
                exit(1)
            }
            Ok(file) => {
                let config_file_reader = BufReader::new(file);
                let mut client_list: Vec<String> = Vec::new();

                for line in config_file_reader.lines() {
                    let line = line.as_ref().unwrap().trim();
                    if line.starts_with("# client_id") {
                        let parts = line
                            .chars()
                            .skip(client_identifier.len())
                            .collect::<String>();
                        client_list.push(parts.trim().to_owned());
                    }
                }
                self.clients = client_list;
            }
        }
    }

    pub fn list_clients(&self) {
        let clients = &self.clients.join(",");
        println!("Registered clients are: \n{}", clients);
    }
}
