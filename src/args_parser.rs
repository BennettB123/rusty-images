// Enumeration for basic errors that can occur while parsing command line arguments
pub enum ArgsParsingError {
    NoFileProvided(String),
    CannotParseWidthOrHeight(String),
    HelpRequested,
}

// struct to hold relevant command line arguments
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

        // try to get image path
        let image_path: &str = match args.get(1) {
            Some(path) => path,
            None => {
                return Err(ArgsParsingError::NoFileProvided(String::from(
                    "A path to an input image is required\n",
                )))
            }
        };

        // try to get width
        let output_width: u32 = match args.get(2) {
            Some(w_str) => match w_str.parse() {
                Ok(w) => w,
                Err(err) => {
                    return Err(ArgsParsingError::CannotParseWidthOrHeight(String::from(
                        format!("Unable to parse width '{}'.\nError: {}\n", args[2], err),
                    )))
                }
            },
            None => CommandLineArgs::DEFAULT_OUTPUT_SIZE,
        };

        // try to get height
        let output_height = match args.get(3) {
            Some(h_str) => match h_str.parse() {
                Ok(h) => h,
                Err(err) => {
                    return Err(ArgsParsingError::CannotParseWidthOrHeight(String::from(
                        format!("Unable to parse height '{}'.\nError: {}\n", args[3], err),
                    )))
                }
            },
            None => CommandLineArgs::DEFAULT_OUTPUT_SIZE,
        };

        Ok(CommandLineArgs {
            image_path: image_path.to_owned(),
            output_width,
            output_height,
        })
    }
}
