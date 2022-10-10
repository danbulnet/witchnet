use witchnet_common::{
    data::DataTypeValue,
    sensor::Sensor,
    distances::{ Distance, DistanceChecked }
};

// pub fn mutual_information<S1: Sensor<DataTypeValue>, S2: Sensor<DataTypeValue>>(
//     s1: &S1, s2: &S2
// ) -> f64 {
//     let x = s1.to_vec();
//     let y = s1.to_vec();

//     let mut mutual_information = 0.0;
//     for xi in x {
//         for yi in y {
//             let mut max_distance: Option<f64> = None;
//             let normalized_distance = match xi.distance(yi) {
//                 DistanceChecked::Comparable(d) => {
//                     let max = 
//                 }
//                 DistanceChecked::Incomparable => 
//             }
//         }
//     }
    
//     mutual_information
// }

mod tests {
    use std::rc::Rc;
    
    use witchnet_common::data::DataTypeValue;

    #[test]
    fn mutual_information() {
        let x: Vec<DataTypeValue> = vec![false.into(), 2.into(), 3.into(), 4.into(), 5.into()];
        let y: Vec<DataTypeValue> = vec![3.0f64.into(), 2.into(), 3.into(), 4.into(), 5.into()];
        // println!("mutual_information {:?}", similarity::mutual_information(&x, &y));
    }
}