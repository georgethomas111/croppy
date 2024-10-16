use image::{GenericImageView, ImageBuffer, Rgba};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: <input_file> <output_file> [<x> <y> <width> <height>] [scale]");
        process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    // Default values for cropping (if not provided by the user)
    let default_x: u32 = 0;
    let default_y: u32 = 0;
    let default_width: u32 = 300;
    let default_height: u32 = 300;

    // Parse cropping parameters or use defaults
    let crop_x = args.get(3).map_or(default_x, |x| x.parse().unwrap_or(default_x));
    let crop_y = args.get(4).map_or(default_y, |y| y.parse().unwrap_or(default_y));
    let crop_width = args.get(5).map_or(default_width, |w| w.parse().unwrap_or(default_width));
    let crop_height = args.get(6).map_or(default_height, |h| h.parse().unwrap_or(default_height));

    // Optional scale flag
    let scale = args.get(7).map_or(false, |s| s == "scale");

    // Load the image
    let img = image::open(input_file).expect("Failed to open image");

    let processed_img = if scale {
        scale_image(&img, crop_width, crop_height)
    } else {
        crop_image(&img, crop_x, crop_y, crop_width, crop_height)
    };

    // Save the processed image (cropped or scaled)
    save_image(processed_img, output_file);
    println!("Image processed ({} mode) and saved as {}", if scale { "scaled" } else { "cropped" }, output_file);
}

/// Crop an image based on the given dimensions
fn crop_image(
    img: &image::DynamicImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let cropped = img.view(x, y, width, height);

    let mut output_image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for (px, py, pixel) in cropped.pixels() {
        output_image.put_pixel(px, py, pixel);
    }

    output_image
}

/// Scale an image to the specified dimensions
fn scale_image(
    img: &image::DynamicImage,
    width: u32,
    height: u32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let scaled_img = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);

    let mut output_image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for (px, py, pixel) in scaled_img.pixels() {
        output_image.put_pixel(px, py, pixel);
    }

    output_image
}

/// Save the image to a file
fn save_image(img: ImageBuffer<Rgba<u8>, Vec<u8>>, output_file: &str) {
    img.save(output_file).expect("Failed to save image");
}

