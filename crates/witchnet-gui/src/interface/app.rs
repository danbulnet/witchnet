use bevy::prelude::*;

use bevy_egui::EguiPlugin;

use crate::{
    resources::{
        appearance::Appearance,
        data::DataFiles,
        magds::{ MainMAGDS, LoadedDatasets },
        layout::Layout
    },
    interface::layout
};

pub fn app() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .insert_resource(Layout::default())
        .insert_resource(MainMAGDS::default())
        .insert_resource(LoadedDatasets::default())
        .insert_resource(DataFiles::default())
        .insert_resource(Appearance::default())
        .add_system(setup)
        .add_system(layout::app_layout)
        .run();
}

fn setup(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_title("witchnet".to_string());
    window.set_maximized(true);
}