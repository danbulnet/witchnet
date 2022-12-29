use crate::approximation;

pub fn compression_factor_data(data: &[[f64; 2]], samples: &[[f64; 2]]) -> f64 {
    compression_factor(data.len(), samples.len())
}

pub fn compression_factor(input_size: usize, output_size: usize) -> f64 {
    input_size as f64 / output_size as f64
}

/// Percentage Root mean square Difference (PRD)
pub fn prd(data: &[[f64; 2]], samples: &[[f64; 2]]) -> anyhow::Result<f64> {
    Ok(nrmse(data, samples)? * 100.0)
}
/// Normalized Root Mean Square Error (NRMSE)
pub fn nrmse(data: &[[f64; 2]], samples: &[[f64; 2]]) -> anyhow::Result<f64> {
    if data.is_empty() { anyhow::bail!("data array is empty") }
    if samples.is_empty() { anyhow::bail!("samples array is empty") }

    let x: Vec<f64> = data.into_iter().map(|point| point[0]).collect();
    let y: Vec<f64> = data.into_iter().map(|point| point[1]).collect();

    let mut approximated_y: Vec<f64> = Vec::new();
    for xi in x {
        approximated_y.push(approximation::approximate_linearly(samples, xi)?);
    }
    
    let mut numerator: f64 = 0.0;
    let mut denominator: f64 = 0.0;
    for (i, approximated_yi) in approximated_y.into_iter().enumerate() {
        let yi = y[i];
        numerator += f64::powi(yi - approximated_yi, 2);
        denominator += yi.powi(2);
    }

    Ok(f64::sqrt(numerator / denominator))
}

/// Root Mean Square Error (RMSE)
pub fn rmse(data: &[[f64; 2]], samples: &[[f64; 2]]) -> anyhow::Result<f64> {
    if data.is_empty() { anyhow::bail!("data array is empty") }
    if samples.is_empty() { anyhow::bail!("samples array is empty") }

    let x: Vec<f64> = data.into_iter().map(|point| point[0]).collect();
    let y: Vec<f64> = data.into_iter().map(|point| point[1]).collect();

    let mut approximated_y: Vec<f64> = Vec::new();
    for xi in x {
        approximated_y.push(approximation::approximate_linearly(samples, xi)?);
    }
    
    let mut numerator: f64 = 0.0;
    let denominator: f64 = y.len() as f64;
    for (i, approximated_yi) in approximated_y.into_iter().enumerate() {
        let yi = y[i];
        numerator += f64::powi(yi - approximated_yi, 2);
    }

    Ok(f64::sqrt(numerator / denominator))
}

mod tests {
    #[test]
    fn compression_factor() {
        assert_eq!(super::compression_factor(50, 5), 10.0);

        assert_eq!(
            super::compression_factor_data(
                &[[1.0, 1.0], [2.0, 2.0], [3.0, 3.0], [4.0, 4.0], [5.0, 5.0]], 
                &[[1.0, 1.0]]
            ),
            5.0
        );
    }

    #[test]
    fn rmse() {
        assert_eq!(
            super::rmse(
                &[[1.0, 1.0], [2.0, 2.0], [3.0, 3.0], [4.0, 4.0], [5.0, 5.0]], 
                &[[1.0, 1.0], [5.0, 5.0]]
            ).unwrap(),
            0.0
        );

        assert_ne!(
            super::rmse(
                &[[1.0, 1.0], [2.0, 2.0], [3.0, 3.0], [4.0, 4.0], [5.0, 5.0]], 
                &[[1.0, 1.0], [5.0, 4.0]]
            ).unwrap(), 
            0.0
        );
    }

    #[test]
    fn nrmse() {
        assert_eq!(
            super::nrmse(
                &[[1.0, 1.0], [2.0, 2.0], [3.0, 3.0], [4.0, 4.0], [5.0, 5.0]], 
                &[[1.0, 1.0], [5.0, 5.0]]
            ).unwrap(),
            0.0
        );

        assert_ne!(
            super::nrmse(
                &[[1.0, 1.0], [2.0, 2.0], [3.0, 3.0], [4.0, 4.0], [5.0, 5.0]], 
                &[[1.0, 1.0], [5.0, 4.0]]
            ).unwrap(), 
            0.0
        );
    }

    #[test]
    fn prd() {
        assert_eq!(
            super::prd(
                &[[1.0, 1.0], [2.0, 2.0], [3.0, 3.0], [4.0, 4.0], [5.0, 5.0]], 
                &[[1.0, 1.0], [5.0, 5.0]]
            ).unwrap(),
            0.0
        );

        assert_ne!(
            super::prd(
                &[[1.0, 1.0], [2.0, 2.0], [3.0, 3.0], [4.0, 4.0], [5.0, 5.0]], 
                &[[1.0, 1.0], [5.0, 4.0]]
            ).unwrap(), 
            0.0
        );
    }
}