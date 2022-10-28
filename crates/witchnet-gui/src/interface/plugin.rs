use bevy::prelude::*;

use bevy_egui::{ egui, EguiPlugin, EguiContext };


use crate::{
    resources::{
        appearance::{ 
            NeuronAppearance, 
            SensorAppearance, 
            ConnectionAppearance, 
            Appearance 
        },
        data::DataFiles,
        magds::{ MainMAGDS, LoadedDatasets }
    },
    interface::{ appearance, data, app }
};

pub struct Interface;

impl Plugin for Interface {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugin(EguiPlugin)
            .insert_resource(MainMAGDS::default())
            .insert_resource(LoadedDatasets::default())
            .insert_resource(DataFiles::default())
            .insert_resource(Appearance::default())
            .insert_resource(NeuronAppearance::default())
            .insert_resource(NeuronAppearance::default())
            .insert_resource(SensorAppearance::default())
            .insert_resource(ConnectionAppearance::default())
            .add_system(appearance::appearance_window);
            // .add_system(data::data_window);
    }
}