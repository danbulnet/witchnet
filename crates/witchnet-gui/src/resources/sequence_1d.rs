use std::default::Default;

#[derive(Debug, Clone)]
pub struct Sequence1D {
    pub data: Option<Vec<[f64; 2]>>,
    pub samples: Option<Vec<[f64; 2]>>,
}

impl Default for Sequence1D {
    fn default() -> Sequence1D {
        Sequence1D { data: None, samples: None }
    }
}