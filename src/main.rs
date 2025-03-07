use reqwest::blocking::{get, Response};
use scraper::{Html, Selector};
use std::io;

fn main() {
    let mut user_input = String::new();
    println!("Enter the URL of the website you want to scrape: ");
    io::stdin().read_line(&mut user_input).unwrap();

    let url: &str = user_input.trim();
    
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
    let selector: Selector = Selector::parse("div").expect("Failed to create selector");

    // Iterate over elements matching the selector
    for element in document.select(&selector) {
        // Extract and print the text content of the element
        let text: String = element.text().collect::<Vec<_>>().join(" ");
        println!("Found div: {}", text);
    }
}
