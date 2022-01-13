use std::ops::Add;

use image::io::Reader as ImageReader;
use image::{GenericImageView, ImageBuffer, RgbImage};
fn main() {
    let image = ImageReader::open("botan.png")
        .expect("Couldn't open image.")
        .decode()
        .expect("Couldn't decode image.");
    let mut output = image.into_rgb8();
    let mut values = String::with_capacity(160 * 160 * 3 * 5 + 1);
    for (_, _, value) in output.enumerate_pixels_mut() {
        values.push_str(&format!(
            "{:#04x},{:#04x},{:#04x},\n",
            value.0[0], value.0[1], value.0[2]
        ));
        let color = match value.0[0] {
            0..=63 => 0,
            64..=127 => 64,
            128..=191 => 128,
            192..=255 => 192,
        };
        value.0 = [color, color, color];
    }
    println!("[{}]", values);
    output.save("output.png").expect("Couldn't save the file.");
}
