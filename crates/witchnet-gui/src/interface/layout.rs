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
        tabular_data::{ TabularDataFiles, DATA_PANEL_SCROLL_WIDTH },
        sequential_data::SequentialDataFiles,
        magds::{ 
            MainMAGDS, 
            MAGDSLoadedDatasets, 
            MAGDSPositions 
        },
        sequential_model::{ 
            SequentialMAGDS, 
            SequentialModelLoadedDatasets, 
            SequentialModelPositions 
        },
        layout::{ 
            Layout, 
            DEFAULT_PANEL_SCROLL_WIDTH, 
            CentralPanel as LayoutCentralPanel 
        }, 
        sequence_1d::Sequence1D,
    },
    interface::{ 
        tabular_data, 
        sequential_data,
        appearance,
        magds_2d,
        magds_3d,
        sequential_model_2d,
        sequence_1d,
        sensors,
        neurons,
        connections
    }
};

use super::flex_points;

pub(crate) fn app_layout(
    mut egui_context: ResMut<EguiContext>,
    mut layout_res: ResMut<Layout>,
    mut tabular_data_files_res: ResMut<TabularDataFiles>,
    mut sequential_data_files_res: ResMut<SequentialDataFiles>,
    mut magds_loaded_datasets_res: ResMut<MAGDSLoadedDatasets>,
    mut sequential_model_loaded_datasets_res: ResMut<SequentialModelLoadedDatasets>,
    mut magds_res: ResMut<MainMAGDS>,
    mut sequential_model_res: ResMut<SequentialMAGDS>,
    mut magds_positions_res: ResMut<MAGDSPositions>,
    mut sequential_model_positions_res: ResMut<SequentialModelPositions>,
    mut sequence_1d_res: ResMut<Sequence1D>,
    mut appearance_res: ResMut<Appearance>,
) {
    top_panel(&mut egui_context, &mut layout_res);
    left_panel(
        &mut egui_context,
        &mut layout_res,
        &mut tabular_data_files_res,
        &mut sequential_data_files_res,
        &mut magds_loaded_datasets_res,
        &mut sequential_model_loaded_datasets_res,
        &mut magds_res,
        &mut sequential_model_res,
        &mut magds_positions_res,
        &mut sequential_model_positions_res,
        &mut appearance_res
    );
    right_panel(
        &mut egui_context, 
        &mut layout_res,
        &mut magds_res, 
        &mut sequence_1d_res,
        &mut appearance_res,
        &mut sequential_data_files_res
    );
    central_panel(
        &mut egui_context, 
        &mut layout_res,
        &mut magds_res,
        &mut sequential_model_res,
        &mut magds_positions_res,
        &mut sequential_model_positions_res,
        &mut sequence_1d_res,
        &mut appearance_res,
        &mut sequential_data_files_res
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
            
            ui.toggle_value(&mut layout_res.tabular_data, "🖹 tabular data");
            
            ui.toggle_value(&mut layout_res.sequential_data, "〰 sequential data");
            // ui.toggle_value(&mut state2, "🖵 appearance");
            ui.toggle_value(&mut layout_res.appearance, "🔧 appearance");
            
            ui.separator();

            let toggole_magds_2d = ui.toggle_value(&mut layout_res.magds_2d, "🔳 magds-2d");
            if toggole_magds_2d.clicked() { layout_res.magds_2d_clicked() }
            
            let toggole_magds_3d = ui.toggle_value(&mut layout_res.magds_3d, "📦 magds-3d");
            if toggole_magds_3d.clicked() { layout_res.magds_3d_clicked() }
            
            let toggole_sequential_model_2d = ui.toggle_value(
                &mut layout_res.sequential_model_2d, "⛓ sequential-model-2d"
            );
            if toggole_sequential_model_2d.clicked() { layout_res.sequential_model_2d_clicked() }
            
            let toggole_flex_points = ui.toggle_value(
                &mut layout_res.sequence_1d, "📈 sequence-1d"
            );
            if toggole_flex_points.clicked() { layout_res.sequence_1d_clicked() }

            ui.separator();

            ui.toggle_value(&mut layout_res.sensors, "❄ sensors");
            ui.toggle_value(&mut layout_res.neurons, "Ψ neurons");
            ui.toggle_value(&mut layout_res.connections, "🎟 connections");
            ui.toggle_value(&mut layout_res.flex_points, "∂ flex-points");
        });
    });
}

fn left_panel(
    egui_context: &mut ResMut<EguiContext>,
    layout_res: &mut ResMut<Layout>,
    tabular_data_files_res: &mut ResMut<TabularDataFiles>,
    sequential_data_files_res: &mut ResMut<SequentialDataFiles>,
    magds_loaded_datasets_res: &mut ResMut<MAGDSLoadedDatasets>,
    sequential_model_loaded_datasets_res: &mut ResMut<SequentialModelLoadedDatasets>,
    magds_res: &mut ResMut<MainMAGDS>,
    sequential_model_res: &mut ResMut<SequentialMAGDS>,
    magds_positions_res: &mut ResMut<MAGDSPositions>,
    sequential_model_positions_res: &mut ResMut<SequentialModelPositions>,
    appearance_res: &mut ResMut<Appearance>,
) {
    if layout_res.tabular_data {
        SidePanel::left("tabular_data_panel")
            .resizable(false)
            .max_width(DATA_PANEL_SCROLL_WIDTH)
            .min_width(DATA_PANEL_SCROLL_WIDTH)
            .show(egui_context.ctx_mut(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("🖹 tabular data");
                });
                ui.separator();
                tabular_data::tabular_data_window(
                    ui,
                    tabular_data_files_res,
                    magds_loaded_datasets_res,
                    magds_res,
                    magds_positions_res,
                    appearance_res
                );
            }
        );
    }
    if layout_res.sequential_data {
        SidePanel::left("sequential_data_panel")
            .resizable(false)
            .max_width(DATA_PANEL_SCROLL_WIDTH)
            .min_width(DATA_PANEL_SCROLL_WIDTH)
            .show(egui_context.ctx_mut(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("〰 sequential data");
                });
                ui.separator();
                sequential_data::sequential_data_window(
                    ui,
                    sequential_data_files_res,
                    sequential_model_loaded_datasets_res,
                    sequential_model_res,
                    sequential_model_positions_res,
                    appearance_res
                );
            }
        );
    }
    if layout_res.appearance {
        SidePanel::left("appearance_panel")
            .resizable(false)
            .max_width(DEFAULT_PANEL_SCROLL_WIDTH)
            .show(egui_context.ctx_mut(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("🔧 appearance");
                });
                ui.separator();
                appearance::appearance_window(ui, appearance_res);
            }
        );
    }
}

fn right_panel(
    egui_context: &mut ResMut<EguiContext>,
    layout_res: &mut ResMut<Layout>,
    magds_res: &mut ResMut<MainMAGDS>,
    sequence_1d_res: &mut ResMut<Sequence1D>,
    appearance_res: &mut ResMut<Appearance>,
    sequential_data_files_res: &mut ResMut<SequentialDataFiles>
) {
    if layout_res.sensors {
        SidePanel::right("sensors_panel")
            .resizable(false)
            .max_width(DEFAULT_PANEL_SCROLL_WIDTH)
            .show(egui_context.ctx_mut(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("❄ sensors");
                });
                ui.separator();
                sensors::sensors(ui, magds_res, appearance_res);
            }
        );
    }
    if layout_res.neurons {
        SidePanel::right("neurons_panel")
            .resizable(false)
            .max_width(DEFAULT_PANEL_SCROLL_WIDTH)
            .show(egui_context.ctx_mut(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Ψ neurons");
                });
                ui.separator();
                neurons::neurons(ui, magds_res, appearance_res);
            }
        );
    }
    if layout_res.connections {
        SidePanel::right("connections_panel")
            .resizable(false)
            .max_width(DEFAULT_PANEL_SCROLL_WIDTH)
            .show(egui_context.ctx_mut(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("🎟 connections");
                });
                ui.separator();
                connections::connections(ui, magds_res, appearance_res);
            }
        );
    }
    if layout_res.flex_points {
        SidePanel::right("flex_points")
            .resizable(false)
            .max_width(DEFAULT_PANEL_SCROLL_WIDTH)
            .show(egui_context.ctx_mut(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("∂ flex-points");
                });
                ui.separator();
                flex_points::flex_points(
                    ui, sequence_1d_res, sequential_data_files_res
                );
            }
        );
    }
}

fn central_panel(
    egui_context: &mut ResMut<EguiContext>,
    layout_res: &mut ResMut<Layout>,
    magds_res: &mut ResMut<MainMAGDS>,
    sequential_model_res: &mut ResMut<SequentialMAGDS>,
    magds_positions_res: &mut ResMut<MAGDSPositions>,
    sequential_model_points_res: &mut ResMut<SequentialModelPositions>,
    sequence_1d_res: &mut ResMut<Sequence1D>,
    appearance_res: &mut ResMut<Appearance>,
    sequential_data_files_res: &mut ResMut<SequentialDataFiles>
) {
    CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        match layout_res.central_panel {
            LayoutCentralPanel::MAGDS2D => {
                magds_2d::simulation(ui, magds_res, magds_positions_res, appearance_res);
            }
            LayoutCentralPanel::MAGDS3D => {
                magds_3d::simulation(ui, magds_res, appearance_res);
            },
            LayoutCentralPanel::SequentialModel2D => {
                sequential_model_2d::simulation(
                    ui, sequential_model_res, sequential_model_points_res, appearance_res
                );
            },
            LayoutCentralPanel::Sequence1D => {
                sequence_1d::simulation(
                    ui, sequence_1d_res, sequential_data_files_res
                );
            },
        }
    });
}