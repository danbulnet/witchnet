use bevy::prelude::*;

use bevy_egui::{ EguiPlugin, EguiContext };


use crate::{
    resources::{
        appearance::{ NeuronAppearance, SensorAppearance, ConnectionAppearance },
        data::DataFiles,
        magds::{ MainMAGDS, LoadedDatasets }
    },
    interface::{ appearance, data }
};

pub struct Interface;

impl Plugin for Interface {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .insert_resource(MainMAGDS::default())
            .insert_resource(LoadedDatasets::default())
            .insert_resource(DataFiles::default())
            .insert_resource(NeuronAppearance::default())
            .insert_resource(NeuronAppearance::default())
            .insert_resource(SensorAppearance::default())
            .insert_resource(ConnectionAppearance::default())
            .add_plugin(EguiPlugin)
            .add_startup_system(setup)
            .add_system(data::data_window)
            .add_system(appearance::appearance_window);
    }
}

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