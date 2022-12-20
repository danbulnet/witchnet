use std::ops::RangeInclusive;

use bevy::prelude::*;

use bevy_egui::egui::{ 
    Ui, 
    plot::{
        Line, 
        Plot,
        PlotPoint, 
        PlotPoints,
        GridInput, 
        GridMark,
        Points,
        MarkerShape
    }
};

use crate::{
    resources::{
        appearance::Appearance,
        sequence_1d::Sequence1D
    }
};

pub(crate) fn simulation(
    ui: &mut Ui,
    sequence_1d_res: &mut ResMut<Sequence1D>,
    _appearance_res: &mut ResMut<Appearance>,
) {
    // let plot = Plot::new("flex-points")
    //     .allow_scroll(false)
    //     .allow_boxed_zoom(true)
    //     // .label_formatter(|name, _value| format!("{name}"))
    //     .show_background(true)
    //     .show_x(true)
    //     .show_y(true)
    //     .data_aspect(1.0);
        // .x_axis_formatter(|_, _| "".to_string())
        // .y_axis_formatter(|_, _| "".to_string())
        // .show_axes(simulation_settings.show_grid);
        if let Some(loaded_name) = &sequence_1d_res.loaded_data_name {
        if let Some(selected_name) = &sequence_1d_res.selected_name {
            if loaded_name != selected_name {
                let mut example = sequence_1d_res.examples.first().unwrap().clone();
                for current_example in &sequence_1d_res.examples {
                    if current_example.0 == *selected_name {
                        example = current_example.clone();
                    }
                };
                sequence_1d_res.loaded_data_name = Some(example.0.clone());
                sequence_1d_res.loaded_data = Some(example.1());
                sequence_1d_res.loaded_samples = Some(
                    sequence_1d_res.loaded_sampling_method.unwrap()(
                        sequence_1d_res.loaded_data.as_ref().unwrap()
                    )
                );
            }
        }
    } else {
        let example = sequence_1d_res.examples.first().unwrap().clone();
        sequence_1d_res.loaded_data_name = Some(example.0.clone());
        sequence_1d_res.loaded_data = Some(example.1());
        sequence_1d_res.loaded_samples = Some(
            sequence_1d_res.loaded_sampling_method.unwrap()(
                sequence_1d_res.loaded_data.as_ref().unwrap()
            )
        );
    }

    sequence_1d(ui, sequence_1d_res);
}

const MINS_PER_DAY: f64 = 24.0 * 60.0;
const MINS_PER_H: f64 = 60.0;

// fn sample_points() -> Vec<[f64; 2]> {
//     let f = |x: f64| {
//         f64::sin(2.0 * x - 2.0) 
//         + x.powi(2).cos() 
//         + 0.5 * f64::cos(3.0 * f64::powi(x - 0.5, 2))
//         + x.tanh()
//     };
//     let s = curve_sampling::Sampling::fun(f, -10.0, 10.0).build();
//     let mut si = s.iter();
//     let mut v = vec![];
//     while let Some(point) = si.next() {
//         v.push(point.unwrap());
//     }
//     v
// }

fn logistic_fn() -> Vec<[f64; 2]> {
    fn days(min: f64) -> f64 {
        MINS_PER_DAY * min
    }

    // y=sin(2x-2)+cos(x^{2})+0.5cos(3(x-0.5)^{2})+tanh(x)
    let values = PlotPoints::from_parametric_callback(
        // move |x| 1.0 / (1.0 + (-2.5 * (x / MINS_PER_DAY - 2.0)).exp()),
        move |x| {
            (
                x,
                f64::sin(2.0 * x - 2.0) 
                + x.powi(2).cos() 
                + 0.5 * f64::cos(3.0 * f64::powi(x - 0.5, 2))
                + x.tanh()
            )
        },
        // days(0.0)..days(5.0),
        -10.0..10.0,
        2000,
    );

    values.points().into_iter().map(|p| [p.x, p.y]).collect()
    // Line::new(values)
}

fn x_grid(input: GridInput) -> Vec<GridMark> {
    // Note: this always fills all possible marks. For optimization, `input.bounds`
    // could be used to decide when the low-interval grids (minutes) should be added.

    let mut marks = vec![];

    let (min, max) = input.bounds;
    let min = min.floor() as i32;
    let max = max.ceil() as i32;

    for i in min..=max {
        let step_size = if i % MINS_PER_DAY as i32 == 0 {
            // 1 day
            MINS_PER_DAY
        } else if i % MINS_PER_H as i32 == 0 {
            // 1 hour
            MINS_PER_H
        } else if i % 5 == 0 {
            // 5min
            5.0
        } else {
            // skip grids below 5min
            continue;
        };

        marks.push(GridMark {
            value: i as f64,
            step_size,
        });
    }

    marks
}

fn sequence_1d(ui: &mut Ui, sequence_1d_res: &mut ResMut<Sequence1D>) {
    fn day(x: f64) -> f64 {
        (x / MINS_PER_DAY).floor()
    }

    fn hour(x: f64) -> f64 {
        (x.rem_euclid(MINS_PER_DAY) / MINS_PER_H).floor()
    }

    fn minute(x: f64) -> f64 {
        x.rem_euclid(MINS_PER_H).floor()
    }

    fn percent(y: f64) -> f64 {
        100.0 * y
    }

    let x_fmt = |x, _range: &RangeInclusive<f64>| {
        if x < 0.0 * MINS_PER_DAY || x >= 5.0 * MINS_PER_DAY {
            // No labels outside value bounds
            String::new()
        } else if is_approx_integer(x / MINS_PER_DAY) {
            // Days
            format!("Day {}", day(x))
        } else {
            // Hours and minutes
            format!("{h}:{m:02}", h = hour(x), m = minute(x))
        }
    };

    let y_fmt = |y, _range: &RangeInclusive<f64>| {
        // Display only integer percentages
        if !is_approx_zero(y) && is_approx_integer(100.0 * y) {
            format!("{:.0}%", percent(y))
        } else {
            String::new()
        }
    };

    let label_fmt = |_s: &str, val: &PlotPoint| {
        format!(
            "Day {d}, {h}:{m:02}\n{p:.2}%",
            d = day(val.x),
            h = hour(val.x),
            m = minute(val.x),
            p = percent(val.y)
        )
    };

    ui.label("Zoom in on the X-axis to see hours and minutes");

    Plot::new("custom_axes")
        .data_aspect(1.0)
        // .data_aspect(2.0 * MINS_PER_DAY as f32)
        // .x_axis_formatter(x_fmt)
        // .y_axis_formatter(y_fmt)
        // .x_grid_spacer(x_grid)
        // .label_formatter(label_fmt)
        .show(ui, |plot_ui| {
            if let Some(data) = &sequence_1d_res.loaded_data {
                plot_ui.line(Line::new(PlotPoints::from(data.clone())));
            }
            
            if let Some(samples) = &sequence_1d_res.loaded_samples {
                let points = Points::new(samples.clone())
                    .name("samples")
                    .filled(true)
                    .radius(5.0)
                    .shape(MarkerShape::Circle);
                plot_ui.points(points);
            }

        });
}

fn is_approx_zero(val: f64) -> bool {
    val.abs() < 1e-6
}

fn is_approx_integer(val: f64) -> bool {
    val.fract().abs() < 1e-6
}