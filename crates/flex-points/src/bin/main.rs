use flex_points::{
    derivative,
    algorithm 
};

fn main() {
    let d = derivative::find_derivative(&[1.0, 2.0, 3.0, 4.0, 5.0], 1, 1);

    let o3 = algorithm::flex_points(
        &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
        &[5.0, 7.0, 8.0, 8.0, 8.0, 8.0, 8.0, 9.0, 10.0],
        &[0.0, 0.5, 0.2, 0.0],
        &[1, 1, 1]
    );

    println!("{:?}", o3);
}