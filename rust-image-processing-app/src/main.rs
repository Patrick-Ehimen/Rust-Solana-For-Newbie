use image::io::Reader as ImageReader;
use rust_image_processing_app::image_processing::{apply_filter, resize_image};
use std::fs::File;
use std::path::Path;

fn main() {
    // Initialize the application and handle command-line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <command> [options]", args[0]);
        std::process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "resize" => {
            if args.len() != 6 {
                eprintln!(
                    "Usage: {} resize <input> <output> <width> <height>",
                    args[0]
                );
                std::process::exit(1);
            }
            let input = &args[2];
            let output = &args[3];
            let width: u32 = args[4].parse().expect("Invalid width");
            let height: u32 = args[5].parse().expect("Invalid height");

            let img = ImageReader::open(input)
                .expect("Failed to open input image")
                .decode()
                .expect("Failed to decode image");
            let resized_img = resize_image(&img, width, height);
            resized_img
                .save(output)
                .expect("Failed to save resized image");
        }
        "filter" => {
            if args.len() != 5 {
                eprintln!("Usage: {} filter <input> <output> <filter_type>", args[0]);
                std::process::exit(1);
            }
            let input = &args[2];
            let output = &args[3];
            let filter_type = &args[4];

            let img = ImageReader::open(input)
                .expect("Failed to open input image")
                .decode()
                .expect("Failed to decode image");
            let filtered_img = apply_filter(&img, filter_type);
            filtered_img
                .save(output)
                .expect("Failed to save filtered image");
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            std::process::exit(1);
        }
    }
}
