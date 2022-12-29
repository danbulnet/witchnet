use bevy::prelude::*;

use bevy_egui::egui::{ 
    Ui, 
    plot::{
        Line, 
        Plot,
        PlotPoints,
        Points,
        MarkerShape
    }
};

use crate::{
    resources::{
        sequence_1d::Sequence1D, 
        sequential_data::SequentialDataFiles
    },
    utils
};

pub(crate) fn simulation(
    ui: &mut Ui,
    sequence_1d_res: &mut ResMut<Sequence1D>,
    sequential_data_files_res: &mut ResMut<SequentialDataFiles>,
) {
    sequence_1d_control(sequence_1d_res, sequential_data_files_res);

    sequence_1d(ui, sequence_1d_res);
}

fn sequence_1d_control(
    sequence_1d_res: &mut ResMut<Sequence1D>,
    sequential_data_files_res: &mut ResMut<SequentialDataFiles>
) {
    if sequence_1d_res.loaded_data_source != sequence_1d_res.selected_data_source {
        sequence_1d_res.loaded_data_source = sequence_1d_res.selected_data_source.clone();
        
        sequence_1d_res.loaded_data = sequence_1d_res.loaded_data_source.data(Some(sequential_data_files_res));

        sequence_1d_res.update_samples();
    }

    if sequence_1d_res.loaded_sampling_method != sequence_1d_res.selected_sampling_method {
        sequence_1d_res.loaded_sampling_method = sequence_1d_res.selected_sampling_method.clone();
        sequence_1d_res.update_samples()
    }
}

fn sequence_1d(ui: &mut Ui, sequence_1d_res: &mut ResMut<Sequence1D>) {
    ui.label("Zoom in on the X-axis to see hours and minutes");

    Plot::new("custom_axes")
        .data_aspect(sequence_1d_res.aspect_ratio)
        // .data_aspect(2.0 * MINS_PER_DAY as f32)
        // .x_axis_formatter(x_fmt)
        // .y_axis_formatter(y_fmt)
        // .x_grid_spacer(x_grid)
        // .label_formatter(label_fmt)
        .show(ui, |plot_ui| {
            let data = &sequence_1d_res.loaded_data;
            plot_ui.line(
                Line::new(PlotPoints::from(data.clone()))
                    .color(utils::color_bevy_to_egui(&sequence_1d_res.line_color))
                    .width(sequence_1d_res.line_width)
            );
            
            let samples = &sequence_1d_res.loaded_samples;
            let points = Points::new(samples.clone())
                .name("samples")
                .filled(true)
                .radius(sequence_1d_res.samples_radius)
                .shape(sequence_1d_res.samples_shape)
                .color(utils::color_bevy_to_egui(&sequence_1d_res.samples_color));
            plot_ui.points(points);
            let approximation = &sequence_1d_res.approximated_samples;
            plot_ui.line(
                Line::new(PlotPoints::from(approximation.clone()))
                    .color(utils::color_bevy_to_egui(&sequence_1d_res.approximation_line_color))
                    .width(sequence_1d_res.approximation_line_width)
            );
        });
}