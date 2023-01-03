use bevy::prelude::*;

use bevy_egui::egui::{ Ui };

use crate::{
    resources::{
        magds::MAGDSMain
    }
};

pub(crate) fn simulation(
    ui: &mut Ui,
    _magds_res: &mut ResMut<MAGDSMain>
) {
    ui.label("magds-3d");
}