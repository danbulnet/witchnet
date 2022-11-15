use std::{
    sync::{ Arc, RwLock },
    collections::HashMap
};

use bevy_egui::egui::{
    TextStyle,
    Align2
};

use magds::asynchronous::sensor::SensorConatiner;

use witchnet_common::{
    sensor::{SensorAsync, SensorData},
    data::DataTypeValue, neuron::NeuronID
};

use crate::{
    resources::{
        magds::PositionXY,
        appearance::{ SensorAppearance, ConnectionAppearance }
    },
    utils,
    widgets::plot::{ 
        RichText,
        PlotUi,
        PlotPoint,
        Points,
        MarkerShape,
        Line, 
        LineStyle,
        PlotPoints,
        Nodes,
        NodeShape
    }
};

fn weight(first: &DataTypeValue, second: &DataTypeValue, range: f32) -> f32 {
    1.0f32 - (first.distance(second) as f32).abs() / range
}

pub(crate) fn sensor(
    ui: &mut PlotUi, 
    name: &str, 
    origin: (f64, f64),
    sensor: Arc<RwLock<SensorConatiner>>,
    settings: &SensorAppearance,
    connection_settings: &ConnectionAppearance,
    flip: bool
) -> (HashMap<NeuronID, [f64; 2]>, f64) {
    let flip_sign = if flip { -1.0 } else { 1.0 };

    let size_f64 = settings.size as f64;

    let sensor = sensor.read().unwrap();
    let sensor_id = sensor.id();
    let values = sensor.values();
    let neurons = sensor.neurons();

    if values.is_empty() { return (HashMap::new(), 0.0f64) }
    let no_elements = values.len();
    let range = if sensor.data_category().is_categorical() {
        values.len() as f32
    } else {
        values.last().unwrap().distance(values.first().unwrap()) as f32
    };

    let points_vec: Vec<[f64; 2]> = (0..no_elements)
        .map(|x| [origin.0 + x as f64 * 0.25 * size_f64, origin.1])
        .collect();

    let title_point = [
        points_vec[no_elements / 2][0], 
        points_vec[no_elements / 2][1] + flip_sign * 1.8 * size_f64
    ];

    for i in 0..no_elements {
        let title_start = [points_vec[i][0], points_vec[i][1] + flip_sign * size_f64 * 0.05];
        let title_end = [title_point[0], title_point[1] - flip_sign * size_f64 * 0.05];
        let second_neuron = neurons[i].read().unwrap();
        let title_connection_name = format!(
            "{} <-> {name} [1.0]", 
            second_neuron.explain_one(sensor_id).unwrap().to_string()
        );

        if i == 0 {
            let title_connections = Line::new(PlotPoints::new(vec![title_start, title_end]))
                .color(utils::color_bevy_to_egui(&connection_settings.color))
                .style(LineStyle::Solid)
                .name(&title_connection_name)
                .width(connection_settings.thickness);
            let nodes = Nodes::new(vec![title_start, title_end])
                .name(&title_connection_name)
                .filled(true)
                .shape(NodeShape::Circle)
                .radius(size_f64 as f32 / 5f32)
                .color(utils::color_bevy_to_egui(&connection_settings.color));
            if connection_settings.show { 
                ui.line(title_connections);
                ui.nodes(nodes);
            }
        } else if i > 0 {
            let start = [points_vec[i - 1][0] + size_f64 * 0.05, points_vec[i - 1][1]];
            let end = [points_vec[i][0] - size_f64 * 0.05, points_vec[i][1]];

            let first_neuron = neurons[i - 1].read().unwrap();
            let connection_name = format!(
                "{} <-> {} [{:.3}]", 
                first_neuron.explain_one(sensor_id).unwrap().to_string(),
                second_neuron.explain_one(sensor_id).unwrap().to_string(),
                weight(&values[i], &values[i - 1], range)
            );

            let connections = Line::new(PlotPoints::new(vec![start, end]))
                .color(utils::color_bevy_to_egui(&connection_settings.color))
                .style(LineStyle::Solid)
                .name(&connection_name)
                .width(connection_settings.thickness);
            let title_connections = Line::new(PlotPoints::new(vec![title_start, title_end]))
                .color(utils::color_bevy_to_egui(&connection_settings.color))
                .style(LineStyle::Solid)
                .name(&title_connection_name)
                .width(connection_settings.thickness);
            let nodes = Nodes::new(vec![start, end, title_start, title_end])
                .name(&connection_name)
                .filled(true)
                .shape(NodeShape::Circle)
                .radius(size_f64 as f32 / 5f32)
                .color(utils::color_bevy_to_egui(&connection_settings.color));

            if connection_settings.show { 
                ui.line(title_connections);
                ui.line(connections);
                ui.nodes(nodes);
            }
        }
    }

    for (i, point) in (&points_vec).into_iter().enumerate() {
        let neuron_count = neurons[i].read().unwrap().counter();
        let value = format!("{} [{neuron_count}]", values[i].to_string());

        let nodes = Nodes::new(vec![*point])
            .name(value)
            .filled(true)
            .shape(NodeShape::Circle)
            .radius(size_f64 as f32)
            .color(utils::color_bevy_to_egui(&settings.primary_color));    
        if settings.show { ui.nodes(nodes); }
    }

    let text = RichText::new(
        PlotPoint::new(
            points_vec[no_elements / 2][0], 
            points_vec[no_elements / 2][1] + flip_sign * 2.0 * size_f64
        ), 
        name
    )
        .name(name)
        .color(utils::color_bevy_to_egui(&settings.text_active_color))
        .text_style(TextStyle::Body)
        .available_width(f32::INFINITY)
        .anchor(Align2::CENTER_CENTER);
    ui.rich_text(text);

    let sensor_node = Nodes::new(vec![title_point])
        .name(name)
        .filled(true)
        .shape(NodeShape::Circle)
        .radius(size_f64 as f32)
        .color(utils::color_bevy_to_egui(&settings.text_active_color));    
    if settings.show { ui.nodes(sensor_node); }

    let first_value = values[0].to_string();
    let text = RichText::new(
        PlotPoint::new(
            points_vec[0][0] - 0.25 * size_f64, 
            points_vec[0][1]
        ), 
        &first_value
    )
        .name(&first_value)
        .color(utils::color_bevy_to_egui(&settings.primary_color))
        .text_style(TextStyle::Small)
        .available_width(35.0 * size_f64 as f32)
        .anchor(Align2::RIGHT_CENTER);
    ui.rich_text(text);

    let last_value = values.last().unwrap().to_string();
    let text = RichText::new(
        PlotPoint::new(
            points_vec.last().unwrap()[0] + 0.25 * size_f64, 
            points_vec.last().unwrap()[1]
        ), 
        &last_value
    )
        .name(&last_value)
        .color(utils::color_bevy_to_egui(&settings.primary_color))
        .text_style(TextStyle::Small)
        .available_width(35.0 * size_f64 as f32)
        .anchor(Align2::LEFT_CENTER);
    ui.rich_text(text);

    let value_point_map: HashMap<NeuronID, [f64; 2]> = HashMap::from_iter(
        neurons.into_iter().map(|x| x.read().unwrap().id()).zip(points_vec.clone().into_iter())
    );

    (value_point_map, points_vec.last().unwrap()[0] - points_vec[0][0])
}

pub(crate) fn sensory_field(
    ui: &mut PlotUi, 
    name: &str, 
    origin: (f64, f64),
    sensor: Arc<RwLock<SensorConatiner>>,
    position_xy_res: &mut PositionXY,
    settings: &SensorAppearance,
    connection_settings: &ConnectionAppearance,
    flip: bool
) {
    let size_f64 = settings.size as f64;

    let sensor = sensor.read().unwrap();
    let sensor_id = sensor.id();
    let values = sensor.values();
    let neurons = sensor.neurons();

    if values.is_empty() { return }
    let no_elements = values.len();
    let range = if sensor.data_category().is_categorical() {
        values.len() as f32
    } else {
        values.last().unwrap().distance(values.first().unwrap()) as f32
    };

    let title_point = position_xy_res.sensors[&sensor_id];

    let sensor_positions = &mut position_xy_res.sensor_neurons;

    for i in 0..no_elements {
        let second_neuron = neurons[i].read().unwrap();
        let second_neuron_pos = sensor_positions[&second_neuron.id()];
        let title_start = [second_neuron_pos.0, second_neuron_pos.1];
        let title_end = [title_point.0, title_point.1];
        let title_connection_name = format!(
            "{} <-> {name} [1.0]", 
            second_neuron.explain_one(sensor_id).unwrap().to_string()
        );

        if i == 0 {
            // let title_connections = Line::new(PlotPoints::new(vec![title_start, title_end]))
            //     .color(utils::color_bevy_to_egui(&connection_settings.color))
            //     .style(LineStyle::Solid)
            //     .name(&title_connection_name)
            //     .width(connection_settings.thickness);
            let nodes = Nodes::new(vec![title_start, title_end])
                .name(&title_connection_name)
                .filled(true)
                .shape(NodeShape::Circle)
                .radius(size_f64 as f32 / 5f32)
                .color(utils::color_bevy_to_egui(&connection_settings.color));
            if connection_settings.show { 
                // ui.line(title_connections);
                ui.nodes(nodes);
            }
        } else if i > 0 {
            let first_neuron = neurons[i - 1].read().unwrap();
            let first_neuron_pos = sensor_positions[&first_neuron.id()];

            let start = [first_neuron_pos.0 + size_f64, first_neuron_pos.1];
            let end = [second_neuron_pos.0 - size_f64, second_neuron_pos.1];

            let connection_name = format!(
                "{} <-> {} [{:.3}]", 
                first_neuron.explain_one(sensor_id).unwrap().to_string(),
                second_neuron.explain_one(sensor_id).unwrap().to_string(),
                weight(&values[i], &values[i - 1], range)
            );

            let connections = Line::new(PlotPoints::new(vec![start, end]))
                .color(utils::color_bevy_to_egui(&connection_settings.color))
                .style(LineStyle::Solid)
                .name(&connection_name)
                .width(connection_settings.thickness);
            // let title_connections = Line::new(PlotPoints::new(vec![title_start, title_end]))
            //     .color(utils::color_bevy_to_egui(&connection_settings.color))
            //     .style(LineStyle::Solid)
            //     .name(&title_connection_name)
            //     .width(connection_settings.thickness);
            let nodes = Nodes::new(vec![start, end])
                .name(&connection_name)
                .filled(true)
                .shape(NodeShape::Circle)
                .radius(size_f64 as f32 / 3f32)
                .color(utils::color_bevy_to_egui(&connection_settings.color));

            if connection_settings.show { 
                // ui.line(title_connections);
                ui.line(connections);
                ui.nodes(nodes);
            }
        }
    }

    for neuron in &neurons {
        let neuron =  neuron.read().unwrap();
        let neuron_count = neuron.counter();
        let value = format!("{} [{neuron_count}]", neuron.value().to_string());

        let neuron_pos = sensor_positions[&neuron.id()];
        let nodes = Nodes::new(vec![[neuron_pos.0, neuron_pos.1]])
            .name(value)
            .filled(true)
            .shape(NodeShape::Circle)
            .radius(size_f64 as f32)
            .color(utils::color_bevy_to_egui(&settings.primary_color));    
        if settings.show { ui.nodes(nodes); }
    }

    // let text = RichText::new(
    //     PlotPoint::new(
    //         points_vec[no_elements / 2][0], 
    //         points_vec[no_elements / 2][1] + flip_sign * 2.0 * size_f64
    //     ), 
    //     name
    // )
    //     .name(name)
    //     .color(utils::color_bevy_to_egui(&settings.text_active_color))
    //     .text_style(TextStyle::Body)
    //     .available_width(f32::INFINITY)
    //     .anchor(Align2::CENTER_CENTER);
    // ui.rich_text(text);

    let sensor_node = Nodes::new(vec![[title_point.0, title_point.1]])
        .name(name)
        .filled(true)
        .shape(NodeShape::Circle)
        .radius(size_f64 as f32)
        .color(utils::color_bevy_to_egui(&settings.text_active_color));    
    if settings.show { ui.nodes(sensor_node); }

    // let first_value = values[0].to_string();
    // let text = RichText::new(
    //     PlotPoint::new(
    //         points_vec[0][0] - 0.25 * size_f64, 
    //         points_vec[0][1]
    //     ), 
    //     &first_value
    // )
    //     .name(&first_value)
    //     .color(utils::color_bevy_to_egui(&settings.primary_color))
    //     .text_style(TextStyle::Small)
    //     .available_width(35.0 * size_f64 as f32)
    //     .anchor(Align2::RIGHT_CENTER);
    // ui.rich_text(text);

    // let last_value = values.last().unwrap().to_string();
    // let text = RichText::new(
    //     PlotPoint::new(
    //         points_vec.last().unwrap()[0] + 0.25 * size_f64, 
    //         points_vec.last().unwrap()[1]
    //     ), 
    //     &last_value
    // )
    //     .name(&last_value)
    //     .color(utils::color_bevy_to_egui(&settings.primary_color))
    //     .text_style(TextStyle::Small)
    //     .available_width(35.0 * size_f64 as f32)
    //     .anchor(Align2::LEFT_CENTER);
    // ui.rich_text(text);

    // let value_point_map: HashMap<NeuronID, [f64; 2]> = HashMap::from_iter(
    //     neurons.into_iter().map(|x| x.read().unwrap().id()).zip(points_vec.clone().into_iter())
    // );

    // (value_point_map, points_vec.last().unwrap()[0] - points_vec[0][0])
}