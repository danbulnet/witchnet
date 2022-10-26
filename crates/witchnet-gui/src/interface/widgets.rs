use std::{
    path::{ PathBuf, Path },
    env,
    collections::BTreeMap
};

use bevy::prelude::*;

use bevy_egui::egui::{ self, Ui, Widget, Rgba, Color32 };

use rfd::{ FileDialog, MessageDialog, MessageLevel };

use witchnet_common::polars;

use crate::{
    resources::{
        magds::MainMAGDS,
        data::{ 
            DataFiles, 
            DataFile, 
            FILE_NAME_OK_COLOR, 
            FILE_NAME_ERR_COLOR, 
            ADDED_TO_MAGDS_COLOR 
        },
        common::{ NEUTRAL_ACTIVE_COLOR, NEUTRAL_INACTIVE_COLOR, STANDARD_TEXT_SIZE }
    },
    utils
};

// pub fn add_magds_button_row(
//     ui: &mut Ui,
//     data_file_res: &mut ResMut<Option<DataFile>>,
//     magds_res: &mut ResMut<MainMAGDS>
// ) {
//     ui.horizontal(|ui| {
//         let load_data_button = ui.button("add to magds");
//         if load_data_button.clicked() {
//             match &data_file_res {
//                 Some(file_path) => {
                    
//                 }
//                 None => ui.label(
//                     egui::RichText::new("").monospace().size(STANDARD_TEXT_SIZE).color(Color32::GRAY)
//                 );
//             };
//         }
//         if data_file_res.loaded {
//             ui.label(
//                 egui::RichText::new("added").monospace().size(STANDARD_TEXT_SIZE).color(ADDED_TO_MAGDS_COLOR)
//             );
//         }
//     });
//     ui.end_row();
// }

pub fn features_list(ui: &mut Ui, data_files_res: &mut ResMut<DataFiles>) {
    if let Some(index) = data_files_res.current {
    let data_file = &mut data_files_res.history[index];
    for (feature, active) in (&mut data_file.features).into_iter() {
        ui.horizontal(|ui| {
            // let label = egui::RichText::new(feature)
            //     .monospace()
            //     .size(STANDARD_TEXT_SIZE)
            //     .color(NEUTRAL_ACTIVE_COLOR);
            // ui.label(label);
            checkbox_row(ui, feature, active);
        });
        ui.end_row();
    }
}
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
            load_button_clicked(ui, label, extensions, data_files_res);
        }
        match data_files_res.current {
            Some(index) => {
                let data_file = &data_files_res.history[index];
                let label = if data_file.data_frame.is_some() {
                    egui::RichText::new(&data_file.name)
                        .monospace()
                        .size(STANDARD_TEXT_SIZE)
                        .color(FILE_NAME_OK_COLOR)
                } else {
                    egui::RichText::new(&data_file.name)
                        .monospace()
                        .size(STANDARD_TEXT_SIZE)
                        .color(FILE_NAME_ERR_COLOR)
                };
                ui.label(label)
            }
            None => ui.label(
                egui::RichText::new("select csv file")
                    .monospace()
                    .size(STANDARD_TEXT_SIZE)
                    .color(NEUTRAL_ACTIVE_COLOR)
            ),
        };
    });
    ui.end_row();
}

fn load_button_clicked(
    ui: &mut Ui, 
    label: &str,
    extensions: &[&str],
    data_files_res: &mut ResMut<DataFiles>
) {
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
        let file_name = if file_name.chars().count() <= 18 {
            file_name
        } else {
            let mut file_name = file_name[..file_name.char_indices()
                .nth(15).unwrap().0].to_string();
            file_name.push_str("...");
            file_name
        };

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
            } else {
                features.extend(
                    data_frame.as_ref().unwrap()
                        .get_column_names()
                        .into_iter()
                        .map(|x| (x.to_string(), true))
                        .collect::<BTreeMap<String, bool>>()
                );
            }
            let data_file = DataFile { name: file_name, path: file_path, data_frame, features };
            data_files_res.history.push(data_file);
            data_files_res.current = Some(data_files_res.history.len() - 1);
        }
    }
}

pub fn checkbox_row(ui: &mut Ui, label: &str, state: &mut bool) {
    ui.horizontal(|ui| {
        let color = if *state { NEUTRAL_ACTIVE_COLOR } else { NEUTRAL_INACTIVE_COLOR };
        let label = egui::RichText::new(label)
            .monospace()
            .size(STANDARD_TEXT_SIZE)
            .color(color);
        ui.label(label);
        ui.checkbox(state, "");
    });
    ui.end_row();
}

pub fn slider_row(
    ui: &mut Ui, label: &str, value: &mut f32, bounds: (f32, f32)) {
    ui.horizontal(|ui| {
        ui.label(label);
        egui::Slider::new(value, (bounds.0)..=(bounds.1)).ui(ui);
    });
    ui.end_row();
}

pub fn color_picker(ui: &mut egui::Ui, color: &mut Color) -> egui::Response {
    let mut egui_color = Rgba::from(utils::color_bevy_to_egui(&color));
    let res = egui::widgets::color_picker::color_edit_button_rgba(
        ui,
        &mut egui_color,
        egui::color_picker::Alpha::Opaque,
    );
    let [r, g, b, a] = egui_color.to_srgba_unmultiplied();
    *color = [
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0,
    ]
    .into();
    res
}

pub fn color_picker_row(ui: &mut Ui, label: &str, color: &mut Color) {
    ui.horizontal(|ui| {
        ui.label(label);
        color_picker(ui, color);
    });
    ui.end_row();
}