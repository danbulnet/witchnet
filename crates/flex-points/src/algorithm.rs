use ndarray::Array1;

use crate::derivative;

/// x is the x coordinate
/// y is the function value
/// m is the vector limiting noise derivatives d2, d3, and d4
/// s is the scaling vector of derivatives d1, d2, d3
pub fn flex_points(
    x: &[f64],
    y: &[f64],
    m: &[f64; 4],
    s: &[usize; 3]
) -> Array1<usize> {
    let first_derivative = derivative::find_derivative(
        y, s[0], 1
    );
    let second_derivative = derivative::find_derivative(
        first_derivative.as_slice().unwrap(), s[0], 2
    );
    let third_derivative = derivative::find_derivative(
        second_derivative.as_slice().unwrap(), s[1], 3
    );
    let fourth_derivative = derivative::find_derivative(
        third_derivative.as_slice().unwrap(), s[2], 4
    );

    let thid_derivative_last_index = third_derivative.len() - 1;

    let mut mx = Array1::<f64>::zeros(thid_derivative_last_index);
    
    for i in 0..thid_derivative_last_index {
        mx[i] = third_derivative[i] * third_derivative[i + 1];
    }

    let o3: Array1<usize> = mx.into_iter().enumerate().filter(|(i, x)| {
        if *x <= 0.0 { true } else { false }
    }).map(|(i, _x)| i)
    .collect();

    o3
}