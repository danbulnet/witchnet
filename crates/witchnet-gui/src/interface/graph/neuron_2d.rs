use std::{
    collections::HashMap,
    sync::{ Arc, RwLock }
};

use bevy_egui::egui::{
    TextStyle,
    Align2
};

use bevy::prelude::*;

use magds::asynchronous::sensor::SensorConatiner;

use witchnet_common::{
    sensor::{SensorAsync, SensorData},
    data::{DataCategory, DataTypeValue}, 
    neuron::{ NeuronAsync, NeuronID }
};

use crate::{
    resources::{
        appearance::{ 
            NeuronAppearance, 
            ConnectionAppearance }, 
        magds::PositionXY
    },
    utils,
    widgets::plot::{
        PlotUi,
        Line, 
        LineStyle,
        PlotPoints,
        Nodes,
        NodeShape,
        RichText,
        PlotPoint
    }
};

// pub fn neurons(
//     ui: &mut PlotUi, 
//     name: &str, 
//     origin: (f64, f64),
//     neurons: &[Arc<RwLock<dyn NeuronAsync>>],
//     settings: &NeuronAppearance,
//     connection_settings: &ConnectionAppearance,
//     width_limit: f64,
//     sensor_point_map: HashMap<NeuronID, [f64; 2]>
// ) {
//     let size_f64 = settings.size as f64;

//     if neurons.is_empty() { return }
//     let no_neurons = neurons.len();

//     let neuron_values: Vec<String> = neurons.into_iter().map(|n| {
//         let neuron = n.read().unwrap();
//         format!("{} [{}]", neuron.id(), neuron.counter())
//     }).collect();

//     let points_vec: Vec<[f64; 2]> = (0..neurons.len())
//         .map(|x| {
//             let current_x = origin.0 + x as f64 * 0.25 * size_f64;
//             let row = (current_x / width_limit).floor() as i64;
//             [current_x - row as f64 * width_limit, origin.1 + row as f64 * 1.0]
//         })
//         .collect();

//     for (i, point) in (&points_vec).into_iter().enumerate() {
//         let nodes = Nodes::new(vec![*point])
//             .name(&neuron_values[i])
//             .filled(true)
//             .shape(NodeShape::Circle)
//             .radius(size_f64 as f32)
//             .color(utils::color_bevy_to_egui(&settings.primary_color));    
//         if settings.show { ui.nodes(nodes); }
//     }

//     for i in 0..no_neurons {
//         let neuron = neurons[i].read().unwrap();
//         let start_top = [points_vec[i][0], points_vec[i][1] + size_f64 * 0.05];
//         let start_bottom = [points_vec[i][0], points_vec[i][1] - size_f64 * 0.05];

//         for sensor in neurons[i].read().unwrap().explain() {
//             let sensor = sensor.read().unwrap();
//             let mut end = sensor_point_map[&sensor.id()];
//             let start: [f64; 2];
//             if end[1] > start_top[1] {
//                 start = start_top;
//                 end[1] -= size_f64 * 0.05;
//             } else {
//                 start = start_bottom;
//                 end[1] += size_f64 * 0.05;
//             }
//             let connection_name = format!(
//                 "{} <-> {} [{:.3}]", 
//                 neuron.id(),
//                 sensor.explain_one(0u32).unwrap().to_string(),
//                 1.0
//             );

//             let connections = Line::new(PlotPoints::new(vec![start, end]))
//                 .color(utils::color_bevy_to_egui(&connection_settings.color))
//                 .style(LineStyle::Solid)
//                 .name(&connection_name)
//                 .width(connection_settings.thickness);
//             let nodes = Nodes::new(vec![start, end])
//                 .name(&connection_name)
//                 .filled(true)
//                 .shape(NodeShape::Circle)
//                 .radius(size_f64 as f32 / 5f32)
//                 .color(utils::color_bevy_to_egui(&connection_settings.color));
    
//             if connection_settings.show { 
//                 ui.nodes(nodes);
//                 ui.line(connections);
//             }

//         }
//     }
// }

pub(crate) fn neurons(
    ui: &mut PlotUi, 
    name: &str, 
    neurons: &[Arc<RwLock<dyn NeuronAsync>>],
    position_xy_res: &mut PositionXY,
    settings: &NeuronAppearance,
    connection_settings: &ConnectionAppearance,
) {
    let size_f64 = settings.size as f64;

    let neuron_positions = &position_xy_res.neurons;
    let sensor_positions = &position_xy_res.sensor_neurons;

    if neurons.is_empty() { return }
    let no_neurons = neurons.len();

    let neuron_values: Vec<String> = neurons.into_iter().map(|n| {
        let neuron = n.read().unwrap();
        format!("{} [{}]", neuron.id(), neuron.counter())
    }).collect();

    // let points_vec: Vec<[f64; 2]> = (0..neurons.len())
    //     .map(|x| {
    //         let current_x = origin.0 + x as f64 * 0.25 * size_f64;
    //         let row = (current_x / width_limit).floor() as i64;
    //         [current_x - row as f64 * width_limit, origin.1 + row as f64 * 1.0]
    //     })
    //     .collect();

    for neuron in neurons {
        let neuron = neuron.read().unwrap();
        let neuron_value = format!("{} [{}]", neuron.id(), neuron.counter());
        let neuron_id = neuron.id();
        let neuron_id_id = neuron.id().id;
        let neuron_activation = neuron.activation();
        let neuron_counter = neuron.counter();
        let neuron_pos = neuron_positions[&neuron_id];
        let neuron_name = format!("{name}: neuron_id_id");
        let neuron_color = if neuron_activation >= 4.0 { 
            &settings.primary_active_color
        } else { &settings.primary_color };
        let nodes = Nodes::new(vec![[neuron_pos.0, neuron_pos.1]])
            .name(&neuron_value)
            .filled(true)
            .shape(NodeShape::Circle)
            .radius(size_f64 as f32)
            .color(utils::color_bevy_to_egui(&neuron_color));    
        if settings.show { ui.nodes(nodes); }

        let start_top = [neuron_pos.0, neuron_pos.1 + size_f64];
        let start_bottom = [neuron_pos.0, neuron_pos.1 - size_f64];

        for sensor in neuron.explain() {
            let sensor = sensor.read().unwrap();
            let end = sensor_positions[&sensor.id()];
            let mut end = [end.0, end.1];
            let start: [f64; 2];
            if end[1] > start_top[1] {
                start = start_top;
                end[1] -= size_f64;
            } else {
                start = start_bottom;
                end[1] += size_f64;
            }
            let connection_name = format!(
                "{} <-> {} [{:.3}]", 
                neuron.id(),
                sensor.explain_one(0u32).unwrap().to_string(),
                1.0
            );

            let connections = Line::new(PlotPoints::new(vec![start, end]))
                .color(utils::color_bevy_to_egui(&connection_settings.color))
                .style(LineStyle::Solid)
                .name(&connection_name)
                .width(connection_settings.thickness);
            // let nodes = Nodes::new(vec![start, end])
            // let nodes = Nodes::new(vec![start])
            //     .name(&connection_name)
            //     .filled(true)
            //     .shape(NodeShape::Circle)
            //     .radius(size_f64 as f32 / connection_settings.connector_prop)
            //     .color(utils::color_bevy_to_egui(&connection_settings.color));
    
            if connection_settings.show { 
                ui.line(connections);
                // if connection_settings.show_connector { ui.nodes(nodes); }
            }
        }

        if settings.show_text {
            let text = RichText::new(
                PlotPoint::new(neuron_pos.0, neuron_pos.1 + size_f64 / 1.5), 
                &format!("{:.3}", neuron_activation)
            ).name(&format!("{neuron_name} activation"))
                .color(utils::color_bevy_to_egui(&settings.text_color))
                .text_size(settings.text_size / 2.0)
                .available_width(f32::INFINITY)
                .anchor(Align2::CENTER_CENTER);
            ui.rich_text(text);
    
            let text = RichText::new(
                PlotPoint::new(neuron_pos.0, neuron_pos.1), 
                &neuron_id_id.to_string()
            ).name(&neuron_name)
                .color(utils::color_bevy_to_egui(&settings.text_marked_color))
                .text_size(settings.text_size)
                .available_width(f32::INFINITY)
                .anchor(Align2::CENTER_CENTER);
            ui.rich_text(text);
    
            let text = RichText::new(
                PlotPoint::new(neuron_pos.0, neuron_pos.1 - size_f64 / 1.5), 
                &neuron_counter.to_string()
            ).name(&format!("{neuron_name} counter"))
                .color(utils::color_bevy_to_egui(&settings.text_color))
                .text_size(settings.text_size / 2.0)
                .available_width(f32::INFINITY)
                .anchor(Align2::CENTER_CENTER);
            ui.rich_text(text);
        }
    }
}