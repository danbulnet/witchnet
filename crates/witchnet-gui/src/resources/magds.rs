use std::{
    default::Default,
    sync::{ Arc, RwLock },
    path::PathBuf,
    collections::HashMap
};

use bevy_egui::egui::Color32;

use witchnet_common::neuron::{ NeuronID, NeuronAsync };

use magds::asynchronous::magds::MAGDS;

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

pub const ADDED_TO_MAGDS_COLOR: Color32 = Color32::from_rgb(194, 232, 148);

pub const BIG_GAP_FACTOR: f64 = 0.5f64;
pub const SMALL_GAP_FACTOR: f64 = 0.3f64;
pub const SENSOR_NEURON_GAP_R_FRACTION: f64 = 1.0f64;

pub const SENSOR_TEXT_CUTOFF: usize = 6;

pub(crate) struct MAGDSMain {
    pub(crate) magds: Arc<RwLock<MAGDS>>,
    pub(crate) appearance: Appearance,
    pub(crate) loaded_datasets: Vec<MAGDSLoadedDataset>,
    pub(crate) positions: MAGDSPositions
}

impl Default for MAGDSMain {
    fn default() -> Self {
        Self {
            magds: MAGDS::new_arc(),
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
            positions: MAGDSPositions::default()
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub(crate) struct MAGDSLoadedDataset {
    pub(crate) name: String,
    pub(crate) path: PathBuf,
    pub(crate) rows: usize,
    pub(crate) rows_total: usize,
    pub(crate) random_pick: bool,
    pub(crate) features: Vec<String>
}

#[derive(Debug)]
pub struct MAGDSLoadedDatasets(pub(crate) Vec<MAGDSLoadedDataset>);

impl Default for MAGDSLoadedDatasets {
    fn default() -> Self { MAGDSLoadedDatasets(Vec::new()) }
}

#[derive(Debug, Clone)]
pub(crate) struct MAGDSPositions {
    pub(crate) group_ids_to_neurons: HashMap<u32, Vec<Arc<RwLock<dyn NeuronAsync>>>>,
    pub(crate) neuron_groups: HashMap<u32, (f64, f64)>,
    pub(crate) neurons: HashMap<NeuronID, (f64, f64)>,
    pub(crate) sensors: HashMap<u32, ((f64, f64), f64)>,
    pub(crate) sensor_neurons: HashMap<NeuronID, (f64, f64)>
}

impl Default for MAGDSPositions {
    fn default() -> Self {
        MAGDSPositions { 
            group_ids_to_neurons: HashMap::new(),
            neuron_groups: HashMap::new(),
            neurons: HashMap::new(), 
            sensors: HashMap::new(),
            sensor_neurons: HashMap::new()
        }
    }
}