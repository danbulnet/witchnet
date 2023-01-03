use std::{
    sync::{ Arc, RwLock }
};

use bevy_egui::egui::{
    Align2
};

use witchnet_common::{
    neuron:: NeuronAsync
};

use crate::{
    resources::{
        appearance::{ 
            NeuronAppearance, 
            ConnectionAppearance }, 
        smagds::SMAGDSPositions
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

pub(crate) fn neurons(
    ui: &mut PlotUi, 
    name: &str, 
    neurons: &[Arc<RwLock<dyn NeuronAsync>>],
    position_xy_res: &mut SMAGDSPositions,
    settings: &NeuronAppearance,
    connection_settings: &ConnectionAppearance,
) {
    let size_f64 = settings.size as f64;

    let neuron_positions = &position_xy_res.neurons;
    let sensor_positions = &position_xy_res.sensor_neurons;

    if neurons.is_empty() { return }

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