pub fn compression_factor_data(data: &[f64], samples: &[f64]) -> f64 {
    compression_factor(data.len(), samples.len())
}

pub fn compression_factor(input_size: usize, output_size: usize) -> f64 {
    input_size as f64 / output_size as f64
}

pub fn prd(data: &[f64], samples: &[f64]) -> f64 {
    0.0
}

mod tests {
    #[test]
    fn compression_factor() {
        assert_eq!(super::compression_factor(50, 5), 10.0);

        assert_eq!(super::compression_factor_data(&[1.0, 2.0, 3.0, 4.0, 5.0], &[1.0]), 5.0);
    }
}