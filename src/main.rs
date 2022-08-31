use models::ServerProfile;
use wireguard_manager::{load_wireguard_config, models, utils::ask};

fn main() {
    println!("Welcome to Wireguard Management App");

    'main: loop {
        let mut server_config = load_wireguard_config();
        let command = ask("select an option");
        match command.as_str() {
            "add_client" => {
                if server_config.is_none() {
                    println!("Looks like there's no active server configuration. Run \"init\" to create new config");
                    continue;
                } else {
                    let mut server_config = server_config.unwrap();
                    let client_name = ask("What's the client name?");
                    server_config.register_client(client_name);
                }
            }
            "remove_client" => {
                if server_config.is_none() {
                    println!("Looks like there's no active server configuration. Run \"init\" to create new config");
                    continue;
                } else {
                    let mut server_config = server_config.unwrap();
                    let client_name = ask("What's the client name?");
                    server_config.unregister_client(client_name);
                }
            }
            "init" => {
                if server_config.is_some() {
                    loop {
                        let should_continue = ask("There's already some server config defined. Running init will overwrite everything. Continue? (y/n)");
                        if should_continue == "n" {
                            continue 'main;
                        }
                        break;
                    }
                }
                println!("We will walk you through the creation of new Wireguard Server config");
                let public_ip = ask("What's the public IP of this server?");
                let private_ip = ask("What should be the private IP for this VPN subnet?");
                let vpn_cidr = ask("What is the CIDR for this current private network?");
                let wan_interface = ask("What's the network interface name which the connects this machine to the internet?");
                let port = ask("What should be the Wireguard Server port?");
                let default_dns = ask("Do you want to set up a custom DNS for this VPN clients? If yes, enter the DNS address");
                let wg_interface = ask("Please choose a name for Wireguard interface");
                let port = if port.is_empty() {
                    None
                } else {
                    let port = port.parse::<u32>();
                    match port {
                        Err(_) => None,
                        Ok(port) => Some(port),
                    }
                };
                let dns = if default_dns.is_empty() {
                    None
                } else {
                    Some(default_dns)
                };
                server_config = Some(ServerProfile::new(
                    public_ip,
                    private_ip,
                    wan_interface,
                    port,
                    dns,
                    None,
                    &wg_interface,
                    vpn_cidr,
                ));
                server_config.as_ref().unwrap().rebuild_config();
                println!("Successfully initialized server configuration.");
                println!("Please run \"systemctl enable wg-quick@{}\" and \"systemctl restart wg-quick@{}\" take the changes into effect",wg_interface,wg_interface);
            }

            "exit" => break,
            _ => {
                println!("Unrecognized input {}. Try again", command);
            }
        }
    }

    // if command == "install" {
    //     install_wireguard();
    // }

    // let test: UserProfile = serde_json::from_value(json!({"name":"asd"})).unwrap();
    // println!("{:?}", test);

    // let server_config = read_server_config();
    // if server_config.is_none() {
    //     println!("Could not read server config. Exiting..");
    //     std::process::exit(1);
    // }

    // let c = ServerProfile::generate(
    //     "100.100.100.100".to_owned(),
    //     "100.100.100.100".to_owned(),
    //     "ens5".to_owned(),
    //     None,
    // );

    // c.persist();

    // if command == "add" {
    //     let profile_name = ask("What is is the name of the user?");
    // }

    // let config = ServerProfile::read_from_config("/home/giri/wireguard_mg".to_owned());

    // if config.is_some() {
    //     let mut config = config.unwrap();

    //     config.list_clients();
    //     config.rebuild_config();
    //     config.port = 9874;
    //     config.rotate();
    // }

    // let mut config = ServerProfile::new(
    //     "192.168.1.1".to_owned(),
    //     "192.168.1.3".to_owned(),
    //     "eth0".to_owned(),
    //     None,
    //     None,
    //     None,
    // );

    // // let mut config = ServerProfile::read_from_config("/home/giri/wireguard_mg".to_owned()).unwrap();
    // let giri = config.register_client("giri".to_owned());
    // let ammu = config.register_client("ammu".to_owned());
}
