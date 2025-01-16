use reqwest::blocking::Client;
use std::error::Error;

pub fn fetch_html(url: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(url).send()?;
    let html = response.text()?;
    Ok(html)
}
