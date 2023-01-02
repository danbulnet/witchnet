use std::{
    sync::Arc,
    default::Default,
    string::ToString
};

use ndarray::Array1;

use rand::{ seq::IteratorRandom, thread_rng };

use bevy_egui::egui::plot::{ MarkerShape, PlotPoints, LineStyle };

use bevy::prelude::*;

use mint::Point2;

use flex_points::{
    algorithm as fp,
    approximation,
    measures
};

use crate::resources::sequential_data::SequentialDataFiles;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum SequenceSelector {
    ComplexTrigonometric,
    ComplexTrigonometricShort,
    Tanh,
    LoadedData(String),
    None
}

impl SequenceSelector {
    pub(crate) fn data(
        &self, loaded_data: Option<&mut ResMut<SequentialDataFiles>>    
    ) -> Vec<[f64; 2]> {
        match self {
            SequenceSelector::ComplexTrigonometric => Self::complex_trigonometric(),
            SequenceSelector::ComplexTrigonometricShort => Self::complex_trigonometric_short(),
            SequenceSelector::Tanh => Self::tanh(),
            SequenceSelector::LoadedData(name) 
                => Self::loaded_data_to_sequence(loaded_data, name),
            SequenceSelector::None => vec![],
        }
    }

    pub(crate) fn loaded_data_to_sequence(
        loaded_data: Option<&mut ResMut<SequentialDataFiles>>, name: &str
    ) -> Vec<[f64; 2]> {
        if let Some(data_files) = loaded_data {
            if let Some(data_file) = data_files.current_data_file() {
                if let Some(data_frame) = &data_file.data_frame {
                    if let Ok(column) = data_frame.column(name) {
                        if column.is_numeric_physical() {
                            if let Ok(float_vec) = column.f64() {
                                return float_vec.into_iter()
                                    .enumerate()
                                    .filter(|(i, x)| x.is_some())
                                    .map(|(i, x)| [i as f64, x.unwrap()])
                                    .collect()
                            }
                        }
                    }
                }
                
                vec![]
            } else { vec![] }
        } else { vec![] }
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

    pub fn complex_trigonometric_short() -> Vec<[f64; 2]> {
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
            -2.0..2.0,
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
    Equal,
    None
}

impl SamplingMethodSelector {
    pub(crate) fn samples_default(&self, data: &[[f64; 2]]) -> Vec<[f64; 2]> {
        match self {
            SamplingMethodSelector::FlexPoints => Self::flex_points_default(data),
            SamplingMethodSelector::RamerDouglasPeucker => Self::rdp_default(data),
            SamplingMethodSelector::Random => Self::random_default(data),
            SamplingMethodSelector::Equal => Self::equal_default(data),
            SamplingMethodSelector::None => vec![],
        }
    }

    pub(crate) fn samples(
        &self, 
        data: &[[f64; 2]],
        sequence_1d: &Sequence1D 
    ) -> Vec<[f64; 2]> {
        match self {
            SamplingMethodSelector::FlexPoints => Self::flex_points(
                data, &sequence_1d.flex_points
            ),
            SamplingMethodSelector::RamerDouglasPeucker => Self::rdp(data, &sequence_1d.rdp),
            SamplingMethodSelector::Random => Self::random(data, sequence_1d.random_sampling_n),
            SamplingMethodSelector::Equal => Self::equal(data, sequence_1d.equal_sampling_n),
            SamplingMethodSelector::None => vec![],
        }
    }

    pub fn random_default(data: &[[f64; 2]]) -> Vec<[f64; 2]> {
        Self::random(data, data.len() / 10)
    }

    pub fn random(data: &[[f64; 2]], mut n: usize) -> Vec<[f64; 2]> {
        let data_len = data.len();
        if data_len == 0 { return vec![] }
        if n > data_len { n = data_len }
        let mut rng = thread_rng();
        let mut result: Vec<[f64; 2]> = data.iter()
            .choose_multiple(&mut rng, n)
            .into_iter()
            .map(|x| *x)
            .collect();

        result.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());

        let data_first = *data.first().unwrap();
        if let Some(f) = result.first() {
            if f != &data_first { result.push(data_first) }
        } else { result.push(data_first) }

        let data_last = *data.last().unwrap();
        if let Some(f) = result.last() {
            if f != &data_last { result.push(data_last) }
        } else { result.push(data_last) }

        result.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());

        result
    }

    pub fn equal_default(data: &[[f64; 2]]) -> Vec<[f64; 2]> {
        Self::equal(data, data.len() / 10)
    }

    pub fn equal(data: &[[f64; 2]], mut n: usize) -> Vec<[f64; 2]> {
        let data_len = data.len();
        if data_len == 0 { return vec![] }
        if n > data_len { n = data_len }

        let step = data_len / n;
        let mut result: Vec<[f64; 2]> = Vec::new();
        for i in 0..n {
            result.push(data[i * step]);
        }
        if n == 0 { result.push(data[0]); }
        let data_last = data.last().unwrap();
        if result.last().unwrap() != data_last {
            result.push(*data_last);
        }

        result
    }

    pub fn flex_points_default(data: &[[f64; 2]]) -> Vec<[f64; 2]> {
        Self::flex_points(data, &FlexPointsParams::default())
    }
    
    pub fn flex_points(data: &[[f64; 2]], config: &FlexPointsParams) -> Vec<[f64; 2]> {
        let x: Vec<f64> = data.into_iter().map(|x| x[0]).collect();
        let y: Vec<f64> = data.into_iter().map(|x| x[1]).collect();

        let output = fp::flex_points(
            &x,
            &y,
            &[
                config.first_derivative, 
                config.second_derivative, 
                config.third_derivative, 
                config.fourth_derivative
            ]
        ).unwrap_or(Array1::<usize>::from_vec(vec![]));

        output.into_iter().map(|i| [x[i], y[i]]).collect()
    }

    pub fn rdp_default(data: &[[f64; 2]]) -> Vec<[f64; 2]> {
        Self::rdp(data, &RamerDouglasPeuckerParams::default())
    }

    pub fn rdp(data: &[[f64; 2]], config: &RamerDouglasPeuckerParams) -> Vec<[f64; 2]> {
        let x: Vec<f64> = data.into_iter().map(|x| x[0]).collect();
        let y: Vec<f64> = data.into_iter().map(|x| x[1]).collect();

        let data_points: Vec<Point2<f64>> = data.into_iter().map(|x| Point2::from(*x)).collect();

        ramer_douglas_peucker::rdp(&data_points, config.epsilon as f64).into_iter()
            .map(|i| [x[i], y[i]])
            .collect()
    }
}

impl ToString for SamplingMethodSelector {
    fn to_string(&self) -> String {
        match self {
            SamplingMethodSelector::FlexPoints => "flex points",
            SamplingMethodSelector::RamerDouglasPeucker => "rdp",
            SamplingMethodSelector::Random => "random",
            SamplingMethodSelector::Equal => "equal",
            SamplingMethodSelector::None => "none",
        }.to_string()
    }
}

pub(crate) struct Sequence1D {
    pub selected_data_source: SequenceSelector,
    pub loaded_data_source: SequenceSelector,
    pub loaded_data: Vec<[f64; 2]>,

    pub selected_sampling_method: SamplingMethodSelector,
    pub loaded_sampling_method: SamplingMethodSelector,
    pub loaded_samples: Vec<[f64; 2]>,
    pub approximated_samples: Vec<[f64; 2]>,
    
    pub line_color: Color,
    pub line_width: f32,
    pub line_width_bounds: (f32, f32),
    pub aspect_ratio: f32,
    pub aspect_ratio_bounds: (f32, f32),
    pub line_style: LineStyle,
    pub line_style_spacing: f32,
    pub line_style_spacing_bounds: (f32, f32),

    pub samples_color: Color,
    pub samples_radius: f32,
    pub samples_bounds: (f32, f32),
    pub samples_shape: MarkerShape,
    pub approximation_line_color: Color,
    pub approximation_line_width: f32,
    pub approximation_line_width_bounds: (f32, f32),
    pub approximation_line_style: LineStyle,
    pub approximation_line_style_spacing: f32,
    pub approximation_line_style_spacing_bounds: (f32, f32),

    pub sampling_measures: SamplingMeasures,

    pub rdp: RamerDouglasPeuckerParams,
    pub flex_points: FlexPointsParams,
    pub random_sampling_n: usize,
    pub equal_sampling_n: usize,
}

impl Default for Sequence1D {
    fn default() -> Sequence1D {
        let loaded_data = SequenceSelector::ComplexTrigonometric.data(None);
        let loaded_samples = SamplingMethodSelector::FlexPoints.samples_default(&loaded_data);
        let loaded_samples_len = loaded_samples.len();
        let approximated_samples = (&loaded_data).into_iter()
            .map(|point| point[0])
            .map(|x| [x, approximation::approximate_linearly(&loaded_samples, x).unwrap()])
            .collect();
        
        let sampling_measures = Self::sampling_measures_data(&loaded_data, &loaded_samples);

        Sequence1D {
            selected_sampling_method: SamplingMethodSelector::FlexPoints,
            loaded_sampling_method: SamplingMethodSelector::FlexPoints,
            loaded_samples,
            approximated_samples,

            selected_data_source: SequenceSelector::ComplexTrigonometric,
            loaded_data_source: SequenceSelector::ComplexTrigonometric,
            loaded_data,

            line_color: Color::Rgba {
                red: 164 as f32 / 255.0, 
                green: 83 as f32 / 255.0, 
                blue: 40 as f32 / 255.0, 
                alpha: 1.0f32 
            },
            line_width: 1.5,
            line_width_bounds: (0.0, 10.0),
            aspect_ratio: 1.0,
            aspect_ratio_bounds: (0.1, 10.0),
            line_style: LineStyle::Solid,
            line_style_spacing: 2.0,
            line_style_spacing_bounds: (1.0, 10.0),

            samples_color: Color::Rgba { 
                red: 30 as f32 / 255.0, 
                green: 129 as f32 / 255.0, 
                blue: 176 as f32 / 255.0, 
                alpha: 0.58f32
            },
            samples_radius: 2.5f32,
            samples_bounds: (0.0, 10.0),
            samples_shape: MarkerShape::Circle,

            approximation_line_color: Color::Rgba { 
                red: 30 as f32 / 255.0, 
                green: 129 as f32 / 255.0, 
                blue: 176 as f32 / 255.0, 
                alpha: 0.5f32
            },
            approximation_line_width: 0.58,
            approximation_line_width_bounds: (0.0, 10.0),
            approximation_line_style: LineStyle::Dotted { spacing: 2.0 },
            approximation_line_style_spacing: 2.0,
            approximation_line_style_spacing_bounds: (1.0, 10.0),

            sampling_measures,

            rdp: RamerDouglasPeuckerParams::default(),
            flex_points: FlexPointsParams::default(),
            random_sampling_n: loaded_samples_len / 10,
            equal_sampling_n: loaded_samples_len / 10
        }
    }
}

impl Sequence1D {
    pub(crate) fn update_samples(&mut self) {
        self.loaded_samples = self.loaded_sampling_method.samples(
            &self.loaded_data, &self
        );

        self.approximated_samples = (&self.loaded_data).into_iter()
            .map(|point| point[0])
            .map(|x| [
                x, 
                approximation::approximate_linearly(&self.loaded_samples, x).unwrap_or(f64::NAN)
            ])
            .filter(|point| !point[1].is_nan())
            .collect();

        self.sampling_measures();
    }

    pub(crate) fn sampling_measures(&mut self) {
        let data = &self.loaded_data;
        let samples = &self.loaded_samples;

        self.sampling_measures = Self::sampling_measures_data(data, samples);
    }

    pub(crate) fn sampling_measures_data(
        data: &[[f64; 2]], samples: &[[f64; 2]]
    ) -> SamplingMeasures {   
        let compression_factor = measures::compression_factor_data(data, samples).ok();
        let rmse = measures::rmse(data, samples).ok();
        let nrmse = measures::nrmse(data, samples).ok();
        let minrmse = measures::minrmse(data, samples).ok();
        let prd = measures::prd(data, samples).ok();
        let nprd = measures::nprd(data, samples).ok();
        let quality_score = measures::quality_score(data, samples).ok();
        let normalized_quality_score = measures::normalized_quality_score(data, samples).ok();

        SamplingMeasures {
            compression_factor,
            rmse,
            nrmse,
            minrmse,
            prd,
            nprd,
            quality_score,
            normalized_quality_score
        }
    }
}

pub struct RamerDouglasPeuckerParams {
    pub epsilon: f32,
    pub epsilon_bounds: (f32, f32)
}

impl Default for RamerDouglasPeuckerParams {
    fn default() -> Self {
        Self { 
            epsilon: 0.05,
            epsilon_bounds: (0.001, 0.2)
        }
    }
}

pub struct FlexPointsParams {
    pub first_derivative: bool,
    pub second_derivative: bool,
    pub third_derivative: bool,
    pub fourth_derivative: bool,
}

impl Default for FlexPointsParams {
    fn default() -> Self {
        Self { 
            first_derivative: true,
            second_derivative: false,
            third_derivative: true,
            fourth_derivative: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SamplingMeasures {
    pub compression_factor: Option<f64>,
    pub rmse: Option<f64>,
    pub nrmse: Option<f64>,
    pub minrmse: Option<f64>,
    pub prd: Option<f64>,
    pub nprd: Option<f64>,
    pub quality_score: Option<f64>,
    pub normalized_quality_score: Option<f64>
}

impl Default for SamplingMeasures {
    fn default() -> Self {
        SamplingMeasures {
            compression_factor: None,
            rmse: None,
            nrmse: None,
            minrmse: None,
            prd: None,
            nprd: None,
            quality_score: None,
            normalized_quality_score: None
        }
    }
}

impl SamplingMeasures {
    pub fn value_to_string(value: &Option<f64>) -> String {
        if let Some(v) = value { format!("{:.3}", v) } else { "".to_string() }
    }
}