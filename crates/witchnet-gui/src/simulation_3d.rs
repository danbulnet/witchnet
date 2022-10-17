use bevy::prelude::*;

use bevy_egui::{egui, EguiContext, EguiPlugin};

use egui::Widget;

pub struct Simulation3D;

impl Plugin for Simulation3D {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugin(EguiPlugin)
            .add_startup_system(setup)
            .add_system(data_window)
            .add_system(appearance_window);
    }
}

fn setup(
    mut egui_ctx: ResMut<EguiContext>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
  
}

fn color_picker_widget(ui: &mut egui::Ui, color: &mut Color) -> egui::Response {
    let [r, g, b, a] = color.as_rgba_f32();
    let mut egui_color: egui::Rgba = egui::Rgba::from_srgba_unmultiplied(
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        (a * 255.0) as u8,
    );
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

fn data_window(mut egui_context: ResMut<EguiContext>) {
    let mut apply = false;
    let mut slider_value: f32 = 0.0;
    let mut checkbox_value: bool = false;

    egui::Window::new("appearance").show(egui_context.ctx_mut(), |ui| {
        egui::Grid::new("sensor").show(ui, |ui| {
            ui.label("sensor");
            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("neuron main color:");
                color_picker_widget(ui, &mut Color::rgba(0f32, 128f32, 128f32, 1f32));
            });
            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("neuron main color:");
                color_picker_widget(ui, &mut Color::rgba(0f32, 128f32, 128f32, 1f32));
            });
            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("Perceptual roughness:");
                egui::Slider::new(&mut slider_value, 0.0..=1.0).ui(ui);
            });
            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("Unlit:");
                ui.checkbox(&mut checkbox_value, "");
            });
            ui.end_row();

            ui.separator();
            ui.end_row();
        });

        egui::Grid::new("neuron").show(ui, |ui| {
            ui.label("neuron");
            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("neuron main color:");
                color_picker_widget(ui, &mut Color::rgba(0f32, 128f32, 128f32, 1f32));
            });
            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("neuron main color:");
                color_picker_widget(ui, &mut Color::rgba(0f32, 128f32, 128f32, 1f32));
            });
            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("Perceptual roughness:");
                egui::Slider::new(&mut slider_value, 0.0..=1.0).ui(ui);
            });
            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("Unlit:");
                ui.checkbox(&mut checkbox_value, "");
            });
            ui.end_row();
        });

        apply = ui.button("Apply").clicked();
    });

    if apply {
        println!("apply_button clicked");
    }
}

fn appearance_window(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("data").show(egui_context.ctx_mut(), |ui| {
        let load_data_button = ui.button("load data");
        if load_data_button.clicked() {
            println!("load_data_button clicked");
        }
    });
}