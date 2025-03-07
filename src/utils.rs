use reqwest::blocking::get;
use std::io;

/// Prompts the user for input and returns the trimmed input as a String.
pub fn prompt_user(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

/// Fetches the HTML content of a given URL.
pub fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = get(url)?;
    if !response.status().is_success() {
        eprintln!("Failed to fetch the page: {}", response.status());
        std::process::exit(1);
    }
    response.text()
}