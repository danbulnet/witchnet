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
        magds::{ MainMAGDS, LoadedDatasets },
        layout::{ 
            Layout, 
            LeftPanel, 
            DEFAULT_PANEL_SCROLL_WIDTH, 
            RightPanel, 
            CentralPanel as LayoutCentralPanel 
        }
    },
    interface::{ 
        data, 
        appearance,
        simulation_2d,
        simulation_3d,
        sensors,
        neurons,
        connections
    }
};

pub(crate) fn app_layout(
    mut egui_context: ResMut<EguiContext>,
    mut layout_res: ResMut<Layout>,
    mut data_files_res: ResMut<DataFiles>,
    mut loaded_datasets_res: ResMut<LoadedDatasets>,
    mut magds_res: ResMut<MainMAGDS>,
    mut appearance_res: ResMut<Appearance>,
) {
    top_panel(&mut egui_context, &mut layout_res);
    left_panel(
        &mut egui_context,
        &mut layout_res,
        &mut data_files_res,
        &mut loaded_datasets_res,
        &mut magds_res,
        &mut appearance_res
    );
    right_panel(
        &mut egui_context, 
        &mut layout_res,
        &mut magds_res, 
        &mut appearance_res
    );
    central_panel(
        &mut egui_context, 
        &mut layout_res,
        &mut magds_res, 
        &mut appearance_res
    );
}

fn top_panel(
    egui_context: &mut ResMut<EguiContext>,
    layout_res: &mut ResMut<Layout>
) {
    TopBottomPanel::top("top_panel").show(egui_context.ctx_mut(), |ui| {
        ui.horizontal_wrapped(|ui| {
            egui_widgets::global_dark_light_mode_switch(ui);
            
            ui.separator();
            
            let data_toggle = ui.toggle_value(&mut layout_res.data, "🖹 data");
            if data_toggle.clicked() { layout_res.data_clicked() }
            // ui.toggle_value(&mut state2, "🖵 appearance");
            let appearance_toggle = ui.toggle_value(&mut layout_res.appearance, "🔧 appearance");
            if appearance_toggle.clicked() { layout_res.appearance_clicked() }

            ui.separator();

            let toggole_2d = ui.toggle_value(&mut layout_res.simulation_2d, "🔳 2D simulation");
            if toggole_2d.clicked() { layout_res.simulation_2d_clicked() }
            let toggole_3d = ui.toggle_value(&mut layout_res.simulation_3d, "📦 3D simulation");
            if toggole_3d.clicked() { layout_res.simulation_3d_clicked() }

            ui.separator();

            let sensors_toggle = ui.toggle_value(&mut layout_res.sensors, "❄ sensors");
            if sensors_toggle.clicked() { layout_res.sensors_clicked() }
            let neurons_toggle = ui.toggle_value(&mut layout_res.neurons, "Ψ neurons");
            if neurons_toggle.clicked() { layout_res.neurons_clicked() }
            let connections_toggle = ui.toggle_value(&mut layout_res.connections, "🎟 connections");
            if connections_toggle.clicked() { layout_res.connections_clicked() }
        });
    });
}

fn left_panel(
    egui_context: &mut ResMut<EguiContext>,
    layout_res: &mut ResMut<Layout>,
    data_files_res: &mut ResMut<DataFiles>,
    loaded_datasets_res: &mut ResMut<LoadedDatasets>,
    magds_res: &mut ResMut<MainMAGDS>,
    appearance_res: &mut ResMut<Appearance>,
) {
    if let Some(lp) = layout_res.left_panel {
        SidePanel::left("left_panel")
            .resizable(false)
            .max_width(DEFAULT_PANEL_SCROLL_WIDTH)
            .min_width(DEFAULT_PANEL_SCROLL_WIDTH)
            .show(egui_context.ctx_mut(), |ui| {
                match lp {
                    LeftPanel::Data => {
                        ui.vertical_centered(|ui| {
                            ui.heading("🖹 data");
                        });
                        ui.separator();
                        data::data_window(
                            ui, data_files_res, loaded_datasets_res, magds_res, appearance_res
                        );
                    }
                    LeftPanel::Appearance => {
                        ui.vertical_centered(|ui| {
                            ui.heading("🔧 appearance");
                        });
                        ui.separator();
                        appearance::appearance_window(ui, appearance_res);
                    }
                };
            }
        );
    }
}

fn right_panel(
    egui_context: &mut ResMut<EguiContext>,
    layout_res: &mut ResMut<Layout>,
    magds_res: &mut ResMut<MainMAGDS>,
    appearance_res: &mut ResMut<Appearance>
) {
    if let Some(rp) = layout_res.right_panel {
        SidePanel::right("right_panel")
            .resizable(false)
            .max_width(DEFAULT_PANEL_SCROLL_WIDTH)
            .min_width(DEFAULT_PANEL_SCROLL_WIDTH)
            .show(egui_context.ctx_mut(), |ui| {
                match rp {
                    RightPanel::Sensors => {
                        ui.vertical_centered(|ui| {
                            ui.heading("❄ sensors");
                        });
                        ui.separator();
                        sensors::sensors(ui, magds_res, appearance_res);
                    }
                    RightPanel::Neurons => {
                        ui.vertical_centered(|ui| {
                            ui.heading("Ψ neurons");
                        });
                        ui.separator();
                        neurons::neurons(ui, magds_res, appearance_res);
                    }
                    RightPanel::Connections => {
                        ui.vertical_centered(|ui| {
                            ui.heading("🎟 connections");
                        });
                        ui.separator();
                        connections::connections(ui, magds_res, appearance_res);
                    }
                };
            }
        );
    }
}

fn central_panel(
    egui_context: &mut ResMut<EguiContext>,
    layout_res: &mut ResMut<Layout>,
    magds_res: &mut ResMut<MainMAGDS>,
    appearance_res: &mut ResMut<Appearance>
) {
    CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        match layout_res.central_panel {
            LayoutCentralPanel::Simulation2D => {
                simulation_2d::simulation(ui, magds_res, appearance_res);
            }
            LayoutCentralPanel::Simulation3D => {
                simulation_3d::simulation(ui, magds_res, appearance_res);
            }
        }
    });
}