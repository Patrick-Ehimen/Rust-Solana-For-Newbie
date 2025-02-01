use crate::api;
use crate::models::ExchangeRateResponse;

/// A simple currency converter.
pub struct CurrencyConverter;

impl CurrencyConverter {
    /// Create a new instance of the currency converter.
    pub fn new() -> Self {
        CurrencyConverter
    }

    /// Convert a given amount of money from one currency to another.
    ///
    /// # Errors
    ///
    /// Returns an error if either the from or to currency is not supported,
    /// or if there is a problem fetching the exchange rate.
    pub fn convert(&self, from: &str, to: &str, amount: f64) -> Result<f64, String> {
        let rate = self.get_exchange_rate(from, to)?;
        Ok(amount * rate)
    }

    /// Fetch the exchange rate from the given from currency to the given to currency.
    ///
    /// # Errors
    ///
    /// Returns an error if there is a problem fetching the exchange rate, or if
    /// the target currency is not supported.
    fn get_exchange_rate(&self, from: &str, to: &str) -> Result<f64, String> {
        let response: ExchangeRateResponse = api::fetch_exchange_rate(from, to)
            .map_err(|e| format!("Failed to fetch exchange rate: {}", e))?;

        if response.success {
            response
                .quotes
                .get(to)
                .cloned()
                .ok_or_else(|| "Target currency not found.".to_string())
        } else {
            Err(response
                .error
                .unwrap_or_else(|| "Unknown error".to_string()))
        }
    }
}
