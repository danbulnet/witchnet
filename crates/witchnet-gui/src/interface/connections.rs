use bevy::prelude::*;

use bevy_egui::egui::{ self, Ui };

use crate::{
    resources::{
        magds::MAGDSMain,
        layout::DEFAULT_PANEL_WIDTH
    }
};

pub(crate) fn connections(
    ui: &mut Ui,
    _magds_res: &mut ResMut<MAGDSMain>
) {
    egui::ScrollArea::vertical()
        .stick_to_bottom(true)
        .show(ui, |ui| {
            ui.set_min_width(DEFAULT_PANEL_WIDTH);
            ui.label("connections");
    });
}