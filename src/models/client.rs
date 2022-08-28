use serde::{Deserialize, Serialize};

use crate::utils::{generate_psk, generate_wg_keys};

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientProfile {
    pub public_key: String,
    private_key: String,
    pub address: String,
    pub dns: Option<String>,
    pub endpoint: String,
    preshared_key: String,
}

impl ClientProfile {
    fn new(name: String, ip: String, endpoint: String, dns: Option<String>) -> ClientProfile {
        let keys = generate_wg_keys();
        let psk = generate_psk();

        return ClientProfile {
            public_key: keys.1,
            private_key: keys.0,
            endpoint,
            dns,
            preshared_key: psk,
            address: ip,
        };
    }
}
