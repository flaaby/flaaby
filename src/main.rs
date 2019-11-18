#[macro_use]
extern crate clap;
extern crate rand;

use clap::{App, SubCommand};

mod flaaby_image_resize;
mod constants;
mod errors;



fn main() {
    // Load config for argument [Options] parsing
    let clap_config = load_yaml!("../yaml/clap_config.yml");

    // Get matches of options
    let _flaaby_match_config = App::from_yaml(clap_config).get_matches();

    // Match subcommand options
    match _flaaby_match_config.subcommand() {
        // Match `resize` subcommand
        (constants::CLI_SUBCOMMAND_RESIZE, Some(_flaaby_resize_config)) => (flaaby_image_resize::start_resize_module(_flaaby_resize_config)),
        _ => (),
    }
}

