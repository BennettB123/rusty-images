// Enumeration for basic errors that can occur while parsing command line arguments

pub enum ArgsParsingError {
    NoFileProvided(String),
    CannotParseWidthOrHeight(String),
    HelpRequested,
}

pub struct CommandLineArgs {
    pub image_path: String,
    pub output_width: u32,
    pub output_height: u32,
}

impl CommandLineArgs {
    pub const DEFAULT_OUTPUT_SIZE: u32 = 50;

    pub fn parse(args: Vec<String>) -> Result<Self, ArgsParsingError> {
        // check all parameters to see if any are "-h" or "--help"
        for arg in &args {
            if arg == "-h" || arg == "--help" {
                return Err(ArgsParsingError::HelpRequested);
            }
        }

        let mut image_path = "";
        let mut output_width = CommandLineArgs::DEFAULT_OUTPUT_SIZE;
        let mut output_height = CommandLineArgs::DEFAULT_OUTPUT_SIZE;

        if args.len() < 2 {
            return Err(ArgsParsingError::NoFileProvided(String::from(
                "A path to an input image is required",
            )));
        }
        if args.len() >= 2 {
            image_path = &args[1];
        }
        if args.len() >= 3 {
            output_width = match args[2].parse() {
                Ok(w) => w,
                Err(err) => {
                    return Err(ArgsParsingError::CannotParseWidthOrHeight(String::from(
                        format!("Unable to parse width '{}'. Error: {}", args[2], err),
                    )))
                }
            };
        }
        if args.len() >= 4 {
            output_height = match args[3].parse() {
                Ok(w) => w,
                Err(err) => {
                    return Err(ArgsParsingError::CannotParseWidthOrHeight(String::from(
                        format!("Unable to parse height '{}'. Error: {}", args[3], err),
                    )))
                }
            };
        }

        Ok(CommandLineArgs {
            image_path: image_path.to_owned(),
            output_width,
            output_height,
        })
    }
}
