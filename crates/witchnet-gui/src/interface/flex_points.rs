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
            data(ui, sequence_1d_res, sequential_data_files_res);
            
            sampling(ui, sequence_1d_res);
            
            measures(ui, sequence_1d_res);
            
            appearance(ui, sequence_1d_res);
    });
}

fn data(
    ui: &mut Ui, 
    sequence_1d_res: &mut ResMut<Sequence1D>,
    sequential_data_files_res: &mut ResMut<SequentialDataFiles>
) {
    Grid::new("flex-points data").show(ui, |ui| {
        ui.vertical(|ui| {
            ui.set_min_width(DEFAULT_PANEL_WIDTH - 25f32);

            w::heading_label(ui, "predefined data", common::NEUTRAL_ACTIVE_COLOR);
            ui.radio_value(
                &mut sequence_1d_res.selected_data_source, 
                SequenceSelector::ComplexTrigonometric, 
                "complex trigonometric"
            );
            ui.radio_value(
                &mut sequence_1d_res.selected_data_source, 
                SequenceSelector::Tanh, 
                "tanh"
            );
            ui.radio_value(
                &mut sequence_1d_res.selected_data_source, 
                SequenceSelector::None, 
                "none"
            );

            if let Some(data_file) = sequential_data_files_res.current_data_file() {
                if let Some(data_frame) = &data_file.data_frame {
                    let mut numeric_columns = vec![];
                    for column in data_frame.get_columns() {
                        if column.is_numeric_physical() {
                            numeric_columns.push(column.name())
                        }
                    }
                    if !numeric_columns.is_empty() {
                        w::heading_label(ui, "loaded data", common::NEUTRAL_ACTIVE_COLOR);
                        for column in numeric_columns {
                            ui.radio_value(
                                &mut sequence_1d_res.selected_data_source, 
                                SequenceSelector::LoadedData(column.to_string()), 
                                column
                            );
                        }
                    }
                }
            }
            ui.separator(); ui.end_row();
        });
    });
}

fn sampling(ui: &mut Ui, sequence_1d_res: &mut ResMut<Sequence1D>) {
    Grid::new("flex-points sampling").show(ui, |ui| {
        ui.vertical(|ui| {
            ui.set_min_width(DEFAULT_PANEL_WIDTH - 25f32);

            let loaded = sequence_1d_res.loaded_sampling_method.clone();

            w::heading_label(ui, "sampling", common::NEUTRAL_ACTIVE_COLOR);

            ui.radio_value(
                &mut sequence_1d_res.selected_sampling_method, 
                SamplingMethodSelector::FlexPoints, 
                "flex-points"
            );

            if sequence_1d_res.selected_sampling_method == SamplingMethodSelector::FlexPoints {
                let id = ui.make_persistent_id("flex_points_settings");
                egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, true)
                    .show_header(ui, |ui| {
                        ui.label("settings");
                    })
                    .body(|ui| {
                        let first_derivative_box = w::checkbox_row(
                            ui, "first derivative", &mut sequence_1d_res.flex_points.first_derivative
                        );
                        if first_derivative_box.as_ref().unwrap().changed() {
                            sequence_1d_res.update_samples()
                        }
                        let second_derivative_box = w::checkbox_row(
                            ui, "second derivative", &mut sequence_1d_res.flex_points.second_derivative
                        );
                        if second_derivative_box.as_ref().unwrap().changed() {
                            sequence_1d_res.update_samples()
                        }
                        let third_derivative_box = w::checkbox_row(
                            ui, "third derivative", &mut sequence_1d_res.flex_points.third_derivative
                        );
                        if third_derivative_box.as_ref().unwrap().changed() {
                            sequence_1d_res.update_samples()
                        }
                        let fourth_derivative_box = w::checkbox_row(
                            ui, "fourth derivative", &mut sequence_1d_res.flex_points.fourth_derivative
                        );
                        if fourth_derivative_box.as_ref().unwrap().changed() {
                            sequence_1d_res.update_samples()
                        }
                    });
            }

            ui.radio_value(
                &mut sequence_1d_res.selected_sampling_method, 
                SamplingMethodSelector::RamerDouglasPeucker, 
                "ramer-douglas-peucker"
            );

            if sequence_1d_res.selected_sampling_method == SamplingMethodSelector::RamerDouglasPeucker {
                let id = ui.make_persistent_id("rdp_settings");
                egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, true)
                    .show_header(ui, |ui| {
                        ui.label("settings");
                    })
                    .body(|ui| {
                        let bounds = sequence_1d_res.rdp.epsilon_bounds.clone();
                        let slider = w::slider_row(
                            ui, 
                            "ε", 
                            &mut sequence_1d_res.rdp.epsilon, 
                            bounds
                        );
                        if slider.as_ref().unwrap().changed() {
                            sequence_1d_res.update_samples()
                        }
                    });
            }
            
            ui.radio_value(
                &mut sequence_1d_res.selected_sampling_method, 
                SamplingMethodSelector::Random, 
                "random"
            );

            if sequence_1d_res.selected_sampling_method == SamplingMethodSelector::Random {
                let id = ui.make_persistent_id("random_sampling_settings");
                egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, true)
                    .show_header(ui, |ui| {
                        ui.label("settings");
                    })
                    .body(|ui| {
                        let bounds = (2, sequence_1d_res.loaded_data.len());
                        let slider = w::slider_row_usize(
                            ui, 
                            "n", 
                            &mut sequence_1d_res.random_sampling_n, 
                            bounds
                        );
                        if slider.as_ref().unwrap().changed() {
                            sequence_1d_res.update_samples()
                        }
                    });
            }
            
            ui.radio_value(
                &mut sequence_1d_res.selected_sampling_method, 
                SamplingMethodSelector::Equal, 
                "equal"
            );

            if sequence_1d_res.selected_sampling_method == SamplingMethodSelector::Equal {
                let id = ui.make_persistent_id("equal_sampling_settings");
                egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, true)
                    .show_header(ui, |ui| {
                        ui.label("settings");
                    })
                    .body(|ui| {
                        let bounds = (2, sequence_1d_res.loaded_data.len());
                        let slider = w::slider_row_usize(
                            ui, 
                            "n", 
                            &mut sequence_1d_res.equal_sampling_n, 
                            bounds
                        );
                        if slider.as_ref().unwrap().changed() {
                            sequence_1d_res.update_samples()
                        }
                    });
            }

            ui.radio_value(
                &mut sequence_1d_res.selected_sampling_method, 
                SamplingMethodSelector::None, 
                "none"
            );
            
            ui.separator(); ui.end_row();
        });
    });
}

fn measures(ui: &mut Ui, sequence_1d_res: &mut ResMut<Sequence1D>) {
    w::heading_label(ui, "measures", common::NEUTRAL_ACTIVE_COLOR);

    let measures = &sequence_1d_res.sampling_measures;
    ui.columns(2, |cols| {
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