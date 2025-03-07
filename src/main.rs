mod config;
mod scraping;
mod utils;

use crate::config::get_config;
use crate::scraping::{scrape_attributes_and_content, display_in_terminal, save_to_json};
use scraper::Html;

fn main() {
    // Get configuration from `config.json` or user input
    let config = get_config();

    // Fetch the HTML content of the page
    let body = utils::fetch_html(&config.url).expect("Failed to fetch HTML content");

    // Parse the HTML content
    let document = Html::parse_document(&body);

    // Scrape data for each tag
    let mut all_scraped_data = Vec::new();
    for tag_config in &config.tags {
        let scraped_data = scrape_attributes_and_content(&document, &tag_config.name, &tag_config.attributes);
        all_scraped_data.extend(scraped_data);
    }

    // Ask the user how they want to view the data
    println!("How do you want to see the scrapped data?");
    println!("1. Terminal");
    println!("2. JSON file");
    let choice = utils::prompt_user("Enter your choice (1 or 2):");

    match choice.as_str() {
        "1" => {
            // Display the data in the terminal
            display_in_terminal(&all_scraped_data);
        }
        "2" => {
            // Save the data to a JSON file
            let filename = utils::prompt_user("Enter the filename to save the JSON data (e.g., output.json):");
            save_to_json(&all_scraped_data, &filename).expect("Failed to save JSON file");
        }
        _ => {
            eprintln!("Invalid choice. Please enter 1 or 2.");
            std::process::exit(1);
        }
    }
}