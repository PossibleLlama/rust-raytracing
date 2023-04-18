mod color;

use color::Color;
use image::{ImageBuffer, Rgb, RgbImage};

const IMAGE_WIDTH: u32 = 256 * 4;
const IMAGE_HEIGHT: u32 = 256 * 4;

fn main() {
    let img = generate_image(IMAGE_WIDTH, IMAGE_HEIGHT);

    generate_png("ray.png", IMAGE_WIDTH, IMAGE_HEIGHT, img);
}

fn generate_image(width: u32, height: u32) -> Vec<Color> {
    let mut img = Vec::<Color>::new();

    for h in (0..height).rev() {
        for w in 0..width {
            let r = w as f64 / (width - 1) as f64;
            let g = 0.25;
            let b = h as f64 / (height - 1) as f64;

            img.push(Color::new(
                255.999 * r,
                255.999 * b,
                255.999 * g,
            ));
        }
    }
    img
}

fn generate_png(name: &str, w: u32, h: u32, fill: Vec<Color>) {
    let mut buffer: RgbImage = ImageBuffer::new(w, h);

    for (y, x, pixel) in buffer.enumerate_pixels_mut() {
        *pixel = match fill.get(x as usize * IMAGE_WIDTH as usize + y as usize) {
            Some(c) => c.color_to_rgb(),
            None => Rgb([255, 255, 255]),
        };
    }

    match buffer.save(format!("output/{name}")) {
        Ok(()) => println!("Finished creating {name}"),
        Err(e) => println!("Error saving file: {e}"),
    };
}
