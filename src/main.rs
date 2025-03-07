use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::io;

// Prompts the user for input and returns the trimmed input as a String.
fn prompt_user(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input) 
        .expect("Failed to read input");
    input.trim().to_string()
}

// Fetches the HTML content of a given URL.
fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = get(url)?;
    if !response.status().is_success() {
        eprintln!("Failed to fetch the page: {}", response.status());
        std::process::exit(1);
    }
    response.text()
}

// Scrapes and prints attributes based on user input.
fn scrape_attributes(document: &Html, tag: &str, attribute: Option<&str>) {
    let selector = Selector::parse(tag).expect("Failed to parse selector");

    for element in document.select(&selector) {
        if let Some(attr) = attribute {
            // Scrape a specific attribute
            if let Some(attr_value) = element.value().attr(attr) {
                println!("Found {}: {}", attr, attr_value);
            }
        } else {
            // Scrape all attributes
            for (attr_name, attr_value) in element.value().attrs() {
                println!("Found attribute {}: {}", attr_name, attr_value);
            }
        }
    }
}

fn main() {
    // Prompt the user for the URL to scrape
    let url = prompt_user("Enter the URL to scrape:");

    // Prompt the user for the HTML tag to scrape (e.g., "a", "img", "h1")
    let tag = prompt_user("Enter the HTML tag to scrape (e.g., 'a', 'img', 'h1'):");

    // Ask the user if they want to scrape all attributes or a specific one
    let all_attributes = prompt_user("Do you want to scrape all attributes? (yes/no):");
    let all_attributes = all_attributes.to_lowercase();

    let attribute = if all_attributes == "yes" {
        None // Scrape all attributes
    } else {
        // Prompt the user for the attribute to scrape (e.g., "href", "src", "class")
        Some(prompt_user("Enter the attribute to scrape (e.g., 'href', 'src', 'class'):"))
    };

    // Fetch the HTML content of the page
    let body = fetch_html(&url).expect("Failed to fetch HTML content");

    // Parse the HTML content
    let document = Html::parse_document(&body);

    // Scrape and print the attributes
    scrape_attributes(&document, &tag, attribute.as_deref());
}