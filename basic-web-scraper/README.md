# Web Scraper

A versatile command-line web scraper built in Rust that can extract content from multiple web pages and save the data in various formats.

## Features

- Multi-URL scraping support
- Extracts headings (h1, h2, h3), paragraphs, and links
- Configurable output formats
- Error handling and logging
- Non-blocking HTML fetching
- Clean and modular architecture

## Prerequisites

- Rust (2021 edition)
- Cargo package manager

## Dependencies

- reqwest (0.11) - For HTTP requests
- scraper (0.13) - For HTML parsing
- serde (1.0) - For data serialization
- serde_json (1.0) - For JSON handling

## Installation

```bash
git clone <repository-url>
cd web-scraper
cargo build
```

## Usage

```bash
cargo run <URL1> <URL2> ... <output_format>
```

### Example

```bash
cargo run https://example.com https://another-site.com json
```

### Output Formats

The scraper supports multiple output formats. Specify the desired format as the last argument:

- json
- txt
- csv

## Project Structure

- `src/main.rs` - Entry point and CLI handling
- `src/fetch.rs` - HTML fetching functionality
- `src/parse.rs` - HTML parsing and content extraction
- `src/store.rs` - Data storage and format handling

## Error Handling

The scraper includes robust error handling for:

- Invalid URLs
- Network failures
- Parsing errors
- File I/O operations

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details
