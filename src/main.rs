use reqwest::blocking::get;
use scraper::{Html, Selector};
use serde_json::json;
use std::fs::File;
use std::io::{self, Write};
use std::collections::HashMap;

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

// Scrapes attributes and text content, returning them as a vector of HashMaps.
fn scrape_attributes_and_content(document: &Html, tag: &str, attribute: Option<&str>) -> Vec<HashMap<String, String>> {
    let selector = Selector::parse(tag).expect("Failed to parse selector");
    let mut results = Vec::new();

    for element in document.select(&selector) {
        let mut element_data = HashMap::new();

        // Scrape attributes
        if let Some(attr) = attribute {
            // Scrape a specific attribute
            if let Some(attr_value) = element.value().attr(attr) {
                element_data.insert(attr.to_string(), attr_value.to_string());
            }
        } else {
            // Scrape all attributes
            for (attr_name, attr_value) in element.value().attrs() {
                element_data.insert(attr_name.to_string(), attr_value.to_string());
            }
        }

        // Scrape text content
        let text_content = element.text().collect::<Vec<_>>().join(" ").trim().to_string();
        if !text_content.is_empty() {
            element_data.insert("text".to_string(), text_content);
        }

        if !element_data.is_empty() {
            results.push(element_data);
        }
    }

    results
}

// Displays the scraped data in the terminal.
fn display_in_terminal(data: &[HashMap<String, String>]) {
    for (index, element_data) in data.iter().enumerate() {
        println!("Element {}:", index + 1);
        for (key, value) in element_data {
            println!("  {}: {}", key, value);
        }
        println!();
    }
}

// Saves the scraped data to a JSON file with pretty formatting.
fn save_to_json(data: &[HashMap<String, String>], filename: &str) -> io::Result<()> {
    let json_data = json!(data);
    let pretty_json = serde_json::to_string_pretty(&json_data)?; // Pretty-print JSON
    let mut file = File::create(filename)?;
    file.write_all(pretty_json.as_bytes())?;
    println!("Data saved to {}", filename);
    Ok(())
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
        None
    } else {
        // Prompt the user for the attribute to scrape (e.g., "href", "src", "class")
        Some(prompt_user("Enter the attribute to scrape (e.g., 'href', 'src', 'class'):"))
    };

    // Fetch the HTML content of the page
    let body = fetch_html(&url).expect("Failed to fetch HTML content");

    // Parse the HTML content
    let document = Html::parse_document(&body);

    // Scrape the attributes and text content
    let scraped_data = scrape_attributes_and_content(&document, &tag, attribute.as_deref());

    // Ask the user how they want to view the data
    println!("How do you want to see the scrapped data?");
    println!("1. Terminal");
    println!("2. JSON file");
    let choice = prompt_user("Enter your choice (1 or 2):");

    match choice.as_str() {
        "1" => {
            // Display the data in the terminal
            display_in_terminal(&scraped_data);
        }
        "2" => {
            // Save the data to a JSON file
            let filename = prompt_user("Enter the filename to save the JSON data (e.g., output.json):");
            save_to_json(&scraped_data, &filename).expect("Failed to save JSON file");
        }
        _ => {
            eprintln!("Invalid choice. Please enter 1 or 2.");
            std::process::exit(1);
        }
    }
}