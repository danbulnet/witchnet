use std::{
    sync::{ Arc, RwLock }
};

use bevy_egui::egui::{
    Align2
};

use magds::asynchronous::magds::MAGDS;
use witchnet_common::{
    neuron:: NeuronAsync
};

use crate::{
    resources::{
        appearance::{ Appearance, Selector, NeuronAppearance }, 
        magds::MAGDSPositions
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
    },
    interface::graph::smagds::smagds_positions
};

pub(crate) fn neurons(
    ui: &mut PlotUi, 
    magds: &MAGDS,
    neuron_group: (u32, &str), 
    neurons: &[Arc<RwLock<dyn NeuronAsync>>],
    positions: &mut MAGDSPositions,
    appearance: &Appearance,
) {
    if neurons.is_empty() { return }

    let (neuron_group_id, neuron_group_name) = neuron_group;

    let neuron_settings = &appearance.neurons[&Selector::One(neuron_group_name.into())];
    let neuron_size = neuron_settings.size as f64;

    for neuron_ptr in neurons {
        let neuron = neuron_ptr.read().unwrap();

        generate_sensors_and_connections(
            ui,
            magds,
            &*neuron,
            appearance,
            positions,
            neuron_size
        );

        generate_neurons_and_labels(
            ui,
            &*neuron,
            neuron_group_name,
            positions,
            neuron_settings,
            neuron_size
        );
    }

    let group_center_pos = positions.neuron_groups[&neuron_group_id];
    let text = RichText::new(
        PlotPoint::new(group_center_pos.0, group_center_pos.1), 
        neuron_group_name
    ).name(&neuron_group_name)
        .color(utils::color_bevy_to_egui(&neuron_settings.primary_active_color))
        .text_size(neuron_settings.text_size * 1.5)
        .available_width(2.25 * neuron_size as f32)
        .anchor(Align2::CENTER_CENTER);
    ui.rich_text(text);
}

fn generate_sensors_and_connections(
    ui: &mut PlotUi, 
    magds: &MAGDS,
    neuron: &dyn NeuronAsync,
    appearance: &Appearance,
    positions: &mut MAGDSPositions,
    neuron_size: f64
) {
    let neuron_pos = positions.neurons[&neuron.id()];
    let start_top = [neuron_pos.0, neuron_pos.1 + neuron_size];
    let start_bottom = [neuron_pos.0, neuron_pos.1 - neuron_size];

    let connection_settings = &appearance.connections[&Selector::All];
    for sensor in neuron.explain() {
        let sensor = sensor.read().unwrap();
        let sensor_parent_id = sensor.id().parent_id;
        let sensor_name = magds.sensor_name(sensor_parent_id).unwrap();
        let sensor_settings = &appearance.sensors[&Selector::One(sensor_name.into())];
        let sensor_size = sensor_settings.size as f64;
        
        let (_title_point, angle) = positions.sensors[&sensor_parent_id];
        let end_origin = positions.sensor_neurons[&sensor.id()];
        let end = [end_origin.0, end_origin.1 - sensor_size];
        let start: [f64; 2];
        if end[1] > start_top[1] { start = start_top; } else { start = start_bottom; }
        let end = {
            let mut point = (end[0], end[1]);
            point = smagds_positions::rotate_point_around_origin(point, end_origin, angle);
            [point.0, point.1]
        };
        
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
        let neuron_connectors = Nodes::new(vec![start])
            .name(&connection_name)
            .filled(true)
            .shape(NodeShape::Circle)
            .radius(neuron_size as f32 / connection_settings.connector_prop)
            .color(utils::color_bevy_to_egui(&connection_settings.color));
        let sensor_connectors = Nodes::new(vec![end])
            .name(&connection_name)
            .filled(true)
            .shape(NodeShape::Circle)
            .radius(sensor_size as f32 / connection_settings.connector_prop)
            .color(utils::color_bevy_to_egui(&connection_settings.color));

        if connection_settings.show { 
            ui.line(connections);
            if connection_settings.show_connector { 
                ui.nodes(sensor_connectors); 
                ui.nodes(neuron_connectors); 
            }
        }
    }
}

fn generate_neurons_and_labels(
    ui: &mut PlotUi, 
    neuron: &dyn NeuronAsync,
    neuron_group_name: &str,
    positions: &mut MAGDSPositions,
    neuron_settings: &NeuronAppearance,
    neuron_size: f64
) {
    let neuron_pos = positions.neurons[&neuron.id()];
    let neuron_value = format!("{} [{}]", neuron.id(), neuron.counter());
    let neuron_id_id = neuron.id().id;
    let neuron_activation = neuron.activation();
    let neuron_counter = neuron.counter();
    let neuron_name = format!("{neuron_group_name}: {neuron_id_id}");
    let neuron_color = if neuron_activation >= 4.0 { 
        &neuron_settings.primary_active_color
    } else { &neuron_settings.primary_color };

    let nodes = Nodes::new(vec![[neuron_pos.0, neuron_pos.1]])
        .name(&neuron_value)
        .filled(true)
        .shape(NodeShape::Circle)
        .radius(neuron_size as f32)
        .color(utils::color_bevy_to_egui(&neuron_color));    
    if neuron_settings.show { ui.nodes(nodes); }

    if neuron_settings.show_text {
        let text = RichText::new(
            PlotPoint::new(neuron_pos.0, neuron_pos.1 + neuron_size / 1.5), 
            &format!("{:.3}", neuron_activation)
        ).name(&format!("{neuron_name} activation"))
            .color(utils::color_bevy_to_egui(&neuron_settings.text_color))
            .text_size(neuron_settings.text_size / 2.0)
            .available_width(f32::INFINITY)
            .anchor(Align2::CENTER_CENTER);
        ui.rich_text(text);

        let text = RichText::new(
            PlotPoint::new(neuron_pos.0, neuron_pos.1), 
            &neuron_id_id.to_string()
        ).name(&neuron_name)
            .color(utils::color_bevy_to_egui(&neuron_settings.text_marked_color))
            .text_size(neuron_settings.text_size)
            .available_width(f32::INFINITY)
            .anchor(Align2::CENTER_CENTER);
        ui.rich_text(text);

        let text = RichText::new(
            PlotPoint::new(neuron_pos.0, neuron_pos.1 - neuron_size / 1.5), 
            &neuron_counter.to_string()
        ).name(&format!("{neuron_name} counter"))
            .color(utils::color_bevy_to_egui(&neuron_settings.text_color))
            .text_size(neuron_settings.text_size / 2.0)
            .available_width(f32::INFINITY)
            .anchor(Align2::CENTER_CENTER);
        ui.rich_text(text);
    }
}