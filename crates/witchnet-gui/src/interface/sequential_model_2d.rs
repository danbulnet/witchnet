use bevy::prelude::*;

use bevy_egui::egui::{
    Ui,
};

use crate::{
    resources::{
        appearance::Appearance,
        sequential_model::{ SequentialMAGDS, SequentialModelPositions }
    },
    interface::graph::sequential_model_2d,
    widgets::plot::{
        Plot
    }
};

pub(crate) fn simulation(
    ui: &mut Ui,
    sequential_model_res: &mut ResMut<SequentialMAGDS>,
    mut sequential_model_positions_res: &mut ResMut<SequentialModelPositions>,
    appearance_res: &mut ResMut<Appearance>,
) {
    let simulation_settings = &mut appearance_res.simulation2d;

    let plot = Plot::new("sequential-model-2d")
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
    ui.label("sequential-model-2d");
        // plot.show(ui, |plot_ui| {
    //     sequential_model_2d::magds(
    //         plot_ui, 
    //         sequential_model_res, 
    //         sequential_model_positions_res, 
    //         appearance_res
    //     );
    // });
}