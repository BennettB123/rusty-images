// TODO:
//  - general refactoring
//    - break out command line arguments parsing
//    - break out logic that converts the image
//  - make command line arguments more explicit
//    - add a "-h" flag to display the help message
//    - give width and height a flag, like "--width" or "--height"
//  - add README

use image::{DynamicImage, GenericImageView};
use std::env;

const DEFAULT_OUTPUT_SIZE: u32 = 50;
const SHADING_CHARS: &str = " \u{2591}\u{2592}\u{2593}\u{2588}";
const ASCII_SHADING_CHARS: &str = " .:-=+*#%@";

fn main() {
    if env::args().len() < 2 {
        print_help_message();
        return;
    }

    let img_path = match env::args().nth(1) {
        Some(f) => f,
        None => panic!("Did not find image path as first command line argument!"),
    };

    let output_width: u32 = match env::args().nth(2) {
        Some(f) => match f.parse::<u32>() {
            Ok(v) => v,
            Err(_) => panic!("Could not parse width argument '{}' as u32!", f),
        },
        None => DEFAULT_OUTPUT_SIZE,
    };

    let output_height: u32 = match env::args().nth(3) {
        Some(f) => match f.parse::<u32>() {
            Ok(v) => v,
            Err(_) => panic!("Could not parse width argument '{}' as u32!", f),
        },
        None => DEFAULT_OUTPUT_SIZE,
    };

    let image = image::open(img_path).unwrap();
    let text_output = textify_my_img(image, output_width, output_height);

    println!("{}", text_output);
}

fn print_help_message() {
    println!(
        "Usage: rusty-images <file-path> [output-width] [output-height]
    
    rusty-images is a command-line tool to generate text art from an image.
    It uses the following Unicode block elements as output: \"{SHADING_CHARS}\".

    Options:
        <file-path>     required    Path to the input image file.
        [output-width]  optional    Width of the output. Must be a valid u32.
                                    Defaults to {DEFAULT_OUTPUT_SIZE}.
        [output-height] optional    height of the output. Must be a valid u32.
                                    Defaults to {DEFAULT_OUTPUT_SIZE}.
    
    Notes:
        If neither [output-width] or [output-height] are provided, they will
        be calculated based on the input image's aspect ratio. The larger value
        between the height and width will become the default ({DEFAULT_OUTPUT_SIZE}), and the other
        will be scaled down to maintain the correct aspect ratio for the image.

        If only one of [output-width] or [output-height] are provided, the other
        will be calculated based on the aspect ratio of the input image.
        For example: input image has 100 width & 200 height, the [output-width]
        was provided as 25. The resulting output's height will be calculated as 50,
        since the image's height is double its width.

        Note that even if the output's height/width match the
        aspect ratio of the input image's height/width, the result can look
        distorted since characters usually aren't the same height and width. The 
        [output-width] and [output-height] should be tweaked until the result
        best matches the input image.
    "
    )
}

fn textify_my_img(input_img: DynamicImage, out_width: u32, out_height: u32) -> String {
    let mut text = String::new();
    let (in_width, in_height) = input_img.dimensions();
    let width_factor = in_width as f32 / out_width as f32;
    let height_factor = in_height as f32 / out_height as f32;

    for col in 0..out_height {
        for row in 0..out_width {
            // get crop of image to convert to single text char
            let x = row as f32 * width_factor;
            let y = col as f32 * height_factor;
            let crop = input_img.crop_imm(
                x as u32,
                y as u32,
                width_factor as u32,
                height_factor as u32,
            );
            let brightness = get_image_brightness(crop);
            text.push(get_character_from_brightness(brightness));
        }

        text.push('\n');
    }

    text
}

// returns a float32 between 0 and 1 that represents the average brightness of an image
// uses the "Relative luminance" method: (0.2126 * r) + (0.7152 * g) + (0.0722 * b)
fn get_image_brightness(img: DynamicImage) -> f32 {
    let mut cumulative_brightness: f32 = 0.0;

    for pixel in img.pixels() {
        let rgba = pixel.2 .0; // extract rgba slice from pix
        let r = rgba[0] as f32;
        let g = rgba[1] as f32;
        let b = rgba[2] as f32;

        let brightness = (0.2126 * r) + (0.7152 * g) + (0.0722 * b);
        cumulative_brightness += map_range((0.0, 255.0), (0.0, 1.0), brightness);
    }

    cumulative_brightness / (img.width() * img.height()) as f32
}

// refactor this to be more performant. using .chars().count() is O(n) since we aren't using ascii
fn get_character_from_brightness(brightness: f32) -> char {
    let mut new_b = brightness;
    if brightness < 0.0 {
        new_b = 0.0;
    }
    if brightness >= 1.0 {
        new_b = 1.0;
    }

    let num_shading_chars = SHADING_CHARS.chars().count();

    let mut index: usize = (num_shading_chars as f32 * new_b).floor() as usize;
    if index >= num_shading_chars {
        index = num_shading_chars - 1;
    }
    return SHADING_CHARS.chars().nth(index).unwrap();
}

fn map_range(from_range: (f32, f32), to_range: (f32, f32), s: f32) -> f32 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}
