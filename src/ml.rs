use rust_bert::pipelines::sentiment::SentimentModel;
use std::collections::HashMap;
use crate::config::MLTask;

fn predict_sentiment(text: &str) -> String {
    let sentiment_model = SentimentModel::new(Default::default()).unwrap();
    let sentiments = sentiment_model.predict(&[text]);
    format!("{:?}", sentiments[0].polarity)
}


/// Extracts entities using a Python model.
fn extract_entities(text: &str) -> Vec<String> {


    text.split_whitespace()
        .filter(|word| word.chars().next().unwrap().is_uppercase())
        .map(|word| word.to_string())
        .collect()
}

/// Processes scraped data with ML tasks.
pub fn process_with_ml(data: &[HashMap<String, String>], ml_tasks: &[MLTask]) -> Vec<HashMap<String, String>> {
    let mut processed_data = Vec::new();

    for element in data {
        let mut element_data = element.clone();

        for task in ml_tasks {
            if task.enabled {
                match task.task_type.as_str() {
                    "sentiment_analysis" => {
                        if let Some(text) = element.get("text") {
                            let sentiment = predict_sentiment(text);
                            element_data.insert("sentiment".to_string(), sentiment);
                        }
                    }
                    "entity_recognition" => {
                        if let Some(text) = element.get("text") {
                            let entities = extract_entities(text);
                            element_data.insert("entities".to_string(), entities.join(", "));
                        }
                    }
                    _ => {}
                }
            }
        }

        processed_data.push(element_data);
    }

    processed_data
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_process_with_ml_sentiment_analysis() {
        let mut data = HashMap::new();
        data.insert("text".to_string(), "I love Rust!".to_string());

        let ml_tasks = vec![
            MLTask {
                task_type: "sentiment_analysis".to_string(),
                enabled: true,
            },
        ];

        let result = process_with_ml(&[data], &ml_tasks);
        assert_eq!(result[0]["sentiment"], "positive");
    }

    #[test]
    fn test_process_with_ml_entity_recognition() {
        let mut data = HashMap::new();
        data.insert("text".to_string(), "Rust is developed by Mozilla.".to_string());

        let ml_tasks = vec![
            MLTask {
                task_type: "entity_recognition".to_string(),
                enabled: true,
            },
        ];

        let result = process_with_ml(&[data], &ml_tasks);

        assert_eq!(result[0]["entities"], "Rust, Mozilla");
    }
}