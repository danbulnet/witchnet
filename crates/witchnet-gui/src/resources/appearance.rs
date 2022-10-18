use std::default::Default;

use bevy::prelude::*;

pub struct NeuronAppearance {
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
            primary_color: Color::rgba(0.0930, 0.316, 0.930, 1.0),
            primary_hover_color: Color::rgba(0.352, 0.493, 0.880, 1.0),
            primary_active_color: Color::rgba(0.583, 0.659, 0.870, 1.0),
        
            secondary_color: Color::rgba(0.500, 0.172, 0.0200, 1.0),
            secondary_hover_color: Color::rgba(0.640, 0.364, 0.237, 1.0),
            secondary_active_color: Color::rgba(0.710, 0.564, 0.497, 1.0),
            
            text_color: Color::rgba(0.220, 0.125, 0.0814, 1.0),
            text_hover_color: Color::rgba(0.320, 0.237, 0.198, 1.0),
            text_active_color: Color::rgba(0.410, 0.334, 0.299, 1.0),
        }
    }
}

pub struct SensorAppearance {
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
            primary_color: Color::rgba(0.0276, 0.420, 0.0210, 1.0),
            primary_hover_color: Color::rgba(0.278, 0.580, 0.273, 1.0),
            primary_active_color: Color::rgba(0.459, 0.710, 0.454, 1.0),
        
            secondary_color: Color::rgba(0.571, 0.590, 0.0118, 1.0),
            secondary_hover_color: Color::rgba(0.788, 0.810, 0.162, 1.0),
            secondary_active_color: Color::rgba(0.865, 0.880, 0.422, 1.0),
            
            text_color: Color::rgba(0.609, 0.630, 0.00, 1.0),
            text_hover_color: Color::rgba(0.779, 0.800, 0.160, 1.0),
            text_active_color: Color::rgba(0.825, 0.840, 0.395, 1.0),
        }
    }
}

pub struct ConnectionAppearance {
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
            color: Color::rgba(0.670, 0.670, 0.663, 1.0),
            hover_color: Color::rgba(0.780, 0.780, 0.772, 1.0),
            active_color: Color::rgba(0.880, 0.880, 0.880, 1.0),

            text_color: Color::rgba(0.750, 0.750, 0.743, 1.0),
            text_hover_color: Color::rgba(0.880, 0.880, 0.889, 1.0),
            text_active_color: Color::rgba(1.0, 1.0, 1.0, 1.0),
        }
    }
}