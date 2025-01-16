use crate::models::ExchangeRateResponse;
use reqwest::blocking::get;
use reqwest::Error;
use std::fmt;
use std::fs::File;
use std::io::Write;

const API_URL: &str =
    "http://api.exchangerate.host/live?access_key=e6076de5058dd98193c344aee6c57cf2";

pub fn fetch_exchange_rate(base: &str, symbols: &str) -> Result<ExchangeRateResponse, FetchError> {
    let url = format!("{}&base={}&symbols={}", API_URL, base, symbols);
    let response_text = get(&url)?.text()?;
    println!("Response body: {}", response_text); // Debug print

    // Save the response body to a JSON file
    let mut file = File::create("response.json")?;
    file.write_all(response_text.as_bytes())?;

    let response = serde_json::from_str::<ExchangeRateResponse>(&response_text)?;
    Ok(response)
}

#[derive(Debug)]
pub enum FetchError {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FetchError::Reqwest(e) => write!(f, "Request error: {}", e),
            FetchError::Serde(e) => write!(f, "Serialization error: {}", e),
            FetchError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl From<reqwest::Error> for FetchError {
    fn from(error: reqwest::Error) -> Self {
        FetchError::Reqwest(error)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(error: serde_json::Error) -> Self {
        FetchError::Serde(error)
    }
}

impl From<std::io::Error> for FetchError {
    fn from(error: std::io::Error) -> Self {
        FetchError::Io(error)
    }
}
