use reqwest::blocking::{get, Response};
use scraper::{Html, Selector};
use std::io;

fn main() {
    let mut user_input = String::new();
    println!("Enter the URL of the website you want to scrape: ");
    io::stdin().read_line(&mut user_input).unwrap();

    let url: &str = user_input.trim();

    // Prompt the user for the HTML tag to scrape (e.g., "a", "img", "h1")
    println!("Enter the HTML tag to scrape (e.g., 'a', 'img', 'h1'):");
    let mut tag = String::new();
    io::stdin()
        .read_line(&mut tag)
        .expect("Failed to read tag input");
    let tag = tag.trim();

    println!("Do you want to scrape all attributes? (yes/no):");
    let mut all_attributes = String::new();
    io::stdin()
        .read_line(&mut all_attributes)
        .expect("Failed to read input");
    let all_attributes = all_attributes.trim().to_lowercase();

    // Prompt the user for the attribute to scrape (e.g., "href", "src", "class")
    let attribute = if all_attributes == "yes" {
        None // Scrape all attributes
    } else {
        // Prompt the user for the attribute to scrape (e.g., "href", "src", "class")
        println!("Enter the attribute to scrape (e.g., 'href', 'src', 'class'):");
        let mut attribute = String::new();
        io::stdin()
            .read_line(&mut attribute)
            .expect("Failed to read attribute input");
        Some(attribute.trim().to_string())
    };
    
    // Send a GET request to the URL
    let response: Response = get(url).expect("Failed to send request");
    if !response.status().is_success() {
        eprintln!("Failed to fetch the page: {}", response.status());
        return;
    }

    // Parse the response body
    let body: String = response.text().expect("Failed to read response text");

    // Parse the HTML
    let document: Html = Html::parse_document(&body);

    // Create a selector
    let selector = Selector::parse(tag).expect("Failed to parse selector");

    // Iterate over elements matching the selector
    for element in document.select(&selector) {
        if let Some(attr) = &attribute {
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
