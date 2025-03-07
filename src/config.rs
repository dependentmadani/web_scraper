use crate::utils;

use serde_json::Value;
use std::fs::File;
use std::path::Path;

/// Represents a tag and its attributes to scrape.
#[derive(Debug)]
pub struct TagConfig {
    pub name: String,
    pub attributes: Vec<String>,
}

/// Represents the configuration for web scraping.
#[derive(Debug)]
pub struct Config {
    pub url: String,
    pub tags: Vec<TagConfig>,
}

/// Reads the configuration from `config.json` or prompts the user for input.
pub fn get_config() -> Config {
    if Path::new("config.json").exists() {
        // Read configuration from `config.json`
        let file = File::open("config.json").expect("Failed to open config.json");
        let config: Value = serde_json::from_reader(file).expect("Failed to parse config.json");

        let url = config["url"].as_str().expect("Missing 'url' in config.json").to_string();
        let tags = config["tags"]
            .as_array()
            .expect("Missing 'tags' in config.json")
            .iter()
            .map(|tag| {
                let name = tag["name"].as_str().expect("Missing 'name' in tag config").to_string();
                let attributes = tag["attributes"]
                    .as_array()
                    .expect("Missing 'attributes' in tag config")
                    .iter()
                    .map(|attr| attr.as_str().expect("Invalid attribute").to_string())
                    .collect();
                TagConfig { name, attributes }
            })
            .collect();

        Config { url, tags }
    } else {
        // Prompt the user for input
        let url = utils::prompt_user("Enter the URL to scrape:");
        let mut tags = Vec::new();

        loop {
            let tag_name = utils::prompt_user("Enter the HTML tag to scrape (e.g., 'a', 'img', 'h1'):");
            let attributes = utils::prompt_user("Enter the attributes to scrape (comma-separated, e.g., 'href,class'):")
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();

            tags.push(TagConfig {
                name: tag_name,
                attributes,
            });

            let more_tags = utils::prompt_user("Do you want to add another tag? (yes/no):");
            if more_tags.to_lowercase() != "yes" {
                break;
            }
        }

        Config { url, tags }
    }
}