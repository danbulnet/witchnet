use std::{
    sync::Arc,
    string::ToString,
    default::Default,
    collections::HashMap
};

use bevy::prelude::*;

use bevy_egui::egui::Vec2;

use crate::interface::transform::{ ScreenTransform, AxisBools };

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum Selector {
    All,
    One(Arc<str>)
}

impl Default for Selector {
    fn default() -> Self { Selector::All }
}

impl ToString for Selector {
    fn to_string(&self) -> String {
        match self {
            Selector::All => "all".to_string(),
            Selector::One(name) => name.to_string()
        }
    }
}

impl Selector {
    pub fn to_arc_str(&self) -> Arc<str> {
        match self {
            Selector::All => "all".into(),
            Selector::One(name) => name.clone()
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Selector::All => "all",
            Selector::One(name) => &*name
        }
    }
}

pub struct Appearance {
    pub simulation2d: Simulation2DAppearance,
    pub sensors: HashMap<Selector, SensorAppearance>,
    pub neurons: HashMap<Selector, NeuronAppearance>,
    pub connections: HashMap<Selector, ConnectionAppearance>,

    pub selected_sensor: Selector,
    pub selected_neuron: Selector,
    pub selected_connection: Selector
}

impl Default for Appearance {
    fn default() -> Self {
        Appearance { 
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
        }
    }
}

#[derive(Debug, Clone)]
pub struct NeuronAppearance {
    pub show: bool,
    pub show_text: bool,

    pub size: f32,
    pub size_bounds: (f32, f32),
    pub text_size: f32,
    pub text_size_bounds: (f32, f32),

    pub rounded: bool,

    pub primary_color: Color,
    pub primary_marked_color: Color,
    pub primary_active_color: Color,

    pub secondary_color: Color,
    pub secondary_marked_color: Color,
    pub secondary_active_color: Color,
    
    pub text_color: Color,
    pub text_marked_color: Color,
    pub text_active_color: Color
}

impl Default for NeuronAppearance {
    fn default() -> Self {
        NeuronAppearance {
            show: true,
            show_text: true,

            size: 0.08f32,
            size_bounds: (0f32, 0.1f32),
            text_size: 5.3f32,
            text_size_bounds: (1.0f32, 10f32),

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
pub struct SensorAppearance {
    pub show: bool,
    pub show_text: bool,

    pub size: f32,
    pub size_bounds: (f32, f32),
    pub text_size: f32,
    pub text_size_bounds: (f32, f32),

    pub level_gap: f32,
    pub level_gap_bounds: (f32, f32),
    
    pub rounded: bool,

    pub primary_color: Color,
    pub primary_marked_color: Color,
    pub primary_active_color: Color,

    pub secondary_color: Color,
    pub secondary_marked_color: Color,
    pub secondary_active_color: Color,
    
    pub text_color: Color,
    pub text_marked_color: Color,
    pub text_active_color: Color
}

impl Default for SensorAppearance {
    fn default() -> Self {
        SensorAppearance {
            show: true,
            show_text: true,

            size: 0.08f32,
            size_bounds: (0f32, 0.1f32),
            text_size: 5.3f32,
            text_size_bounds: (1.0f32, 10f32),
            
            level_gap: 2.5f32,
            level_gap_bounds: (1.0f32, 5.0f32),

            rounded: true,

            primary_color: Color::rgba(0.8, 0.8, 0.8, 1.0),
            primary_marked_color: Color::rgba(0.278, 0.580, 0.273, 1.0),
            primary_active_color: Color::rgba(0.459, 0.710, 0.454, 1.0),
        
            secondary_color: Color::rgba(0.571, 0.590, 0.0118, 1.0),
            secondary_marked_color: Color::rgba(0.788, 0.810, 0.162, 1.0),
            secondary_active_color: Color::rgba(0.865, 0.880, 0.422, 1.0),
            
            text_color: Color::rgba(0.0, 0.0, 0.0, 0.7),
            text_marked_color: Color::rgba(0.0, 0.0, 0.0, 0.85),
            text_active_color: Color::rgba(0.0, 0.0, 0.0, 1.0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionAppearance {
    pub show: bool,
    pub show_text: bool,
    
    pub thickness: f32,
    pub thickness_bounds: (f32, f32),
    pub text_size: f32,
    pub text_size_bounds: (f32, f32),

    pub show_connector: bool,
    pub connector_prop: f32,
    pub connector_prop_bounds: (f32, f32),

    pub curved: bool,

    pub color: Color,
    pub marked_color: Color,
    pub active_color: Color,

    pub text_color: Color,
    pub text_marked_color: Color,
    pub text_active_color: Color
}

impl Default for ConnectionAppearance {
    fn default() -> Self {
        ConnectionAppearance {
            show: true,
            show_text: true,
            
            thickness: 0.08f32,
            thickness_bounds: (0.001f32, 1f32),
            text_size: 0.8f32,
            text_size_bounds: (0.1f32, 1f32),

            show_connector: true,
            connector_prop: 3.5f32,
            connector_prop_bounds: (2f32, 10f32),

            curved: true,

            color: Color::rgba(1.0, 1.0, 1.0, 0.5),
            marked_color: Color::rgba(1.0, 1.0, 1.0, 0.75),
            active_color: Color::rgba(1.0, 1.0, 1.0, 1.0),

            text_color: Color::rgba(0.0, 0.0, 0.0, 0.7),
            text_marked_color: Color::rgba(0.0, 0.0, 0.0, 0.85),
            text_active_color: Color::rgba(0.0, 0.0, 0.0, 1.0),
        }
    }
}
pub struct Simulation2DAppearance {
    pub show_grid: [bool; 2]
}

impl Default for Simulation2DAppearance {
    fn default() -> Self {
        Simulation2DAppearance {
            show_grid: [false, false]
        }
    }
}