use anyhow::{ Result, Context };

use witchnet_common::{
    data::{DataTypeValue, DataCategory},
    sensor::Sensor,
    distances::{ Distance, DistanceChecked }
};

pub fn mutual_information<S1: Sensor<DataTypeValue>, S2: Sensor<DataTypeValue>>(
    s1: &S1, s2: &S2
) -> Result<f64> {
    let mut mutual_information = 0.0;

    // let x = s1.to_vec();
    // let y = s1.to_vec();

    // if x.len() < 2 || y.len() < 2 { 
    //     anyhow::bail!("both sensors must have more than 2 elements")
    // }

    // let x_is_numerical = DataCategory::from(x.first().unwrap()).is_numerical();
    // let y_is_numerical = DataCategory::from(y.first().unwrap()).is_numerical();

    // let x_max: Option<f64> = if x_is_numerical {
    //     x.into_iter().max().context("can't find max value in numerical sensor s1")?.to_f64()
    // } else { None };
    // let y_max: Option<f64> = None;

    // for xi in x {
    //     for yi in y {
    //         let mut max_distance: Option<f64> = None;
    //         let normalized_distance = match xi.distance(&yi) {
    //             DistanceChecked::Comparable(d) => {
    //                 let max = 
    //             }
    //             DistanceChecked::Incomparable => 
    //         }
    //     }
    // }
    
    Ok(mutual_information)
}

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