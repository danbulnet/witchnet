use bevy::prelude::*;

use bevy_egui::EguiPlugin;

use crate::interface::plugin::Interface;

pub struct Simulation3D;

impl Plugin for Simulation3D {
    fn build(&self, app: &mut App) {
        app.add_plugin(Interface);
    }
}