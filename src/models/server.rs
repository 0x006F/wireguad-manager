use crate::utils::generate_wg_keys;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, read_to_string, File},
    process::exit,
};

use super::ClientProfile;

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
    pub clients: Option<Vec<ClientProfile>>,
}

impl ServerProfile {
    pub fn new(
        public_ip: String,
        private_ip: String,
        wan_interface: String,
        port: Option<i32>,
        default_dns: Option<String>,
        clients: Option<Vec<ClientProfile>>,
    ) -> ServerProfile {
        let (private_key, public_key) = generate_wg_keys();
        let config = ServerProfile {
            public_key,
            private_key,
            public_ip,
            private_ip,
            port: port.unwrap_or(51820),
            wan_interface,
            dns: default_dns,
            clients,
        };
        config.persist(None);
        return config;
    }

    fn persist(&self, wireguard_path: Option<String>) {
        let wireguard_install_path = wireguard_path.unwrap_or("/home/giri/wireguard_mg".to_owned());
        let json_string = serde_json::to_string_pretty(&self).unwrap();
        let file_write_result =
            fs::write(format!("{}/conf.json", wireguard_install_path), json_string);

        if file_write_result.is_err() {
            println!("Failed to write server config. Will exit");
            println!("{}", file_write_result.err().unwrap());
        }
    }

    pub fn rotate(&mut self) {
        let new_keys = generate_wg_keys();
        self.private_key = new_keys.0;
        self.public_key = new_keys.1;
        self.persist(None);
    }

    pub fn read_from_config(wireguard_path: String) -> Option<ServerProfile> {
        let conf_json_path = format!("{}/conf.json", wireguard_path);
        let contents = std::fs::read_to_string(conf_json_path);

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
        config_file_path.push_str("conf");
        config_file_path.push_str(".json");

        let config_file = File::open(&config_file_path);
        match config_file {
            Err(_) => {
                println!("Could not read config file.");
                exit(1)
            }
            Ok(file) => {
                // let config_file_reader = BufReader::new(file);
                // let mut client_list: Vec<String> = Vec::new();

                // for line in config_file_reader.lines() {
                //     let line = line.as_ref().unwrap().trim();
                //     if line.starts_with("# client_id") {
                //         let parts = line
                //             .chars()
                //             .skip(client_identifier.len())
                //             .collect::<String>();
                //         client_list.push(parts.trim().to_owned());
                //     }
                // }
                // self.clients = client_list;
            }
        }
    }

    pub fn list_clients(&self) {
        println!("Registered clients are");
    }

    pub fn rebuild_config(&self) {
        let config_path = WIREGUARD_PATH.to_owned() + "/conf.json";
        let config_contents = read_to_string(config_path);

        match config_contents {
            Err(err) => {
                println!("Could not load config JSON. {}", err.to_string());
                exit(1)
            }
            Ok(content) => {
                let mut profile: ServerProfile = serde_json::from_str(&content).unwrap();
                profile.extract_clients();

                let clients_block = &profile
                    .clients
                    .unwrap_or(vec![])
                    .iter()
                    .map(|x| {
                        let mut client_line = String::new();
                        client_line.push_str(format!("# client_id: {}\n", x.name.trim()).as_str());
                        client_line.push_str("[Peer]\n");
                        client_line.push_str(format!("PublicKey = {}\n", x.public_key.trim()).as_str());
                        client_line.push_str(format!("PresharedKey = {}\n", x.psk.trim()).as_str());
                        client_line.push_str(format!("AllowedIPs = {}\n", x.address.trim()).as_str());
                        return client_line;
                    })
                    .collect::<Vec<String>>()
                    .join("\n");

                let mut interface_block = String::new();
                interface_block.push_str(format!("Address = {}\n", profile.private_ip).as_str());
                interface_block.push_str("SaveConfig = true\n");
                interface_block
                    .push_str(format!("PrivateKey = {}\n", profile.private_key).as_str());
                interface_block.push_str(format!("ListenPort = {}\n", profile.port).as_str());
                interface_block.push_str(format!("PostUp = iptables -A FORWARD -i {} -j ACCEPT; iptables -t nat -A POSTROUTING -o {} -j MASQUERADE\n",profile.wan_interface, profile.private_key).as_str());
                interface_block.push_str(format!("PostDown = iptables -D FORWARD -i {} -j ACCEPT; iptables -t nat -D POSTROUTING -o {} -j MASQUERADE\n",profile.wan_interface, profile.private_key).as_str());
                println!("{}", &clients_block);

                let final_string = format!("[Interface]\n{}\n\n{}", interface_block, clients_block);
                std::fs::write(
                    format!("{}/{}.conf", &WIREGUARD_PATH, profile.wan_interface),
                    final_string,
                )
                .unwrap();
            }
        }
    }

    pub fn register_client(&mut self, client_name: String) {
        let client_config = ClientProfile::new(
            client_name,
            format!("{}:{}", &self.public_ip, &self.port),
            "ip".to_owned(),
        );

        let clients = &self.clients.clone();
        match clients {
            None => {
                let new_client_vec = vec![client_config];
                self.clients = Some(new_client_vec);
            }
            Some(clients) => {
                let mut clients = clients.clone();
                clients.push(client_config);
                self.clients = Some(clients.to_vec());
            }
        }
        self.persist(None);
        self.rebuild_config();
    }
}
