use crate::models::ExchangeRateResponse;
use reqwest::blocking::get;

const API_URL: &str =
    "http://api.exchangerate.host/live?access_key=e6076de5058dd98193c344aee6c57cf2";

pub fn fetch_exchange_rate(
    base: &str,
    symbols: &str,
) -> Result<ExchangeRateResponse, reqwest::Error> {
    let url = format!("{}&base={}&symbols={}", API_URL, base, symbols);
    let response = get(&url)?.json::<ExchangeRateResponse>()?;
    Ok(response)
}
