pub mod models;
pub mod utils;

use models::ServerProfile;

pub fn load_wireguard_config() -> Option<ServerProfile> {
    ServerProfile::read_from_config("/etc/wireguard".to_owned())
}
