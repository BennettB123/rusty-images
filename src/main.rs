use image::{DynamicImage, GenericImageView};
use std::env;

const OUTPUT_SCALING_FACTOR: u32 = 100;
const ASCII_SHADES: [char; 5] = [' ', '\u{2591}', '\u{2592}', '\u{2593}', '\u{2588}'];

fn main() {
    let img_path = match env::args().nth(1) {
        Some(f) => f,
        None => panic!("Did not find image path as first command line argument!"),
    };

    let image = image::open(img_path).unwrap();
    let ascii = asciify_my_img(image);

    println!("{}", ascii);
}

fn asciify_my_img(input_img: DynamicImage) -> String {
    let mut ascii = String::new();
    let (width, height) = input_img.dimensions();
    let width_factor = width / OUTPUT_SCALING_FACTOR;
    let height_factor = height / OUTPUT_SCALING_FACTOR;

    for col in 0..OUTPUT_SCALING_FACTOR {
        for row in 0..OUTPUT_SCALING_FACTOR {
            // get cropped view of image
            let x = row * width_factor;
            let y = col * height_factor;
            let crop = input_img.crop_imm(x, y, width_factor, height_factor);
            let brightness = get_image_brightness(crop);
            ascii.push(convert_brightness_to_ascii(brightness));
        }

        ascii.push('\n');
    }

    ascii
}

// returns a float32 between 0 and 1 that represents the average brightness of an image
// uses the "Relative luminance" method
fn get_image_brightness(img: DynamicImage) -> f32 {
    let mut cumulative_brightness: f32 = 0.0;

    for pix in img.pixels() {
        let rgba = pix.2 .0; // extract rgba slice from pix
        let r = rgba[0] as f32;
        let g = rgba[1] as f32;
        let b = rgba[2] as f32;

        let brightness = (0.2126 * r) + (0.7152 * g) + (0.0722 * b);
        cumulative_brightness += map_range((0.0, 255.0), (0.0, 1.0), brightness);
    }

    cumulative_brightness / (img.width() * img.height()) as f32
}

fn convert_brightness_to_ascii(brightness: f32) -> char {
    let mut new_b = brightness;
    if brightness < 0.0 {
        new_b = 0.0;
    }
    if brightness >= 1.0 {
        new_b = 1.0;
    }

    let mut index: usize = (ASCII_SHADES.len() as f32 * new_b).floor() as usize;
    if index >= ASCII_SHADES.len() {
        index = ASCII_SHADES.len() - 1;
    }
    return ASCII_SHADES[index];
}

fn map_range(from_range: (f32, f32), to_range: (f32, f32), s: f32) -> f32 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}
