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
        sequence_1d::Sequence1D,
        layout::DEFAULT_PANEL_WIDTH,
        common
    },
    interface::widgets as w,
    utils
};

pub(crate) fn flex_points(
    ui: &mut Ui,
    sequence_1d: &mut ResMut<Sequence1D>,
    appearance_res: &mut ResMut<Appearance>
) {
    egui::ScrollArea::vertical()
        .stick_to_bottom(true)
        .show(ui, |mut ui| {
            ui.set_min_width(DEFAULT_PANEL_WIDTH);
            
            Grid::new("sensor").show(&mut ui, |ui| {
                let label_color = common::NEUTRAL_ACTIVE_COLOR;
                
                let data_examples: Vec<Option<Arc<str>>> = (&sequence_1d.data_examples)
                    .into_iter()
                    .map(|x| Some(x.0.clone()))
                    .collect();
                w::combobox_str_row(
                    ui, 
                    "examples", 
                    &mut sequence_1d.selected_data_name, 
                    &data_examples,
                    label_color
                );

                let sampling_methods: Vec<Option<Arc<str>>> = (&sequence_1d.sampling_methods)
                    .into_iter()
                    .map(|x| Some(x.0.clone()))
                    .collect();
                w::combobox_str_row(
                    ui, 
                    "sampling", 
                    &mut sequence_1d.selected_sampling_method_name, 
                    &sampling_methods,
                    label_color
                );
        
                // w::checkbox_row(ui, "show", &mut sensor.show);
        
                // w::slider_row(ui, "size", &mut sensor.size, sensor.size_bounds);
            });
    });
}