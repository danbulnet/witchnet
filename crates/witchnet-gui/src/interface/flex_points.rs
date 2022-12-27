use bevy::prelude::*;

use bevy_egui::egui::{ 
    self, 
    Ui,
    Grid,
    ComboBox,
    plot::MarkerShape
};

use crate::{
    resources::{
        appearance::Appearance,
        sequence_1d::{ Sequence1D, SamplingMethodSelector, SequenceSelector },
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
        .show(ui, |ui| {
            data(ui, sequence_1d_res);
            
            sampling(ui, sequence_1d_res);    
            
            appearance(ui, sequence_1d_res);
    });
}

fn data(ui: &mut Ui, sequence_1d_res: &mut ResMut<Sequence1D>) {
    Grid::new("flex-points data").show(ui, |ui| {
        ui.vertical(|ui| {
            ui.set_min_width(DEFAULT_PANEL_WIDTH - 25f32);

            w::heading_label(ui, "data", common::NEUTRAL_ACTIVE_COLOR);
            ui.radio_value(
                &mut sequence_1d_res.selected_data_source, 
                SequenceSelector::ComplexTrigonometric, 
                "complex trigonometric"
            );
            ui.radio_value(
                &mut sequence_1d_res.selected_data_source, 
                SequenceSelector::Tanh, 
                "tanh"
            );
            ui.radio_value(
                &mut sequence_1d_res.selected_data_source, 
                SequenceSelector::LoadedData(0), 
                "loaded data"
            );
            ui.radio_value(
                &mut sequence_1d_res.selected_data_source, 
                SequenceSelector::None, 
                "none"
            );
            ui.separator(); ui.end_row();
        });
    });
}

fn sampling(ui: &mut Ui, sequence_1d_res: &mut ResMut<Sequence1D>) {
    Grid::new("flex-points sampling").show(ui, |ui| {
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
    w::heading_label(ui, "line settings", common::NEUTRAL_ACTIVE_COLOR);

    w::color_picker_row(ui, "line color", &mut sequence_1d_res.line_color);

    let bounds = sequence_1d_res.line_width_bounds.clone();
    w::slider_row(
        ui, 
        "size", 
        &mut sequence_1d_res.line_width, 
        bounds
    );

    ui.separator(); ui.end_row();

    w::heading_label(ui, "samples settings", common::NEUTRAL_ACTIVE_COLOR);

    w::color_picker_row(ui, "samples color", &mut sequence_1d_res.samples_color);

    let bounds = sequence_1d_res.samples_bounds.clone();
    w::slider_row(
        ui, 
        "size", 
        &mut sequence_1d_res.samples_radius, 
        bounds
    );

    ui.horizontal(|ui| {
        w::heading_label(ui, "shape", common::NEUTRAL_COLOR);
        let current_shape_str = utils::shape_to_string(&sequence_1d_res.samples_shape);
        ComboBox::from_id_source(&current_shape_str)
            .selected_text(utils::shrink_str(&current_shape_str, 25))
            .show_ui(ui, |ui| {
                let values = [
                    MarkerShape::Circle,
                    MarkerShape::Diamond,
                    MarkerShape::Square,
                    MarkerShape::Cross,
                    MarkerShape::Plus,
                    MarkerShape::Up,
                    MarkerShape::Down,
                    MarkerShape::Left,
                    MarkerShape::Right,
                    MarkerShape::Asterisk
                ];
                for value in values {
                    ui.selectable_value(
                        &mut sequence_1d_res.samples_shape, 
                        value, 
                        utils::shape_to_string(&value)
                    );
                }
            }
        );
    });

    ui.separator(); ui.end_row();
}