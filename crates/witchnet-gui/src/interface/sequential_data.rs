use std::{
    env,
    sync::{ Arc, RwLock }
};

use bevy::prelude::*;

use bevy_egui::egui::{ self, Ui, RichText, Grid };

use rfd::FileDialog;

use smagds::asynchronous::smagds::SMAGDS;

use crate::{
    interface::{
        widgets,
        graph::smagds::smagds_positions
    },
    resources::{
        appearance::{ Appearance, Selector },
        common::{
            NEUTRAL_ACTIVE_COLOR,
            NEUTRAL_COLOR,
            NEUTRAL_INACTIVE_COLOR, 
            STANDARD_TEXT_SIZE, 
            SMALL_TEXT_SIZE,
            STANDARD_MONOSPACE_TEXT_SIZE 
        },
        sequential_data::{
            SequentialDataFiles,
            FILE_NAME_OK_COLOR,
            FILE_NAME_ERR_COLOR,
            DATA_PANEL_WIDTH
        },
        smagds::{ 
            SMAGDSMain,
            SMAGDSLoadedDatasets,
            SMAGDSLoadedDataset,
            SMAGDSPositions,
            ADDED_TO_SEQUENTIAL_MODEL_COLOR,
            SAMPLING_METHOD_COLOR
        }, 
        sequence_1d::{ Sequence1D, SequenceSelector, SamplingMethodSelector }, 
        layout::DEFAULT_PANEL_WIDTH
    },
    utils
};

pub(crate) fn sequential_data_window(
    ui: &mut Ui,
    data_files_res: &mut ResMut<SequentialDataFiles>,
    smagds_res: &mut ResMut<SMAGDSMain>,
    appearance_res: &mut ResMut<Appearance>,
    sequence_1d_res: &mut ResMut<Sequence1D>
) {
    egui::ScrollArea::vertical()
        .stick_to_bottom(true)
        .show(ui, |ui| {
            ui.set_min_width(DATA_PANEL_WIDTH);

            file_button_row(ui, "load", &["csv"], data_files_res);
            
            // data_points(ui, data_files_res);

            // features_list(ui, data_files_res);

            data(ui, sequence_1d_res, data_files_res);
            
            sampling(ui, sequence_1d_res);
            
            add_magds_button_row(
                ui, 
                data_files_res,
                smagds_res,
                sequence_1d_res
            );

            loaded_files(ui, &mut smagds_res.loaded_datasets);
        });
}

pub fn file_button_row(
    ui: &mut Ui, 
    label: &str,
    extensions: &[&str],
    data_files_res: &mut ResMut<SequentialDataFiles>
) {
    ui.horizontal(|ui| {
        let load_data_button = ui.button(label);
        if load_data_button.clicked() {
            load_button_clicked(extensions, data_files_res);
        }
        match data_files_res.current {
            Some(index) => {
                let data_file = &data_files_res.history[index];
                let label = if data_file.data_frame.is_some() {
                    RichText::new(utils::shrink_str(&data_file.name, 23))
                        .monospace()
                        .size(STANDARD_MONOSPACE_TEXT_SIZE)
                        .color(FILE_NAME_OK_COLOR)
                } else {
                    RichText::new(utils::shrink_str(&data_file.name, 23))
                        .monospace()
                        .size(STANDARD_MONOSPACE_TEXT_SIZE)
                        .color(FILE_NAME_ERR_COLOR)
                };
                ui.label(label)
            }
            None => ui.label(
                RichText::new("select csv file")
                    .monospace()
                    .size(STANDARD_MONOSPACE_TEXT_SIZE)
                    .color(NEUTRAL_COLOR)
            ),
        };
    });
    ui.separator(); ui.end_row();
}

fn load_button_clicked(extensions: &[&str], mut data_files_res: &mut ResMut<SequentialDataFiles>) {
    let file_path = FileDialog::new()
        .add_filter("", extensions)
        .set_directory(env::current_dir().unwrap())
        .pick_file();

    if let Some(fp) = file_path {
        SequentialDataFiles::load_data(fp, &mut data_files_res)
    }
}

pub fn data_points(ui: &mut Ui, data_files_res: &mut ResMut<SequentialDataFiles>) {
    if let Some(data_file) = data_files_res.current_data_file() {
        ui.separator(); ui.end_row();
        ui.label(egui::RichText::new("data points").color(NEUTRAL_ACTIVE_COLOR).strong());
        if let Some(df) = &mut data_file.data_frame {
            let nrows = df.height();
            widgets::slider_row_usize(
                ui, "limit", &mut data_file.rows_limit, (usize::min(1, nrows), nrows)
            );
            widgets::checkbox_row(ui, "equal sampling", &mut data_file.equal_sampling);
        }
    }
}

pub(crate) fn features_list(ui: &mut Ui, data_files_res: &mut ResMut<SequentialDataFiles>) {
    if let Some(data_file) = data_files_res.current_data_file() {
        ui.separator(); ui.end_row();
        ui.label(egui::RichText::new("features").color(NEUTRAL_ACTIVE_COLOR).strong());
        for (feature, active) in (&mut data_file.features).into_iter() {
            let label = ui.selectable_label(*active, feature);
            if label.clicked() {
                *active = !*active;
            }
            ui.end_row();
        }
    }
}

pub(crate) fn add_magds_button_row(
    ui: &mut Ui,
    data_files_res: &mut ResMut<SequentialDataFiles>,
    mut smagds_res: &mut SMAGDSMain,
    sequence_1d_res: &mut ResMut<Sequence1D>
) {
    if !sequence_1d_res.loaded_samples.is_empty() {
        ui.horizontal(|ui| {
            let add_button = ui.button("generate smagds");
            if add_button.clicked() {
                sequence_1d_control(sequence_1d_res, data_files_res);

                let sampled_data: Vec<_> = (&sequence_1d_res.loaded_samples).into_iter()
                    .map(|point| (point[0] as f32, point[1] as f32))
                    .collect();
                    
                let &mut SMAGDSMain { smagds, appearance, loaded_datasets: loaded_dataset, positions } = &mut smagds_res;

                *smagds = Some(
                    Arc::new(RwLock::new(SMAGDS::new(&sampled_data).unwrap()))
                );

                let (sensors_names, neurons_names) = {
                    let smagds = smagds.as_ref().unwrap().read().unwrap();
                    (smagds.magds.sensors_names(), smagds.magds.neurons_names())
                };

                let sensor_appearance = appearance.sensors[&Selector::All].clone();
                for sensor_name in sensors_names {
                    let sensor_key = &Selector::One(sensor_name.clone());
                    if !appearance.sensors.contains_key(sensor_key) {
                        appearance.sensors.insert(
                            sensor_key.clone(), sensor_appearance.clone()
                        );
                    }
                }
                let neuron_appearance = appearance.neurons[&Selector::All].clone();
                for neuron_name in neurons_names {
                    let neuron_key = &Selector::One(neuron_name.clone());
                    if !appearance.neurons.contains_key(neuron_key) {
                        appearance.neurons.insert(
                            neuron_key.clone(), neuron_appearance.clone()
                        );
                    }
                }
            
                let name = match &sequence_1d_res.loaded_data_source {
                    SequenceSelector::ComplexTrigonometric => "complex trigonometric",
                    SequenceSelector::ComplexTrigonometricShort => {
                        "complex trigonometric short" 
                    },
                    SequenceSelector::Tanh => "tanh",
                    SequenceSelector::LoadedData(selected_data_name) => selected_data_name,
                    SequenceSelector::None => "none",
                };

                let loaded_dataset = SMAGDSLoadedDataset {
                    name: name.to_owned(),
                    sampling_method: sequence_1d_res.selected_sampling_method.to_string(),
                    sequence_length: sequence_1d_res.loaded_data.len(),
                    samples: sampled_data.len()
                };
                smagds_res.loaded_datasets = vec![loaded_dataset];
                
                let mut smagds = smagds_res.smagds.as_ref().unwrap().write().unwrap();
                let magds = &mut smagds.magds;

                smagds_positions::set_positions(
                    &magds,
                    (0.0, 0.0),
                    &mut smagds_res.positions,
                    &smagds_res.appearance
                );
            }
        });
        ui.separator(); ui.end_row();
    }
}

pub(crate) fn loaded_files(ui: &mut Ui, loaded_datasets: &mut [SMAGDSLoadedDataset]) {
    ui.label(RichText::new("smagds loaded data").color(NEUTRAL_ACTIVE_COLOR).strong());
    ui.end_row();
    
    if loaded_datasets.is_empty() {
        let label_widget = RichText::new("no data")
            .monospace()
            .size(STANDARD_TEXT_SIZE)
            .color(NEUTRAL_INACTIVE_COLOR);
        ui.label(label_widget);
    }

    for dataset in loaded_datasets {
        let name_label = RichText::new(&dataset.name)
            .monospace()
            .size(STANDARD_MONOSPACE_TEXT_SIZE)
            .color(ADDED_TO_SEQUENTIAL_MODEL_COLOR);
        ui.label(name_label);
        
        let sampling_method_label = RichText::new(
            format!("sampling method: {}", &dataset.sampling_method)
        )
            .size(SMALL_TEXT_SIZE)
            .color(SAMPLING_METHOD_COLOR);
        ui.label(sampling_method_label);

        let rows_text = format!(
            "sampled {} from {} data points",
            dataset.samples,
            dataset.sequence_length
        );
        let label_widget = RichText::new(utils::shrink_str(&rows_text, 48))
            .size(SMALL_TEXT_SIZE)
            .color(NEUTRAL_COLOR);
        ui.label(label_widget);
    }
}

fn data(
    ui: &mut Ui, 
    sequence_1d_res: &mut ResMut<Sequence1D>,
    sequential_data_files_res: &mut ResMut<SequentialDataFiles>
) {
    Grid::new("flex-points data").show(ui, |ui| {
        ui.vertical(|ui| {
            ui.set_min_width(DEFAULT_PANEL_WIDTH - 25f32);

            widgets::heading_label(ui, "predefined data", NEUTRAL_ACTIVE_COLOR);
            
            ui.radio_value(
                &mut sequence_1d_res.selected_data_source, 
                SequenceSelector::ComplexTrigonometric, 
                "complex trigonometric"
            );
            ui.radio_value(
                &mut sequence_1d_res.selected_data_source, 
                SequenceSelector::ComplexTrigonometricShort, 
                "complex trigonometric short"
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
                        widgets::heading_label(ui, "loaded data", NEUTRAL_ACTIVE_COLOR);
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

            widgets::heading_label(ui, "sampling", NEUTRAL_ACTIVE_COLOR);

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
                        let first_derivative_box = widgets::checkbox_row(
                            ui, "first derivative", &mut sequence_1d_res.flex_points.first_derivative
                        );
                        if first_derivative_box.as_ref().unwrap().changed() {
                            sequence_1d_res.update_samples()
                        }
                        let second_derivative_box = widgets::checkbox_row(
                            ui, "second derivative", &mut sequence_1d_res.flex_points.second_derivative
                        );
                        if second_derivative_box.as_ref().unwrap().changed() {
                            sequence_1d_res.update_samples()
                        }
                        let third_derivative_box = widgets::checkbox_row(
                            ui, "third derivative", &mut sequence_1d_res.flex_points.third_derivative
                        );
                        if third_derivative_box.as_ref().unwrap().changed() {
                            sequence_1d_res.update_samples()
                        }
                        let fourth_derivative_box = widgets::checkbox_row(
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
                        let slider = widgets::slider_row(
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
                        let slider = widgets::slider_row_usize(
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
                        let slider = widgets::slider_row_usize(
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