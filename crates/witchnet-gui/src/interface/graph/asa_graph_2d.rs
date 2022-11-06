use bevy_egui::egui::{
    Color32,
    plot::{ Polygon, PlotUi, LineStyle }
};

use crate::{
    resources::appearance::SensorAppearance,
    interface::shapes,
    utils
};

pub fn element(
    ui: &mut PlotUi, 
    name: &str, 
    origin: (f64, f64), 
    settings: &SensorAppearance
) {
    shapes::rounded_box_r25r01(
        ui, 
        name, 
        origin, 
        (settings.size as f64, settings.size as f64), 
        settings.rounded, 
        utils::color_bevy_to_egui(&settings.primary_color)
    );
}