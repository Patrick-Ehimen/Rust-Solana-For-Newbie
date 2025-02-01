use crate::api;
use crate::models::ExchangeRateResponse;
use std::collections::HashMap;

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
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_convert_success() {
            let converter = CurrencyConverter::new();
            let from = "USD";
            let to = "EUR";
            let amount = 100.0;

            // Mock the API response
            let mut quotes = HashMap::new();
            quotes.insert(to.to_string(), 0.85);
            let response = ExchangeRateResponse {
                success: true,
                quotes,
                error: None,
            };

            api::mock_fetch_exchange_rate(from, to, Ok(response));

            let result = converter.convert(from, to, amount);
            assert_eq!(result.unwrap(), 85.0);
        }

        #[test]
        fn test_convert_unsupported_currency() {
            let converter = CurrencyConverter::new();
            let from = "USD";
            let to = "XYZ";
            let amount = 100.0;

            // Mock the API response
            let response = ExchangeRateResponse {
                success: false,
                quotes: HashMap::new(),
                error: Some("Unsupported currency".to_string()),
            };

            api::mock_fetch_exchange_rate(from, to, Ok(response));

            let result = converter.convert(from, to, amount);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "Unsupported currency");
        }

        #[test]
        fn test_convert_api_failure() {
            let converter = CurrencyConverter::new();
            let from = "USD";
            let to = "EUR";
            let amount = 100.0;

            // Mock the API response
            api::mock_fetch_exchange_rate(from, to, Err("API failure".to_string()));

            let result = converter.convert(from, to, amount);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "Failed to fetch exchange rate: API failure");
        }

        #[test]
        fn test_convert_zero_amount() {
            let converter = CurrencyConverter::new();
            let from = "USD";
            let to = "EUR";
            let amount = 0.0;

            // Mock the API response
            let mut quotes = HashMap::new();
            quotes.insert(to.to_string(), 0.85);
            let response = ExchangeRateResponse {
                success: true,
                quotes,
                error: None,
            };

            api::mock_fetch_exchange_rate(from, to, Ok(response));

            let result = converter.convert(from, to, amount);
            assert_eq!(result.unwrap(), 0.0);
        }

        #[test]
        fn test_convert_negative_amount() {
            let converter = CurrencyConverter::new();
            let from = "USD";
            let to = "EUR";
            let amount = -100.0;

            // Mock the API response
            let mut quotes = HashMap::new();
            quotes.insert(to.to_string(), 0.85);
            let response = ExchangeRateResponse {
                success: true,
                quotes,
                error: None,
            };

            api::mock_fetch_exchange_rate(from, to, Ok(response));

            let result = converter.convert(from, to, amount);
            assert_eq!(result.unwrap(), -85.0);
        }
    }
}