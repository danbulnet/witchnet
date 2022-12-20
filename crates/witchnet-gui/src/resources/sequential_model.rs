use std::{
    default::Default,
    sync::{ Arc, RwLock },
    path::PathBuf,
    collections::HashMap
};

use bevy_egui::egui::Color32;

use witchnet_common::neuron::NeuronID;

use magds::asynchronous::magds::MAGDS;

pub const ADDED_TO_SEQUENTIAL_MODEL_COLOR: Color32 = Color32::from_rgb(194, 232, 148);

pub const BIG_GAP_FACTOR: f32 = 2.5f32;
pub const SMALL_GAP_FACTOR: f32 = 0.3f32;
pub const SENSOR_NEURON_GAP_R_FRACTION: f32 = 1.2f32;

pub const SENSOR_TEXT_CUTOFF: usize = 6;

pub struct SequentialMAGDS(pub Arc<RwLock<MAGDS>>);

impl Default for SequentialMAGDS {
    fn default() -> Self { SequentialMAGDS(MAGDS::new_arc()) }
}

#[derive(Debug, Clone)]
pub(crate) struct SequentialModelLoadedDataset {
    pub(crate) name: String,
    pub(crate) path: PathBuf,
    pub(crate) rows: usize,
    pub(crate) rows_total: usize,
    pub(crate) exequal_sampling: bool,
    pub(crate) features: Vec<String>
}

#[derive(Debug)]
pub struct SequentialModelLoadedDatasets(pub(crate) Vec<SequentialModelLoadedDataset>);

impl Default for SequentialModelLoadedDatasets {
    fn default() -> Self { SequentialModelLoadedDatasets(Vec::new()) }
}

#[derive(Debug, Clone)]
pub(crate) struct SequentialModelPositions {
    pub(crate) neurons: HashMap<NeuronID, (f64, f64)>,
    pub(crate) sensors: HashMap<u32, ((f64, f64), f64)>,
    pub(crate) sensor_neurons: HashMap<NeuronID, (f64, f64)>
}

impl Default for SequentialModelPositions {
    fn default() -> Self {
        SequentialModelPositions { 
            neurons: HashMap::new(), 
            sensors: HashMap::new(),
            sensor_neurons: HashMap::new()
        }
    }
}