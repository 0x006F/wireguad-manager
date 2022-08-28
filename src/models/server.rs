use crate::utils::generate_wg_keys;
use serde::{Deserialize, Serialize};
use std::fs;

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
                return serde_json::from_str(&content).unwrap();
            }
            Err(_) => {
                return None;
            }
        }
    }
}
