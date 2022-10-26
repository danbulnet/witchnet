use bevy::prelude::*;

use bevy_egui::{ 
    egui::{ self, Ui, Window, Align2, Grid }, 
    EguiContext 
};

use crate::{
    resources::{
        appearance::{
            NeuronAppearance,
            SensorAppearance,
            ConnectionAppearance,
            APPEARANCE_X,
            MIN_APPEARANCE_WIDTH
        },
        common::INTERFACE_PADDING
    },
    interface::widgets as w,
    utils
};

pub(crate) fn appearance_window(
    mut egui_context: ResMut<EguiContext>,
    mut windows: ResMut<Windows>,
    neuron_appearance: ResMut<NeuronAppearance>,
    sensor_appearance: ResMut<SensorAppearance>,
    connection_appearance: ResMut<ConnectionAppearance>
) {
    let window = windows.get_primary_mut().unwrap();
    let max_height = window.height();

    Window::new("appearance")
        .anchor(Align2::LEFT_TOP, [APPEARANCE_X, INTERFACE_PADDING])
        .scroll2([false, true])
        .fixed_size([MIN_APPEARANCE_WIDTH, max_height])
        .show(egui_context.ctx_mut(), |ui| {
            ui.set_min_width(MIN_APPEARANCE_WIDTH);

            sensor_settings(ui, sensor_appearance);
            ui.separator(); ui.end_row();

            neuron_settings(ui, neuron_appearance);
            ui.separator(); ui.end_row();

            connection_settings(ui, connection_appearance);
        });
}

fn sensor_settings(
    mut ui: &mut Ui,
    mut resource: ResMut<SensorAppearance>
) {
    Grid::new("sensor").show(&mut ui, |ui| {
        ui.label(
            egui::RichText::new("sensor")
                .color(utils::color_bevy_to_egui(&resource.primary_active_color))
                .strong()
        );
        ui.end_row();

        w::checkbox_row(ui, "show:", &mut resource.show);

        let size_bounds = resource.size_bounds;
        w::slider_row(ui, "size:", &mut resource.size, size_bounds);

        w::color_picker_row(ui, "primary color:", &mut resource.primary_color);
        w::color_picker_row(ui, "primary hover color:", &mut resource.primary_hover_color);
        w::color_picker_row(ui, "primary active color:", &mut resource.primary_active_color);

        w::color_picker_row(ui, "secondary color:", &mut resource.secondary_color);
        w::color_picker_row(ui, "secondary hover color:", &mut resource.secondary_hover_color);
        w::color_picker_row(ui, "secondary active color:", &mut resource.secondary_active_color);
        
        w::checkbox_row(ui, "show text:", &mut resource.show_text);
        
        let text_size_bounds = resource.text_size_bounds;
        w::slider_row(ui, "text size:", &mut resource.text_size, text_size_bounds);
        
        w::color_picker_row(ui, "text color:", &mut resource.text_color);
        w::color_picker_row(ui, "text hover color:", &mut resource.text_hover_color);
        w::color_picker_row(ui, "text active color:", &mut resource.text_active_color);
    });
}

fn neuron_settings(
    mut ui: &mut Ui,
    mut resource: ResMut<NeuronAppearance>
) {
    Grid::new("neuron").show(&mut ui, |ui| {
        ui.label(
            egui::RichText::new("neuron")
                .color(utils::color_bevy_to_egui(&resource.primary_active_color))
                .strong()
        );
        ui.end_row();

        w::checkbox_row(ui, "show:", &mut resource.show);

        let size_bounds = resource.size_bounds;
        w::slider_row(ui, "size:", &mut resource.size, size_bounds);

        w::color_picker_row(ui, "primary color:", &mut resource.primary_color);
        w::color_picker_row(ui, "primary hover color:", &mut resource.primary_hover_color);
        w::color_picker_row(ui, "primary active color:", &mut resource.primary_active_color);

        w::color_picker_row(ui, "secondary color:", &mut resource.secondary_color);
        w::color_picker_row(ui, "secondary hover color:", &mut resource.secondary_hover_color);
        w::color_picker_row(ui, "secondary active color:", &mut resource.secondary_active_color);
        
        w::checkbox_row(ui, "show text:", &mut resource.show_text);
        
        let text_size_bounds = resource.text_size_bounds;
        w::slider_row(ui, "text size:", &mut resource.text_size, text_size_bounds);
        
        w::color_picker_row(ui, "text color:", &mut resource.text_color);
        w::color_picker_row(ui, "text hover color:", &mut resource.text_hover_color);
        w::color_picker_row(ui, "text active color:", &mut resource.text_active_color);
    });
}

fn connection_settings(
    mut ui: &mut Ui,
    mut resource: ResMut<ConnectionAppearance>
) {
    Grid::new("connection").show(&mut ui, |ui| {
        ui.label(
            egui::RichText::new("connection")
                .color(utils::color_bevy_to_egui(&resource.active_color))
                .strong()
        );
        ui.end_row();

        w::checkbox_row(ui, "show:", &mut resource.show);

        let thickness_bounds = resource.thickness_bounds;
        w::slider_row(ui, "thickness:", &mut resource.thickness, thickness_bounds);

        w::color_picker_row(ui, "color:", &mut resource.color);
        w::color_picker_row(ui, "hover color:", &mut resource.hover_color);
        w::color_picker_row(ui, "active color:", &mut resource.active_color);
        
        w::checkbox_row(ui, "show text:", &mut resource.show_text);
        
        let text_size_bounds = resource.text_size_bounds;
        w::slider_row(ui, "text size:", &mut resource.text_size, text_size_bounds);
        
        w::color_picker_row(ui, "text color:", &mut resource.text_color);
        w::color_picker_row(ui, "text hover color:", &mut resource.text_hover_color);
        w::color_picker_row(ui, "text active color:", &mut resource.text_active_color);
    });
}