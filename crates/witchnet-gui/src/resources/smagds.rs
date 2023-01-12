use std::{
    default::Default,
    sync::{ Arc, RwLock },
    collections::HashMap
};

use bevy::prelude::*;

use bevy_egui::egui::Color32;

use witchnet_common::neuron::{ NeuronID, NeuronAsync };

use smagds::asynchronous::smagds::{ SMAGDS, SMAGDSParams };

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

pub const BIG_GAP_FACTOR: f64 = 3.0f64;
pub const SENSOR_SMALL_GAP_FACTOR: f64 = 0.3f64;
pub const NEURON_SMALL_GAP_FACTOR: f64 = 0.5f64;
pub const SENSOR_NEURON_GAP_R_FACTOR: f64 = 1.0f64;

pub const SENSOR_TEXT_CUTOFF: usize = 6;

pub(crate) struct SMAGDSMain {
    pub(crate) smagds: Option<Arc<RwLock<SMAGDS>>>,
    pub(crate) appearance: Appearance,
    pub(crate) loaded_datasets: Vec<SMAGDSLoadedDataset>,
    pub(crate) positions: SMAGDSPositions,
    pub(crate) params: SMAGDSParams
}

impl Default for SMAGDSMain {
    fn default() -> Self { 
        Self {
            smagds: None,
            appearance: Appearance { 
                simulation2d: Simulation2DAppearance::default(),
                sensors: HashMap::from([(Selector::All, SensorAppearance::default())]),
                neurons: HashMap::from([(Selector::All, Self::neuron_appearance())]),
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
            positions: SMAGDSPositions::default(),
            params: SMAGDSParams::default(),
        }
    }
}

impl SMAGDSMain {
    pub fn neuron_appearance() -> NeuronAppearance {
        NeuronAppearance {
            show: true,
            show_text: true,

            size: 0.2f32,
            size_bounds: (0f32, 10.0f32),
            text_size: 7.0f32,
            text_size_bounds: (1.0f32, 20f32),

            rounded: true,

            primary_color: Color::rgba(0.8, 0.8, 0.8, 1.0),
            primary_marked_color: Color::rgba(0.352, 0.493, 0.880, 1.0),
            primary_active_color: Color::rgba(0.583, 0.659, 0.870, 1.0),
        
            secondary_color: Color::rgba(0.500, 0.172, 0.0200, 1.0),
            secondary_marked_color: Color::rgba(0.640, 0.364, 0.237, 1.0),
            secondary_active_color: Color::rgba(0.710, 0.564, 0.497, 1.0),
            
            text_color: Color::rgba(0.0, 0.0, 0.0, 0.7),
            text_marked_color: Color::rgba(0.0, 0.0, 0.0, 0.85),
            text_active_color: Color::rgba(0.0, 0.0, 0.0, 1.0),
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
    pub(crate) group_ids_to_neurons: HashMap<u32, Vec<Arc<RwLock<dyn NeuronAsync>>>>,
    pub(crate) neuron_groups: HashMap<u32, (f64, f64)>,
    pub(crate) neurons: HashMap<NeuronID, (f64, f64)>,
    pub(crate) sensors: HashMap<u32, ((f64, f64), f64)>,
    pub(crate) sensor_neurons: HashMap<NeuronID, (f64, f64)>
}

impl Default for SMAGDSPositions {
    fn default() -> Self {
        SMAGDSPositions { 
            group_ids_to_neurons: HashMap::new(),
            neuron_groups: HashMap::new(),
            neurons: HashMap::new(), 
            sensors: HashMap::new(),
            sensor_neurons: HashMap::new()
        }
    }
}