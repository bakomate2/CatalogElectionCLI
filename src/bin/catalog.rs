use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::Name;
use std::env;
use std::process;
use dotenv_codegen::dotenv;

fn main() {
    // Load .env variables at compile time
    let username = dotenv!("USERNAME").to_uppercase();
    let password = dotenv!("PASSWORD");

    // Check if the captcha code was provided as an argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: catalog <captcha_code>");
        process::exit(1);
    }
    let captcha_code = &args[1].to_uppercase();

    // URL of the login page
    let login_url = "https://catalog.inf.elte.hu/Account/Login";

    // Create a client to persist cookies
    let client = Client::builder()
        .cookie_store(true)
        .build()
        .expect("Failed to create reqwest client");

    // Get the login page to retrieve __AntiXsrfToken cookie
    let response = client.get(login_url).send().expect("Failed to get login page");
    let document = Document::from_read(response).expect("Failed to parse login page");

    // Extract hidden form fields
    let mut hidden_fields = std::collections::HashMap::new();
    for node in document.find(Name("input")).filter(|n| n.attr("type") == Some("hidden")) {
        if let (Some(name), Some(value)) = (node.attr("name"), node.attr("value")) {
            hidden_fields.insert(name.to_string(), value.to_string());
        }
    }

    // Form data for the login request
    let mut login_data = std::collections::HashMap::new();
    login_data.insert("ctl00$MainContent$Email", username);
    login_data.insert("ctl00$MainContent$Password", password.to_owned());
    login_data.insert("ctl00$MainContent$captcha", captcha_code.to_string());
    login_data.insert("ctl00$MainContent$ctl06", "Log+in".to_string());
    login_data.insert("__EVENTTARGET", "".to_string());
    login_data.insert("__EVENTARGUMENT", "".to_string());
    login_data.insert("__VIEWSTATE", hidden_fields.get("__VIEWSTATE").unwrap().to_string());
    login_data.insert("__VIEWSTATEGENERATOR", hidden_fields.get("__VIEWSTATEGENERATOR").unwrap().to_string());
    login_data.insert("__EVENTVALIDATION", hidden_fields.get("__EVENTVALIDATION").unwrap().to_string());

    // Send the POST request to login
    let response = client.post(login_url)
        .form(&login_data)
        .send()
        .expect("Failed to send login request");

    // Check if the login was successful
    if response.url().as_str() == "https://catalog.inf.elte.hu/Student/student" {
        let document = Document::from_read(response).expect("Failed to parse student page");
        if let Some(selected_option) = document.find(Name("option")).find(|n| n.attr("selected").is_some()) {
            println!("\x1b[92mLogin successful to Catalog ({})!\x1b[0m", selected_option.text());
        }
    } else {
        println!("\x1b[91mLogin failed to Catalog!\x1b[0m");
    }
}