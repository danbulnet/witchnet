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
        appearance::Appearance,
        sequence_1d::Sequence1D
    },
    utils
};

pub(crate) fn simulation(
    ui: &mut Ui,
    sequence_1d_res: &mut ResMut<Sequence1D>,
    _appearance_res: &mut ResMut<Appearance>,
) {
    if let Some(loaded_name) = &sequence_1d_res.loaded_data_name {
        if let Some(selected_name) = &sequence_1d_res.selected_data_name {
            if loaded_name != selected_name {
                let mut example = sequence_1d_res.data_examples.first().unwrap().clone();
                for current_example in &sequence_1d_res.data_examples {
                    if current_example.0 == *selected_name {
                        example = current_example.clone();
                    }
                };
                sequence_1d_res.loaded_data_name = Some(example.0.clone());
                sequence_1d_res.loaded_data = Some(example.1());
                sequence_1d_res.loaded_samples = Some(
                    sequence_1d_res.loaded_sampling_method.samples(
                        sequence_1d_res.loaded_data.as_ref().unwrap()
                    )
                );
            }
        }
    } else {
        let example = sequence_1d_res.data_examples.first().unwrap().clone();
        sequence_1d_res.loaded_data_name = Some(example.0.clone());
        sequence_1d_res.loaded_data = Some(example.1());
        sequence_1d_res.loaded_samples = Some(
            sequence_1d_res.loaded_sampling_method.samples(
                sequence_1d_res.loaded_data.as_ref().unwrap()
            )
        );
    }

    if sequence_1d_res.loaded_sampling_method != sequence_1d_res.selected_sampling_method {
        sequence_1d_res.loaded_sampling_method = sequence_1d_res.selected_sampling_method.clone();
        sequence_1d_res.loaded_samples = Some(
            sequence_1d_res.loaded_sampling_method.samples(
                sequence_1d_res.loaded_data.as_ref().unwrap()
            )
        )
    }

    sequence_1d(ui, sequence_1d_res);
}


fn sequence_1d(ui: &mut Ui, sequence_1d_res: &mut ResMut<Sequence1D>) {
    ui.label("Zoom in on the X-axis to see hours and minutes");

    Plot::new("custom_axes")
        .data_aspect(1.0)
        // .data_aspect(2.0 * MINS_PER_DAY as f32)
        // .x_axis_formatter(x_fmt)
        // .y_axis_formatter(y_fmt)
        // .x_grid_spacer(x_grid)
        // .label_formatter(label_fmt)
        .show(ui, |plot_ui| {
            if let Some(data) = &sequence_1d_res.loaded_data {
                plot_ui.line(
                    Line::new(PlotPoints::from(data.clone()))
                        .color(utils::color_bevy_to_egui(&sequence_1d_res.line_color))
                        .width(1.0)
                );
            }
            
            if let Some(samples) = &sequence_1d_res.loaded_samples {
                let points = Points::new(samples.clone())
                    .name("samples")
                    .filled(true)
                    .radius(sequence_1d_res.samples_radius)
                    .shape(MarkerShape::Circle)
                    .color(utils::color_bevy_to_egui(&sequence_1d_res.samples_color));
                plot_ui.points(points);
            }

        });
}