use std::default::Default;

use bevy::prelude::*;

pub struct NeuronAppearance {
    pub body_color: Color,
    pub body_hover_color: Color,
    pub body_active_color: Color,

    pub border_color: Color,
    pub border_hover_color: Color,
    pub border_active_color: Color,
    
    pub text_color: Color,
    pub text_hover_color: Color,
    pub text_active_color: Color,

    pub connector_color: Color,
    pub connector_hover_color: Color,
    pub connector_active_color: Color,
    
    pub connector_text_color: Color,
    pub connector_text_hover_color: Color,
    pub connector_text_active_color: Color, 
}

impl Default for NeuronAppearance {
    fn default() -> Self {
        NeuronAppearance {
            body_color: Color::rgba(0.1, 0.1, 0.44, 1.0),
            body_hover_color: Color::rgba(0.1, 0.1, 0.44, 1.0),
            body_active_color: Color::rgba(0.1, 0.1, 0.44, 1.0),
        
            border_color: Color::rgba(0.1, 0.1, 0.44, 1.0),
            border_hover_color: Color::rgba(0.1, 0.1, 0.44, 1.0),
            border_active_color: Color::rgba(0.1, 0.1, 0.44, 1.0),
            
            text_color: Color::rgba(0.1, 0.1, 0.44, 1.0),
            text_hover_color: Color::rgba(0.1, 0.1, 0.44, 1.0),
            text_active_color: Color::rgba(0.1, 0.1, 0.44, 1.0),
        
            connector_color: Color::rgba(0.1, 0.1, 0.44, 1.0),
            connector_hover_color: Color::rgba(0.1, 0.1, 0.44, 1.0),
            connector_active_color: Color::rgba(0.1, 0.1, 0.44, 1.0),
            
            connector_text_color: Color::rgba(0.1, 0.1, 0.44, 1.0),
            connector_text_hover_color: Color::rgba(0.1, 0.1, 0.44, 1.0),
            connector_text_active_color: Color::rgba(0.1, 0.1, 0.44, 1.0)
        }
    }
}