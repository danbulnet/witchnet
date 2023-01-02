use bevy::prelude::*;

use bevy_egui::egui::{
    Ui,
};

use crate::{
    resources::{
        appearance::Appearance,
        smagds::{ SMAGDSMain, SMAGDSPositions }
    },
    interface::graph::smagds::smagds_2d,
    widgets::plot::{
        Plot
    }
};

pub(crate) fn simulation(
    ui: &mut Ui,
    smagds_res: &mut ResMut<SMAGDSMain>,
    smagds_positions_res: &mut ResMut<SMAGDSPositions>,
    appearance_res: &mut ResMut<Appearance>,
) {
    let simulation_settings = &mut appearance_res.simulation2d;

    if smagds_res.smagds.is_some() {
        let plot = Plot::new("smagds-2d")
            .allow_scroll(false)
            .allow_boxed_zoom(true)
            .label_formatter(|name, _value| format!("{name}"))
            .show_background(true)
            .show_x(true)
            .show_y(true)
            .data_aspect(1.0)
            .x_axis_formatter(|_, _| "".to_string())
            .y_axis_formatter(|_, _| "".to_string())
            .show_axes(simulation_settings.show_grid);
        plot.show(ui, |plot_ui| {
            smagds_2d::smagds(plot_ui, smagds_res, smagds_positions_res, appearance_res);
        });
    } else {
        ui.label("click generate smagds button on sequential data pane");
    }
}