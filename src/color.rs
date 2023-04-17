use image::Rgb;

pub struct Color {
    rgb: [f32; 3],
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { rgb: [r, g, b] }
    }

    pub fn color_to_rgb(&self) -> Rgb<u8> {
        Rgb([self.rgb[0] as u8, self.rgb[1] as u8, self.rgb[2] as u8])
    }
}
