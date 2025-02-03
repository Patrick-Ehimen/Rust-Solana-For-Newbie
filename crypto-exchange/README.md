# Crypto Exchange

This project is a simple implementation of a cryptocurrency exchange written in Rust. It provides functionalities for executing trades, managing orders, and retrieving market data.

## Project Structure

```
crypto-exchange
├── src
│   ├── main.rs          # Entry point of the application
│   ├── exchange
│   │   └── mod.rs      # Exchange logic implementation
│   ├── models
│   │   └── mod.rs      # Data models for orders, trades, and users
│   └── utils
│       └── mod.rs      # Utility functions for logging and error handling
├── Cargo.toml           # Rust package configuration
└── README.md            # Project documentation
```

## Setup Instructions

1. Ensure you have Rust and Cargo installed on your machine. You can download them from [rust-lang.org](https://www.rust-lang.org/).

2. Clone the repository:

   ```
   git clone https://github.com/yourusername/crypto-exchange.git
   cd crypto-exchange
   ```

3. Build the project:

   ```
   cargo build
   ```

4. Run the application:

   ```
   cargo run
   ```

## Usage Examples

- To execute a trade, send a request to the `/trade` endpoint with the necessary parameters.
- To retrieve market data, use the `/market` endpoint.

## Overview

This crypto exchange allows users to create orders, execute trades, and view market data in real-time. The application is designed to be modular, making it easy to extend and maintain.