use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use bevy_egui::EguiPlugin;

use crate::{
    resources::{
        appearance::Appearance,
        data::DataFiles,
        magds::{ MainMAGDS, LoadedDatasets, PositionXY },
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
        .insert_resource(PositionXY::default())
        .insert_resource(DataFiles::default())
        .insert_resource(Appearance::default())
        .add_system(setup)
        // .add_system(setup2)
        .add_system(layout::app_layout)
        .run();
}

fn setup(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_title("witchnet".to_string());
    window.set_maximized(true);
}

// fn setup2(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     commands.spawn_bundle(Camera2dBundle::default());
//     commands.spawn_bundle(MaterialMesh2dBundle {
//         mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
//         transform: Transform::default().with_scale(Vec3::splat(128.)),
//         material: materials.add(ColorMaterial::from(Color::PURPLE)),
//         ..default()
//     });
// }