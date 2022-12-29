use crate::approximation;

pub fn compression_factor_data(data: &[[f64; 2]], samples: &[[f64; 2]]) -> anyhow::Result<f64> {
    compression_factor(data.len(), samples.len())
}

pub fn compression_factor(input_size: usize, output_size: usize) -> anyhow::Result<f64> {
    if input_size == 0 { anyhow::bail!("input_size is zero") }
    if output_size == 0 { anyhow::bail!("output_size is zero") }
    Ok(input_size as f64 / output_size as f64)
}

/// Percentage Root mean square Difference (PRD)
pub fn prd(data: &[[f64; 2]], samples: &[[f64; 2]]) -> anyhow::Result<f64> {
    Ok(nrmse(data, samples)? * 100.0)
}

/// Normalized Percentage Root mean square Difference (PRD)
pub fn nprd(data: &[[f64; 2]], samples: &[[f64; 2]]) -> anyhow::Result<f64> {
    Ok(minrmse(data, samples)? * 100.0)
}

/// Mean Independent Normalized Root Mean Square Error (NRMSE)
pub fn minrmse(data: &[[f64; 2]], samples: &[[f64; 2]]) -> anyhow::Result<f64> {
    if data.is_empty() { anyhow::bail!("data array is empty") }
    if samples.is_empty() { anyhow::bail!("samples array is empty") }

    let x: Vec<f64> = data.into_iter().map(|point| point[0]).collect();
    let y: Vec<f64> = data.into_iter().map(|point| point[1]).collect();

    let mean = (&y).into_iter().sum::<f64>() / y.len() as f64;

    let mut approximated_y: Vec<f64> = Vec::new();
    for xi in x {
        approximated_y.push(approximation::approximate_linearly(samples, xi)?);
    }
    
    let mut numerator: f64 = 0.0;
    let mut denominator: f64 = 0.0;
    for (i, approximated_yi) in approximated_y.into_iter().enumerate() {
        let yi = y[i];
        numerator += f64::powi(yi - approximated_yi, 2);
        denominator += f64::powi(yi - mean, 2);
    }

    Ok(f64::sqrt(numerator / denominator))
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

pub fn quality_score(data: &[[f64; 2]], samples: &[[f64; 2]]) -> anyhow::Result<f64> {
    let cf = compression_factor_data(data, samples)?;
    let prd = prd(data, samples)?;
    Ok(cf / prd)
}

pub fn normalized_quality_score(data: &[[f64; 2]], samples: &[[f64; 2]]) -> anyhow::Result<f64> {
    let cf = compression_factor_data(data, samples)?;
    let prd = nprd(data, samples)?;
    Ok(cf / prd)
}

mod tests {
    #[test]
    fn compression_factor() {
        assert_eq!(super::compression_factor(50, 5).unwrap(), 10.0);

        assert_eq!(
            super::compression_factor_data(
                &[[1.0, 1.0], [2.0, 2.0], [3.0, 3.0], [4.0, 4.0], [5.0, 5.0]], 
                &[[1.0, 1.0]]
            ).unwrap(),
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
    fn minrmse() {
        assert_eq!(
            super::minrmse(
                &[[1.0, 1.0], [2.0, 2.0], [3.0, 3.0], [4.0, 4.0], [5.0, 5.0]], 
                &[[1.0, 1.0], [5.0, 5.0]]
            ).unwrap(),
            0.0
        );

        assert_ne!(
            super::minrmse(
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

    #[test]
    fn nprd() {
        assert_eq!(
            super::nprd(
                &[[1.0, 1.0], [2.0, 2.0], [3.0, 3.0], [4.0, 4.0], [5.0, 5.0]], 
                &[[1.0, 1.0], [5.0, 5.0]]
            ).unwrap(),
            0.0
        );

        assert_ne!(
            super::nprd(
                &[[1.0, 1.0], [2.0, 2.0], [3.0, 3.0], [4.0, 4.0], [5.0, 5.0]], 
                &[[1.0, 1.0], [5.0, 4.0]]
            ).unwrap(), 
            0.0
        );
    }

    #[test]
    fn quality_score() {
        assert_eq!(
            super::quality_score(
                &[[1.0, 1.0], [2.0, 2.0], [3.0, 3.0], [4.0, 4.0], [5.0, 5.0]], 
                &[[1.0, 1.0], [5.0, 5.0]]
            ).unwrap(),
            f64::INFINITY
        );

        assert_ne!(
            super::quality_score(
                &[[1.0, 1.0], [2.0, 2.0], [3.0, 3.0], [4.0, 4.0], [5.0, 5.0]], 
                &[[1.0, 1.0], [5.0, 4.0]]
            ).unwrap(), 
            0.0
        );
    }

    #[test]
    fn normalized_quality_score() {
        assert_eq!(
            super::normalized_quality_score(
                &[[1.0, 1.0], [2.0, 2.0], [3.0, 3.0], [4.0, 4.0], [5.0, 5.0]], 
                &[[1.0, 1.0], [5.0, 5.0]]
            ).unwrap(),
            f64::INFINITY
        );

        assert_ne!(
            super::normalized_quality_score(
                &[[1.0, 1.0], [2.0, 2.0], [3.0, 3.0], [4.0, 4.0], [5.0, 5.0]], 
                &[[1.0, 1.0], [5.0, 4.0]]
            ).unwrap(), 
            0.0
        );
    }
}