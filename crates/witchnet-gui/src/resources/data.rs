use std::{
    path::PathBuf,
    default::Default,
    collections::BTreeMap
};

use polars::prelude::*;

use bevy_egui::egui::Color32;

pub const MIN_DATA_WIDTH: f32 = 150f32;
pub const DATA_X: f32 = 25f32;

pub const FILE_NAME_OK_COLOR: Color32 = Color32::from_rgb(194, 232, 148);
pub const FILE_NAME_ERR_COLOR: Color32 = Color32::from_rgb(232, 148, 148);

pub const ADDED_TO_MAGDS_COLOR: Color32 = Color32::from_rgb(194, 232, 148);

#[derive(Debug, Clone)]
pub(crate) struct DataFile {
    pub(crate) name: String,
    pub(crate) path: PathBuf,
    pub(crate) data_frame: Option<DataFrame>,
    pub(crate) features: BTreeMap<String, bool>
}

#[derive(Debug)]
pub struct DataFiles{
    pub(crate) current: Option<usize>,
    pub(crate) history: Vec<DataFile>
}

impl Default for DataFiles {
    fn default() -> Self { 
        DataFiles{ current: None, history: Vec::new() } 
    }
}