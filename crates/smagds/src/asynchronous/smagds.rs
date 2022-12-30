use std::sync::{ Arc, RwLock };

use magds::asynchronous::magds::MAGDS;

use witchnet_common::{
    data::{ DataTypeValue, DataPoint2D, DataType, DataDeductor },
    sensor::SensorData
};

#[derive(Debug, Clone)]
pub struct SMAGDS {
    pub(crate) magds: Arc<RwLock<MAGDS>>,
    pub(crate) data: Vec<DataPoint2D>
}

impl SMAGDS {
    pub fn new<X: SensorData + DataDeductor, Y: SensorData + DataDeductor>(
        data: &[(X, Y)]
    ) -> anyhow::Result<Self> where DataTypeValue: From<X> + From<Y> {
        if data.len() < 2 { anyhow::bail!("data length must be >= 2") }

        let mut converted_data: Vec<DataPoint2D> = data.into_iter()
            .map(|(x, y)| {
                DataPoint2D {
                    x: (*dyn_clone::clone_box(x)).into(), 
                    y: (*dyn_clone::clone_box(y)).into()
                }
            }).collect();
        converted_data.sort_unstable_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

        Ok(
            Self {
                magds: Arc::new(RwLock::new(MAGDS::new())),
                data: converted_data
            }
        )
    }

    pub fn add<X: SensorData + DataDeductor, Y: SensorData + DataDeductor>(
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

    fn prepare_sensory_fields<X: SensorData + DataDeductor, Y: SensorData + DataDeductor>(
        &mut self, data: &[(X, Y)]
    ) where DataTypeValue: From<X> + From<Y> {
        let mut magds = self.magds.write().unwrap();
        let x_data_type = data[0].0.data_type();
        let y_data_type = data[0].1.data_type();
        
        magds.create_sensor("x", x_data_type);
        magds.create_sensor("y", y_data_type);
        magds.create_sensor("x_interval", y_data_type);
        magds.create_sensor("y_interval", y_data_type);
        magds.create_sensor("y_entry", y_data_type);
        magds.create_sensor("same_absolute_patterns_interval", y_data_type);
        magds.create_sensor("same_relative_patterns_interval", y_data_type);
        magds.create_sensor("different_absolute_patterns_interval", y_data_type);
        magds.create_sensor("different_relative_patterns_interval", y_data_type);
    }
}

mod tests {
    use super::SMAGDS;

    #[test]
    fn new() {
        let smagds = SMAGDS::new(&[(1, 1), (2, 3), (3, 5)]).unwrap();
        assert_eq!(smagds.data.len(), 3);
        println!("{:?}", smagds);

        let smagds = SMAGDS::new(&[(1, 1.0), (2, 3.0), (3, 5.0)]).unwrap();
        assert_eq!(smagds.data.len(), 3);
        println!("{:?}", smagds);

        let smagds = SMAGDS::new(
            &[(1, "1.0".to_owned()), (2, "3.0".to_owned()), (3, "5.0".to_owned())]
        ).unwrap();
        assert_eq!(smagds.data.len(), 3);
        println!("{:?}", smagds);
    }

    #[test]
    fn add() {
        let smagds = SMAGDS::new(&Vec::<(usize, f64)>::new());
        assert!(smagds.is_err());

        let smagds = SMAGDS::new(&vec![(1usize, 1.0)]);
        assert!(smagds.is_err());

        let mut smagds = SMAGDS::new(&vec![(1, 1.0), (2, 2.0)]).unwrap();
        
        smagds.add(&[(3, 3.0), (4, 4.0), (5, 5.0)]).unwrap();
        assert_eq!(smagds.data.len(), 5);
        println!("{:?}", smagds);

        assert!(
            smagds.add(
                &[(1, "1.0".to_owned()), (2, "3.0".to_owned()), (3, "5.0".to_owned())]
            ).is_err()
        );

        assert!(
            smagds.add(
                &[(1.0, 1.0), (2.0, 3.0), (3.0, 5.0)]
            ).is_err()
        );
    }
}