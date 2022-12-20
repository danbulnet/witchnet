use std::{
    env,
    sync::Arc
};

use bevy::prelude::*;

use bevy_egui::egui::{ self, Ui, RichText };

use rfd::FileDialog;

use witchnet_common::{
    sensor::SensorAsync, 
    connection::collective::defining::ConstantOneWeightAsync
};

use magds::asynchronous::parser;

use crate::{
    interface::{
        widgets,
        graph::sequential_model_positions
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
        sequential_model::{ 
            SequentialMAGDS,
            SequentialModelLoadedDatasets,
            SequentialModelLoadedDataset,
            SequentialModelPositions,
            ADDED_TO_SEQUENTIAL_MODEL_COLOR
        }
    }
};

pub(crate) fn sequential_data_window(
    ui: &mut Ui,
    data_files_res: &mut ResMut<SequentialDataFiles>,
    loaded_datasets_res: &mut ResMut<SequentialModelLoadedDatasets>,
    magds_res: &mut ResMut<SequentialMAGDS>,
    position_xy_res: &mut ResMut<SequentialModelPositions>,
    appearance_res: &mut ResMut<Appearance>,
) {
    egui::ScrollArea::vertical()
        .stick_to_bottom(true)
        .show(ui, |ui| {
            ui.set_min_width(DATA_PANEL_WIDTH);

            file_button_row(ui, "load", &["csv"], data_files_res);
            
            data_points(ui, data_files_res);

            features_list(ui, data_files_res);
            
            add_magds_button_row(
                ui, 
                data_files_res, 
                loaded_datasets_res,
                magds_res,
                position_xy_res,
                appearance_res
            );

            loaded_files(ui, loaded_datasets_res);
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
                    RichText::new(widgets::shrink_str(&data_file.name, 23))
                        .monospace()
                        .size(STANDARD_MONOSPACE_TEXT_SIZE)
                        .color(FILE_NAME_OK_COLOR)
                } else {
                    RichText::new(widgets::shrink_str(&data_file.name, 23))
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
    ui.end_row();
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
            widgets::checkbox_row(ui, "exequal sampling", &mut data_file.exequal_sampling);
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
    loaded_datasets_res: &mut ResMut<SequentialModelLoadedDatasets>,
    sequential_model_res: &mut ResMut<SequentialMAGDS>,
    position_xy_res: &mut ResMut<SequentialModelPositions>,
    appearance_res: &mut ResMut<Appearance>
) {
    if let Some(data_file) = data_files_res.current_data_file() {
        ui.separator(); ui.end_row();
        ui.horizontal(|ui| {
            let add_button = ui.button("add to sequential model");
            if add_button.clicked() {
                if let Some(df) = &data_file.data_frame {
                    let df_name = &data_file.name;
                    // {
                        // let df_name = df_name.strip_suffix(".csv").unwrap_or(df_name);
                        // let skip_features: Vec<&str> = (&data_file.features).into_iter()
                        //     .filter(|(_key, value)| !**value)
                        //     .map(|(key, _value)| &**key)
                        //     .collect();
                        // let mut magds = magds_res.0.write().unwrap();
                        // parser::add_df_to_magds(
                        //     &mut magds, 
                        //     df_name, 
                        //     df, 
                        //     &skip_features, 
                        //     data_file.rows_limit, 
                        //     data_file.random_pick,
                        //     Arc::new(ConstantOneWeightAsync),
                        //     0.00001,
                        //     1
                        // );
                    // }

                    // let sequential_model = sequential_model_res.0.read().unwrap();
                    // for sensor in sequential_model.sensors() {
                    //     let mut sensor = sensor.write().unwrap();
                    //     let value = sensor.values().first().unwrap().clone();
                    //     let _ = sensor.activate(&value, 1.0f32, true, true);
                    // }
                    
                    // let sensor_appearance = appearance_res.sensors[&Selector::All].clone();
                    // for sensor_name in sequential_model.sensors_names() {
                    //     let sensor_key = &Selector::One(sensor_name.clone());
                    //     if !appearance_res.sensors.contains_key(sensor_key) {
                    //         appearance_res.sensors.insert(
                    //             sensor_key.clone(), sensor_appearance.clone()
                    //         );
                    //     }
                    // }
                    // let neuron_appearance = appearance_res.neurons[&Selector::All].clone();
                    // for neuron_name in sequential_model.neurons_names() {
                    //     let neuron_key = &Selector::One(neuron_name.clone());
                    //     if !appearance_res.neurons.contains_key(neuron_key) {
                    //         appearance_res.neurons.insert(
                    //             neuron_key.clone(), neuron_appearance.clone()
                    //         );
                    //     }
                    // }
                    
                    let loaded_dataset = SequentialModelLoadedDataset { 
                        name: df_name.to_string(), 
                        path: data_file.path.clone(),
                        rows: data_file.rows_limit,
                        rows_total: df.height(),
                        exequal_sampling: data_file.exequal_sampling,
                        features: (&data_file.features).into_iter()
                            .filter(|(_key, value)| **value)
                            .map(|(key, _value)| key.clone())
                            .collect()
                    };
                    loaded_datasets_res.0.push(loaded_dataset);

                //     sequential_model_positions::set_positions(
                //         &sequential_model,
                //         (0.0, 0.0),
                //         position_xy_res, 
                //         appearance_res
                //     );
                }
            }
        });
    }
    ui.end_row();
}

pub(crate) fn loaded_files(ui: &mut Ui, loaded_datasets_res: &mut ResMut<SequentialModelLoadedDatasets>) {
    ui.separator(); ui.end_row();
    ui.label(RichText::new("loaded data").color(NEUTRAL_ACTIVE_COLOR).strong());
    ui.end_row();
    
    if loaded_datasets_res.0.is_empty() {
        let label_widget = RichText::new("no data")
            .monospace()
            .size(STANDARD_TEXT_SIZE)
            .color(NEUTRAL_INACTIVE_COLOR);
        ui.label(label_widget);
    }

    for dataset in &loaded_datasets_res.0 {
        let label_widget = RichText::new(&dataset.name)
            .monospace()
            .size(STANDARD_MONOSPACE_TEXT_SIZE)
            .color(ADDED_TO_SEQUENTIAL_MODEL_COLOR);
        ui.label(label_widget);

        let rows_text = format!(
            "{} of {} {} rows",
            dataset.rows,
            dataset.rows_total,
            if dataset.exequal_sampling { "random" } else { "consecutive" }
        );
        let label_widget = RichText::new(widgets::shrink_str(&rows_text, 48))
            .size(SMALL_TEXT_SIZE)
            .color(NEUTRAL_COLOR);
        ui.label(label_widget);

        for feature in &dataset.features {
            let label_widget = RichText::new(widgets::shrink_str(feature, 48))
                .size(SMALL_TEXT_SIZE)
                .color(NEUTRAL_INACTIVE_COLOR);
            ui.label(label_widget);
        }
    }
}