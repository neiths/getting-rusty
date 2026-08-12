
use std::env::current_exe;
use std::io::{BufWriter, BufReader, Write};
use std::fs::{File};
use std::path::Path;
use rustc_serialize::{ json, Encodable, Decodable };

static SETTING_FILENAME: &'static str = "settings.json";

pub struct Settings {
    pub asset_folder: String,
    pub window_size: [u32; 2],
    pub window_background_color: [f32; 3],
    pub comment1_offset_y: f64,
    pub comment2_offset_y: f64,
    pub board_padding: f64,
    pub board_size: [f64; 2],
    pub board_offset_y: f64,
    pub tile_width: i32,
    pub tile_height: i32,
    pub tile_size: f64,
    pub tile_padding: f64,
    pub tile_background_color: [f32; 3],
    pub tiles_colors: Vec<[f32; 3]>,
    pub tile_unknow_color: [f32; 3],
    pub tile_move_time: f64,
    pub tile_new_time: f64,
    pub tile_combine_time: f64,
    pub best_rect: [f64; 4],
    pub score_rect: [f64; 4],
    pub label_color: [f32; 3],
    pub button_color: [f32; 3],
    pub text_dark_color: [f32; 3],
    pub text_light_color: [f32; 3],
}

