use std::{
    fmt::format,
    fs::{create_dir_all, metadata, read_to_string, write},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::utils::{generate_psk, generate_wg_keys};

use super::ServerProfile;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientProfile {
    pub name: String,
    pub public_key: String,
    pub psk: String,
    private_key: String,
    pub address: String,
    pub dns: Option<String>,
    pub endpoint: String,
    server_public_key: String,
    server_endpoint: String,
    server_port: u32,
}

impl ClientProfile {
    pub fn new(name: String, server_endpoint: String, ip: String) -> Self {
        let new_keys = generate_wg_keys();
        let new_psk = generate_psk();
        let client_config = ClientProfile {
            name,
            psk: new_psk,
            private_key: new_keys.0,
            public_key: new_keys.1,
            dns: None,
            address: ip,
            endpoint: server_endpoint,
            server_endpoint: "".to_owned(),
            server_port: 51820,
            server_public_key: "".to_owned(),
        };
        return client_config;
    }
    // fn rotate_credentials(&mut self) {
    //     let new_psk = generate_psk();
    //     let new_keys = generate_wg_keys();

    //     self.psk = new_psk;
    //     self.private_key = new_keys.0;
    //     self.public_key = new_keys.1;

    //     // Save Config
    //     self.persist();
    // }

    fn save_config(&self) {
        let wireguard_install_path = "/home/giri/wireguard_mg";
        create_dir_all(format!("{}/clients/{}", &wireguard_install_path, self.name)).unwrap();

        let config_file_path = format!(
            "{}/clients/{}/{}.json",
            &wireguard_install_path, self.name, self.name
        );
        println!("{:?}", self);
        write(
            config_file_path,
            serde_json::to_string_pretty(self).unwrap(),
        )
        .unwrap()
    }

    pub fn persist(&self, server: &ServerProfile) {
        self.save_config();
        self.generate_conf(server, None);
    }

    pub fn load(client_name: String) -> Option<ClientProfile> {
        let wireguard_install_path = "/home/giri/wireguard_mg";
        let client_config_path = format!("{}/clients/{}.json", wireguard_install_path, client_name);

        let is_exist = metadata(&client_config_path).is_ok();
        if !is_exist {
            return None;
        }

        let config_contents_result = read_to_string(client_config_path);

        match config_contents_result {
            Err(_) => {
                println!("Could not load config for {}", client_name);
                return None;
            }
            Ok(contents) => {
                let jsonified = serde_json::from_str::<ClientProfile>(&contents);

                match jsonified {
                    Err(_) => {
                        println!("Was able to load config JSON for {}. But looks like that JSON is malformed.",client_name);
                        return None;
                    }
                    Ok(client_config) => return Some(client_config),
                }
            }
        }

        // let psk_path = format!(
        //     "{}/clients/{}/{}_psk",
        //     wireguard_install_path, client_name, client_name
        // );

        // let private_key_path = format!(
        //     "{}/clients/{}/{}_key",
        //     wireguard_install_path, client_name, client_name
        // );

        // let pub_key_path = format!(
        //     "{}/clients/{}/{}_pub",
        //     wireguard_install_path, client_name, client_name
        // );

        // let psk = read_to_string(psk_path).unwrap();
        // let private_key = read_to_string(private_key_path).unwrap();
        // let public_key = read_to_string(pub_key_path).unwrap();
        // return ClientProfile {
        //     name: client_name,
        //     public_key,
        //     psk,
        //     private_key,
        //     address: (),
        //     dns: (),
        //     endpoint: (),
        // };
    }

    pub fn generate_conf(&self, server: &ServerProfile, path: Option<String>) {
        let wireguard_install_path = "/home/giri/wireguard_mg";

        let conf_path = path.unwrap_or(format!(
            "{}/clients/{}/{}.conf",
            wireguard_install_path, self.name, self.name
        ));

        let mut interface_block = format!(
            "[Interface]\nPrivateKey = {}\nAddress = {}\n",
            self.private_key, self.address
        );

        if self.dns.is_some() {
            interface_block.push_str(format!("DNS = {}", &self.dns.as_ref().unwrap()).as_str());
        }

        let peer_block = format!("[Peer]\nPublicKey = {}\nPresharedKey = {}\nAllowedIPs = {}\nEndpoint = {}\nPersistentKeepAlive = 25",server.public_key,self.psk,server.vpn_cidr,format!("{}:{}",server.public_ip,server.port));

        write(conf_path, format!("{}\n\n{}", interface_block, peer_block)).unwrap();
    }
}
