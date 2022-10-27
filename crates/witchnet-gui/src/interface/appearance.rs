use bevy::prelude::*;

use bevy_egui::{ 
    egui::{ Ui, Window, Align2, Grid }, 
    EguiContext 
};

use crate::{
    resources::{
        appearance::{
            Selector,
            Appearance,
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
    mut appearance: ResMut<Appearance>
) {
    let window = windows.get_primary_mut().unwrap();
    let max_height = window.height();

    Window::new("appearance")
        .anchor(Align2::LEFT_TOP, [APPEARANCE_X, INTERFACE_PADDING])
        .scroll2([false, true])
        .fixed_size([MIN_APPEARANCE_WIDTH, max_height])
        .show(egui_context.ctx_mut(), |ui| {
            ui.set_min_width(MIN_APPEARANCE_WIDTH);

            sensor_settings(ui, &mut appearance);
            ui.separator(); ui.end_row();

            neuron_settings(ui, &mut appearance);
            ui.separator(); ui.end_row();

            connection_settings(ui, &mut appearance);
        });
}

fn sensor_settings(mut ui: &mut Ui, appearance: &mut ResMut<Appearance>) {
    Grid::new("sensor").show(&mut ui, |ui| {
        let selector = appearance.selected_sensor.clone();
        let sensor = appearance.sensors.get(&selector).unwrap();
        let label_color = utils::color_bevy_to_egui(&sensor.primary_active_color);
        let sensors: Vec<Selector> = appearance.sensors.keys().cloned().collect();
        w::combobox_row(
            ui, 
            "sensor", 
            &mut appearance.selected_sensor, 
            &sensors,
            label_color
        );
        let sensor = appearance.sensors.get_mut(&selector).unwrap();

        w::checkbox_row(ui, "show", &mut sensor.show);

        let size_bounds = sensor.size_bounds;
        w::slider_row(ui, "size", &mut sensor.size, size_bounds);

        w::color_picker_row(ui, "primary color", &mut sensor.primary_color);
        w::color_picker_row(ui, "primary hover color", &mut sensor.primary_hover_color);
        w::color_picker_row(ui, "primary active color", &mut sensor.primary_active_color);

        w::color_picker_row(ui, "secondary color", &mut sensor.secondary_color);
        w::color_picker_row(ui, "secondary hover color", &mut sensor.secondary_hover_color);
        w::color_picker_row(ui, "secondary active color", &mut sensor.secondary_active_color);
        
        w::checkbox_row(ui, "show text", &mut sensor.show_text);
        
        let text_size_bounds = sensor.text_size_bounds;
        w::slider_row(ui, "text size", &mut sensor.text_size, text_size_bounds);
        
        w::color_picker_row(ui, "text color", &mut sensor.text_color);
        w::color_picker_row(ui, "text hover color", &mut sensor.text_hover_color);
        w::color_picker_row(ui, "text active color", &mut sensor.text_active_color);
    });
}

fn neuron_settings(mut ui: &mut Ui, appearance: &mut ResMut<Appearance>) {
    Grid::new("neuron").show(&mut ui, |ui| {
        let selector = appearance.selected_neuron.clone();
        let neuron = appearance.neurons.get(&selector).unwrap();
        let label_color = utils::color_bevy_to_egui(&neuron.primary_active_color);
        let neurons: Vec<Selector> = appearance.neurons.keys().cloned().collect();
        w::combobox_row(
            ui, 
            "neuron", 
            &mut appearance.selected_neuron, 
            &neurons,
            label_color
        );
        let neuron = appearance.neurons.get_mut(&selector).unwrap();
        w::checkbox_row(ui, "show", &mut neuron.show);

        let size_bounds = neuron.size_bounds;
        w::slider_row(ui, "size", &mut neuron.size, size_bounds);

        w::color_picker_row(ui, "primary color", &mut neuron.primary_color);
        w::color_picker_row(ui, "primary hover color", &mut neuron.primary_hover_color);
        w::color_picker_row(ui, "primary active color", &mut neuron.primary_active_color);

        w::color_picker_row(ui, "secondary color", &mut neuron.secondary_color);
        w::color_picker_row(ui, "secondary hover color", &mut neuron.secondary_hover_color);
        w::color_picker_row(ui, "secondary active color", &mut neuron.secondary_active_color);
        
        w::checkbox_row(ui, "show text", &mut neuron.show_text);
        
        let text_size_bounds = neuron.text_size_bounds;
        w::slider_row(ui, "text size", &mut neuron.text_size, text_size_bounds);
        
        w::color_picker_row(ui, "text color", &mut neuron.text_color);
        w::color_picker_row(ui, "text hover color", &mut neuron.text_hover_color);
        w::color_picker_row(ui, "text active color", &mut neuron.text_active_color);
    });
}

fn connection_settings(mut ui: &mut Ui, appearance: &mut ResMut<Appearance>) {
    Grid::new("connection").show(&mut ui, |ui| {
        let selector = appearance.selected_connection.clone();
        let connection = appearance.connections.get(&selector).unwrap();
        let label_color = utils::color_bevy_to_egui(&connection.active_color);
        let connections: Vec<Selector> = appearance.connections.keys().cloned().collect();
        w::combobox_row(
            ui, 
            "connection", 
            &mut appearance.selected_connection, 
            &connections,
            label_color
        );
        let connection = appearance.connections.get_mut(&selector).unwrap();
        w::checkbox_row(ui, "show", &mut connection.show);

        let thickness_bounds = connection.thickness_bounds;
        w::slider_row(ui, "thickness", &mut connection.thickness, thickness_bounds);

        w::color_picker_row(ui, "color", &mut connection.color);
        w::color_picker_row(ui, "hover color", &mut connection.hover_color);
        w::color_picker_row(ui, "active color", &mut connection.active_color);
        
        w::checkbox_row(ui, "show text", &mut connection.show_text);
        
        let text_size_bounds = connection.text_size_bounds;
        w::slider_row(ui, "text size", &mut connection.text_size, text_size_bounds);
        
        w::color_picker_row(ui, "text color", &mut connection.text_color);
        w::color_picker_row(ui, "text hover color", &mut connection.text_hover_color);
        w::color_picker_row(ui, "text active color", &mut connection.text_active_color);
    });
}