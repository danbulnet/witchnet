use bevy::prelude::*;

use bevy_egui::{ 
    egui::{ 
        Window,
        Align2,
    }, 
    EguiContext 
};

use magds::simple::magds::MAGDS;

use crate::{
    interface::widgets,
    resources::{
        common::INTERFACE_PADDING,
        data::{ 
            DataFilePath, 
            DataFileName, 
            MIN_DATA_WIDTH,
            DATA_X,
            FILE_NAME_COLOR
        }
    }
};

pub(crate) fn data_window(
    mut egui_context: ResMut<EguiContext>, 
    mut windows: ResMut<Windows>,
    mut file_path_res: ResMut<DataFilePath>,
    mut file_name_res: ResMut<DataFileName>
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
                &mut file_path_res,
                &mut file_name_res,
                FILE_NAME_COLOR
            );
        });
}