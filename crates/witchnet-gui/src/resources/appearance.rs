use std::{
    sync::Arc,
    string::ToString,
    default::Default,
    collections::HashMap
};

use bevy::prelude::*;

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

    pub primary_color: Color,
    pub primary_hover_color: Color,
    pub primary_active_color: Color,

    pub secondary_color: Color,
    pub secondary_hover_color: Color,
    pub secondary_active_color: Color,
    
    pub text_color: Color,
    pub text_hover_color: Color,
    pub text_active_color: Color
}

impl Default for NeuronAppearance {
    fn default() -> Self {
        NeuronAppearance {
            show: true,
            show_text: true,

            size: 20f32,
            size_bounds: (0f32, 100f32),
            text_size: 10f32,
            text_size_bounds: (0f32, 50f32),

            primary_color: Color::rgba(0.0930, 0.316, 0.930, 1.0),
            primary_hover_color: Color::rgba(0.352, 0.493, 0.880, 1.0),
            primary_active_color: Color::rgba(0.583, 0.659, 0.870, 1.0),
        
            secondary_color: Color::rgba(0.500, 0.172, 0.0200, 1.0),
            secondary_hover_color: Color::rgba(0.640, 0.364, 0.237, 1.0),
            secondary_active_color: Color::rgba(0.710, 0.564, 0.497, 1.0),
            
            text_color: Color::rgba(0.530, 0.530, 0.535, 1.0),
            text_hover_color: Color::rgba(0.680, 0.680, 0.680, 1.0),
            text_active_color: Color::rgba(0.810, 0.810, 0.802, 1.0),
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

    pub primary_color: Color,
    pub primary_hover_color: Color,
    pub primary_active_color: Color,

    pub secondary_color: Color,
    pub secondary_hover_color: Color,
    pub secondary_active_color: Color,
    
    pub text_color: Color,
    pub text_hover_color: Color,
    pub text_active_color: Color
}

impl Default for SensorAppearance {
    fn default() -> Self {
        SensorAppearance {
            show: true,
            show_text: true,

            size: 20f32,
            size_bounds: (0f32, 100f32),
            text_size: 10f32,
            text_size_bounds: (0f32, 50f32),

            primary_color: Color::rgba(0.0276, 0.420, 0.0210, 1.0),
            primary_hover_color: Color::rgba(0.278, 0.580, 0.273, 1.0),
            primary_active_color: Color::rgba(0.459, 0.710, 0.454, 1.0),
        
            secondary_color: Color::rgba(0.571, 0.590, 0.0118, 1.0),
            secondary_hover_color: Color::rgba(0.788, 0.810, 0.162, 1.0),
            secondary_active_color: Color::rgba(0.865, 0.880, 0.422, 1.0),
            
            text_color: Color::rgba(0.530, 0.530, 0.535, 1.0),
            text_hover_color: Color::rgba(0.680, 0.680, 0.680, 1.0),
            text_active_color: Color::rgba(0.810, 0.810, 0.802, 1.0),
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

    pub color: Color,
    pub hover_color: Color,
    pub active_color: Color,

    pub text_color: Color,
    pub text_hover_color: Color,
    pub text_active_color: Color
}

impl Default for ConnectionAppearance {
    fn default() -> Self {
        ConnectionAppearance {
            show: true,
            show_text: true,
            
            thickness: 50f32,
            thickness_bounds: (0f32, 100f32),
            text_size: 10f32,
            text_size_bounds: (0f32, 50f32),

            color: Color::rgba(0.670, 0.670, 0.663, 1.0),
            hover_color: Color::rgba(0.780, 0.780, 0.772, 1.0),
            active_color: Color::rgba(0.880, 0.880, 0.880, 1.0),

            text_color: Color::rgba(0.750, 0.750, 0.743, 1.0),
            text_hover_color: Color::rgba(0.880, 0.880, 0.889, 1.0),
            text_active_color: Color::rgba(1.0, 1.0, 1.0, 1.0),
        }
    }
}