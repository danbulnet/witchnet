use std::{
    path::{ PathBuf, Path },
    env
};

use bevy::prelude::*;

use bevy_egui::egui::{ self, Ui, Widget, Rgba, Color32 };

use rfd::FileDialog;

use crate::{
    resources::data::{ DataFilePath, DataFileName },
    utils
};

pub fn file_button_row(
    ui: &mut Ui, 
    label: &str,
    extensions: &[&str],
    file_path_res: &mut ResMut<DataFilePath>,
    file_name_res: &mut ResMut<DataFileName>,
    file_name_color: Color32
) {
    ui.horizontal(|ui| {
        let load_data_button = ui.button(label);
        if load_data_button.clicked() {
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
                let prepared = if file_name.chars().count() <= 18 {
                    file_name
                } else {
                    let mut file_name = file_name[..file_name.char_indices()
                        .nth(15).unwrap().0].to_string();
                    file_name.push_str("...");
                    file_name
                };
                file_path_res.0 = file_path;
                file_name_res.0 = Some(prepared.to_string());
            } else {
                file_path_res.0 = None;
                file_name_res.0 = None;
            }
        }
        match &file_name_res.0 {
            Some(file_name) => ui.label(
                egui::RichText::new(file_name).monospace().size(11f32).color(file_name_color)
            ),
            None => ui.label(
                egui::RichText::new("select csv file").monospace().size(11f32).color(Color32::GRAY)
            ),
        };
    });
    ui.end_row();
}

pub fn checkbox_row(ui: &mut Ui, label: &str, state: &mut bool) {
    ui.horizontal(|ui| {
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