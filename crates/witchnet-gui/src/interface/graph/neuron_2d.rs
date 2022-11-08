use std::sync::{ Arc, RwLock };

use bevy_egui::egui::{
    TextStyle,
    Align2
};

use magds::asynchronous::sensor::SensorConatiner;

use witchnet_common::{
    sensor::{SensorAsync, SensorData},
    data::{DataCategory, DataTypeValue}, 
    neuron::NeuronAsync
};

use crate::{
    resources::appearance::{ NeuronAppearance, ConnectionAppearance },
    utils,
    widgets::plot::{ 
        RichText,
        PlotUi,
        PlotPoint,
        Points,
        MarkerShape,
        Line, 
        LineStyle,
        PlotPoints
    }
};

pub fn neurons(
    ui: &mut PlotUi, 
    name: &str, 
    origin: (f64, f64),
    neurons: &[Arc<RwLock<dyn NeuronAsync>>],
    settings: &NeuronAppearance,
    connection_settings: &ConnectionAppearance,
    width_limit: f64
) {
    let size_f64 = settings.size as f64;

    if neurons.is_empty() { return }

    let neuron_values: Vec<String> = neurons.into_iter().map(|n| {
        let neuron = n.read().unwrap();
        format!("{:?} [{}]", neuron.id(), neuron.counter())
    }).collect();

    let points_vec: Vec<[f64; 2]> = (0..neurons.len())
        .map(|x| {
            let current_x = origin.0 + x as f64 * 0.25 * size_f64;
            let row = (current_x / width_limit).floor() as i64;
            [current_x - row as f64 * width_limit, origin.1 + row as f64 * 1.0]
        })
        .collect();

    for (i, point) in (&points_vec).into_iter().enumerate() {
        let points = Points::new(vec![*point])
            .name(&neuron_values[i])
            .filled(true)
            .shape(MarkerShape::Circle)
            .radius(size_f64 as f32)
            .color(utils::color_bevy_to_egui(&settings.primary_color));    
        if settings.show { ui.points(points); }
    }
}