use std::{
    default::Default,
    sync::{ Arc, RwLock },
    path::PathBuf,
    collections::HashMap
};

use bevy_egui::egui::Color32;

use witchnet_common::neuron::NeuronID;

use magds::asynchronous::magds::MAGDS;

pub const ADDED_TO_MAGDS_COLOR: Color32 = Color32::from_rgb(194, 232, 148);

pub const BIG_GAP_FACTOR: f32 = 5f32;
pub const SMALL_GAP_FACTOR: f32 = 1f32;

pub struct MainMAGDS(pub Arc<RwLock<MAGDS>>);

impl Default for MainMAGDS {
    fn default() -> Self { MainMAGDS(MAGDS::new_arc()) }
}

#[derive(Debug, Clone)]
pub(crate) struct LoadedDataset {
    pub(crate) name: String,
    pub(crate) path: PathBuf,
    pub(crate) rows: usize,
    pub(crate) rows_total: usize,
    pub(crate) random_pick: bool,
    pub(crate) features: Vec<String>
}

#[derive(Debug)]
pub struct LoadedDatasets(pub(crate) Vec<LoadedDataset>);

impl Default for LoadedDatasets {
    fn default() -> Self { LoadedDatasets(Vec::new()) }
}

#[derive(Debug, Clone)]
pub(crate) struct PositionXY {
    pub(crate) neurons: HashMap<NeuronID, (f64, f64)>,
    pub(crate) sensors: HashMap<u32, (f64, f64)>
}

impl Default for PositionXY {
    fn default() -> Self {
        PositionXY { neurons: HashMap::new(), sensors: HashMap::new() }
    }
}