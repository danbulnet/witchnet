use bevy::prelude::*;

use bevy_egui::EguiPlugin;
use crate::{
    resources::appearance::{ NeuronAppearance, SensorAppearance, ConnectionAppearance },
    ui::systems
};

pub struct UI;

impl Plugin for UI {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .insert_resource(NeuronAppearance::default())
            .insert_resource(SensorAppearance::default())
            .insert_resource(ConnectionAppearance::default())
            .add_plugin(EguiPlugin)
            .add_startup_system(systems::setup)
            .add_system(systems::data_window)
            .add_system(systems::appearance_window);
    }
}