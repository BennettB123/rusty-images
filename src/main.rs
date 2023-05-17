// TODO:
//  - general refactoring
//    - break out logic that converts the image
//  - add README

use image::{DynamicImage, GenericImageView};
use std::env;

mod args_parser;
use args_parser::{ArgsParsingError, CommandLineArgs};

const SHADING_CHARS: &str = " \u{2591}\u{2592}\u{2593}\u{2588}";
const ASCII_SHADING_CHARS: &str = " .:-=+*#%@";

fn main() {
    let args = match CommandLineArgs::parse(env::args().collect()) {
        Ok(args) => args,
        Err(parse_error) => match parse_error {
            ArgsParsingError::NoFileProvided(err)
            | ArgsParsingError::CannotParseWidthOrHeight(err) => {
                println!("{}", err);
                return;
            }
            ArgsParsingError::HelpRequested => {
                print_help_message();
                return;
            }
        },
    };

    let image = match image::open(&args.image_path) {
        Ok(img) => img,
        Err(err) => {
            println!("Could not read image file with path '{}'.", args.image_path);
            println!("Error: {err}");
            return;
        }
    };

    let text_output = textify_my_img(image, args.output_width, args.output_height);
    println!("{}", text_output);
}

fn print_help_message() {
    println!(
        "Usage: rusty-images <file-path> [output-width] [output-height]

rusty-images is a command-line tool to generate text art from an image.
It uses the following Unicode block elements as output: \"{0}\".

Arguments:
    <file-path>     required    Path to the input image file.
    [output-width]  optional    Width of the output. Must be a valid u32.
                                Defaults to {1}.
    [output-height] optional    height of the output. Must be a valid u32.
                                Defaults to {1}.
    -h | --help                 Prints the output that you are reading now.

Notes:
    Even if the [output-width] and [output-height] match the
    aspect ratio of the input image's height/width, the result can look
    distorted since characters usually aren't the same height and width.
    These values should be tweaked until the result best matches the
    input image.
",
        SHADING_CHARS,
        args_parser::CommandLineArgs::DEFAULT_OUTPUT_SIZE
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
