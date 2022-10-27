use std::{
    sync::Arc,
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
    fmt::{ Display, Formatter, Result as FmtResult }
};

use anyhow::Result;

use asa_graphs::neural::graph::ASAGraph;
use witchnet_common::{
    neuron::{ Neuron, NeuronID },
    data::{ DataType, DataTypeValue, DataCategory },
    sensor::Sensor
};

use crate::neuron::simple_neuron::SimpleNeuron;

use super::sensor::SensorConatiner;

pub struct MAGDS {
    pub(crate) sensors: HashMap<u32, Rc<RefCell<SensorConatiner>>>,
    pub(crate) sensor_names: HashMap<u32, Rc<str>>,
    pub(crate) sensor_ids: HashMap<Rc<str>, Vec<u32>>,
    pub(crate) neurons: Vec<Rc<RefCell<dyn Neuron>>>,
    pub(crate) neuron_group_names: HashMap<u32, Rc<str>>,
    pub(crate) neuron_group_ids: HashMap<Rc<str>, Vec<u32>>
}

impl MAGDS {
    pub fn new() -> MAGDS {
        MAGDS { 
            sensors: HashMap::new(),
            sensor_names: HashMap::new(),
            sensor_ids: HashMap::new(),
            neurons: Vec::new(),
            neuron_group_names: HashMap::new(),
            neuron_group_ids: HashMap::new()
        }
    }
    
    pub fn new_rc() -> Rc<RefCell<MAGDS>> {
        Rc::new(RefCell::new(
            MAGDS { 
                sensors: HashMap::new(),
                sensor_names: HashMap::new(),
                sensor_ids: HashMap::new(),
                neurons: Vec::new(),
                neuron_group_names: HashMap::new(),
                neuron_group_ids: HashMap::new()
            }
        ))
    }

    pub fn create_sensor(
        &mut self, name: &str, data_type: DataType
    ) -> (Rc<RefCell<SensorConatiner>>, u32) {
        let name_rc: Rc<str> = Rc::from(name);
        let new_id: u32 = *self.sensors.keys().max().unwrap_or(&0) + 1;

        let sensor = match data_type {
            DataType::Bool => SensorConatiner::Bool(ASAGraph::<bool>::new_box(new_id)),
            DataType::U8 => SensorConatiner::U8(ASAGraph::<u8>::new_box(new_id)),
            DataType::U16 => SensorConatiner::U16(ASAGraph::<u16>::new_box(new_id)),
            DataType::U32 => SensorConatiner::U32(ASAGraph::<u32>::new_box(new_id)),
            DataType::U64 => SensorConatiner::U64(ASAGraph::<u64>::new_box(new_id)),
            DataType::U128 => SensorConatiner::U128(ASAGraph::<u128>::new_box(new_id)),
            DataType::USize => SensorConatiner::USize(ASAGraph::<usize>::new_box(new_id)),
            DataType::I8 => SensorConatiner::I8(ASAGraph::<i8>::new_box(new_id)),
            DataType::I16 => SensorConatiner::I16(ASAGraph::<i16>::new_box(new_id)),
            DataType::I32 => SensorConatiner::I32(ASAGraph::<i32>::new_box(new_id)),
            DataType::I64 => SensorConatiner::I64(ASAGraph::<i64>::new_box(new_id)),
            DataType::I128 => SensorConatiner::I128(ASAGraph::<i128>::new_box(new_id)),
            DataType::ISize => SensorConatiner::ISize(ASAGraph::<isize>::new_box(new_id)),
            DataType::F32 => SensorConatiner::F32(ASAGraph::<f32>::new_box(new_id)),
            DataType::F64 => SensorConatiner::F64(ASAGraph::<f64>::new_box(new_id)),
            DataType::ArcStr => SensorConatiner::ArcStr(ASAGraph::<Arc<str>>::new_box(new_id)),
            DataType::String => SensorConatiner::String(ASAGraph::<String>::new_box(new_id)),
            DataType::Unknown => panic!("unknown data type sensor is not allowed")
        };

        let sensor_ptr = Rc::new(RefCell::new(sensor));
        self.sensors.insert(new_id, sensor_ptr.clone());

        self.sensor_names.insert(new_id, name_rc.clone());
        match self.sensor_ids.get_mut(name) {
            Some(v) => v.push(new_id),
            None => { self.sensor_ids.insert(name_rc, vec![new_id]); }
        }

        (sensor_ptr, new_id)
    }

    pub fn add_sensor(
        &mut self, name: &str, sensor: Rc<RefCell<SensorConatiner>>
    ) -> (Rc<RefCell<SensorConatiner>>, u32) {
        let new_id: u32 = *self.sensors.keys().max().unwrap_or(&0) + 1;
        let name_rc: Rc<str> = Rc::from(name);

        self.sensors.insert(new_id, sensor.clone());

        self.sensor_names.insert(new_id, name_rc.clone());
        match self.sensor_ids.get_mut(name) {
            Some(v) => v.push(new_id),
            None => { self.sensor_ids.insert(name_rc, vec![new_id]); }
        }

        (sensor, new_id)
    }

    pub fn sensor(&self, id: u32) -> Option<&Rc<RefCell<SensorConatiner>>> {
        self.sensors.get(&id)
    }

    pub fn sensor_ids(&self, name: &str) -> Option<&[u32]> { 
        match self.sensor_ids.get(name.into()) {
            Some(id) => Some(id),
            None => None
        }
    }

    pub fn sensor_name(&self, id: u32) -> Option<&str> { 
        match self.sensor_names.get(&id) {
            Some(id) => Some(&id),
            None => None
        }
    }

    pub fn sensors_names(&self) -> Vec<Rc<str>> { 
        self.sensor_ids.keys().cloned().collect()
    }

    pub fn sensor_data_type(&self, id: u32) -> Option<DataType> { 
        Some(self.sensors.get(&id)?.borrow().data_type())
    }

    pub fn sensor_data_category(&self, id: u32) -> Option<DataCategory> { 
        Some(self.sensors.get(&id)?.borrow().data_category())
    }

    pub fn sensor_insert(
        &mut self, id: u32, item: &DataTypeValue
    ) -> Option<Rc<RefCell<dyn Neuron>>> {
        Some(self.sensors.get_mut(&id)?.borrow_mut().insert(item))
    }
    
    pub fn sensor_search(
        &self, id: u32, item: &DataTypeValue
    ) -> Option<Rc<RefCell<dyn Neuron>>> { 
        self.sensors.get(&id)?.borrow().search(item) 
    }

    pub fn sensor_activate(
        &mut self, 
        id: u32, 
        item: &DataTypeValue,
        signal: f32,
        propagate_horizontal: bool, 
        propagate_vertical: bool
    ) -> Result<f32> {
        self.sensors
            .get_mut(&id)
            .unwrap_or(Err(anyhow::anyhow!("sensor {} doesn't exists", id))?)
            .borrow_mut()
            .activate(item, signal, propagate_horizontal, propagate_vertical)
    }

    pub fn sensor_deactivate(
        &mut self, 
        id: u32, 
        item: &DataTypeValue,
        propagate_horizontal: bool, 
        propagate_vertical: bool
    ) -> Result<()> {
        self.sensors
            .get_mut(&id)
            .unwrap_or(Err(anyhow::anyhow!("sensor {} doesn't exists", id))?)
            .borrow_mut()
            .deactivate(item, propagate_horizontal, propagate_vertical)
    }

    pub fn deactivate_whole_sensor(&mut self, id: u32) -> Result<()> {
        self.sensors
            .get_mut(&id)
            .unwrap_or(Err(anyhow::anyhow!("sensor {} doesn't exists", id))?)
            .borrow_mut()
            .deactivate_sensor();
        Ok(())
    }
    
    pub fn create_neuron(
        &mut self, id: NeuronID
    ) -> Rc<RefCell<dyn Neuron>> {
        let neuron = SimpleNeuron::new(id) as Rc<RefCell<dyn Neuron>>;
        self.neurons.push(neuron.clone());
        neuron
    }
    
    pub fn add_neuron(
        &mut self, neuron: Rc<RefCell<dyn Neuron>>
    ) {
        self.neurons.push(neuron.clone());
    }

    pub fn neuron_from_id(&self, id: &NeuronID) -> Option<Rc<RefCell<dyn Neuron>>> {
        for neuron in &self.neurons {
            if neuron.borrow().id() == *id { return Some(neuron.clone()) }
        }
        None
    }

    pub fn neurons_names(&self) -> Vec<Rc<str>> { 
        self.neuron_group_ids.keys().cloned().collect()
    }

    pub fn neuron(&self, id: u32, parent_id: u32) -> Option<Rc<RefCell<dyn Neuron>>> {
        let neuron_id = NeuronID::new(id, parent_id);
        for neuron in &self.neurons {
            if neuron.borrow().id() == neuron_id { return Some(neuron.clone()) }
        }
        None
    }

    pub fn deactivate(&mut self) {
        for sensor in &mut self.sensors.values() { sensor.borrow_mut().deactivate_sensor(); }
        for neuron in &mut self.neurons { neuron.borrow_mut().deactivate(false, false); }
    }

    pub fn add_neuron_group(&mut self, group_name: &str, group_id: u32) {
        let group_name_rc: Rc<str> = group_name.into();
        self.neuron_group_names.insert(group_id, group_name_rc.clone());
        match self.neuron_group_ids.get_mut(&group_name_rc) {
            Some(v) => v.push(group_id),
            None => { self.neuron_group_ids.insert(group_name_rc, vec![group_id]); }
        }
    }

    pub fn neuron_group_ids(&self, name: &str) -> Option<&[u32]> { 
        match self.neuron_group_ids.get(name.into()) {
            Some(id) => Some(id),
            None => None
        }
    }

    pub fn neuron_group_name(&self, id: u32) -> Option<&str> { 
        match self.neuron_group_names.get(&id) {
            Some(id) => Some(&id),
            None => None
        }
    }
}

impl Display for MAGDS {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        writeln!(f, "========== sensors ==========")?;
        let mut number = 1;
        for (id, sensor) in &self.sensors {
            writeln!(f, "{number}: {id}")?;
            writeln!(f, "{}", sensor.borrow())?;
            number += 1;
        }

        let mut number = 1;
        writeln!(f, "========== neurons ==========")?;
        for neuron in &self.neurons {
            writeln!(f, "{number}: {}", neuron.borrow())?;
            number += 1;
        }

        writeln!(f, "========== ======= ==========")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::Arc,
        rc::Rc,
        cell::RefCell
    };

    use asa_graphs::neural::graph::ASAGraph;
    
    use witchnet_common::{
        neuron::NeuronID,
        sensor::Sensor,
        data::DataType,
        polars as polars_common
    };
    
    use crate::neuron::simple_neuron::SimpleNeuron;

    use super::{
        MAGDS,
        super::parser
    };

    #[test]
    fn create_magds() {
        let mut magds = MAGDS::new();

        let mut sensor_1 = ASAGraph::<i32>::new_box(1) as Box<dyn Sensor<i32>>;
        for i in 1..=9 { sensor_1.insert(&i); }

        let mut sensor_2 = ASAGraph::<String, 3>::new_box(2) as Box<dyn Sensor<String>>;
        for i in 1..=9 { sensor_2.insert(&i.to_string()); }

        let parent_id = 1u32;
        let neuron_1 = SimpleNeuron::new(
            NeuronID { id: 1, parent_id }
        );
        let neuron_2 = SimpleNeuron::new(
            NeuronID { id: 2, parent_id }
        );

        magds.add_neuron_group("1", parent_id);
        println!("{:?}", magds.neuron_group_ids);
        assert_eq!(magds.neuron_group_ids("1").unwrap().first().unwrap(), &1);
        assert_eq!(magds.neuron_group_name(1).unwrap(), "1");

        magds.add_sensor("test_1".into(), Rc::new(RefCell::new(sensor_1.into())));
        magds.add_sensor("test_2".into(), Rc::new(RefCell::new(sensor_2.into())));
        assert_eq!(magds.sensor_names[&1], Rc::from("test_1"));
        assert_eq!(magds.sensor_names[&2], Rc::from("test_2"));
        assert_eq!(magds.sensor_name(1).unwrap(), "test_1");
        assert_eq!(magds.sensor_name(2).unwrap(), "test_2");

        magds.add_neuron(neuron_1);
        magds.add_neuron(neuron_2);

        let sensor_1_from_magds = magds.sensor(1).unwrap();
        sensor_1_from_magds.borrow_mut().insert(&10.into());
        sensor_1_from_magds.borrow_mut().insert(&11.into());
        sensor_1_from_magds.borrow_mut().insert(&12.into());
        assert!(sensor_1_from_magds.borrow().as_i32().is_some());

        let mut sum = 0;
        for i in 1..=12 {
            let el = sensor_1_from_magds.borrow().search(&(i as i32).into());
            assert!(el.is_some());
            let neuron_id = NeuronID { id: i, parent_id: 1 };
            let el = el.unwrap();
            assert_eq!(el.borrow().id(), neuron_id);
            let counter = el.borrow().counter();
            sum += counter;
            assert_eq!(counter, 1);
        }
        assert_eq!(sum, 12);

        let sensor_2_from_magds = magds.sensor(2).unwrap();
        sensor_2_from_magds.borrow_mut().insert(&10.to_string().into());
        sensor_2_from_magds.borrow_mut().insert(&11.to_string().into());
        sensor_2_from_magds.borrow_mut().insert(&12.to_string().into());

        let mut sum = 0;
        for i in 1..=12 {
            let el = sensor_2_from_magds.borrow().search(&(i.to_string()).into());
            assert!(el.is_some());
            let neuron_id = NeuronID { id: i, parent_id: 2 };
            let el = el.unwrap();
            println!("i {i} el_id {} ket {}", el.borrow().id(), el.borrow().value());
            assert_eq!(el.borrow().id(), neuron_id);
            let counter = el.borrow().counter();
            sum += counter;
            assert_eq!(counter, 1);
        }
        assert_eq!(sum, 12);

        let neuron_1_id = NeuronID::new(1, 1);
        let neuron_1_from_magds = magds.neuron_from_id(&neuron_1_id).unwrap();
        assert_eq!(neuron_1_from_magds.borrow().id(), neuron_1_id);
        let neuron_1_from_magds = magds.neuron(1, 1).unwrap();
        assert_eq!(neuron_1_from_magds.borrow().id(), neuron_1_id);

        let (_rcstr_test_sensor, rcstr_test_id) = 
            magds.create_sensor("rcstr_test", DataType::ArcStr);
        let text: Arc<str> = Arc::from("test");
        let sensor_element = magds.sensor_insert(rcstr_test_id, &text.into());
        assert!(sensor_element.is_some())
    }

    #[test]
    fn magds_activation() {
        let df = polars_common::csv_to_dataframe("data/iris.csv", &vec![]).unwrap();
        let magds = parser::magds_from_df("iris", &df);
        println!("{magds}");

        let sepal_length_id = *magds.sensor_ids("sepal.length").unwrap().first().unwrap();
        let sl43 = magds.sensor_search(sepal_length_id, &4.3_f64.into()).unwrap();
        let sl57 = magds.sensor_search(sepal_length_id, &5.7_f64.into()).unwrap();
        let sl58 = magds.sensor_search(sepal_length_id, &5.8_f64.into()).unwrap();
        let sl59 = magds.sensor_search(sepal_length_id, &5.9_f64.into()).unwrap();
        let sl79 = magds.sensor_search(sepal_length_id, &7.9_f64.into()).unwrap();
        let neuron_15 = magds.neuron(15, 1).unwrap();
        let neuron_16 = magds.neuron(16, 1).unwrap();
       
        sl58.borrow_mut().activate(1.0, false, false);
        assert_eq!(sl57.borrow().activation(), 0.0_f32);
        assert_eq!(sl58.borrow().activation(), 1.0_f32);
        assert_eq!(sl59.borrow().activation(), 0.0_f32);
        assert_eq!(neuron_15.borrow().activation(), 0.0_f32);
        
        sl58.borrow_mut().deactivate(false, false);
        assert_eq!(sl58.borrow().activation(), 0.0_f32);
        assert_eq!(neuron_15.borrow().activation(), 0.0_f32);

        sl58.borrow_mut().activate(1.0, false, true);
        assert_eq!(sl57.borrow().activation(), 0.0_f32);
        assert_eq!(sl58.borrow().activation(), 1.0_f32);
        assert_eq!(sl59.borrow().activation(), 0.0_f32);
        // assert_eq!(neuron_15.borrow().activation(), 1.0_f32 / 7_f32);
        assert_eq!(neuron_15.borrow().activation(), 1.0_f32 / 1_f32);
        assert_eq!(neuron_16.borrow().activation(), 0.0_f32);

        sl58.borrow_mut().deactivate(false, false);
        assert_eq!(sl58.borrow().activation(), 0.0_f32);
        assert_eq!(neuron_15.borrow().activation(), 1.0_f32 / 1_f32);
        // assert_eq!(neuron_15.borrow().activation(), 1.0_f32 / 7_f32);

        sl58.borrow_mut().deactivate(false, true);
        assert_eq!(sl58.borrow().activation(), 0.0_f32);
        assert_eq!(neuron_15.borrow().activation(), 0.0_f32);

        sl58.borrow_mut().activate(1.0, true, true);
        assert_eq!(sl43.borrow().activation(), 0.0_f32);
        assert_eq!(sl57.borrow().activation(), 0.9722222_f32);
        assert_eq!(sl58.borrow().activation(), 1.0_f32);
        assert_eq!(sl59.borrow().activation(), 0.9722222_f32);
        assert_eq!(sl79.borrow().activation(), 0.0_f32);
        // assert_eq!(neuron_15.borrow().activation(), 1.0_f32 / 7_f32);
        assert_eq!(neuron_15.borrow().activation(), 1.0_f32 / 1_f32);
        // assert_eq!(neuron_16.borrow().activation(), 0.9722222_f32 / 8_f32);
        assert_eq!(neuron_16.borrow().activation(), 0.9722222_f32 / 1_f32);

        sl58.borrow_mut().deactivate(true, true);
        assert_eq!(sl43.borrow().activation(), 0.0_f32);
        assert_eq!(sl57.borrow().activation(), 0.0_f32);
        assert_eq!(sl58.borrow().activation(), 0.0_f32);
        assert_eq!(sl59.borrow().activation(), 0.0_f32);
        assert_eq!(sl79.borrow().activation(), 0.0_f32);
        assert_eq!(neuron_15.borrow().activation(), 0.0_f32);
        assert_eq!(neuron_16.borrow().activation(), 0.0_f32);
    }
}