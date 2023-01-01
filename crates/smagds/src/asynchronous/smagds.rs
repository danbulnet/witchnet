use std::{
    sync::{ Arc, RwLock },
    collections::HashMap
};

use magds::{
    asynchronous::{
        magds::MAGDS, 
        sensor::SensorConatiner
    },
    neuron::simple_neuron_async::SimpleNeuron
};

use witchnet_common::{
    data::{ DataTypeValue, DataPoint2D, DataType, DataDeductor },
    sensor::{ SensorData, SensorAsync }, 
    neuron::{ NeuronAsync, NeuronID }, 
    connection::{
        ConnectionKind,
        collective::defining::ConstantOneWeightAsync
    }
};

#[derive(Debug, Clone)]
pub struct SMAGDSParams {
    pub max_pattern_length: Option<DataTypeValue>,
    pub max_pattern_level: usize,
}

impl Default for SMAGDSParams {
    fn default() -> Self {
        Self {
            max_pattern_length: None,
            max_pattern_level: 10
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SMAGDSSensors {
    x: Arc<RwLock<SensorConatiner>>,
    x_interval: Arc<RwLock<SensorConatiner>>,
    y: Arc<RwLock<SensorConatiner>>,
    y_interval: Arc<RwLock<SensorConatiner>>,
    y_entry: Arc<RwLock<SensorConatiner>>,
    same_absolute_patterns_interval: Arc<RwLock<SensorConatiner>>,
    same_relative_patterns_interval: Arc<RwLock<SensorConatiner>>,
    different_absolute_patterns_interval: Arc<RwLock<SensorConatiner>>,
    different_relative_patterns_interval: Arc<RwLock<SensorConatiner>>
}

#[derive(Debug, Clone)]
pub(crate) struct SMAGDSNeuronGropuIds {
    pattern_levels: HashMap<usize, u32>,
    same_absolute_patterns_interval: u32,
    same_relative_patterns_interval: u32,
    different_absolute_patterns_interval: u32,
    different_relative_patterns_interval: u32
}

#[derive(Debug, Clone)]
pub struct SMAGDS {
    pub(crate) magds: MAGDS,
    pub(crate) data: Vec<DataPoint2D>,
    pub(crate) sensors: SMAGDSSensors,
    pub(crate) neuron_group_ids: SMAGDSNeuronGropuIds,
    pub params: SMAGDSParams
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
        converted_data.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

        let params = SMAGDSParams::default();
        let mut magds = MAGDS::new();
        let mut smagds = Self {
            sensors: Self::prepare_sensory_fields(&mut magds, &converted_data),
            neuron_group_ids: Self::prepare_neuron_gropus(&mut magds, params.max_pattern_level),
            params,
            magds: MAGDS::new(),
            data: converted_data,
        };

        smagds.create_neurons();

        Ok(smagds)
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
                self.data.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
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

    fn prepare_sensory_fields(magds: &mut MAGDS, data: &[DataPoint2D]) -> SMAGDSSensors {
        let x_data_type: DataType = (&data[0].x).into();
        let y_data_type: DataType = (&data[0].y).into();

        let (x, _) = magds.create_sensor("x", x_data_type);
        let (x_interval, _) = magds.create_sensor("x_interval", DataType::F64);
        let (y, _) = magds.create_sensor("y", y_data_type);
        let (y_interval, _) = magds.create_sensor("y_interval", DataType::F64);
        let (y_entry, _) = magds.create_sensor("y_entry", y_data_type);
        let (same_absolute_patterns_interval, _) = 
            magds.create_sensor("same_absolute_patterns_interval", DataType::F64);
        let (same_relative_patterns_interval, _) = 
            magds.create_sensor("same_relative_patterns_interval", DataType::F64);
        let (different_absolute_patterns_interval, _) = 
            magds.create_sensor("different_absolute_patterns_interval", DataType::F64);
        let (different_relative_patterns_interval, _) = 
            magds.create_sensor("different_relative_patterns_interval", DataType::F64);

        SMAGDSSensors {
            x,
            x_interval,
            y,
            y_interval,
            y_entry,
            same_absolute_patterns_interval,
            same_relative_patterns_interval,
            different_absolute_patterns_interval,
            different_relative_patterns_interval
        }
    }

    fn prepare_neuron_gropus(
        magds: &mut MAGDS, max_pattern_level: usize
    ) -> SMAGDSNeuronGropuIds {
        let mut ids = SMAGDSNeuronGropuIds {
            pattern_levels: HashMap::new(),
            same_absolute_patterns_interval: 1,
            same_relative_patterns_interval: 2,
            different_absolute_patterns_interval: 3,
            different_relative_patterns_interval: 4
        };

        let mut next_pattern_lvl_id: u32 = 1000;
        let mut next_level: usize = 1;
        for lvl in 1..=max_pattern_level {
            ids.pattern_levels.insert(next_level, next_pattern_lvl_id);
            magds.add_neuron_group(&format!("patterns_level_{next_level}"), next_pattern_lvl_id);

            next_pattern_lvl_id += 1;
            next_level += 1;
        }
        magds.add_neuron_group(
            "same_absolute_patterns_interval", ids.same_absolute_patterns_interval
        );
        magds.add_neuron_group(
            "same_relative_patterns_interval", ids.same_relative_patterns_interval
        );
        magds.add_neuron_group(
            "different_absolute_patterns_interval", ids.different_absolute_patterns_interval
        );
        magds.add_neuron_group(
            "different_relative_patterns_interval", ids.different_relative_patterns_interval
        );

        ids
    }

    fn create_neurons(&mut self) {
        let magds = &mut self.magds;
        let data = &self.data;
        let SMAGDSParams { max_pattern_length, max_pattern_level } = &self.params;

        let mut x = self.sensors.x.write().unwrap();
        let mut x_interval = self.sensors.x_interval.write().unwrap();
        let mut y = self.sensors.y.write().unwrap();
        let mut y_interval = self.sensors.y_interval.write().unwrap();
        let mut y_entry = self.sensors.y_entry.write().unwrap();
        let mut sapi = self.sensors.same_absolute_patterns_interval.write().unwrap();
        let mut srpi = self.sensors.same_relative_patterns_interval.write().unwrap();
        let mut dapi = self.sensors.different_absolute_patterns_interval.write().unwrap();
        let mut drpi = self.sensors.different_relative_patterns_interval.write().unwrap();

        let patterns_id = &self.neuron_group_ids.pattern_levels;

        let mut patterns: Vec<Vec<Arc<RwLock<dyn NeuronAsync>>>> = Vec::new();
        
        for i in 1..data.len() {
            let first_point = &data[i - 1];
            let second_point = &data[i];
            let points_x_interval = second_point.x.distance(&first_point.x);
            let points_y_interval = second_point.y.distance(&first_point.y);

            let x1_sn = x.insert(&first_point.x);
            let x2_sn = x.insert(&second_point.x);
            let x_interval_sn = x_interval.insert(&points_x_interval.into());

            let y1_sn = y.insert(&first_point.y);
            let y2_sn = y.insert(&second_point.y);
            let y_interval_sn = y_interval.insert(&points_y_interval.into());
            let y_entry_sn = y_entry.insert(&first_point.y);

            let pattern_lvl1_neuron = SimpleNeuron::new_custom(
                NeuronID{ id: i as u32, parent_id: patterns_id[&1] },
                Arc::new(ConstantOneWeightAsync)
            );
            magds.add_neuron(pattern_lvl1_neuron.clone());

            for sn in [x1_sn, x2_sn, x_interval_sn, y1_sn, y2_sn, y_interval_sn, y_entry_sn] {
                sn.write().unwrap().connect_bilateral(
                    pattern_lvl1_neuron.clone(), false, ConnectionKind::Defining
                ).unwrap();
            }

            if i >= 2 {
                for j in (0..(i - 2)).rev() {
    
                }
            }
        }
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