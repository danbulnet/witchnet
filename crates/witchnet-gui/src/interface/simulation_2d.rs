use bevy::prelude::*;

use bevy_egui::egui::{
    Ui,
    RichText,
    Label,
    Color32,
    Stroke,
    Sense,
    Frame,
    Rect,
    PointerButton,
    CursorIcon,
    emath::{ Pos2, RectTransform, Rot2 },
};

use witchnet_common::sensor::SensorAsync;

use crate::{
    resources::{
        appearance::{ Appearance, Selector },
        magds::MainMAGDS,
        common::NEUTRAL_COLOR
    },
    utils,
    interface::{
        shapes,
        graph::asa_graph_2d,
        transform::{ ScreenTransform, PlotBounds }
    },
    widgets::plot::{
        Plot,
        Legend,
        Points,
        MarkerShape,
        Text,
        PlotPoint
    }
};

pub(crate) fn simulation(
    ui: &mut Ui,
    magds_res: &mut ResMut<MainMAGDS>,
    appearance_res: &mut ResMut<Appearance>,
) {
    let simulation_settings = &mut appearance_res.simulation2d;

    let plot = Plot::new("lines_demo")
        .allow_scroll(false)
        .allow_boxed_zoom(false)
        .label_formatter(|name, _value| format!("{name}"))
        .show_background(true)
        .show_x(true)
        .show_y(true)
        .data_aspect(1.0)
        .x_axis_formatter(|_, _| "".to_string())
        .y_axis_formatter(|_, _| "".to_string())
        .show_axes(simulation_settings.show_grid);
    plot.show(ui, |plot_ui| {
        let neuron_settings = &appearance_res.neurons[&Selector::All];
        let sensor_settings = &appearance_res.sensors[&Selector::All];
        let connection_settings = &appearance_res.connections[&Selector::All];

        let magds = magds_res.0.read().unwrap();
        let sensors = magds.sensors();
        let neurons = magds.neurons();

        for (i, sensor) in sensors.into_iter().enumerate() {
            let sensor_id = sensor.read().unwrap().id();
            let sensor_name = magds.sensor_name(sensor_id).unwrap();
            asa_graph_2d::sensors(
                plot_ui, 
                &sensor_name, 
                (0.0, i as f64 * 100.0), 
                sensor.clone(), 
                sensor_settings,
                connection_settings
            );
        }
    });
}