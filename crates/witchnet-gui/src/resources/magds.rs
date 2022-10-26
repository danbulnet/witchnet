use std::{
    default::Default,
    sync::{ Arc, RwLock },
    path::PathBuf
};

use bevy_egui::egui::Color32;

use magds::asynchronous::magds::MAGDS;

pub const ADDED_TO_MAGDS_COLOR: Color32 = Color32::from_rgb(194, 232, 148);

pub struct MainMAGDS(pub Arc<RwLock<MAGDS>>);

impl Default for MainMAGDS {
    fn default() -> Self { MainMAGDS(MAGDS::new_arc()) }
}

#[derive(Debug, Clone)]
pub(crate) struct LoadedDataset {
    pub(crate) name: String,
    pub(crate) path: PathBuf,
    pub(crate) rows: u32,
    pub(crate) features: Vec<String>
}

#[derive(Debug)]
pub struct LoadedDatasets(pub(crate) Vec<LoadedDataset>);

impl Default for LoadedDatasets {
    fn default() -> Self { LoadedDatasets(Vec::new()) }
}