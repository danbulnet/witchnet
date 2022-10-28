use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use witchnet_gui::{
    simulation_3d::Simulation3D,
    interface::{ app, plugin::Interface },
};

fn main() {
    // App::new().add_plugin(Simulation3D).run();
    app::app()
}