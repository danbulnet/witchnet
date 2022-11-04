use std::f64::consts::TAU;

use bevy::prelude::*;

use bevy_egui::egui::{ 
    Ui,
    plot::{
        Plot,
        Legend,
        Points,
        MarkerShape,
        Polygon,
        PlotPoints
    }
};

use crate::{
    resources::{
        appearance::{ Appearance, Selector },
        magds::MainMAGDS
    },
    utils
};

pub(crate) fn simulation(
    ui: &mut Ui,
    _magds_res: &mut ResMut<MainMAGDS>,
    appearance_res: &mut ResMut<Appearance>,
) {
    ui.label("simulation 2D");
    let plot = Plot::new("lines_demo")
        .legend(Legend::default())
        .allow_boxed_zoom(false)
        .label_formatter(|name, _value| format!("{name}"))
        .show_background(false)
        .show_x(true)
        .show_y(true)
        .show_axes([false, false]);
    plot.show(ui, |plot_ui| {
        let settings = &appearance_res.neurons[&Selector::All];
        let points = Points::new(vec![
            [1.0, 0.0],
            [2.0, 0.5],
            [3.0, 0.0],
            [4.0, 0.5],
            [5.0, 0.0],
        ])
            .name(format!("neurons"))
            .filled(true)
            .radius(settings.size)
            .shape(MarkerShape::Circle)
            .color(utils::color_bevy_to_egui(&settings.primary_color));

        if settings.show { plot_ui.points(points); }

        let polygon = Polygon::new(PlotPoints::from_parametric_callback(
            |t| (4.0 * t.sin() + 2.0 * t.cos(), 4.0 * t.cos() + 2.0 * t.sin()),
            0.0..TAU,
            100,
        ));

        plot_ui.polygon(polygon.name("Convex polygon"));
    });
}