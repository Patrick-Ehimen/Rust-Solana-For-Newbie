# Rust URL Shortener

This project is a simple URL shortener built in Rust. It provides functionalities to shorten URLs and retrieve the original URLs using a unique short code.

## Project Structure

```
rust-url-shortener
├── src
│   ├── main.rs         # Entry point of the application
│   ├── lib.rs          # Library defining main functionalities
│   ├── routes          # Routing logic for the application
│   │   └── mod.rs
│   ├── handlers        # Request handlers for the routes
│   │   └── mod.rs
│   ├── models          # Data structures used in the application
│   │   └── mod.rs
│   └── utils           # Utility functions for various tasks
│       └── mod.rs
├── Cargo.toml          # Configuration file for the Rust project
└── README.md           # Documentation for the project
```

## Setup Instructions

1. Ensure you have Rust installed on your machine. You can install it from [rust-lang.org](https://www.rust-lang.org/).
2. Clone the repository:
   ```
   git clone https://github.com/<reponame>/rust-url-shortener.git
   ```
3. Navigate to the project directory:
   ```
   cd rust-url-shortener
   ```
4. Build the project:
   ```
   cargo build
   ```

## Usage

To run the application, use the following command:

```
cargo run
```

Once the server is running, you can use the following endpoints:

- **POST /shorten**: To shorten a URL.
- **GET /:code**: To retrieve the original URL using the short code.

## Contributing

Feel free to submit issues or pull requests for improvements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
