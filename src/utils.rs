use reqwest::blocking::get;
use std::io;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum FetchError {
    NetworkError(reqwest::Error),
    InvalidResponse(String),
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FetchError::NetworkError(err) => write!(f, "Network error: {}", err),
            FetchError::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
        }
    }
}

impl Error for FetchError {}

/// Prompts the user for input and returns the trimmed input as a String.
pub fn prompt_user(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

/// Fetches the HTML content of a given URL.
pub fn fetch_html(url: &str) -> Result<String, FetchError> {
    // Send a GET request
    let response = get(url).map_err(FetchError::NetworkError)?;

    // Check if the request was successful
    if !response.status().is_success() {
        return Err(FetchError::InvalidResponse(format!(
            "Failed to fetch the page: {}",
            response.status()
        )));
    }

    // Read the response body as text
    let body = response.text().map_err(FetchError::NetworkError)?;
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_html_success() {
        // Use a test URL that returns a known response
        let url = "https://example.com";
        let result = fetch_html(url);

        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_fetch_html_invalid_url() {
        let url = "https://thisurldoesnotexist.com";
        let result = fetch_html(url);

        assert_eq!(result.is_err(), true);
    }
}