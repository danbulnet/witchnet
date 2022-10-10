use witchnet_common::{data::DataTypeValue, sensor::SensorData};

pub fn mutual_information(x: &[DataTypeValue], y: &[DataTypeValue]) -> f64 {
    let first_x = x.first().unwrap();
    let first_y = y.first().unwrap();
    let distance = first_x.distance(first_y);
    distance
}

mod tests {
    use std::rc::Rc;
    
    use witchnet_common::data::DataTypeValue;

    #[test]
    fn mutual_information() {
        let x: Vec<DataTypeValue> = vec![1.0f32.into(), 2.into(), 3.into(), 4.into(), 5.into()];
        let y: Vec<DataTypeValue> = vec![3.0f32.into(), 2.into(), 3.into(), 4.into(), 5.into()];
        println!("{}", super::mutual_information(&x, &y));
    }
}