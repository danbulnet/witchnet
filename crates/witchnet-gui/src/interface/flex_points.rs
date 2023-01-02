use bevy::prelude::*;

use bevy_egui::egui::{ 
    self, 
    Ui,
    Grid,
    ComboBox,
    plot::{ MarkerShape, LineStyle }
};

use crate::{
    resources::{
        sequence_1d::{ Sequence1D, SamplingMethodSelector, SequenceSelector, SamplingMeasures },
        layout::DEFAULT_PANEL_WIDTH,
        common, 
        sequential_data::SequentialDataFiles
    },
    interface::widgets as w,
    utils
};

pub(crate) fn flex_points(
    ui: &mut Ui,
    sequence_1d_res: &mut ResMut<Sequence1D>,
    sequential_data_files_res: &mut ResMut<SequentialDataFiles>
) {
    egui::ScrollArea::vertical()
        .stick_to_bottom(true)
        .show(ui, |ui| {           
            measures(ui, sequence_1d_res);
            
            appearance(ui, sequence_1d_res);
    });
}

fn measures(ui: &mut Ui, sequence_1d_res: &mut ResMut<Sequence1D>) {
    w::heading_label(ui, "measures", common::NEUTRAL_ACTIVE_COLOR);

    let measures = &sequence_1d_res.sampling_measures;
    ui.columns(2, |cols| {
        cols[0].label("sampled points");
        cols[1].label(
            format!(
                "{} / {}",
                sequence_1d_res.loaded_samples.len(),
                sequence_1d_res.loaded_data.len()
            )
        );

        cols[0].label("cf");
        cols[1].label(SamplingMeasures::value_to_string(&measures.compression_factor));

        cols[0].label("rmse");
        cols[1].label(SamplingMeasures::value_to_string(&measures.rmse));

        cols[0].label("nrmse");
        cols[1].label(SamplingMeasures::value_to_string(&measures.nrmse));

        cols[0].label("minrmse");
        cols[1].label(SamplingMeasures::value_to_string(&measures.minrmse));

        cols[0].label("prd");
        cols[1].label(SamplingMeasures::value_to_string(&measures.prd));

        cols[0].label("nprd");
        cols[1].label(SamplingMeasures::value_to_string(&measures.nprd));

        cols[0].label("qs");
        cols[1].label(SamplingMeasures::value_to_string(&measures.quality_score));

        cols[0].label("nqs");
        cols[1].label(SamplingMeasures::value_to_string(&measures.normalized_quality_score));
    });

    ui.separator(); ui.end_row();
}

fn appearance(ui: &mut Ui, sequence_1d_res: &mut ResMut<Sequence1D>) {
    line_settings(ui, sequence_1d_res);

    ui.separator(); ui.end_row();

    samples_settings(ui, sequence_1d_res);

    ui.separator(); ui.end_row();

    approximation_line_settings(ui, sequence_1d_res);

    ui.separator(); ui.end_row();
}

fn line_settings(ui: &mut Ui, sequence_1d_res: &mut ResMut<Sequence1D>) {
    w::heading_label(ui, "line settings", common::NEUTRAL_ACTIVE_COLOR);

    w::color_picker_row(ui, "color", &mut sequence_1d_res.line_color);

    let bounds = sequence_1d_res.line_width_bounds.clone();
    w::slider_row(
        ui, 
        "size", 
        &mut sequence_1d_res.line_width, 
        bounds
    );
    
    let bounds = sequence_1d_res.aspect_ratio_bounds.clone();
    w::slider_row(
        ui, 
        "h:w", 
        &mut sequence_1d_res.aspect_ratio, 
        bounds
    );

    ui.horizontal(|ui| {
        w::heading_label(ui, "style", common::NEUTRAL_COLOR);
        let current_style_str = utils::line_style_to_string(
            &sequence_1d_res.line_style
        );
        ComboBox::from_id_source("line_style")
            .selected_text(utils::shrink_str(&current_style_str, 25))
            .show_ui(ui, |ui| {
                let values = [
                    LineStyle::Solid,
                    LineStyle::Dashed { 
                        length: sequence_1d_res.line_style_spacing 
                    },
                    LineStyle::Dotted { 
                        spacing: sequence_1d_res.line_style_spacing 
                    },
                ];
                for value in values {
                    ui.selectable_value(
                        &mut sequence_1d_res.line_style, 
                        value, 
                        utils::line_style_to_string(&value)
                    );
                }
            }
        );
    });

    #[allow(unused)]
    if &sequence_1d_res.line_style != &LineStyle::Solid {
        let bounds = sequence_1d_res.line_style_spacing_bounds.clone();
        let slider = w::slider_row(
            ui, 
            "spacing", 
            &mut sequence_1d_res.line_style_spacing, 
            bounds
        );
        if slider.as_ref().unwrap().changed() {
            match &sequence_1d_res.line_style {
                LineStyle::Dashed { length } => {
                    sequence_1d_res.line_style = LineStyle::Dashed { 
                        length: sequence_1d_res.line_style_spacing 
                    }
                },
                LineStyle::Dotted { spacing } => {
                    sequence_1d_res.line_style = LineStyle::Dotted { 
                        spacing: sequence_1d_res.line_style_spacing 
                    }
                },
                _ => (),
            }
        }
    }
}

fn samples_settings(ui: &mut Ui, sequence_1d_res: &mut ResMut<Sequence1D>) {
    w::heading_label(ui, "samples settings", common::NEUTRAL_ACTIVE_COLOR);

    w::color_picker_row(ui, "color", &mut sequence_1d_res.samples_color);

    let bounds = sequence_1d_res.samples_bounds.clone();
    w::slider_row(
        ui, 
        "size", 
        &mut sequence_1d_res.samples_radius, 
        bounds
    );

    ui.horizontal(|ui| {
        w::heading_label(ui, "shape", common::NEUTRAL_COLOR);
        let current_shape_str = utils::shape_to_string(&sequence_1d_res.samples_shape);
        ComboBox::from_id_source(&current_shape_str)
            .selected_text(utils::shrink_str(&current_shape_str, 25))
            .show_ui(ui, |ui| {
                let values = [
                    MarkerShape::Circle,
                    MarkerShape::Diamond,
                    MarkerShape::Square,
                    MarkerShape::Cross,
                    MarkerShape::Plus,
                    MarkerShape::Up,
                    MarkerShape::Down,
                    MarkerShape::Left,
                    MarkerShape::Right,
                    MarkerShape::Asterisk
                ];
                for value in values {
                    ui.selectable_value(
                        &mut sequence_1d_res.samples_shape, 
                        value, 
                        utils::shape_to_string(&value)
                    );
                }
            }
        );
    });
}

fn approximation_line_settings(ui: &mut Ui, sequence_1d_res: &mut ResMut<Sequence1D>) {
    w::heading_label(ui, "approximation line settings", common::NEUTRAL_ACTIVE_COLOR);

    w::color_picker_row(ui, "color", &mut sequence_1d_res.approximation_line_color);

    let bounds = sequence_1d_res.approximation_line_width_bounds.clone();
    w::slider_row(
        ui, 
        "size", 
        &mut sequence_1d_res.approximation_line_width, 
        bounds
    );

    ui.horizontal(|ui| {
        w::heading_label(ui, "style", common::NEUTRAL_COLOR);
        let current_style_str = utils::line_style_to_string(
            &sequence_1d_res.approximation_line_style
        );
        ComboBox::from_id_source("approximation_line_style")
            .selected_text(utils::shrink_str(&current_style_str, 25))
            .show_ui(ui, |ui| {
                let values = [
                    LineStyle::Solid,
                    LineStyle::Dashed { 
                        length: sequence_1d_res.approximation_line_style_spacing 
                    },
                    LineStyle::Dotted { 
                        spacing: sequence_1d_res.approximation_line_style_spacing 
                    },
                ];
                for value in values {
                    ui.selectable_value(
                        &mut sequence_1d_res.approximation_line_style, 
                        value, 
                        utils::line_style_to_string(&value)
                    );
                }
            }
        );
    });

    #[allow(unused)]
    if &sequence_1d_res.approximation_line_style != &LineStyle::Solid {
        let bounds = sequence_1d_res.approximation_line_style_spacing_bounds.clone();
        let slider = w::slider_row(
            ui, 
            "spacing", 
            &mut sequence_1d_res.approximation_line_style_spacing, 
            bounds
        );
        if slider.as_ref().unwrap().changed() {
            match &sequence_1d_res.approximation_line_style {
                LineStyle::Dashed { length } => {
                    sequence_1d_res.approximation_line_style = LineStyle::Dashed { 
                        length: sequence_1d_res.approximation_line_style_spacing 
                    }
                },
                LineStyle::Dotted { spacing } => {
                    sequence_1d_res.approximation_line_style = LineStyle::Dotted { 
                        spacing: sequence_1d_res.approximation_line_style_spacing 
                    }
                },
                _ => (),
            }
        }
    }
}