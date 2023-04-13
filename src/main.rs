use image::{RgbImage, ImageBuffer, Rgb};

fn main() {
    const IMAGE_WIDTH: u32 = 256*4;
    const IMAGE_HEIGHT: u32 = 256*4;
    const MAX_COLOR: u32 = 256;

    generate_png(IMAGE_WIDTH, IMAGE_HEIGHT, MAX_COLOR);
}

fn generate_png(w: u32, h: u32, c:u32) {
    let mut buffer: RgbImage = ImageBuffer::new(w, h);

    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let r = x as f64 / (w-1) as f64;
        let g = 0.25;
        let b = y as f64 / (h-1) as f64;

        let int_r = (((c as f64) - 0.001) * r) as u8;
        let int_g = (((c as f64) - 0.001) * g) as u8;
        let int_b = (((c as f64) - 0.001) * b) as u8;

        *pixel = Rgb([int_r, int_g, int_b]);
    }

    match buffer.save("output/ray.png") {
        Ok(()) => println!("Done"),
        Err(e) => println!("Error saving file: {e}"),
    };
}
