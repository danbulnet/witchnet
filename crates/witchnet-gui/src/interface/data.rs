use std::{
    env,
    collections::BTreeMap
};

use bevy::prelude::*;

use bevy_egui::{ 
    egui::{
        self,
        Ui,
        RichText,
        Window,
        Align2,
    }, 
    EguiContext 
};

use rfd::{ FileDialog, MessageDialog, MessageLevel };

use witchnet_common::polars;

use magds::asynchronous::parser;

use crate::{
    interface::widgets,
    resources::{
        appearance::{ Appearance, Selector },
        common::{
            INTERFACE_PADDING,
            NEUTRAL_ACTIVE_COLOR,
            NEUTRAL_COLOR,
            NEUTRAL_INACTIVE_COLOR, 
            STANDARD_TEXT_SIZE, 
            SMALL_TEXT_SIZE,
            STANDARD_MONOSPACE_TEXT_SIZE 
        },
        data::{ 
            DataFiles, 
            DataFile, 
            MIN_DATA_WIDTH, 
            DATA_X, 
            FILE_NAME_OK_COLOR,
            FILE_NAME_ERR_COLOR
        },
        magds::{ MainMAGDS, LoadedDatasets, LoadedDataset, ADDED_TO_MAGDS_COLOR }
    }
};

pub(crate) fn data_window(
    mut egui_context: ResMut<EguiContext>, 
    mut windows: ResMut<Windows>,
    mut data_files_res: ResMut<DataFiles>,
    mut loaded_datasets_res: ResMut<LoadedDatasets>,
    mut magds_res: ResMut<MainMAGDS>,
    mut appearance_res: ResMut<Appearance>,
) {
    let window = windows.get_primary_mut().unwrap();
    let max_height = window.height();

    Window::new("data")
        .anchor(Align2::LEFT_TOP, [DATA_X, INTERFACE_PADDING])
        .scroll2([false, true])
        .fixed_size([MIN_DATA_WIDTH, max_height])
        .show(egui_context.ctx_mut(), |ui| {
            ui.set_min_width(MIN_DATA_WIDTH);

            file_button_row(ui, "load", &["csv"], &mut data_files_res);
            
            data_points(ui, &mut data_files_res);

            features_list(ui, &mut data_files_res);
            
            add_magds_button_row(
                ui, 
                &mut data_files_res, 
                &mut loaded_datasets_res,
                &mut magds_res, 
                &mut appearance_res
            );

            loaded_files(ui, &mut loaded_datasets_res);
        });
}

pub fn file_button_row(
    ui: &mut Ui, 
    label: &str,
    extensions: &[&str],
    data_files_res: &mut ResMut<DataFiles>
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

fn load_button_clicked(extensions: &[&str], data_files_res: &mut ResMut<DataFiles>) {
    let file_path = FileDialog::new()
        .add_filter("", extensions)
        .set_directory(env::current_dir().unwrap())
        .pick_file();

    let file_name = match &file_path {
        Some(file_path) => {
            match file_path.file_name() {
                Some(file_name) => file_name.to_os_string().into_string().ok(),
                None => None
            }
        }
        None => None
    };

    if let Some(file_name) = file_name {
        let mut found = false;
        for (i, data_file) in (&data_files_res.history).into_iter().enumerate() {
            if &data_file.path ==  file_path.as_ref().unwrap() {
                data_files_res.current = Some(i);
                found = true;
                break
            }
        }

        if !found {
            let file_path = file_path.unwrap();
            let data_frame = polars::csv_to_dataframe(
                file_path.as_os_str().to_str().unwrap(), &vec![]
            ).ok();
            let mut features: BTreeMap<String, bool> = BTreeMap::new();
            if data_frame.is_none() {
                MessageDialog::new().set_level(MessageLevel::Error)
                    .set_title("file loading error")
                    .set_description(&format!("error converting {} to dataframe", file_name))
                    .show();
                data_files_res.current = None;
            } else {
                features.extend(
                    data_frame.as_ref().unwrap()
                        .get_column_names()
                        .into_iter()
                        .map(|x| (x.to_string(), true))
                        .collect::<BTreeMap<String, bool>>()
                );
                let nrows = if let Some(df) = &data_frame { df.height() } else { 0 };
                let data_file = DataFile { 
                    name: file_name, 
                    path: file_path, 
                    data_frame, 
                    features,
                    rows_limit: nrows,
                    random_pick: false
                };
                data_files_res.history.push(data_file);
                data_files_res.current = Some(data_files_res.history.len() - 1);
            }
        }
    }
}

pub fn data_points(ui: &mut Ui, data_files_res: &mut ResMut<DataFiles>) {
    if let Some(data_file) = data_files_res.current_data_file() {
        ui.separator(); ui.end_row();
        ui.label(egui::RichText::new("data points").color(NEUTRAL_ACTIVE_COLOR).strong());
        if let Some(df) = &mut data_file.data_frame {
            let nrows = df.height();
            widgets::slider_row_usize(
                ui, "limit", &mut data_file.rows_limit, (usize::min(1, nrows), nrows)
            );
            widgets::checkbox_row(ui, "random pick", &mut data_file.random_pick);
        }
    }
}

pub fn features_list(ui: &mut Ui, data_files_res: &mut ResMut<DataFiles>) {
    if let Some(data_file) = data_files_res.current_data_file() {
        ui.separator(); ui.end_row();
        ui.label(egui::RichText::new("features").color(NEUTRAL_ACTIVE_COLOR).strong());
        for (feature, active) in (&mut data_file.features).into_iter() {
            let label = ui.selectable_label(*active, &widgets::shrink_str(feature, 29));
            if label.clicked() {
                *active = !*active;
            }
            ui.end_row();
        }
    }
}

pub fn add_magds_button_row(
    ui: &mut Ui,
    data_files_res: &mut ResMut<DataFiles>,
    loaded_datasets_res: &mut ResMut<LoadedDatasets>,
    magds_res: &mut ResMut<MainMAGDS>,
    appearance: &mut ResMut<Appearance>
) {
    if let Some(data_file) = data_files_res.current_data_file() {
        ui.separator(); ui.end_row();
        ui.horizontal(|ui| {
            let add_button = ui.button("add to magds");
            if add_button.clicked() {
                if let Some(df) = &data_file.data_frame {
                    let df_name = &data_file.name;
                    let mut magds = magds_res.0.write().unwrap();
                    let df_name = df_name.strip_suffix(".csv").unwrap_or(df_name);
                    let skip_features: Vec<&str> = (&data_file.features).into_iter()
                        .filter(|(_key, value)| !**value)
                        .map(|(key, _value)| &**key)
                        .collect();
                    parser::add_df_to_magds(
                        &mut magds, df_name, df, &skip_features, data_file.rows_limit, data_file.random_pick
                    );
                    
                    let default_sensor_appearance = appearance.sensors[&Selector::All].clone();
                    for sensor_name in magds.sensors_names() {
                        let sensor_key = &Selector::One(sensor_name.clone());
                        if !appearance.sensors.contains_key(sensor_key) {
                            appearance.sensors.insert(
                                sensor_key.clone(), default_sensor_appearance.clone()
                            );
                        }
                    }
                    let default_neuron_appearance = appearance.neurons[&Selector::All].clone();
                    for neuron_name in magds.neurons_names() {
                        let neuron_key = &Selector::One(neuron_name.clone());
                        if !appearance.neurons.contains_key(neuron_key) {
                            appearance.neurons.insert(
                                neuron_key.clone(), default_neuron_appearance.clone()
                            );
                        }
                    }
                    
                    let loaded_dataset = LoadedDataset { 
                        name: df_name.to_string(), 
                        path: data_file.path.clone(),
                        rows: data_file.rows_limit,
                        rows_total: df.height(),
                        random_pick: data_file.random_pick,
                        features: (&data_file.features).into_iter()
                            .filter(|(_key, value)| **value)
                            .map(|(key, _value)| key.clone())
                            .collect()
                    };
                    loaded_datasets_res.0.push(loaded_dataset);
                }
            }
        });
    }
    ui.end_row();
}

pub(crate) fn loaded_files(ui: &mut Ui, loaded_datasets_res: &mut ResMut<LoadedDatasets>) {
    ui.separator(); ui.end_row();
    ui.label(egui::RichText::new("loaded data").color(NEUTRAL_ACTIVE_COLOR).strong());
    ui.end_row();
    
    if loaded_datasets_res.0.is_empty() {
        let label_widget = RichText::new("no data")
            .monospace()
            .size(STANDARD_TEXT_SIZE)
            .color(NEUTRAL_INACTIVE_COLOR);
        ui.label(label_widget);
    }

    for dataset in &loaded_datasets_res.0 {
        let label_widget = RichText::new(widgets::shrink_str(&dataset.name, 29))
            .monospace()
            .size(STANDARD_MONOSPACE_TEXT_SIZE)
            .color(ADDED_TO_MAGDS_COLOR);
        ui.label(label_widget);

        let rows_text = format!(
            "{} of {} {} rows",
            dataset.rows,
            dataset.rows_total,
            if dataset.random_pick { "random" } else { "consecutive" }
        );
        let label_widget = RichText::new(widgets::shrink_str(&rows_text, 42))
            .size(SMALL_TEXT_SIZE)
            .color(NEUTRAL_COLOR);
        ui.label(label_widget);

        for feature in &dataset.features {
            let label_widget = RichText::new(widgets::shrink_str(feature, 42))
                .size(SMALL_TEXT_SIZE)
                .color(NEUTRAL_INACTIVE_COLOR);
            ui.label(label_widget);
        }
    }
}