use std::fs::write;

use serde::{Deserialize, Serialize};

use crate::utils::{generate_psk, generate_wg_keys};

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientProfile {
    pub name: String,
    pub public_key: String,
    private_key: String,
    pub address: String,
    pub dns: Option<String>,
    pub endpoint: String,
    preshared_key: String,
    config_path: Option<String>,
}

impl ClientProfile {
    fn rotate_credentials(&mut self) {
        let new_psk = generate_psk();
        let new_keys = generate_wg_keys();

        self.preshared_key = new_psk;
        self.private_key = new_keys.0;
        self.public_key = new_keys.1;

        // Save Config
        self.persist();
    }

    fn persist(&self) {
        if self.config_path.is_none() {
            println!("WARNING: Could not save config details for {}", &self.name);
        }

        let config_path = self.config_path.as_ref().unwrap().clone();

        // Save PSK
        let mut psk_file_name = config_path.clone();
        psk_file_name.push_str(&self.name);
        psk_file_name.push_str("_psk");

        write(&psk_file_name, &self.preshared_key).unwrap();

        // Save Public Key
        let mut public_key_path = config_path.clone();
        public_key_path.push_str(&self.name);
        public_key_path.push_str("_pub");

        write(&public_key_path, &self.preshared_key).unwrap();

        // Save Private Key
        let mut private_key_path = config_path.clone();
        private_key_path.push_str(&self.name);
        private_key_path.push_str("_key");

        write(&private_key_path, &self.preshared_key).unwrap();
    }
}
