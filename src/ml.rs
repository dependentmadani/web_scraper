use pyo3::prelude::*;
use std::collections::HashMap;
use std::ffi::CString;
use crate::config::MLTask;

// Predicts sentiment using a Python model.
fn predict_sentiment(text: &str) -> PyResult<String> {
    Python::with_gil(|py| {
        let predict_sentiment: Py<PyAny> = PyModule::from_code(
            py,
            &CString::new(r#"
            import joblib

            def predict_sentiment(text):
                model = joblib.load("sentiment_model.pkl")
                return model.predict([text])[0]
            "#).unwrap(),
            &CString::new("predict_sentiment.py").unwrap(),
            &CString::new("predict_sentiment").unwrap(),
        )?
        .getattr("predict_sentiment")?
        .into();

        let result: String = predict_sentiment.call1(py, (text,))?.extract(py)?;
        Ok(result)
    })
}

// Extracts entities using a Python model.
fn extract_entities(text: &str) -> PyResult<Vec<String>> {
    Python::with_gil(|py| {
        let extract_entities: Py<PyAny> = PyModule::from_code(
            py,
            &CString::new(r#"
            import spacy

            nlp = spacy.load("en_core_web_sm")

            def extract_entities(text):
                doc = nlp(text)
                return [ent.text for ent in doc.ents]
            "#).unwrap(),
            &CString::new("extract_entities.py").unwrap(),
            &CString::new("extract_entities").unwrap(),
        )?
        .getattr("extract_entities")?
        .into();

        let result: Vec<String> = extract_entities.call1(py, (text,))?.extract(py)?;
        Ok(result)
    })
}

// Processes scraped data with ML tasks.
pub fn process_with_ml(data: &[HashMap<String, String>], ml_tasks: &[MLTask]) -> Vec<HashMap<String, String>> {
    let mut processed_data = Vec::new();

    for element in data {
        let mut element_data = element.clone();

        for task in ml_tasks {
            if task.enabled {
                match task.task_type.as_str() {
                    "sentiment_analysis" => {
                        if let Some(text) = element.get("text") {
                            let sentiment = predict_sentiment(text).unwrap_or_else(|_| "unknown".to_string());
                            element_data.insert("sentiment".to_string(), sentiment);
                        }
                    }
                    "entity_recognition" => {
                        if let Some(text) = element.get("text") {
                            let entities = extract_entities(text).unwrap_or_else(|_| Vec::new());
                            println!("{:?}", entities);
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