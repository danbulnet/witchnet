use std::sync::Arc;

use bevy::prelude::*;

use bevy_egui::egui::{ 
    self, 
    Ui,
    Grid
};

use crate::{
    resources::{
        appearance::Appearance,
        sequence_1d::{ Sequence1D, SamplingMethodSelector },
        layout::DEFAULT_PANEL_WIDTH,
        common
    },
    interface::widgets as w,
    utils
};

pub(crate) fn flex_points(
    ui: &mut Ui,
    sequence_1d_res: &mut ResMut<Sequence1D>,
    appearance_res: &mut ResMut<Appearance>
) {
    egui::ScrollArea::vertical()
        .stick_to_bottom(true)
        .show(ui, |mut ui| {
            examples(ui, sequence_1d_res);
            
            sampling(ui, sequence_1d_res);    
            
            appearance(ui, sequence_1d_res);
    });
}

fn examples(ui: &mut Ui, sequence_1d_res: &mut ResMut<Sequence1D>) {
    let data_examples: Vec<Option<Arc<str>>> = (&sequence_1d_res.data_examples)
        .into_iter()
        .map(|x| Some(x.0.clone()))
        .collect();
    w::combobox_str_row(
        ui, 
        "examples", 
        &mut sequence_1d_res.selected_data_name, 
        &data_examples,
        common::NEUTRAL_ACTIVE_COLOR
    );

    ui.separator(); ui.end_row();
}

fn sampling(ui: &mut Ui, sequence_1d_res: &mut ResMut<Sequence1D>) {
    Grid::new("flex-points").show(ui, |ui| {
        ui.vertical(|ui| {
            ui.set_min_width(DEFAULT_PANEL_WIDTH - 25f32);

            w::heading_label(ui, "sampling", common::NEUTRAL_ACTIVE_COLOR);
            ui.radio_value(
                &mut sequence_1d_res.selected_sampling_method, 
                SamplingMethodSelector::FlexPoints, 
                "flex-points"
            );
            ui.radio_value(
                &mut sequence_1d_res.selected_sampling_method, 
                SamplingMethodSelector::RamerDouglasPeucker, 
                "ramer-douglas-peucker"
            );
            ui.radio_value(
                &mut sequence_1d_res.selected_sampling_method, 
                SamplingMethodSelector::Random, 
                "random"
            );
            ui.radio_value(
                &mut sequence_1d_res.selected_sampling_method, 
                SamplingMethodSelector::None, 
                "none"
            );
            ui.separator(); ui.end_row();
        });
    });
}

fn appearance(ui: &mut Ui, sequence_1d_res: &mut ResMut<Sequence1D>) {
    w::heading_label(ui, "appearance", common::NEUTRAL_ACTIVE_COLOR);

    w::color_picker_row(ui, "line color", &mut sequence_1d_res.line_color);

    let bounds = sequence_1d_res.samples_bounds.clone();
    w::slider_row(
        ui, 
        "sr", 
        &mut sequence_1d_res.samples_radius, 
        bounds
    );

    w::color_picker_row(ui, "samples color", &mut sequence_1d_res.samples_color);

    ui.separator(); ui.end_row();
}