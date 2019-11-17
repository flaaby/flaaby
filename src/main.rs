#[macro_use]
extern crate clap;
extern crate rand;

use clap::{App, SubCommand, ArgMatches};
use std::path::{PathBuf, Path, MAIN_SEPARATOR};
use rand::Rng;
use rand::distributions::Alphanumeric;
use colored::Colorize;
use std::process::exit;
use std::env;
use std::borrow::BorrowMut;

mod flaaby_image_resize;
mod constants;
mod errors;

#[derive(Debug)]                    // Added for printing purpose will remove after completion of version 0.1.0
pub struct resize_struct {
    input_file: String,             // Holds the value of the input file [Image] to resize
    output_file: String,            // Holds the value of the output file [Image] which is resized
    width: i32,                     // Holds the desired width for resizing
    height: i32,                    // Holds the desired height for resizing
    keep_aspect_ratio: bool,        // Holds the check to keep aspect ratio
    width_const: bool,              // Holds the check to keep width constant
    height_const: bool,             // Holds the check to keep height constant
    save_here: bool,                // Holds the check to save output in current working directory
}

impl resize_struct {

    // Setter for input file [Image]
    fn set_input_file (&mut self, input_file: String) {
        self.input_file = input_file;
    }

    // Setter for output file [Image]
    fn set_output_file (&mut self, output_file: String) {
        self.output_file = output_file;
    }

    // Setter for width [Image Width]
    fn set_width (&mut self, width: i32) {
        self.width = width;
    }

    // Setter for height [Image Height]
    fn set_height (&mut self, height: i32) {
        self.height = height;
    }

    // Setter for keep aspect ratio [Image Aspect Ratio]
    fn set_keep_aspect_ratio (&mut self, keep_aspect_ratio: bool) {
        self.keep_aspect_ratio = keep_aspect_ratio;
    }

    // Setter for resize width const [Image Const Width]
    fn set_width_const (&mut self, width_const: bool) {
        self.width_const = width_const;
    }

    // Setter for resize height const [Image Const Height]
    fn set_height_const (&mut self, height_const: bool) {
        self.height_const = height_const;
    }

    // Setter for save here [Image Save Here]
    fn set_save_here (&mut self, save_here: bool) {
        self.save_here = save_here;
    }

    // Getter for input file [Image]
    fn get_input_file (&self) -> &str {
        &self.input_file
    }

    // Getter for output file [Image]
    fn get_output_file (&self) -> &str {
        &self.output_file
    }

    // Getter for width [Image Width]
    fn get_width (&self) -> i32 {
        self.width
    }

    // Getter for height [Image Height]
    fn get_height (&self) -> i32 {
        self.height
    }

    // Getter for keep aspect ratio [Image Aspect Ratio]
    fn get_keep_aspect_ratio (&self) -> bool {
        self.keep_aspect_ratio
    }

    // Getter for resize width const [Image Const Width]
    fn get_width_const (&self) -> bool {
        self.width_const
    }

    // Getter for resize height const [Image Const Height]
    fn get_height_const (&self) -> bool {
        self.height_const
    }

    // Getter for save here [Image Save Here]
    fn get_save_here (&self) -> bool {
        self.save_here
    }

}


// Function to generate RANDOM string with desired length
fn generate_random_string(count: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(count)
        .collect::<String>()
}

// Function to generate the output file covering all cases. user provides | user doesn't provide
fn generate_flaaby_output_filename (input_file: &str, save_here: bool) -> String {
    let dir = env::current_dir().unwrap();
    let seperator_offset = input_file.rfind(MAIN_SEPARATOR).unwrap();
    let mut filename: String = "".to_string();
    let str_len = input_file.len();
    let dot_seperator_offset = input_file.rfind(".").unwrap();
    if seperator_offset > 0 && !save_here{
        filename = format!("{}{}{}{}",
                            &input_file[0..dot_seperator_offset],
                            "_flaaby_resized_",
                            generate_random_string(5),
                            &input_file[dot_seperator_offset..str_len]
                            );
    } else {
        filename = format!("{}{}{}{}{}{}",
                           dir.to_str().unwrap(),
                           "/",
                           &input_file[(seperator_offset + 1)..dot_seperator_offset],
                           "_flaaby_resized_",
                           generate_random_string(5),
                           &input_file[dot_seperator_offset..str_len]
                          );
    }
    filename

}

// Function for resize subcommand initiation
fn start_resize_module (resize_config: &ArgMatches) {

    let mut resizer: resize_struct = resize_struct {
        input_file: "".to_string(),
        output_file: "".to_string(),
        width: -1,
        height: -1,
        keep_aspect_ratio: false,
        width_const: false,
        height_const: false,
        save_here: false
    };

    // Match `file` option
    match resize_config.value_of(constants::CLI_RESIZE_OPTION_FILE) {
        Some(file_to_open) => {
            if Path::new(file_to_open).exists() {
                resizer.set_input_file(file_to_open.to_string());
            } else {
                println!(
                    "[{}]  {} \n[{}] {}",
                    errors::CLI_ERROR_KEYWORD.red(),
                    errors::CLI_ERROR_INVALID_PATH,
                    errors::CLI_DESCRIPTION_KEYWORD.cyan(),
                    "File doesnot exit."
                );
                exit(1);
            }
        }
        None => {

        }
    }

    // Check `save-here` option
    if resize_config.occurrences_of(constants::CLI_RESIZE_OPTION_SAVE_HERE) > 0 {
        resizer.set_save_here(true);
    }

    // Match `output` option
    match resize_config.value_of(constants::CLI_RESIZE_OPTION_OUTPUT) {
        Some(file_to_write) => {
            let seperator_offset = file_to_write.rfind(MAIN_SEPARATOR);
            let directory_to_write = &file_to_write[0..seperator_offset.unwrap()];
            if Path::new(directory_to_write).exists() {
                resizer.set_output_file(file_to_write.to_string());
            } else {
                println!(
                    "[{}]  {} \n[{}] {}",
                    errors::CLI_ERROR_KEYWORD.red(),
                    errors::CLI_ERROR_INVALID_PATH,
                    errors::CLI_DESCRIPTION_KEYWORD.cyan(),
                    "Parent directory for the file specified doesnot exist"
                );
                exit(1);
            }
        }
        None => {
            resizer.set_output_file(generate_flaaby_output_filename(resizer.get_input_file(), resizer.get_save_here()));
        }
    }

    // Match `width` option
    match resize_config.value_of(constants::CLI_RESIZE_OPTION_WIDTH) {
        Some(image_resize_width) => {
            let resize_width: i32 = image_resize_width.parse().expect("Invalid width to perform resizing");
            if resize_width <= 0 {
                println!(
                    "[{}]  {} \n[{}] {}",
                    errors::CLI_ERROR_KEYWORD.red(),
                    errors::CLI_ERROR_INVALID_VALUE,
                    errors::CLI_DESCRIPTION_KEYWORD.cyan(),
                    "Width cannot be zero or negative."
                );
                exit(1);
            } else {
                resizer.set_width(resize_width);
            }
        }
        None => {
            println!(
                "[{}]  {} \n[{}] {}",
                errors::CLI_ERROR_KEYWORD.red(),
                errors::CLI_ERROR_INVALID_USAGE,
                errors::CLI_DESCRIPTION_KEYWORD.cyan(),
                "--width <size>| -W <size> missing"
            );
            exit(1);
        }
    }

    // Match `height` option
    match resize_config.value_of(constants::CLI_RESIZE_OPTION_HEIGHT) {
        Some(image_resize_height) => {
            let resize_height: i32 = image_resize_height.parse().expect("Invalid height to perform resizing");
            if resize_height <= 0 {
                println!(
                    "[{}]  {} \n[{}] {}",
                    errors::CLI_ERROR_KEYWORD.red(),
                    errors::CLI_ERROR_INVALID_VALUE,
                    errors::CLI_DESCRIPTION_KEYWORD.cyan(),
                    "Height cannot be zero or negative."
                );
                exit(1);
            } else {
                resizer.set_height(resize_height);
            }
        }
        None => {
            println!(
                "[{}]  {} \n[{}] {}",
                errors::CLI_ERROR_KEYWORD.red(),
                errors::CLI_ERROR_INVALID_USAGE,
                errors::CLI_DESCRIPTION_KEYWORD.cyan(),
                "--height <size>| -H <size> missing"
            );
            exit(1);
        }
    }

    // Check `keep-aspect-ratio` option
    if resize_config.occurrences_of(constants::CLI_RESIZE_OPTION_KEEP_ASPECT_RATIO) > 0 {
        if resize_config.occurrences_of(constants::CLI_RESIZE_OPTION_FIXED_HEIGHT) > 0 || resize_config.occurrences_of(constants::CLI_RESIZE_OPTION_FIXED_WIDTH) > 0 {
            println!(
                "[{}]  {} \n[{}] {}",
                errors::CLI_ERROR_KEYWORD.red(),
                errors::CLI_ERROR_INVALID_USAGE,
                errors::CLI_DESCRIPTION_KEYWORD.cyan(),
                errors::CLI_ERROR_RESIZE_CLUBBING
            );
            exit(1);
        } else {
            resizer.set_keep_aspect_ratio(true);
        }
    // Check `fixed-width` option
    } else if resize_config.occurrences_of(constants::CLI_RESIZE_OPTION_FIXED_WIDTH) > 0 {
        if resize_config.occurrences_of(constants::CLI_RESIZE_OPTION_FIXED_HEIGHT) > 0 {
            println!(
                "[{}]  {} \n[{}] {}",
                errors::CLI_ERROR_KEYWORD.red(),
                errors::CLI_ERROR_INVALID_USAGE,
                errors::CLI_DESCRIPTION_KEYWORD.cyan(),
                errors::CLI_ERROR_RESIZE_CLUBBING
            );
            exit(1);
        } else {
            resizer.set_width_const(true);
        }
     // Check `fixed-height` option
    } else if resize_config.occurrences_of(constants::CLI_RESIZE_OPTION_FIXED_HEIGHT) > 0 {
        resizer.set_height_const(true);
    }

    flaaby_image_resize::resize(resizer);
}

fn main() {
    // Load config for argument [Options] parsing
    let clap_config = load_yaml!("../yaml/clap_config.yml");

    // Get matches of options
    let _flaaby_match_config = App::from_yaml(clap_config).get_matches();

    // Match subcommand options
    match _flaaby_match_config.subcommand() {
        // Match `resize` subcommand
        (constants::CLI_SUBCOMMAND_RESIZE, Some(_flaaby_resize_config)) => (start_resize_module(_flaaby_resize_config)),
        _ => (),
    }
}

