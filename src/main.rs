extern crate image;

use image::{GenericImageView, ImageBuffer, Rgba};

fn main() {
    // Load the image
    let img_path = "200mmx200mm_1px.png";
    let img = image::open(img_path).expect("Failed to open image");

    // Create a new image buffer with the same dimensions
    let mut img_buffer = ImageBuffer::new(img.width(), img.height());

    // Iterate through each pixel
    for (x, y, pixel) in img.pixels() {
        // Replace any red, yellow, or blue pixels with black
        if is_red(pixel) || is_yellow(pixel) || is_blue(pixel) {
            img_buffer.put_pixel(x, y, Rgba([0, 0, 0, 255]));
        } else {
            img_buffer.put_pixel(x, y, pixel);
        }
    }

    // Save the new image
    let output_path = "1pixel_clean.png";
    img_buffer.save(output_path).expect("Failed to save image");
}

fn is_red(pixel: Rgba<u8>) -> bool {
    pixel[0] > 128 && pixel[1] < 128 && pixel[2] < 128
}

fn is_yellow(pixel: Rgba<u8>) -> bool {
    pixel[0] > 128 && pixel[1] > 128 && pixel[2] < 128
}

fn is_blue(pixel: Rgba<u8>) -> bool {
    pixel[0] < 128 && pixel[1] < 128 && pixel[2] > 128
}
