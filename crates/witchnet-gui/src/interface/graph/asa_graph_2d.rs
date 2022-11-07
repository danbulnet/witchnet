use std::sync::{ Arc, RwLock };

use bevy_egui::egui::{
    Color32,
    plot::{ Text, PlotUi, PlotPoint, Points, MarkerShape }
};

use magds::asynchronous::sensor::SensorConatiner;

use witchnet_common::{
    sensor::SensorAsync,
    data::DataTypeValue
};

use crate::{
    resources::appearance::SensorAppearance,
    interface::shapes,
    utils
};

pub fn elements_poly(
    ui: &mut PlotUi, 
    name: &str, 
    origin: (f64, f64), 
    sensor: Arc<RwLock<SensorConatiner>>,
    settings: &SensorAppearance
) {
    let sensor = sensor.read().unwrap();
    let values = sensor.values();
    let neurons = sensor.neurons();

    let points: Vec<[f64; 2]> = (0..neurons.len())
        .map(|x| [x as f64 * 2.0 * settings.size as f64, 0f64])
        .collect();

    for point in points {
        shapes::rounded_box_r25r01(
            ui, 
            name, 
            (point[0], point[1]), 
            (settings.size as f64, settings.size as f64), 
            settings.rounded, 
            utils::color_bevy_to_egui(&settings.primary_color)
        );
    
        let text_position = PlotPoint::new(
            point[0] + settings.size as f64 / 10.0, 
            point[1] + settings.size as f64 / 2.0
        );
        let text = Text::new(text_position, "wow")
            .name("text")
            .color(utils::color_bevy_to_egui(&settings.text_color));
        ui.text(text);
    }
}

pub fn elements(
    ui: &mut PlotUi, 
    name: &str, 
    origin: (f64, f64),
    sensor: Arc<RwLock<SensorConatiner>>,
    settings: &SensorAppearance
) {
    let sensor = sensor.read().unwrap();
    let values = sensor.values();
    let neurons = sensor.neurons();

    let points_vec: Vec<[f64; 2]> = (0..neurons.len())
        .map(|x| [x as f64 * 2.0 * settings.size as f64, origin.1])
        .collect();

    let points = Points::new(points_vec.clone())
        .name(name)
        .filled(true)
        .radius(settings.size)
        .shape(MarkerShape::Diamond)
        .color(utils::color_bevy_to_egui(&settings.primary_color));
    if settings.show { ui.points(points); }

    let text_position = PlotPoint::new(
        origin.0, 
        origin.1
    );
    for (i, value) in (&values).into_iter().enumerate() {
        let text = Text::new(PlotPoint::new(points_vec[i][0], points_vec[i][1]), value.to_string())
            .name("text")
            .color(utils::color_bevy_to_egui(&settings.text_active_color));
        ui.text(text);
    }
}