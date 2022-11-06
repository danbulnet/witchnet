use bevy::prelude::*;

use bevy_egui::egui::{ 
    Ui,
    plot::{
        Plot,
        Legend,
        Points,
        MarkerShape,
        Text,
        PlotPoint
    }
};

use crate::{
    resources::{
        appearance::{ Appearance, Selector },
        magds::MainMAGDS
    },
    utils,
    interface::shapes
};

pub(crate) fn simulation(
    ui: &mut Ui,
    _magds_res: &mut ResMut<MainMAGDS>,
    appearance_res: &mut ResMut<Appearance>,
) {
    let simulation_settings = &mut appearance_res.simulation2d;

    let plot = Plot::new("lines_demo")
        .legend(Legend::default())
        .allow_boxed_zoom(false)
        .label_formatter(|name, _value| format!("{name}"))
        .show_background(false)
        .show_x(true)
        .show_y(true)
        .data_aspect(1.0)
        .x_axis_formatter(|_, _| "".to_string())
        .y_axis_formatter(|_, _| "".to_string())
        .show_axes(simulation_settings.show_grid);
    plot.show(ui, |plot_ui| {
        let settings = &appearance_res.neurons[&Selector::All];
        let points = Points::new(vec![
            [1.0, 0.0],
            [2.0, 0.5],
            [3.0, 0.0],
            [4.0, 0.5],
            [25.0, 0.0],
        ])
            .name(format!("neurons"))
            .filled(true)
            .radius(settings.size)
            .shape(MarkerShape::Square)
            .color(utils::color_bevy_to_egui(&settings.primary_color));

        if settings.show { plot_ui.points(points); }

        shapes::rounded_box_r25r01(
            plot_ui,
            "test",
            (0.0, 0.0),
            (2.0, 3.0),
            true,
            utils::color_bevy_to_egui(&settings.secondary_color)
        );

        shapes::rounded_box_r25r01(
            plot_ui,
            "test2",
            (1.0, 1.0),
            (2.5, 1.0),
            false,
            utils::color_bevy_to_egui(&settings.text_color)
        );

        Text::new(PlotPoint::new(0.0, 0.0), "wow").name("Text")
    });
}