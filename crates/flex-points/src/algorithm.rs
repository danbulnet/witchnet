use std::collections::HashSet;

use ndarray::{ Array1, Axis, ArrayView };

use crate::derivative;

/// x is the x coordinate
/// y is the function value
/// derivatives_used is an array of boolean values that indicates which derivatives should be use
/// e.g. if `derivatives_used = &[true, false, true, false]` 
/// then the first and the third derivative will be used
pub fn flex_points(
    x: &[f64],
    y: &[f64],
    derivatives_used: &[bool; 4],
) -> anyhow::Result<Array1<usize>> {
    if x.len() != y.len() { anyhow::bail!("x and y should be the same length") }
    if y.is_empty() { anyhow::bail!("input array is empty") }

    let first_derivative = derivative::find_derivative(
        x, y, 1
    );
    let second_derivative = derivative::find_derivative(
        x, first_derivative.as_slice().unwrap(), 2
    );
    let third_derivative = derivative::find_derivative(
        x, second_derivative.as_slice().unwrap(), 3
    );
    let fourth_derivative = derivative::find_derivative(
        x, third_derivative.as_slice().unwrap(), 4
    );

    let mut output = Array1::<usize>::from_vec(vec![]);

    if derivatives_used[0] {
        let output1 = find_negative_derivatives(&first_derivative);
        output.append(Axis(0), output1.view()).unwrap();
    }
    if derivatives_used[1] {
        let output2 = find_negative_derivatives(&second_derivative);
        output.append(Axis(0), output2.view()).unwrap();
    }
    if derivatives_used[2] {
        let output3 = find_negative_derivatives(&third_derivative);
        output.append(Axis(0), output3.view()).unwrap();
    }
    if derivatives_used[3] {
        let output4 = find_negative_derivatives(&fourth_derivative);
        output.append(Axis(0), output4.view()).unwrap();
    }

    Ok(output)
}

fn find_negative_derivatives(derivatives: &Array1<f64>) -> Array1<usize> {
    let derivatives_last_index = derivatives.len() - 1;
    let mut mx = Array1::<f64>::zeros(derivatives_last_index);  
    for i in 0..derivatives_last_index {
        mx[i] = derivatives[i] * derivatives[i + 1];
    }

    let o3: Array1<usize> = mx.into_iter().enumerate().filter(|(_i, x)| {
        if *x <= 0.0 { true } else { false }
    }).map(|(i, _x)| i).collect();

    let mut o3_tail: Array1<usize> = if let Some(first) = o3.first() {
        if first != &0usize {
            let mut new_array = Array1::<usize>::from_vec(vec![0usize]);
            new_array.append(Axis(0), o3.view()).unwrap();
            new_array
        } else {
            o3
        }
    } else {
        Array1::<usize>::from_vec(vec![0usize])
    };

    let o3_tails: Array1<usize> = {
        let last = o3_tail.last().unwrap();
        let last_index = derivatives.len() - 1;
        if last != &last_index {
            o3_tail.append(Axis(0), ArrayView::from(&vec![last_index])).unwrap();
            o3_tail
        } else {
            o3_tail
        }
    };

    o3_tails
}

mod tests {
    #[test]
    fn flex_points() {
        let o3 = super::flex_points(
            &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
            &[5.0, 7.0, 8.0, 8.0, 8.0, 8.0, 8.0, 9.0, 10.0],
            &[true, false, true, false]
        ).unwrap();

        assert!(o3.len() > 0);
    
        println!("{:?}", o3);
    }
}