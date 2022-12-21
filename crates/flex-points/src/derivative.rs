use ndarray::Array1;

/// finds derivative of the function sampled in data variable
/// step determines the step size (increment on x axis)
/// zeros specifies how many values of derivative will be set to zero at the
/// beginning and at the end of the function
/// derivative is computed from function values before and after sample point
/// divided by twice the step size
pub fn find_derivative(
    data: &[f64], step: usize, zeros: usize
) -> Array1<f64> {
    let data_len = data.len();
    let end_index = data_len - 1;
    let mut derivative = Array1::<f64>::zeros(data_len);
    
    for i in 0..(data_len - 2 * zeros) {
        derivative[i + zeros] = (data[i + zeros + 1] - data[i + zeros - 1]) / (2.0 * step as f64);
    }

    if data_len >= zeros + 1 {
        for i in 0..zeros {
            derivative[i] = derivative[zeros];
        }
    }

    for i in (end_index - zeros)..(data_len) {
        derivative[i] = derivative[end_index - zeros]
    }

    derivative
}

mod tests {
    #[test]
    fn find_derivative() {
        let d = super::find_derivative(&[1.0, 2.0, 3.0, 4.0, 5.0], 1, 1);
        assert_eq!(d.as_slice().unwrap(), &[1.0, 1.0, 1.0, 1.0, 1.0]);

        let d = super::find_derivative(&[1.0, 2.0, 3.0, 4.0, 5.0], 1, 2);
        assert_eq!(d.as_slice().unwrap(), &[1.0, 1.0, 1.0, 1.0, 1.0]);
    }
}