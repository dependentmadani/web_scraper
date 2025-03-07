use scraper::{Html, Selector};
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};

/// Scrapes attributes and text content, returning them as a vector of HashMaps.
pub fn scrape_attributes_and_content(document: &Html, tag: &str, attributes: &[String]) -> Vec<HashMap<String, String>> {
    let selector = Selector::parse(tag).expect("Failed to parse selector");
    let mut results = Vec::new();

    for element in document.select(&selector) {
        let mut element_data = HashMap::new();

        // Scrape specified attributes
        for attr in attributes {
            if let Some(attr_value) = element.value().attr(attr) {
                element_data.insert(attr.to_string(), attr_value.to_string());
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

/// Displays the scraped data in the terminal.
pub fn display_in_terminal(data: &[HashMap<String, String>]) {
    for (index, element_data) in data.iter().enumerate() {
        println!("Element {}:", index + 1);
        for (key, value) in element_data {
            println!("  {}: {}", key, value);
        }
        println!();
    }
}

/// Saves the scraped data to a JSON file with pretty formatting.
pub fn save_to_json(data: &[HashMap<String, String>], filename: &str) -> io::Result<()> {
    let json_data = json!(data);
    let pretty_json = serde_json::to_string_pretty(&json_data)?; // Pretty-print JSON
    let mut file = File::create(filename)?;
    file.write_all(pretty_json.as_bytes())?;
    println!("Data saved to {}", filename);
    Ok(())
}