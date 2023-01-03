use bevy::prelude::*;

use bevy_egui::egui::{ Ui };

use crate::{
    resources::{
        appearance::Appearance,
        magds::MAGDSMain
    }
};

pub(crate) fn simulation(
    ui: &mut Ui,
    _magds_res: &mut ResMut<MAGDSMain>,
    _appearance_res: &mut ResMut<Appearance>,
) {
    ui.label("magds-3d");
}