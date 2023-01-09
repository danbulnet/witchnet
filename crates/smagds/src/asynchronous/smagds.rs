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
    pub const EPSILON: f32 = 0.00005;
    pub const SIGNAL_SIMILARITY_THRESHOLD: f32 = 0.95;

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
        let SMAGDSParams { max_pattern_length, max_pattern_level } = &self.params;

        let mut x_interval = self.sensors.x_interval.write().unwrap();
        let mut y = self.sensors.y.write().unwrap();
        let mut y_interval = self.sensors.y_interval.write().unwrap();
        let mut y_entry = self.sensors.y_entry.write().unwrap();
        let mut sapi = self.sensors.same_absolute_patterns_interval.write().unwrap();
        let mut srpi = self.sensors.same_relative_patterns_interval.write().unwrap();
        let mut dapi = self.sensors.different_absolute_patterns_interval.write().unwrap();
        let mut drpi = self.sensors.different_relative_patterns_interval.write().unwrap();

        let absolute_pattern_id = &mut self.neuron_groups.absolute_pattern_level;
        let relative_pattern_id = &mut self.neuron_groups.relative_pattern_level;
        let mut absolute_pattern_neurons = &mut self.absolute_pattern_neurons;
        let mut relative_pattern_neurons = &mut self.relative_pattern_neurons;
        
        let patterns: HashMap<usize, Arc<RwLock<dyn NeuronAsync>>> = HashMap::new();
        let th = Self::SIGNAL_SIMILARITY_THRESHOLD;
        for i in 1..data.len() {
            let first_point = &data[i - 1]; let y1 = &first_point.y;
            let second_point = &data[i]; let y2 = &second_point.y;
            let x_diff = second_point.x.distance(&first_point.x);
            let y_diff = second_point.y.distance(&first_point.y);
            
            let (x_diff_sn, _) = x_interval.fuzzy_search(&x_diff.into(), th, false).unwrap();
            let (y1_sn, _) = y.fuzzy_search(&y1.clone().into(), th, false).unwrap();
            let (y2_sn, _) = y.fuzzy_search(&y2.clone().into(), th, false).unwrap();
            let (y_diff_sn, _) = y_interval.fuzzy_search(&y_diff.into(), th, false).unwrap();
            let (y_entry_sn, _) = y_entry.fuzzy_search(&y1.clone().into(), th, false).unwrap();

            let absolute_pattern_lvl1_neuron = Self::add_absolute_pattern_neuron(
                magds, 1,
                absolute_pattern_neurons, &self.neuron_groups,
                &y1_sn, &x_diff_sn, &y2_sn, &y_entry_sn
            );
            // let relative_pattern_lvl1_neuron = Self::add_relative_pattern_neuron(
            //     magds, 1,
            //     absolute_pattern_neurons, &self.neuron_groups,
            //     None, &x_diff_sn, &y_diff_sn, &y_entry_sn
            // );

            // let mut current_pattern_len = x_diff;
            // let mut current_absolute_pattern = absolute_pattern_lvl1_neuron;
            // let mut current_relative_pattern = relative_pattern_lvl1_neuron;
            // for j in (i + 1)..usize::min(i + max_pattern_level, data.len()) {
            //     let first_point = &data[i - 1];
            //     let second_point = &data[i]; let y2 = &second_point.y;
            //     let x_diff = second_point.x.distance(&first_point.x);
            //     let y_diff = second_point.y.distance(&first_point.y);

            //     if let Some(max_pattern_length) = max_pattern_length {
            //         current_pattern_len += x_diff;
            //         if current_pattern_len > *max_pattern_length { break }
            //     }

            //     let (x_diff_sn, _) = x_interval.fuzzy_search(&x_diff.into(), th, false).unwrap();
            //     let (y2_sn, _) = y.fuzzy_search(&y2.clone().into(), th, false).unwrap();
            //     let (y_diff_sn, _) = y_interval.fuzzy_search(&y_diff.into(), th, false).unwrap();
                
            //     let level = j - i;
            //     current_absolute_pattern = Self::add_absolute_pattern_neuron(
            //         magds, level,
            //         absolute_pattern_neurons, &self.neuron_groups,
            //         &current_absolute_pattern, &x_diff_sn, &y2_sn, &y_entry_sn
            //     );
            //     current_relative_pattern = Self::add_relative_pattern_neuron(
            //         magds, level,
            //         absolute_pattern_neurons, &self.neuron_groups,
            //         Some(&current_relative_pattern), &x_diff_sn, &y_diff_sn, &y_entry_sn
            //     );
            // }
        }
    }

    fn insert_data_to_sensors(&mut self) {
        let data = &self.data;
        if data.len() < 2 { return }

        let mut x_interval = self.sensors.x_interval.write().unwrap();
        let mut y = self.sensors.y.write().unwrap();
        let mut y_interval = self.sensors.y_interval.write().unwrap();
        let mut y_entry = self.sensors.y_entry.write().unwrap();

        let mut x_interval_min = data[1].x.distance(&data[0].x);
        let mut x_interval_max = data[1].x.distance(&data[0].x);
        let mut y_min = data[0].y.clone();
        let mut y_max = data[0].y.clone();
        let mut y_interval_min = data[1].y.distance(&data[0].y);
        let mut y_interval_max = data[1].y.distance(&data[0].y);
        let mut y_entry_min = data[0].y.clone();
        let mut y_entry_max = data[0].y.clone();

        for i in 1..data.len() {
            let first_point = &data[i - 1];
            let second_point = &data[i];
            let points_x_interval = second_point.x.distance(&first_point.x);
            let points_y_interval = second_point.y.distance(&first_point.y);

            if second_point.y < y_min { 
                y_min = second_point.y.clone(); 
                y_entry_min = second_point.y.clone(); 
            }
            if second_point.y > y_max { 
                y_max = second_point.y.clone(); 
                y_entry_max = second_point.y.clone() 
            }
            if points_x_interval < x_interval_min { x_interval_min = points_x_interval }
            if points_x_interval > x_interval_max { x_interval_max = points_x_interval }
            if points_y_interval < y_interval_min { y_interval_min = points_y_interval }
            if points_y_interval > y_interval_max { y_interval_max = points_y_interval }
        }

        x_interval.insert(&x_interval_min.into()); x_interval.insert(&x_interval_max.into());
        y.insert(&y_min); y.insert(&y_max);
        y_interval.insert(&y_interval_min.into()); y_interval.insert(&y_interval_max.into());
        y_entry.insert(&y_entry_min); y_entry.insert(&y_entry_max);

        for i in 1..data.len() {
            let first_point = &data[i - 1];
            let second_point = &data[i];
            let x_diff = second_point.x.distance(&first_point.x);
            let y_diff = second_point.y.distance(&first_point.y);

            Self::fuzzy_search_insert(&mut x_interval, x_diff.into());

            Self::fuzzy_search_insert(&mut y, first_point.y.clone().into());
            Self::fuzzy_search_insert(&mut y, second_point.y.clone().into());
            Self::fuzzy_search_insert(&mut y_interval, y_diff.into());
            Self::fuzzy_search_insert(&mut y_entry, first_point.y.clone().into());

            y.insert(&first_point.y);
            y.insert(&second_point.y);
            y_interval.insert(&y_diff.into());
            y_entry.insert(&first_point.y);
        }
    }

    fn add_pattern_neuron(magds: &mut MAGDS, neuron_id: NeuronID) -> Arc<RwLock<SimpleNeuron>> {
        let neuron = SimpleNeuron::new_custom(neuron_id, Arc::new(ConstantOneWeightAsync));
        magds.add_neuron(neuron.clone());
        neuron
    }

    fn add_absolute_pattern_neuron(
        magds: &mut MAGDS,
        level: usize, 
        absolute_pattern_neurons: &mut HashMap<usize, Vec<Arc<RwLock<dyn NeuronAsync>>>>,
        group_ids: &SMAGDSNeuronGropuIds,
        base_pattern_sn: &Arc<RwLock<dyn NeuronAsync>>,
        x_diff_sn: &Arc<RwLock<dyn NeuronAsync>>,
        y2_sn: &Arc<RwLock<dyn NeuronAsync>>,
        y_entry_sn: &Arc<RwLock<dyn NeuronAsync>>
    ) -> Arc<RwLock<dyn NeuronAsync>> {
        base_pattern_sn.write().unwrap().activate(1.0, false, true);
        y2_sn.write().unwrap().activate(1.0, false, true);
        x_diff_sn.write().unwrap().activate(1.0, false, true);

        let mut activated_neurons: Vec<_> = (&absolute_pattern_neurons[&level]).into_iter()
            .filter(|neuron| neuron.read().unwrap().activation() >= 3.0 - Self::EPSILON)
            .cloned().collect();
        activated_neurons.sort_by(
            |a, b| a.read().unwrap().activation().partial_cmp(
                &b.read().unwrap().activation()
            ).unwrap()
        );
        let absolute_pattern_lvl1_neuron = if activated_neurons.is_empty() {
            let absolute_neuron_vec = absolute_pattern_neurons.get_mut(&level).unwrap();
            let absolute_neuron = Self::add_pattern_neuron(magds, NeuronID {
                id: absolute_neuron_vec.len() as u32, 
                parent_id: group_ids.absolute_pattern_level[&level] 
            });
            absolute_neuron_vec.push(absolute_neuron.clone());

            for sn in [&x_diff_sn, &base_pattern_sn, &y2_sn, &y_entry_sn] {
                sn.write().unwrap().connect_bilateral(
                    absolute_neuron.clone(), false, ConnectionKind::Defining
                ).unwrap();
            }

            absolute_neuron
        } else {
            if activated_neurons.len() > 1 { 
                log::warn!("activated_absolute_lvl{level}_neurons len > 1"); 
            }
            let absolute_neuron = activated_neurons[0].clone();
            absolute_neuron.write().unwrap().increment_counter();
            absolute_neuron
        };

        base_pattern_sn.write().unwrap().deactivate(false, true);
        y2_sn.write().unwrap().deactivate(false, true);
        x_diff_sn.write().unwrap().deactivate(false, true);
        
        absolute_pattern_lvl1_neuron
    }

    fn add_relative_pattern_neuron(
        magds: &mut MAGDS,
        level: usize, 
        relative_pattern_neurons: &mut HashMap<usize, Vec<Arc<RwLock<dyn NeuronAsync>>>>,
        group_ids: &SMAGDSNeuronGropuIds,
        base_pattern_sn: Option<&Arc<RwLock<dyn NeuronAsync>>>,
        x_diff_sn: &Arc<RwLock<dyn NeuronAsync>>,
        y_diff_sn: &Arc<RwLock<dyn NeuronAsync>>,
        y_entry_sn: &Arc<RwLock<dyn NeuronAsync>>
    ) -> Arc<RwLock<dyn NeuronAsync>> {
        x_diff_sn.write().unwrap().activate(1.0, false, true);
        y_diff_sn.write().unwrap().activate(1.0, false, true);
        let mut threshold = 2.0;
        if level > 1 { 
            base_pattern_sn.unwrap().write().unwrap().activate(1.0, false, true);
            threshold = 3.0;
        }

        let mut activated_neurons: Vec<_> = (&relative_pattern_neurons[&level]).into_iter()
            .filter(|neuron| neuron.read().unwrap().activation() >= threshold - Self::EPSILON)
            .cloned().collect();
        activated_neurons.sort_by(
            |a, b| a.read().unwrap().activation().partial_cmp(
                &b.read().unwrap().activation()
            ).unwrap()
        );
        let relative_pattern_lvl1_neuron = if activated_neurons.is_empty() {
            let relative_neuron_vec = relative_pattern_neurons.get_mut(&level).unwrap();
            let relative_neuron = Self::add_pattern_neuron(magds, NeuronID {
                id: relative_neuron_vec.len() as u32, 
                parent_id: group_ids.relative_pattern_level[&level] 
            });
            relative_neuron_vec.push(relative_neuron.clone());

            let mut to_connect = vec![&x_diff_sn, &y_diff_sn, &y_entry_sn];
            if level > 1 { to_connect.push(base_pattern_sn.as_ref().unwrap()); }
            for sn in [&x_diff_sn, &y_diff_sn, &y_entry_sn] {
                sn.write().unwrap().connect_bilateral(
                    relative_neuron.clone(), false, ConnectionKind::Defining
                ).unwrap();
            }

            relative_neuron
        } else {
            if activated_neurons.len() > 1 { 
                log::warn!("activated_relative_lvl{level}_neurons len > 1"); 
            }
            let relative_neuron = activated_neurons[0].clone();
            relative_neuron.write().unwrap().increment_counter();
            relative_neuron
        };

        x_diff_sn.write().unwrap().deactivate(false, true);
        y_diff_sn.write().unwrap().deactivate(false, true);
        if level > 1 { base_pattern_sn.unwrap().write().unwrap().deactivate(false, true); }
        
        relative_pattern_lvl1_neuron
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

    fn fuzzy_search_insert(
        sensor: &mut SensorConatiner, data: DataTypeValue
    ) -> Arc<RwLock<dyn NeuronAsync>> {
        if let Some((sn, _)) = sensor.fuzzy_search(
            &data, Self::SIGNAL_SIMILARITY_THRESHOLD, false
        ) {
            sn.write().unwrap().increment_counter();
            sn
        } else { sensor.insert(&data.into()) }
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