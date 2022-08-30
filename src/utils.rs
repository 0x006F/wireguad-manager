use std::{
    io::{self, BufRead, Write},
    process::{Command, Stdio},
};

pub fn generate_wg_keys() -> (String, String) {
    let private_key = std::process::Command::new("wg")
        .arg("genkey")
        .output()
        .unwrap();
    let private_key = String::from_utf8(private_key.stdout)
        .unwrap()
        .trim()
        .to_string();
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

    let public_key = String::from_utf8(public_key.wait_with_output().unwrap().stdout)
        .unwrap()
        .trim()
        .to_string();
    return (private_key, public_key);
}

pub fn ask(question: &str) -> String {
    print!("{}: ", question);
    io::stdout().flush().unwrap();
    let mut answer_string = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut answer_string).unwrap();
    return answer_string.trim().to_string();
}

pub fn generate_psk() -> String {
    let psk = std::process::Command::new("wg")
        .arg("genpsk")
        .output()
        .unwrap();
    let psk = String::from_utf8(psk.stdout).unwrap().trim().to_owned();
    return psk;
}
