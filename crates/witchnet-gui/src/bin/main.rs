use bevy::prelude::*;

use witchnet_gui::simulation_3d::Simulation3D;

fn main() {
    App::new().add_plugin(Simulation3D).run();
}