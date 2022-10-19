use std::{
    path::PathBuf,
    default::Default
};

use bevy_egui::egui::Color32;

pub const MIN_DATA_WIDTH: f32 = 150f32;
pub const DATA_X: f32 = 25f32;

pub const FILE_NAME_COLOR: Color32 = Color32::from_rgb(194, 232, 148);

pub struct DataFilePath(pub Option<PathBuf>);

impl Default for DataFilePath {
    fn default() -> Self { DataFilePath(None) }
}

pub struct DataFileName(pub Option<String>);

impl Default for DataFileName {
    fn default() -> Self { DataFileName(None) }
}