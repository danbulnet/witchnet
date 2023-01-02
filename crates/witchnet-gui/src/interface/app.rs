use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use bevy_egui::EguiPlugin;

use crate::{
    resources::{
        appearance::Appearance,
        tabular_data::TabularDataFiles,
        sequential_data::SequentialDataFiles,
        magds::{ MainMAGDS, MAGDSLoadedDatasets, MAGDSPositions },
        smagds::SMAGDSMain,
        sequence_1d::Sequence1D,
        layout::Layout, 
        args::ProgramArgs
    },
    interface::layout
};

pub fn app(args: Vec<String>) {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .insert_resource(Layout::default())
        .insert_resource(MainMAGDS::default())
        .insert_resource(SMAGDSMain::default())
        .insert_resource(MAGDSLoadedDatasets::default())
        .insert_resource(MAGDSPositions::default())
        .insert_resource(TabularDataFiles::default())
        .insert_resource(SequentialDataFiles::default())
        .insert_resource(Sequence1D::default())
        .insert_resource(Appearance::default())
        .insert_resource(ProgramArgs::from(args))
        .add_startup_system(setup)
        .add_startup_system(ProgramArgs::handle_args)
        .add_system(layout::app_layout)
        // .add_system(setup2)
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