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
    pub max_pattern_length: Option<f64>,
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
    absolute_pattern_level: HashMap<usize, u32>,
    relative_pattern_level: HashMap<usize, u32>,
    same_absolute_patterns_interval: u32,
    same_relative_patterns_interval: u32,
    different_absolute_patterns_interval: u32,
    different_relative_patterns_interval: u32
}

#[derive(Debug, Clone)]
pub struct SMAGDS {
    pub magds: MAGDS,
    pub data: Vec<DataPoint2D>,
    pub(crate) sensors: SMAGDSSensors,
    pub(crate) neuron_groups: SMAGDSNeuronGropuIds,
    pub(crate) absolute_pattern_neurons: HashMap<usize, Vec<Arc<RwLock<dyn NeuronAsync>>>>,
    pub(crate) relative_pattern_neurons: HashMap<usize, Vec<Arc<RwLock<dyn NeuronAsync>>>>,
    pub params: SMAGDSParams
}

impl SMAGDS {
    pub const EPSILON: f32 = 0.00001;

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
        let pattern_level_neuron_counter = Self::prepare_pattern_level_neurons(
            params.max_pattern_level
        );
        let absolute_pattern_neurons = Self::prepare_pattern_level_neurons(
            params.max_pattern_level
        );
        let mut smagds = Self {
            sensors: Self::prepare_sensory_fields(&mut magds, &converted_data),
            neuron_groups: Self::prepare_neuron_gropus(&mut magds, params.max_pattern_level),
            params,
            magds,
            data: converted_data,
            relative_pattern_neurons: absolute_pattern_neurons.clone(),
            absolute_pattern_neurons
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

        let (x_interval, _) = magds.create_sensor("x interval", DataType::F64);
        let (y, _) = magds.create_sensor("y", y_data_type);
        let (y_interval, _) = magds.create_sensor("y interval", DataType::F64);
        let (y_entry, _) = magds.create_sensor("y entry", y_data_type);
        let (same_absolute_patterns_interval, _) = 
            magds.create_sensor("same absolute patterns interval", x_data_type);
        let (same_relative_patterns_interval, _) = 
            magds.create_sensor("same relative patterns interval", y_data_type);
        let (different_absolute_patterns_interval, _) = 
            magds.create_sensor("different absolute patterns interval", x_data_type);
        let (different_relative_patterns_interval, _) = 
            magds.create_sensor("different relative patterns interval", y_data_type);

        SMAGDSSensors {
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
            absolute_pattern_level: HashMap::new(),
            relative_pattern_level: HashMap::new(),
            same_absolute_patterns_interval: 1,
            same_relative_patterns_interval: 2,
            different_absolute_patterns_interval: 3,
            different_relative_patterns_interval: 4
        };

        for lvl in 1..=max_pattern_level {
            let absolute_id = 4 + lvl as u32;
            let relative_id = 4 + max_pattern_level as u32 + lvl as u32;
            ids.absolute_pattern_level.insert(lvl, absolute_id);
            ids.relative_pattern_level.insert(lvl, relative_id);
            magds.add_neuron_group(&format!("absolute pattern level {lvl}"), Some(absolute_id));
            magds.add_neuron_group(&format!("relative pattern level {lvl}"), Some(relative_id));
        }
        magds.add_neuron_group(
            "same absolute patterns interval", Some(ids.same_absolute_patterns_interval)
        );
        magds.add_neuron_group(
            "same relative patterns interval", Some(ids.same_relative_patterns_interval)
        );
        magds.add_neuron_group(
            "different absolute patterns interval", Some(ids.different_absolute_patterns_interval)
        );
        magds.add_neuron_group(
            "different relative patterns interval", Some(ids.different_relative_patterns_interval)
        );

        ids
    }

    fn create_neurons(&mut self) {
        self.insert_data_to_sensors();

        let mut magds = &mut self.magds;
        
        let data = &self.data;
        let data_len = data.len();
        let SMAGDSParams { max_pattern_length, max_pattern_level } = &self.params;

        let x_interval = self.sensors.x_interval.write().unwrap();
        let y = self.sensors.y.write().unwrap();
        let y_interval = self.sensors.y_interval.write().unwrap();
        let y_entry = self.sensors.y_entry.write().unwrap();
        let sapi = self.sensors.same_absolute_patterns_interval.write().unwrap();
        let srpi = self.sensors.same_relative_patterns_interval.write().unwrap();
        let dapi = self.sensors.different_absolute_patterns_interval.write().unwrap();
        let drpi = self.sensors.different_relative_patterns_interval.write().unwrap();

        let absolute_pattern_id = &mut self.neuron_groups.absolute_pattern_level;
        let relative_pattern_id = &mut self.neuron_groups.relative_pattern_level;
        let mut absolute_pattern_neurons = &mut self.absolute_pattern_neurons;
        let mut relative_pattern_neurons = &mut self.relative_pattern_neurons;
        
        let patterns: HashMap<usize, Arc<RwLock<dyn NeuronAsync>>> = HashMap::new();
        for i in 1..data.len() {
            let first_point = &data[i - 1];
            let second_point = &data[i];
            let points_x_interval = second_point.x.distance(&first_point.x);
            let points_y_interval = second_point.y.distance(&first_point.y);
            
            let x_interval_sn = x_interval.search(&points_x_interval.into()).unwrap();

            let y1_sn = y.search(&first_point.y).unwrap();
            let y2_sn = y.search(&second_point.y).unwrap();
            let y_interval_sn = y_interval.search(&points_y_interval.into()).unwrap();
            let y_entry_sn = y_entry.search(&first_point.y).unwrap();

            let (
                absolute_pattern_lvl1_neuron, relative_pattern_lvl1_neuron
            ) = Self::add_pattern_ra_neurons(
                magds,
                1, 
                &mut absolute_pattern_neurons, 
                &mut relative_pattern_neurons, 
                &self.neuron_groups
            );

            for sn in [&x_interval_sn, &y1_sn, &y2_sn, &y_entry_sn] {
                sn.write().unwrap().connect_bilateral(
                    absolute_pattern_lvl1_neuron.clone(), false, ConnectionKind::Defining
                ).unwrap();
            }

            for sn in [&x_interval_sn, &y_interval_sn, &y_entry_sn] {
                sn.write().unwrap().connect_bilateral(
                    relative_pattern_lvl1_neuron.clone(), false, ConnectionKind::Defining
                ).unwrap();
            }

            let mut current_pattern_len = points_x_interval;
            let mut current_absolute_pattern = absolute_pattern_lvl1_neuron;
            let mut current_relative_pattern = relative_pattern_lvl1_neuron;
            for j in (i + 1)..usize::min(i + max_pattern_level, data.len()) {
                let first_point = &data[j - 1];
                let second_point = &data[j];
                let points_x_interval = second_point.x.distance(&first_point.x);
                let points_y_interval = second_point.y.distance(&first_point.y);

                if let Some(max_pattern_length) = max_pattern_length {
                    current_pattern_len += points_x_interval;
                    if current_pattern_len > *max_pattern_length { break }
                }
                
                let y2_sn = y.search(&second_point.y).unwrap();
                let x_interval_sn = x_interval.search(&points_x_interval.into()).unwrap();
                let y_interval_sn = y_interval.search(&points_y_interval.into()).unwrap();
                
                let level = j - i;
                let (
                    absolute_pattern_neuron, relative_pattern_neuron
                ) = Self::add_pattern_ra_neurons(
                    magds,
                    level, 
                    &mut absolute_pattern_neurons, 
                    &mut relative_pattern_neurons, 
                    &self.neuron_groups
                );

                
                for sn in [&x_interval_sn, &y1_sn, &y2_sn, &y_entry_sn] {
                    sn.write().unwrap().connect_bilateral(
                        absolute_pattern_neuron.clone(), false, ConnectionKind::Defining
                    ).unwrap();
                }

                for sn in [&x_interval_sn, &y_interval_sn, &y_entry_sn] {
                    sn.write().unwrap().connect_bilateral(
                        relative_pattern_neuron.clone(), false, ConnectionKind::Defining
                    ).unwrap();
                }
            }
        }
    }

    fn insert_data_to_sensors(&mut self) {
        let data = &self.data;
        let mut x_interval = self.sensors.x_interval.write().unwrap();
        let mut y = self.sensors.y.write().unwrap();
        let mut y_interval = self.sensors.y_interval.write().unwrap();
        let mut y_entry = self.sensors.y_entry.write().unwrap();

        for i in 1..data.len() {
            let first_point = &data[i - 1];
            let second_point = &data[i];
            let points_x_interval = second_point.x.distance(&first_point.x);
            let points_y_interval = second_point.y.distance(&first_point.y);
            x_interval.insert(&points_x_interval.into());

            y.insert(&first_point.y);
            y.insert(&second_point.y);
            y_interval.insert(&points_y_interval.into());
            y_entry.insert(&first_point.y);
        }
    }

    fn add_pattern_neuron(magds: &mut MAGDS, neuron_id: NeuronID) -> Arc<RwLock<SimpleNeuron>> {
        let neuron = SimpleNeuron::new_custom(neuron_id, Arc::new(ConstantOneWeightAsync));
        magds.add_neuron(neuron.clone());
        neuron
    }

    fn add_pattern_ra_neurons(
        magds: &mut MAGDS,
        level: usize, 
        absolute_pattern_neurons: &mut HashMap<usize, Vec<Arc<RwLock<dyn NeuronAsync>>>>,
        relative_pattern_neurons: &mut HashMap<usize, Vec<Arc<RwLock<dyn NeuronAsync>>>>,
        group_ids: &SMAGDSNeuronGropuIds
    ) -> (Arc<RwLock<SimpleNeuron>>, Arc<RwLock<SimpleNeuron>>) {
        let absolute_neuron_vec = absolute_pattern_neurons.get_mut(&level).unwrap();
        let absolute_neuron = Self::add_pattern_neuron(magds, NeuronID {
            id: absolute_neuron_vec.len() as u32, parent_id: group_ids.absolute_pattern_level[&level] 
        });
        absolute_neuron_vec.push(absolute_neuron.clone());

        let relative_neuron_vec = relative_pattern_neurons.get_mut(&level).unwrap();
        let relative_neuron = Self::add_pattern_neuron(magds, NeuronID { 
            id: relative_neuron_vec.len() as u32, parent_id: group_ids.relative_pattern_level[&level] 
        });
        relative_neuron_vec.push(relative_neuron.clone());

        (absolute_neuron, relative_neuron)
    }

    fn prepare_pattern_level_neurons(
        max_pattern_level: usize
    ) -> HashMap<usize, Vec<Arc<RwLock<dyn NeuronAsync>>>> {
        let mut result = HashMap::new();
        for lvl in 1..=max_pattern_level {
            result.insert(lvl, vec![]);
        }
        result
    }

    // fn level_1_pattern_exists() -> bool {

    // }
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