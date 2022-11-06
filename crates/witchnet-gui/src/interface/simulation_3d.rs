use bevy::prelude::*;

use bevy_egui::egui::{ Ui };

use crate::{
    resources::{
        appearance::Appearance,
        magds::MainMAGDS
    }
};

pub(crate) fn simulation(
    ui: &mut Ui,
    _magds_res: &mut ResMut<MainMAGDS>,
    _appearance_res: &mut ResMut<Appearance>,
) {
    ui.label("simulation 3D");
}