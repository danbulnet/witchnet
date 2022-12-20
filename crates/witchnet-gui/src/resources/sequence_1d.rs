use std::{
    sync::Arc,
    default::Default
};

type SeriesGenerator = fn() -> Vec<[f64; 2]>;
type SamplesGenerator = fn(&[[f64; 2]]) -> Vec<[f64; 2]>;

pub struct Sequence1D {
    pub selected_name: Option<Arc::<str>>,
    pub loaded_data_name: Option<Arc::<str>>,
    pub loaded_data: Option<Vec<[f64; 2]>>,
    pub examples: Vec<(Arc::<str>, SeriesGenerator)>,

    pub selected_sampling_method: Option<Arc::<str>>,
    pub loaded_sampling_method_name: Option<Arc::<str>>,
    pub loaded_sampling_method: Option<SamplesGenerator>,
    pub loaded_samples: Option<Vec<[f64; 2]>>,
    pub sampling_methods: Vec<(Arc::<str>, SamplesGenerator)>,
}

impl Default for Sequence1D {
    fn default() -> Sequence1D {
        let mut examples = Vec::new();
        let default_example: (Arc<str>, SeriesGenerator) = (
            "complex_trig".into(),
            examples::complex_trigonometric as fn() -> Vec<[f64; 2]>
        );
        examples.push(default_example.clone());
        examples.push(("tanh".into(), examples::tanh));

        let mut sampling_methods = Vec::new();
        let default_sampling_method: (Arc<str>, SamplesGenerator) = (
            "random".into(), 
            sampling::random as fn(&[[f64; 2]]) -> Vec<[f64; 2]>
        );
        sampling_methods.push(default_sampling_method.clone());

        let loaded_name = Some(default_example.0.clone());
        let loaded_data = Some(default_example.1());
        let loaded_sampling_method_name = Some(default_sampling_method.0.clone());
        let loaded_sampling_method = Some(default_sampling_method.1);
        let loaded_samples = Some(default_sampling_method.1(loaded_data.as_ref().unwrap()));

        Sequence1D {
            selected_name: None,
            loaded_data_name: loaded_name,
            loaded_data, 
            examples,
            selected_sampling_method: None,
            loaded_sampling_method_name,
            loaded_sampling_method,
            loaded_samples,
            sampling_methods
        }
    }
}

pub mod examples {
    use bevy_egui::egui::plot::PlotPoints;

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

pub mod sampling {
    use rand::{ seq::IteratorRandom, thread_rng };

    pub fn random(data: &[[f64; 2]]) -> Vec<[f64; 2]> {
        let mut rng = thread_rng();
        data.iter()
            .choose_multiple(&mut rng, data.len() / 100)
            .into_iter()
            .map(|x| *x)
            .collect()
        
    }
}