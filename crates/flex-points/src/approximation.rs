pub fn approximate_linearly(data: &[[f64; 2]], target_x: f64) -> anyhow::Result<f64> {
    if data.is_empty() { anyhow::bail!("input array is empty") }
    
    let last_index = data.len() - 1;
    
    if target_x < data[0][0] || target_x > data[last_index][0] { 
        anyhow::bail!("target out of range") 
    }

    for (i, point) in data.into_iter().enumerate() {
        let x0 = point[0];
        let y0 = point[1];
        if target_x == x0 {
            return Ok(y0)
        } else if target_x < x0 {
            let x1 = data[i - 1][0];
            let y1 = data[i - 1][1];
            let slope = (y1 - y0) / (x1 - x0);
            let intercept = y0 - slope * x0;
            return Ok(slope * target_x + intercept)
        }
    }

    anyhow::bail!("you shouldn't be here, something went wrong")
}

mod tests {
    #[test]
    fn approximate() {
        assert_eq!(
            super::approximate_linearly(
                &[[1.0, 3.0], [2.0, 5.0], [3.0, 7.0], [5.0, 11.0]],
                4.0
            ).unwrap(), 
            9.0
        );
    }
}