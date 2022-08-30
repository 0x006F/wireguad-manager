use std::fs::{create_dir_all, read_to_string, write};

use serde::{Deserialize, Serialize};

use crate::utils::{generate_psk, generate_wg_keys};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientProfile {
    pub name: String,
    pub public_key: String,
    pub psk: String,
    private_key: String,
    pub address: String,
    pub dns: Option<String>,
    pub endpoint: String,
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
        };
        return client_config;
    }
    fn rotate_credentials(&mut self) {
        let new_psk = generate_psk();
        let new_keys = generate_wg_keys();

        self.psk = new_psk;
        self.private_key = new_keys.0;
        self.public_key = new_keys.1;

        // Save Config
        self.persist();
    }

    pub fn persist(&self) {
        let wireguard_install_path = "/home/giri/wireguard_mg";
        create_dir_all(format!("{}/clients/{}", &wireguard_install_path, self.name)).unwrap();

        // Save PSK
        let psk_path = format!(
            "{}/clients/{}/{}_psk",
            wireguard_install_path, &self.name, &self.name
        );

        write(&psk_path, &self.psk).unwrap();

        // Save Public Key
        let pub_key_path = format!(
            "{}/clients/{}/{}_pub",
            wireguard_install_path, &self.name, &self.name
        );

        write(&pub_key_path, &self.public_key).unwrap();

        // Save Private Key
        let private_key_path = format!(
            "{}/clients/{}/{}_key",
            wireguard_install_path, &self.name, &self.name
        );

        write(&private_key_path, &self.private_key).unwrap();
    }

    pub fn load(&mut self) {
        let wireguard_install_path = "/home/giri/wireguard_mg";
        let psk_path = format!(
            "{}/clients/{}/{}_psk",
            wireguard_install_path, &self.name, &self.name
        );

        let private_key_path = format!(
            "{}/clients/{}/{}_key",
            wireguard_install_path, &self.name, &self.name
        );

        let pub_key_path = format!(
            "{}/clients/{}/{}_pub",
            wireguard_install_path, &self.name, &self.name
        );

        let psk = read_to_string(psk_path).unwrap();
        let private_key = read_to_string(private_key_path).unwrap();
        let public_key = read_to_string(pub_key_path).unwrap();
        self.psk = psk;
        self.private_key = private_key;
        self.public_key = public_key;
    }
}
