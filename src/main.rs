use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::io::{self, BufRead, Read, Write};
use std::process::{Command, Stdio};

const WIREGUARD_PATH: &str = "/home/giri/wireguard_mg";

#[derive(Serialize, Deserialize, Debug)]
struct UserProfile {
    public_key: String,
    private_key: String,
    address: String,
    dns: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct ServerProfile {
    public_key: String,
    private_key: String,
    public_ip: String,
    private_ip: String,
    port: i32,
    wan_interface: String,
}

fn read_server_config() -> Option<ServerProfile> {
    let contents = std::fs::read_to_string("/etc/wireguard/server.json");

    match contents {
        Ok(content) => {
            return serde_json::from_str(&content).unwrap();
        }
        Err(_) => {
            return None;
        }
    }
}

impl ServerProfile {
    fn generate(
        public_ip: String,
        private_ip: String,
        wan_interface: String,
        port: Option<i32>,
    ) -> ServerProfile {
        let private_key = std::process::Command::new("wg")
            .arg("genkey")
            .output()
            .unwrap();
        let private_key = String::from_utf8(private_key.stdout).unwrap();
        let mut public_key = Command::new("wg")
            .arg("pubkey")
            .stderr(Stdio::null())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        public_key
            .stdin
            .take()
            .unwrap()
            .write_all(private_key.as_bytes())
            .unwrap();

        // let output = public_key.wait_with_output().unwrap();
        let public_key = String::from_utf8(public_key.wait_with_output().unwrap().stdout).unwrap();
        println!("private: {}", private_key);
        println!("public: {}", public_key);
        return {
            ServerProfile {
                public_key,
                private_key,
                public_ip,
                private_ip,
                port: port.unwrap_or(6412),
                wan_interface,
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
}

fn ask(question: &str) -> String {
    print!("{}: ", question);
    io::stdout().flush().unwrap();
    let mut answer_string = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut answer_string).unwrap();
    return answer_string.trim().to_string();
}

fn main() {
    println!("Welcome to Wireguard Management App");
    let command = ask("select an option");
    if command == "install" {
        let cmd_output = Command::new("apt")
            .arg("install")
            .arg("wireguard")
            .output()
            .unwrap();
        if !&cmd_output.status.success() {
            print!("There was something went wrong with install.");
            println!("{}", String::from_utf8_lossy(&cmd_output.stderr));
            return;
        } else {
            println!("Wireguard installed successfully");
            return;
        }
    }

    // let test: UserProfile = serde_json::from_value(json!({"name":"asd"})).unwrap();
    // println!("{:?}", test);

    // let server_config = read_server_config();
    // if server_config.is_none() {
    //     println!("Could not read server config. Exiting..");
    //     std::process::exit(1);
    // }

    let c = ServerProfile::generate(
        "100.100.100.100".to_owned(),
        "100.100.100.100".to_owned(),
        "ens5".to_owned(),
        None,
    );

    c.persist();

    // if command == "add" {
    //     let profile_name = ask("What is is the name of the user?");
    // }
}
