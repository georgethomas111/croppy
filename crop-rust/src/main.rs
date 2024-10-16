use image::{GenericImageView, ImageBuffer, Rgba};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 7 {
        eprintln!("Usage: <input_file> <output_file> <x> <y> <width> <height>");
        process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];
    let crop_x: u32 = args[3].parse().expect("Failed to parse x coordinate");
    let crop_y: u32 = args[4].parse().expect("Failed to parse y coordinate");
    let crop_width: u32 = args[5].parse().expect("Failed to parse width");
    let crop_height: u32 = args[6].parse().expect("Failed to parse height");

    let img = image::open(input_file).expect("Failed to open image");

    let cropped_img = crop_image(&img, crop_x, crop_y, crop_width, crop_height);

    save_image(cropped_img, output_file);
    println!("Image cropped and saved as {}", output_file);
}

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

fn save_image(img: ImageBuffer<Rgba<u8>, Vec<u8>>, output_file: &str) {
    img.save(output_file).expect("Failed to save cropped image");
}

