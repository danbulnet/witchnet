use bevy::prelude::*;

use bevy_egui::{ 
    egui::{ 
        Window,
        Align2,
    }, 
    EguiContext 
};

use crate::{
    interface::widgets,
    resources::{
        common::INTERFACE_PADDING,
        data::{ 
            DataFiles,
            MIN_DATA_WIDTH,
            DATA_X
        },
        magds::MainMAGDS
    }
};

pub(crate) fn data_window(
    mut egui_context: ResMut<EguiContext>, 
    mut windows: ResMut<Windows>,
    mut data_files_res: ResMut<DataFiles>,
    mut magds_res: ResMut<MainMAGDS>,
) {
    let window = windows.get_primary_mut().unwrap();
    let max_height = window.height();

    Window::new("data")
        .anchor(Align2::LEFT_TOP, [DATA_X, INTERFACE_PADDING])
        .scroll2([false, true])
        .fixed_size([MIN_DATA_WIDTH, max_height])
        .show(egui_context.ctx_mut(), |ui| {
            ui.set_min_width(MIN_DATA_WIDTH);

            widgets::file_button_row(
                ui, 
                "load", 
                &["csv"], 
                &mut data_files_res
            );

            widgets::features_list(ui, &mut data_files_res);
        });
}