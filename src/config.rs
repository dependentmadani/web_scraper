use crate::utils;

use serde_json::Value;
use std::fs::File;
use std::path::Path;

// Represents an ML task configuration.
#[derive(Debug)]
pub struct MLTask {
    pub task_type: String,
    pub enabled: bool,
}

// Represents a tag and its attributes to scrape.
#[derive(Debug)]
pub struct TagConfig {
    pub name: String,
    pub attributes: Vec<String>,
}

// Represents the configuration for web scraping.
#[derive(Debug)]
pub struct Config {
    pub url: String,
    pub tags: Vec<TagConfig>,
    pub ml_tasks: Vec<MLTask>,
}

// Reads the configuration from `config.json` or prompts the user for input.
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

        let ml_tasks = config["ml_tasks"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|task| {
                let task_type = task["type"].as_str().expect("Missing 'type' in ML task config").to_string();
                let enabled = task["enabled"].as_bool().unwrap_or(false);
                MLTask { task_type, enabled }
            })
            .collect();

        Config { url, tags, ml_tasks }
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

        // Prompt the user for ML tasks
        let mut ml_tasks = Vec::new();
        let enable_sentiment = utils::prompt_user("Do you want to enable sentiment analysis? (yes/no):");
        if enable_sentiment.to_lowercase() == "yes" {
            ml_tasks.push(MLTask {
                task_type: "sentiment_analysis".to_string(),
                enabled: true,
            });
        }

        let enable_ner = utils::prompt_user("Do you want to enable language detection? (yes/no):");
        if enable_ner.to_lowercase() == "yes" {
            ml_tasks.push(MLTask {
                task_type: "language_detection".to_string(),
                enabled: true,
            });
        }

        Config { url, tags, ml_tasks }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_get_config_from_file() {
        // Create a temporary config.json file
        let config_json = r#"
        {
            "url": "https://example.com",
            "tags": [
                {
                    "name": "a",
                    "attributes": ["href", "class"]
                }
            ],
            "ml_tasks": [
                {
                    "type": "sentiment_analysis",
                    "enabled": true
                }
            ]
        }
        "#;

        let mut file = File::create("config.json").unwrap();
        file.write_all(config_json.as_bytes()).unwrap();

        // Test the function
        let config = get_config();

        assert_eq!(config.url, "https://example.com");
        assert_eq!(config.tags.len(), 1);
        assert_eq!(config.tags[0].name, "a");
        assert_eq!(config.tags[0].attributes, vec!["href", "class"]);
        assert_eq!(config.ml_tasks.len(), 1);
        assert_eq!(config.ml_tasks[0].task_type, "sentiment_analysis");
        assert!(config.ml_tasks[0].enabled);

        // Clean up
        std::fs::remove_file("config.json").unwrap();
    }
}