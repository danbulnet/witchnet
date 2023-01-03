use bevy::prelude::*;

use bevy_egui::egui::{
    Ui,
};

use crate::{
    resources::{
        magds::MAGDSMain
    },
    interface::{
        graph::magds::magds_2d
    },
    widgets::plot::{
        Plot
    }
};

pub(crate) fn simulation(
    ui: &mut Ui,
    magds_res: &mut ResMut<MAGDSMain>
) {
    let plot = Plot::new("magds-2d")
        .allow_scroll(false)
        .allow_boxed_zoom(true)
        .label_formatter(|name, _value| format!("{name}"))
        .show_background(true)
        .show_x(true)
        .show_y(true)
        .data_aspect(1.0)
        .x_axis_formatter(|_, _| "".to_string())
        .y_axis_formatter(|_, _| "".to_string())
        .show_axes(magds_res.appearance.simulation2d.show_grid);
    plot.show(ui, |plot_ui| {
        magds_2d::magds(plot_ui, magds_res);
    });
}