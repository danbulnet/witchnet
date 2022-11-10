use bevy::prelude::*;

use bevy_egui::egui::{
    Ui,
};

use witchnet_common::sensor::SensorAsync;

use crate::{
    resources::{
        appearance::{ Appearance, Selector },
        magds::{ MainMAGDS, PositionXY }
    },
    utils,
    interface::{
        graph::magds_2d
    },
    widgets::plot::{
        Plot
    }
};

pub(crate) fn simulation(
    ui: &mut Ui,
    magds_res: &mut ResMut<MainMAGDS>,
    position_xy_res: &mut ResMut<PositionXY>,
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
        magds_2d::magds(plot_ui, magds_res, position_xy_res, appearance_res);
    });
}