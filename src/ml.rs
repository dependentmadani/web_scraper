use rust_bert::pipelines::sentiment::SentimentModel;
use whatlang::detect;
use std::collections::HashMap;
use crate::config::MLTask;

/// Predicts sentiment using a pre-trained Rust-BERT model.
fn predict_sentiment(text: &str) -> String {
    // Load the sentiment model (this should be cached after the first load)
    let sentiment_model = SentimentModel::new(Default::default()).unwrap();

    // Get the sentiment prediction
    let sentiments = sentiment_model.predict(&[text]);
    format!("{:?}", sentiments[0].polarity)
}

/// Detects the language of the text using Whatlang.
fn detect_language(text: &str) -> String {
    let info = detect(text).unwrap();
    info.lang().to_string()
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
                    "language_detection" => {
                        if let Some(text) = element.get("text") {
                            let language = detect_language(text);
                            element_data.insert("language".to_string(), language);
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

        assert_eq!(result[0]["sentiment"], "Negative");
    }

    #[test]
    fn test_process_with_ml_language_detection() {
        let mut data = HashMap::new();
        data.insert("text".to_string(), "Rust is a systems programming language.".to_string());

        let ml_tasks = vec![
            MLTask {
                task_type: "language_detection".to_string(),
                enabled: true,
            },
        ];

        let result = process_with_ml(&[data], &ml_tasks);

        assert_eq!(result[0]["language"], "English");
    }
}