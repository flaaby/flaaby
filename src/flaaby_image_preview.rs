extern crate raster;

use percentage::Percentage;
use std::cmp;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use raster::{editor, filter, ResizeMode, BlurMode};
use clap::ArgMatches;
use std::path::{Path, MAIN_SEPARATOR};
use colored::Colorize;
use std::process::exit;
use std::env;
use crate::{constants, errors};
use crate::flaaby_image_resize::generate_flaaby_output_filename;

#[derive(FromPrimitive)]
enum QualityLevel {
    VeryLow = 1,
    Low = 2,
    Moderate = 3,
    High = 4,
    VeryHigh = 5,
}

struct preview_struct {
    input_file              : String,
    output_file             : String,
    save_here               : bool,
    quality_level           : i8,
}

impl preview_struct {

    // Setter for input [Image]
    fn set_input_file (&mut self, input_file: String) {
        self.input_file = input_file;
    }

    // Setter for output [Image]
    fn set_output_file (&mut self, output_file: String) {
        self.output_file = output_file;
    }

    // Setter for save here [Image Save Here]
    fn set_save_here (&mut self, save_here: bool) {
        self.save_here = save_here;
    }

    // Setter for quality level [Image Quality Level]
    fn set_quality_level (&mut self, quality_level: i8) {
        self.quality_level = quality_level;
    }

    // Getter for input [Image]
    fn get_input_file (&self) -> &str {
        &self.input_file
    }

    // Getter for output [Image]
    fn get_output_file (&self) -> &str {
        &self.output_file
    }

    // Getter for save here [Image Save Here]
    fn get_save_here (&self) -> bool {
        self.save_here
    }

    // Getter for quality level [Image Quality Level]
    fn get_quality_level (&self) -> i8 {
        self.quality_level
    }

}

pub fn start_preview_module(preview_config: &ArgMatches) {
    let mut previewer: preview_struct = preview_struct {
        input_file: "".to_string(),
        output_file: "".to_string(),
        save_here: false,
        quality_level: 3
    };

    // Check for `file` option
    match preview_config.value_of(constants::CLI_PREVIEW_OPTION_FILE) {
        Some(file_to_open) => {
            if Path::new(file_to_open).exists() {
                previewer.set_input_file(file_to_open.to_string());
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
    if preview_config.occurrences_of(constants::CLI_PREVIEW_OPTION_SAVE_HERE) > 0 {
        previewer.set_save_here(true);
    }

    // Check `output` option
    match preview_config.value_of(constants::CLI_PREVIEW_OPTION_OUTPUT) {
        Some(file_to_write) => {
            let seperator_offset = file_to_write.rfind(MAIN_SEPARATOR);
            let directory_to_write = &file_to_write[0..seperator_offset.unwrap()];
            if Path::new(directory_to_write).exists() {
                previewer.set_output_file(file_to_write.to_string());
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
            previewer.set_output_file(generate_flaaby_output_filename(previewer.get_input_file(), previewer.get_save_here()));
        }
    }

    match preview_config.value_of(constants::CLI_PREVIEW_OPTION_QUALITY_LEVEL) {
        Some(preview_quality_level) => {
            let quality_level: i8 = preview_quality_level.parse().expect("Invalid quality level to preview");
            if quality_level <= 0 || quality_level > 5 {
                println!(
                    "[{}]  {} \n[{}] {}",
                    errors::CLI_ERROR_KEYWORD.red(),
                    errors::CLI_ERROR_INVALID_VALUE,
                    errors::CLI_DESCRIPTION_KEYWORD.cyan(),
                    "Quality Value can be in a range of 1-5 only"
                );
                exit(1);
            }
            previewer.set_quality_level(quality_level);
        }

        None => {
            previewer.set_quality_level(3); // Default value - 3 [Medium Quality Level]
        }
    }

    preview(previewer);

}

fn size_from_percentage (input: i32, percentage: i32) -> i32 {
    let percent = Percentage::from(percentage);
    cmp::max(percent.apply_to(input) , 25)
}

fn preview (previewer: preview_struct) -> i32 {
    let result: i32 = 0;
    let mut file_to_preview = raster::open(previewer.get_input_file()).unwrap();
    let width: i32 = file_to_preview.width;
    let height: i32 = file_to_preview.height;
    let mut preview_width: i32 = 0;
    let mut preview_height: i32 = 0;
    match FromPrimitive::from_i8(previewer.get_quality_level()) {

        Some(QualityLevel::VeryLow) => {
            preview_width = size_from_percentage(width, 5);
            preview_height = size_from_percentage(height, 5);
        },
        Some(QualityLevel::Low) => {
            preview_width = size_from_percentage(width, 10);
            preview_height = size_from_percentage(height, 10);
        },
        Some(QualityLevel::Moderate) => {
            preview_width = size_from_percentage(width, 25);
            preview_height = size_from_percentage(height, 25);
        },
        Some(QualityLevel::High) => {
            preview_width = size_from_percentage(width, 40);
            preview_height = size_from_percentage(height, 40);
        },
        Some(QualityLevel::VeryHigh) => {
            preview_width = size_from_percentage(width, 60);
            preview_height = size_from_percentage(height, 60);
        },
        None => {

        }
        _ => {}

    }
    editor::resize(&mut file_to_preview, preview_width,preview_height, ResizeMode::Fit).unwrap();
    filter::blur(&mut file_to_preview, BlurMode::Box).unwrap();
    editor::resize(&mut file_to_preview, width, height, ResizeMode::Fit).unwrap();
    raster::save(&file_to_preview, previewer.get_output_file());
    result
}