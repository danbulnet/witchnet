use bevy::prelude::*;

use bevy_egui::{ egui, EguiContext };

use crate::{
    resources::appearance::{ NeuronAppearance, SensorAppearance, ConnectionAppearance },
    ui::widgets
};

pub(crate) fn setup(
    mut egui_ctx: ResMut<EguiContext>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {

}

pub(crate) fn appearance_window(
    mut egui_context: ResMut<EguiContext>,
    mut neuron_appearance: ResMut<NeuronAppearance>,
    mut sensor_appearance: ResMut<SensorAppearance>,
    mut connection_appearance: ResMut<ConnectionAppearance>
) {
    let mut apply = false;
    let mut slider_value: f32 = 0.0;
    let mut checkbox_value: bool = false;

    egui::Window::new("appearance").show(egui_context.ctx_mut(), |ui| {
        egui::Grid::new("sensor").show(ui, |ui| {
            ui.label("sensor"); ui.end_row();

            ui.horizontal(|ui| {
                ui.label("primary color:");
                widgets::color_picker_widget(ui, &mut sensor_appearance.primary_color);
            }); ui.end_row();

            ui.horizontal(|ui| {
                ui.label("primary hover color:");
                widgets::color_picker_widget(ui, &mut sensor_appearance.primary_hover_color);
            }); ui.end_row();

            ui.horizontal(|ui| {
                ui.label("primary active color:");
                widgets::color_picker_widget(ui, &mut sensor_appearance.primary_active_color);
            }); ui.end_row();
            ui.horizontal(|ui| {
                ui.label("secondary color:");
                widgets::color_picker_widget(ui, &mut sensor_appearance.secondary_color);
            }); ui.end_row();

            ui.horizontal(|ui| {
                ui.label("secondary hover color:");
                widgets::color_picker_widget(ui, &mut sensor_appearance.secondary_hover_color);
            }); ui.end_row();

            ui.horizontal(|ui| {
                ui.label("secondary active color:");
                widgets::color_picker_widget(ui, &mut sensor_appearance.secondary_active_color);
            }); ui.end_row();

            ui.separator(); ui.end_row();
        });

        egui::Grid::new("neuron").show(ui, |ui| {
            ui.label("neuron"); ui.end_row();

            ui.horizontal(|ui| {
                ui.label("primary color:");
                widgets::color_picker_widget(ui, &mut neuron_appearance.primary_color);
            }); ui.end_row();

            ui.horizontal(|ui| {
                ui.label("primary hover color:");
                widgets::color_picker_widget(ui, &mut neuron_appearance.primary_hover_color);
            }); ui.end_row();

            ui.horizontal(|ui| {
                ui.label("primary active color:");
                widgets::color_picker_widget(ui, &mut neuron_appearance.primary_active_color);
            }); ui.end_row();
            ui.horizontal(|ui| {
                ui.label("secondary color:");
                widgets::color_picker_widget(ui, &mut neuron_appearance.secondary_color);
            }); ui.end_row();

            ui.horizontal(|ui| {
                ui.label("secondary hover color:");
                widgets::color_picker_widget(ui, &mut neuron_appearance.secondary_hover_color);
            }); ui.end_row();

            ui.horizontal(|ui| {
                ui.label("secondary active color:");
                widgets::color_picker_widget(ui, &mut neuron_appearance.secondary_active_color);
            }); ui.end_row();

            ui.separator(); ui.end_row();
        });

        egui::Grid::new("connection").show(ui, |ui| {
            ui.label("connection"); ui.end_row();

            ui.horizontal(|ui| {
                ui.label("color:");
                widgets::color_picker_widget(ui, &mut connection_appearance.color);
            }); ui.end_row();

            ui.horizontal(|ui| {
                ui.label("hover color:");
                widgets::color_picker_widget(ui, &mut connection_appearance.hover_color);
            }); ui.end_row();

            ui.horizontal(|ui| {
                ui.label("active color:");
                widgets::color_picker_widget(ui, &mut connection_appearance.active_color);
            }); ui.end_row();
        });

        apply = ui.button("Apply").clicked();
    });

    if apply {
        println!("apply_button clicked");
    }
}

pub(crate) fn data_window(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("data").show(egui_context.ctx_mut(), |ui| {
        let load_data_button = ui.button("load data");
        if load_data_button.clicked() {
            println!("load_data_button clicked");
        }
    });
}