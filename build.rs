use std::fs;
use std::io::{self, BufRead};
use std::process::exit;

fn main() {
    // Check if .env file exists
    if fs::metadata(".env").is_err() {
        eprint!("Error: .env file not found");
        exit(1);
    }

    // Read the .env file
    let file = fs::File::open(".env").expect("Error: Unable to open .env file");
    let reader = io::BufReader::new(file);

    // Check for USERNAME and PASSWORD
    let mut username_found = false;
    let mut password_found = false;
    let mut username = None;

    for line in reader.lines() {
        let line = line.expect("Error: Unable to read line from .env file");
        if line.starts_with("USERNAME=") {
            username_found = true;
            username = Some(line.split("=").collect::<Vec<&str>>()[1].to_string());
        }
        if line.starts_with("PASSWORD=") {
            password_found = true;
        }
    }

    // Check if USERNAME and PASSWORD were found
    if !username_found {
        eprint!("Error: USERNAME not set in .env file");
        exit(1);
    }
    if !password_found {
        eprint!("Error: PASSWORD not set in .env file");
        exit(1);
    }

    // Check USERNAME constraints
    let username = username.unwrap();
    if username.chars().count() != 6 {
        eprint!("Error: USERNAME must be 6 characters long");
        exit(1);
    }
    if username.chars().any(|c| !c.is_ascii_alphanumeric()) {
        eprint!("Error: USERNAME must contain only (ascii) alphanumeric characters");
        exit(1);
    }

    println!("cargo:rerun-if-changed=.env");
}