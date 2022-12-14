use std::sync::Arc;

use bevy::prelude::*;

use bevy_egui::egui::{ 
    self,
    Ui,
    Widget,
    Rgba,
    FontFamily,
    RichText,
    Slider,
    ComboBox,
    Response,
    Color32
};

use crate::{
    resources::{
        appearance::Selector,
        common::{ NEUTRAL_COLOR, NEUTRAL_INACTIVE_COLOR,  STANDARD_TEXT_SIZE }
    },
    utils
};

pub fn heading_label(ui: &mut Ui, text: &str, label_color: Color32) {
    let label_widget = RichText::new(text)
        .family(FontFamily::Proportional)
        .size(STANDARD_TEXT_SIZE)
        .strong()
        .color(label_color);
    ui.label(label_widget);
}

pub fn combobox_str_row(
    ui: &mut Ui, 
    id: &str, 
    selected: &mut Option<Arc::<str>>,
    values: &[Option<Arc::<str>>],
    label_color: Color32
) -> Response {
    let combobox = ui.horizontal(|ui| {
        heading_label(ui, id, label_color);

        let selected_text = if let Some(text) = selected {
            &*text
        } else {
            "click to see"
        };

        ComboBox::from_id_source(id)
            .selected_text(utils::shrink_str(selected_text, 25))
            .show_ui(ui, |ui| {
                for value in values {
                    if let Some(v) = value {
                        ui.selectable_value(selected, Some(v.clone()), &**v);
                    }
                }
            }
        );
    });
    ui.end_row();
    combobox.response
}

pub fn combobox_row(
    ui: &mut Ui, 
    id: &str, 
    selected: &mut Selector,
    values: &[Selector],
    label_color: Color32
) -> Response {
    let combobox = ui.horizontal(|ui| {
        let label_widget = RichText::new(id)
            .family(FontFamily::Proportional)
            .size(STANDARD_TEXT_SIZE)
            .strong()
            .color(label_color);
        ui.label(label_widget);

        ComboBox::from_id_source(id)
            .selected_text(utils::shrink_str(selected.to_str(), 25))
            .show_ui(ui, |ui| {
                ui.selectable_value(selected, Selector::All, &*Selector::All.to_arc_str());
                for value in values {
                    if *value != Selector::All {
                        ui.selectable_value(
                            selected, 
                            (*value).clone(), 
                            &*value.to_arc_str()
                        );
                    }
                }
            }
        );
    });
    ui.end_row();
    combobox.response
}

pub fn checkbox_row(
    ui: &mut Ui, label: &str, state: &mut bool
) -> Option<Response> {
    let mut checkbox = None;
    ui.horizontal(|ui| {
        let color = if *state { NEUTRAL_COLOR } else { NEUTRAL_INACTIVE_COLOR };
        let label_widget = RichText::new(label)
            .family(FontFamily::Proportional)
            .size(STANDARD_TEXT_SIZE)
            .color(color);
        ui.label(label_widget);
        checkbox = Some(ui.checkbox(state, ""));
    });
    ui.end_row();
    checkbox
}

pub fn slider_row(
    ui: &mut Ui, label: &str, value: &mut f32, bounds: (f32, f32)
) -> Option<Response> {
    let mut slider = None;
    ui.horizontal(|ui| {
        let label_widget = RichText::new(label)
            .family(FontFamily::Proportional)
            .size(STANDARD_TEXT_SIZE)
            .color(NEUTRAL_COLOR);
        ui.label(label_widget);
        slider = Some(Slider::new(value, (bounds.0)..=(bounds.1)).ui(ui));
    });
    ui.end_row();
    slider
}

pub fn slider_row_f64(
    ui: &mut Ui, label: &str, value: &mut f64, bounds: (f64, f64)
) -> Option<Response> {
    let mut slider = None;
    ui.horizontal(|ui| {
        let label_widget = RichText::new(label)
            .family(FontFamily::Proportional)
            .size(STANDARD_TEXT_SIZE)
            .color(NEUTRAL_COLOR);
        ui.label(label_widget);
        slider = Some(Slider::new(value, (bounds.0)..=(bounds.1)).ui(ui));
    });
    ui.end_row();
    slider
}

pub fn slider_row_usize(
    ui: &mut Ui, label: &str, value: &mut usize, bounds: (usize, usize)
) -> Option<Response> {
    let mut slider = None;
    ui.horizontal(|ui| {
        let label_widget = RichText::new(label)
            .family(FontFamily::Proportional)
            .size(STANDARD_TEXT_SIZE)
            .color(NEUTRAL_COLOR);
        ui.label(label_widget);
        slider = Some(Slider::new(value, (bounds.0)..=(bounds.1)).ui(ui));
    });
    ui.end_row();
    slider
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
        let label_widget = RichText::new(label)
            .family(FontFamily::Proportional)
            .size(STANDARD_TEXT_SIZE)
            .color(NEUTRAL_COLOR);
        ui.label(label_widget);
        color_picker(ui, color);
    });
    ui.end_row();
}