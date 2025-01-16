mod fetch;
mod parse;
mod store;

use fetch::fetch_html;
use parse::parse_html;
use std::env;
use store::store_data;

fn main() {
    // Collect CLI arguments
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: web_scraper <URL1> <URL2> ... <output_format>");
        return;
    }

    // Extract output format (last argument)
    let output_format = args.last().unwrap().to_string();
    let urls = &args[..args.len() - 1];

    // Iterate over multiple URLs
    for url in urls.iter() {
        // Fetch HTML
        let html = fetch_html(url).expect("Failed to fetch HTML");

        // Parse HTML
        let data = parse_html(&html);

        // Generate output file name based on URL
        let output_file_name = format!(
            "output_{}.{}",
            url.replace("http://", "")
                .replace("https://", "")
                .replace("/", "_"),
            output_format
        );

        // Store data
        store_data(&data, &output_format, &output_file_name);
    }
}
