// This file contains the core functionality of the image processing library.
// It exports functions for resizing images and applying filters.

pub mod image_processing {
    use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

    pub fn resize_image(img: &DynamicImage, width: u32, height: u32) -> DynamicImage {
        img.resize(width, height, image::imageops::FilterType::Lanczos3)
    }

    pub fn apply_filter(img: &DynamicImage, filter_type: &str) -> DynamicImage {
        match filter_type {
            "grayscale" => img.grayscale(),
            "invert" => {
                let (width, height) = img.dimensions();
                let mut img_buffer = ImageBuffer::new(width, height);
                for (x, y, pixel) in img.pixels() {
                    let Rgba(data) = pixel;
                    img_buffer.put_pixel(x, y, Rgba([255 - data[0], 255 - data[1], 255 - data[2], data[3]]));
                }
                DynamicImage::ImageRgba8(img_buffer)
            },
            _ => img.clone(),
        }
    }
}