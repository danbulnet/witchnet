use bevy::prelude::Color;

use bevy_egui::egui::{ self, Ui, Widget, Rgba };

use crate::utils;

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