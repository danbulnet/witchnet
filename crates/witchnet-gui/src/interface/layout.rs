use std::borrow::BorrowMut;

use bevy::prelude::*;

use bevy_egui::{ 
    egui::{
        widgets as egui_widgets,
        TopBottomPanel,
        CentralPanel,
        SidePanel
    }, 
    EguiContext 
};

use crate::{
    resources::{
        appearance::Appearance,
        data::DataFiles,
        magds::{ MainMAGDS, LoadedDatasets }
    },
    interface::{ data, appearance }
};

pub fn app_layout(
    mut egui_context: ResMut<EguiContext>, 
    mut windows: ResMut<Windows>,
    mut data_files_res: ResMut<DataFiles>,
    mut loaded_datasets_res: ResMut<LoadedDatasets>,
    mut magds_res: ResMut<MainMAGDS>,
    mut appearance_res: ResMut<Appearance>,
) {
    top_panel(&mut egui_context);
    left_panel(
        &mut egui_context,
        &mut data_files_res,
        &mut loaded_datasets_res,
        &mut magds_res,
        &mut appearance_res
    );
    right_panel(&mut egui_context);
    bottom_panel(&mut egui_context);
    central_panel(&mut egui_context);
}

fn top_panel(egui_context: &mut EguiContext) {
    TopBottomPanel::top("top_panel").show(egui_context.ctx_mut(), |ui| {
        ui.horizontal_wrapped(|ui| {
            egui_widgets::global_dark_light_mode_switch(ui);
            
            ui.separator();
            
            let mut state = true;
            ui.toggle_value(&mut state, "🖹 data");

            let mut state2 = true;
            // ui.toggle_value(&mut state2, "🖵 appearance");
            ui.toggle_value(&mut state2, "🔧 appearance");

            ui.separator();

            let mut state = true;
            ui.toggle_value(&mut state, "❄ sensors");
            let mut state2 = true;
            ui.toggle_value(&mut state2, "Ψ neurons");
            let mut state3 = true;
            ui.toggle_value(&mut state3, "🎟 connections");
        });
    });
}

fn left_panel(
    egui_context: &mut ResMut<EguiContext>,
    data_files_res: &mut ResMut<DataFiles>,
    loaded_datasets_res: &mut ResMut<LoadedDatasets>,
    magds_res: &mut ResMut<MainMAGDS>,
    appearance_res: &mut ResMut<Appearance>,
) {
    SidePanel::left("left_panel")
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("🖹 data");
            });

            ui.separator();

            data::data_window(
                ui, data_files_res, loaded_datasets_res, magds_res, appearance_res
            );
        }
    );
}

fn right_panel(egui_context: &mut EguiContext) {
    SidePanel::right("right_panel")
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("❄ sensors");
            });

            ui.separator();
        }
    );
}

fn bottom_panel(egui_context: &mut EguiContext) {
    TopBottomPanel::bottom("bottom_panel").show(egui_context.ctx_mut(), |ui| {
        ui.horizontal_wrapped(|ui| {
            let mut state = true;
            ui.toggle_value(&mut state, "🔳 2D simulation");
            let mut state2 = true;
            ui.toggle_value(&mut state2, "📦 3D simulation");
        });
    });
}

fn central_panel(egui_context: &mut EguiContext) {
    CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label("central_panel")
        });
    });
}