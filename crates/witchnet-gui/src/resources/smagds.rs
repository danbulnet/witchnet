use std::{
    default::Default,
    sync::{ Arc, RwLock },
    collections::HashMap
};

use bevy_egui::egui::Color32;

use witchnet_common::neuron::NeuronID;

use smagds::asynchronous::smagds::SMAGDS;

use crate::{
    resources::appearance::{
        Appearance,
        Simulation2DAppearance,
        Selector,
        SensorAppearance,
        NeuronAppearance,
        ConnectionAppearance
    }, 
};

pub const ADDED_TO_SEQUENTIAL_MODEL_COLOR: Color32 = Color32::from_rgb(194, 232, 148);
pub const SAMPLING_METHOD_COLOR: Color32 = Color32::from_rgb(170, 150, 100);

pub const BIG_GAP_FACTOR: f32 = 2.5f32;
pub const SMALL_GAP_FACTOR: f32 = 0.3f32;
pub const SENSOR_NEURON_GAP_R_FRACTION: f32 = 1.2f32;

pub const SENSOR_TEXT_CUTOFF: usize = 6;

pub(crate) struct SMAGDSMain {
    pub(crate) smagds: Option<Arc<RwLock<SMAGDS>>>,
    pub(crate) appearance: Appearance,
    pub(crate) loaded_datasets: Vec<SMAGDSLoadedDataset>,
    pub(crate) positions: SMAGDSPositions
}

impl Default for SMAGDSMain {
    fn default() -> Self { 
        Self {
            smagds: None,
            appearance: Appearance { 
                simulation2d: Simulation2DAppearance::default(),
                sensors: HashMap::from([(Selector::All, SensorAppearance::default())]),
                neurons: HashMap::from([(Selector::All, NeuronAppearance::default())]),
                connections: HashMap::from([
                    (Selector::All, ConnectionAppearance::default()),
                    (
                        Selector::One(Arc::<str>::from("asa-graph-nodes")), 
                        ConnectionAppearance::default()
                    ),
                    (
                        Selector::One(Arc::<str>::from("sensor-sensor")), 
                        ConnectionAppearance::default()
                    ),
                    (
                        Selector::One(Arc::<str>::from("sensor-neuron")), 
                        ConnectionAppearance::default()
                    ),
                    (
                        Selector::One(Arc::<str>::from("neuron-neuron")), 
                        ConnectionAppearance::default()
                    ),
                ]),
    
                selected_sensor: Selector::default(), 
                selected_neuron: Selector::default(), 
                selected_connection: Selector::default()
            },
            loaded_datasets: vec![],
            positions: SMAGDSPositions::default()
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SMAGDSLoadedDataset {
    pub(crate) name: String,
    pub(crate) sampling_method: String,
    pub(crate) sequence_length: usize,
    pub(crate) samples: usize
}

#[derive(Debug)]
pub struct SMAGDSLoadedDatasets(pub(crate) Vec<SMAGDSLoadedDataset>);

impl Default for SMAGDSLoadedDatasets {
    fn default() -> Self { SMAGDSLoadedDatasets(Vec::new()) }
}

#[derive(Debug, Clone)]
pub(crate) struct SMAGDSPositions {
    pub(crate) neurons: HashMap<NeuronID, (f64, f64)>,
    pub(crate) sensors: HashMap<u32, ((f64, f64), f64)>,
    pub(crate) sensor_neurons: HashMap<NeuronID, (f64, f64)>
}

impl Default for SMAGDSPositions {
    fn default() -> Self {
        SMAGDSPositions { 
            neurons: HashMap::new(), 
            sensors: HashMap::new(),
            sensor_neurons: HashMap::new()
        }
    }
}