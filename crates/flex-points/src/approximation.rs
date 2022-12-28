/// assumes that x is sorted and paired with y
pub fn approximate_linearly(x: &[f64], y: &[f64], target_x: f64) -> anyhow::Result<f64> {
    if x.len() != y.len() { anyhow::bail!("x and y should be the same length") }
    if y.is_empty() { anyhow::bail!("input array is empty") }

    let last_index = y.len() - 1;
    for (i, x0) in x.into_iter().enumerate() {
        if target_x >= *x0 {
            if i == last_index { return Ok(*x0) }
            let y0 = y[i];
            let y1 = y[i + 1];
            let x1 = x[i + 1];
            let slope = (y1 - y0) / (x1 - x0);
            let intercept = y0 - slope * x0;
            let formula = |x: f64| slope * x + intercept;
            return Ok(formula(target_x))
        }
    }

    anyhow::bail!("you shouldn't be here, something went wrong")
}

mod tests {
    #[test]
    fn approximate() {
        assert_eq!(
            super::approximate_linearly(
                &[1.0, 2.0, 3.0, 5.0],
                &[3.0, 5.0, 7.0, 11.0],
                4.0
            ).unwrap(), 
            9.0
        )
    }
}