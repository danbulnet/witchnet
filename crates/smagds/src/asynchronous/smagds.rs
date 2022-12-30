use std::sync::{ Arc, RwLock };

use magds::asynchronous::magds::MAGDS;

use witchnet_common::{
    data::{ DataTypeValue, DataPoint2D, DataType },
    sensor::SensorData
};

#[derive(Debug, Clone)]
pub struct SMAGDS {
    pub(crate) magds: Arc<RwLock<MAGDS>>,
    pub(crate) data: Vec<DataPoint2D>
}

impl SMAGDS {
    pub fn new() -> Self {
        Self {
            magds: MAGDS::new_arc(),
            data: vec![]
        }
    }
    
    pub fn new_from_data<X: SensorData, Y: SensorData>(
        data: &[(X, Y)]
    ) -> Self where DataTypeValue: From<X> + From<Y> {
        let mut converted_data: Vec<DataPoint2D> = data.into_iter()
            .map(|(x, y)| {
                DataPoint2D {
                    x: (*dyn_clone::clone_box(x)).into(), 
                    y: (*dyn_clone::clone_box(y)).into()
                }
            }).collect();
        converted_data.sort_unstable_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

        Self {
            magds: MAGDS::new_arc(),
            data: converted_data
        }
    }

    pub fn add<X: SensorData, Y: SensorData>(
        &mut self, data: &[(X, Y)]
    ) -> anyhow::Result<()> where DataTypeValue: From<X> + From<Y> {
        if data.is_empty() { return Ok(()) }

        let mut converted_data: Vec<DataPoint2D> = data.into_iter()
            .map(|(x, y)| {
                DataPoint2D {
                    x: (*dyn_clone::clone_box(x)).into(), 
                    y: (*dyn_clone::clone_box(y)).into()
                }
            }).collect();
        
        let first_converted = &converted_data[0];
        let is_loaded_data_empty = self.data.is_empty();
        if is_loaded_data_empty || first_converted.x.is_type_same_as(&self.data[0].x) {
            if is_loaded_data_empty || first_converted.y.is_type_same_as(&self.data[0].y) {
                self.data.append(&mut converted_data);
                self.data.sort_unstable_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
            } else {
                anyhow::bail!(
                    "y: input data type {} and loaded data type {} are different", 
                    DataType::from(&first_converted.y),
                    DataType::from(&self.data[0].y),
                )
            }
        } else { 
            anyhow::bail!(
                "x: input data type {} and loaded data type {} are different", 
                DataType::from(&first_converted.x),
                DataType::from(&self.data[0].x),
            )
        }

        Ok(())
    }
}

mod tests {
    use super::SMAGDS;

    #[test]
    fn new() {
        let smagds = SMAGDS::new();
        assert_eq!(smagds.data.len(), 0);
        println!("{:?}", smagds);

        let smagds = SMAGDS::new_from_data(&[(1, 1), (2, 3), (3, 5)]);
        assert_eq!(smagds.data.len(), 3);
        println!("{:?}", smagds);

        let smagds = SMAGDS::new_from_data(&[(1, 1.0), (2, 3.0), (3, 5.0)]);
        assert_eq!(smagds.data.len(), 3);
        println!("{:?}", smagds);

        let smagds = SMAGDS::new_from_data(
            &[(1, "1.0".to_owned()), (2, "3.0".to_owned()), (3, "5.0".to_owned())]
        );
        assert_eq!(smagds.data.len(), 3);
        println!("{:?}", smagds);
    }

    #[test]
    fn add() {
        let mut smagds = SMAGDS::new();
        smagds.add(&[(1, 1.0), (2, 3.0), (3, 5.0)]);
        assert_eq!(smagds.data.len(), 3);
        println!("{:?}", smagds);
    }
}