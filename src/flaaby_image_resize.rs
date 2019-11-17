extern crate raster;

use std::path::PathBuf;
use crate::resize_struct;
use raster::{editor, ResizeMode, Image, BlendMode, PositionMode, Color};

// Function to resize
pub fn resize (resizer: resize_struct) -> i32 {
    let result: i32 = 0;
    let mut file_to_resize = raster::open(resizer.get_input_file()).unwrap();
    if resizer.get_width_const() {
        editor::resize(&mut file_to_resize, resizer.get_width(), resizer.get_height(),ResizeMode::ExactWidth).unwrap();
    } else if resizer.get_height_const() {
        editor::resize(&mut file_to_resize, resizer.get_width(), resizer.get_height(),ResizeMode::ExactHeight).unwrap();
    } else if resizer.get_keep_aspect_ratio() {
        let mut background_pane = Image::blank(resizer.get_width(), resizer.get_height());
        editor::fill(&mut background_pane, Color::hex("#FFFFFF00").unwrap()).unwrap();
        let file_to_resize = editor::blend(&background_pane, &file_to_resize, BlendMode::Normal, 1.0, PositionMode::Center, 0, 0);
    } else {
        editor::resize(&mut file_to_resize, resizer.get_width(), resizer.get_height(),ResizeMode::Exact).unwrap();
    }
    raster::save(&file_to_resize, resizer.get_output_file()).unwrap();
    result
}