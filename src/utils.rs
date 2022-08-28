use std::{
    io::{Write, self, BufRead},
    process::{Command, Stdio},
};

pub fn generate_wg_keys() -> (String, String) {
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
