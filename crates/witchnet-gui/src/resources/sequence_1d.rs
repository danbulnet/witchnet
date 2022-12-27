use std::{
    sync::Arc,
    default::Default
};

use rand::{ seq::IteratorRandom, thread_rng };

use bevy_egui::egui::plot::{ MarkerShape, PlotPoints };

use bevy::prelude::Color;

use mint::Point2;

use flex_points::algorithm as fp;

use crate::resources::sequential_data::SequentialDataFile;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum SequenceSelector {
    ComplexTrigonometric,
    Tanh,
    LoadedData(usize),
    None
}

impl SequenceSelector {
    pub(crate) fn data(&self) -> Vec<[f64; 2]> {
        match self {
            SequenceSelector::ComplexTrigonometric => Self::complex_trigonometric(),
            SequenceSelector::Tanh => Self::tanh(),
            SequenceSelector::LoadedData(_) => Self::loaded_data_to_sequence(),
            SequenceSelector::None => vec![],
        }
    }

    pub(crate) fn loaded_data_to_sequence() -> Vec<[f64; 2]> {
        vec![]
    }

    pub fn tanh() -> Vec<[f64; 2]> {
        let values = PlotPoints::from_parametric_callback(
            move |x| (x, 1.0 / (1.0 + (-2.5 * x).exp())),
            -10.0..10.0,
            2000,
        );
        values.points().into_iter().map(|p| [p.x, p.y]).collect()
    }

    pub fn complex_trigonometric() -> Vec<[f64; 2]> {
        let values = PlotPoints::from_parametric_callback(
            move |x| {
                (
                    x,
                    f64::sin(2.0 * x - 2.0) 
                    + x.powi(2).cos() 
                    + 0.5 * f64::cos(3.0 * f64::powi(x - 0.5, 2))
                    + x.tanh()
                )
            },
            -10.0..10.0,
            2000,
        );
        values.points().into_iter().map(|p| [p.x, p.y]).collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum SamplingMethodSelector {
    FlexPoints,
    RamerDouglasPeucker,
    Random,
    None
}

impl SamplingMethodSelector {
    pub(crate) fn samples(&self, data: &[[f64; 2]]) -> Vec<[f64; 2]> {
        match self {
            SamplingMethodSelector::FlexPoints => Self::flex_points(data),
            SamplingMethodSelector::RamerDouglasPeucker => Self::rdp(data),
            SamplingMethodSelector::Random => Self::random(data),
            SamplingMethodSelector::None => vec![],
        }
    }

    pub fn random(data: &[[f64; 2]]) -> Vec<[f64; 2]> {
        let mut rng = thread_rng();
        data.iter()
            .choose_multiple(&mut rng, data.len() / 100)
            .into_iter()
            .map(|x| *x)
            .collect()
    }

    pub fn flex_points(data: &[[f64; 2]]) -> Vec<[f64; 2]> {
        let x: Vec<f64> = data.into_iter().map(|x| x[0]).collect();
        let y: Vec<f64> = data.into_iter().map(|x| x[1]).collect();

        let output = fp::flex_points(
            &x,
            &y,
            &[0.0, 0.5, 0.2, 0.0],
            &[25, 25, 50]
        );

        output.into_iter().map(|i| [x[i], y[i]]).collect()
    }

    pub fn rdp(data: &[[f64; 2]]) -> Vec<[f64; 2]> {
        let x: Vec<f64> = data.into_iter().map(|x| x[0]).collect();
        let y: Vec<f64> = data.into_iter().map(|x| x[1]).collect();

        let data_points: Vec<Point2<f64>> = data.into_iter().map(|x| Point2::from(*x)).collect();

        ramer_douglas_peucker::rdp(&data_points, 0.05).into_iter()
            .map(|i| [x[i], y[i]])
            .collect()
    }
}

pub(crate) struct Sequence1D {
    pub selected_data_source: SequenceSelector,
    pub loaded_data_source: SequenceSelector,
    pub loaded_data: Vec<[f64; 2]>,

    pub selected_sampling_method: SamplingMethodSelector,
    pub loaded_sampling_method: SamplingMethodSelector,
    pub loaded_samples: Vec<[f64; 2]>,
    
    pub line_color: Color,
    pub line_width: f32,
    pub line_width_bounds: (f32, f32),
    pub samples_color: Color,
    pub samples_radius: f32,
    pub samples_bounds: (f32, f32),
    pub samples_shape: MarkerShape,
}

impl Default for Sequence1D {
    fn default() -> Sequence1D {
        let loaded_data = SequenceSelector::ComplexTrigonometric.data();

        Sequence1D {
            selected_sampling_method: SamplingMethodSelector::FlexPoints,
            loaded_sampling_method: SamplingMethodSelector::FlexPoints,
            loaded_samples: SamplingMethodSelector::FlexPoints.samples(&loaded_data),

            selected_data_source: SequenceSelector::ComplexTrigonometric,
            loaded_data_source: SequenceSelector::ComplexTrigonometric,
            loaded_data,

            line_color: Color::Rgba { 
                red: 135 as f32 / 255.0, 
                green: 62 as f32 / 255.0, 
                blue: 35 as f32 / 255.0, 
                alpha: 1.0f32 
            },
            line_width: 1.0,
            line_width_bounds: (0.0, 10.0),
            samples_color: Color::Rgba { 
                red: 30 as f32 / 255.0, 
                green: 129 as f32 / 255.0, 
                blue: 176 as f32 / 255.0, 
                alpha: 0.8f32
            },
            samples_radius: 5.0f32,
            samples_bounds: (0.0, 10.0),
            samples_shape: MarkerShape::Circle
        }
    }
}