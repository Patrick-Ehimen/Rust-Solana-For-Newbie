use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct ExchangeRateResponse {
    pub success: bool,
    pub rates: HashMap<String, f64>,
    pub error: Option<String>,
}
