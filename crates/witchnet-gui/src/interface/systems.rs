use bevy::prelude::*;

use bevy_egui::{ 
    egui::{ 
        self, 
        Ui,
        Window,
        Pos2,
        Align2,
        Rect,
        Grid
    }, 
    EguiContext 
};

use crate::{
    resources::appearance::{
        NeuronAppearance,
        SensorAppearance,
        ConnectionAppearance
    },
    interface::widgets as w,
    utils
};

pub(crate) fn setup(
    mut egui_ctx: ResMut<EguiContext>,
    mut windows: ResMut<Windows>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let window = windows.get_primary_mut().unwrap();
    window.set_maximized(true);
}

pub(crate) fn appearance_window(
    mut egui_context: ResMut<EguiContext>,
    mut windows: ResMut<Windows>,
    neuron_appearance: ResMut<NeuronAppearance>,
    sensor_appearance: ResMut<SensorAppearance>,
    connection_appearance: ResMut<ConnectionAppearance>
) {
    let window = windows.get_primary_mut().unwrap();
    let max_height = 2f32 * window.height() - 50f32;

    Window::new("appearance")
        .anchor(Align2::LEFT_TOP, [150f32, 25f32])
        .scroll2([false, true])
        .fixed_size([100f32, max_height])
        .show(egui_context.ctx_mut(), |ui| {
            sensor_settings(ui, sensor_appearance);
            ui.separator(); ui.end_row();

            neuron_settings(ui, neuron_appearance);
            ui.separator(); ui.end_row();

            connection_settings(ui, connection_appearance);
        });
}

pub(crate) fn data_window(
    mut egui_context: ResMut<EguiContext>, mut windows: ResMut<Windows>
) {
    let window = windows.get_primary_mut().unwrap();
    let max_height = 2f32 * window.height() - 50f32;

    Window::new("data")
        .anchor(Align2::LEFT_TOP, [25f32, 25f32])
        .scroll2([false, true])
        .fixed_size([100f32, max_height])
        .show(egui_context.ctx_mut(), |ui| {
            let load_data_button = ui.button("load data");
            if load_data_button.clicked() {
                println!("load_data_button clicked");
            }
        });
}

fn sensor_settings(
    mut ui: &mut Ui,
    mut resource: ResMut<SensorAppearance>
) {
    Grid::new("sensor").show(&mut ui, |ui| {
        ui.label(egui::RichText::new("sensor")
            .color(utils::color_bevy_to_egui(&resource.primary_active_color))); 
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
        ui.label(egui::RichText::new("neuron")
            .color(utils::color_bevy_to_egui(&resource.primary_active_color))); 
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
        ui.label(egui::RichText::new("connection")
            .color(utils::color_bevy_to_egui(&resource.active_color))); 
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