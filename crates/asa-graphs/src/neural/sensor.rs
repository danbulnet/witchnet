use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
    marker::PhantomData
};

use witchnet_common::{
    data::{ DataCategory, DataType, DataDeductor, DataTypeValue },
    neuron::{ Neuron, NeuronID },
    sensor::{ Sensor, SensorData }
};

use super::graph::ASAGraph;

impl<Key, const ORDER: usize> Sensor<Key> for ASAGraph<Key, ORDER> 
where 
    Key: SensorData, 
    [(); ORDER + 1]:, 
    PhantomData<Key>: DataDeductor,
    DataTypeValue: From<Key>
{
    fn id(&self) -> u32 { self.id() }

    fn data_type(&self) -> DataType { self.data_type() }

    fn data_category(&self) -> DataCategory { self.data_category() }

    fn insert(&mut self, item: &Key) -> Rc<RefCell<dyn Neuron>> {
        self.insert(item)
    }

    fn search(&self, item: &Key) -> Option<Rc<RefCell<dyn Neuron>>> { 
        match self.search(item) {
            Some(n) => Some(n as Rc<RefCell<dyn Neuron>>),
            None => None
        }
    }

    fn activate(
        &mut self, 
        item: &Key, 
        signal: f32, 
        propagate_horizontal: bool, 
        propagate_vertical: bool
    ) -> Result<(HashMap<NeuronID, Rc<RefCell<dyn Neuron>>>, f32), String> {
        self.activate(item, signal, propagate_horizontal, propagate_vertical)
    }

    fn deactivate(
        &mut self, 
        item: &Key, 
        propagate_horizontal: bool, 
        propagate_vertical: bool
    ) -> Result<(), String> {
        self.deactivate(item, propagate_horizontal, propagate_vertical)
    }

    fn deactivate_sensor(&mut self) { self.deactivate_sensor() }

    fn neurons(&self) -> Vec<Rc<RefCell<dyn Neuron>>> {
        self.into_iter().map(|x| x.clone() as Rc<RefCell<dyn Neuron>>).collect()
    }

    fn values(&self) -> Vec<Key> {
        self.into_iter().map(|e| *dyn_clone::clone_box(&e.borrow().key)).collect()
    }
}

#[cfg(test)]
mod tests {
    use witchnet_common::{
        data::DataCategory,
        neuron::Neuron
    };

    use super::super::element::Element;
    use super::super::graph::ASAGraph;
    
    #[test]
    fn sensor() {
        let threshold = Element::<i32, 3>::INTERELEMENT_ACTIVATION_THRESHOLD;

        let mut graph = ASAGraph::<i32, 3>::new(1);
        for i in (1..=9).rev() { graph.insert(&i); }
        
        assert_eq!(graph.id(), 1);
        assert_eq!(graph.data_category(), DataCategory::Numerical);

        let neurons = graph.activate(&5, 1.0f32, true, true);
        assert!(neurons.is_ok());
        assert_eq!(neurons.unwrap().0.len(), 0);
        
        if threshold == 0.8f32 {
            for (i, element) in graph.into_iter().enumerate() {
                let activation = element.borrow().activation();
                match i + 1 {
                    1 => assert_eq!(activation, 0.0f32),
                    2 => assert_eq!(activation, 0.0f32),
                    3 => assert_eq!(activation, 0.765625f32),
                    4 => assert_eq!(activation, 0.875f32),
                    5 => assert_eq!(activation, 1.0f32),
                    6 => assert_eq!(activation, 0.875f32),
                    7 => assert_eq!(activation, 0.765625f32),
                    8 => assert_eq!(activation, 0.0f32),
                    9 => assert_eq!(activation, 0.0f32),
                    _ => {}
                };
            }
            let result = graph.deactivate(&4, true, true);
            assert!(result.is_ok());
            for element in graph.into_iter() {
                let activation = element.borrow().activation();
                assert_eq!(activation, 0.0f32)
            }

            let neurons = graph.activate(&5, 1.0f32, true, true);
            assert!(neurons.is_ok());
            for (i, element) in graph.into_iter().enumerate() {
                let activation = element.borrow().activation();
                match i + 1 {
                    1 => assert_eq!(activation, 0.0f32),
                    2 => assert_eq!(activation, 0.0f32),
                    3 => assert_eq!(activation, 0.765625f32),
                    4 => assert_eq!(activation, 0.875f32),
                    5 => assert_eq!(activation, 1.0f32),
                    6 => assert_eq!(activation, 0.875f32),
                    7 => assert_eq!(activation, 0.765625f32),
                    8 => assert_eq!(activation, 0.0f32),
                    9 => assert_eq!(activation, 0.0f32),
                    _ => {}
                };
            }
            graph.deactivate_sensor();
            for element in graph.into_iter() {
                let activation = element.borrow().activation();
                assert_eq!(activation, 0.0f32)
            }

            let neurons = graph.activate(&5, 1.0f32, false, false);
            assert!(neurons.is_ok());
            let neurons = graph.activate(&8, 1.0f32, false, false);
            assert!(neurons.is_ok());
            assert_eq!(neurons.unwrap().0.len(), 0);
            for (i, element) in graph.into_iter().enumerate() {
                let activation = element.borrow().activation();
                match i + 1 {
                    1 => assert_eq!(activation, 0.0f32),
                    2 => assert_eq!(activation, 0.0f32),
                    3 => assert_eq!(activation, 0.0f32),
                    4 => assert_eq!(activation, 0.0f32),
                    5 => assert_eq!(activation, 1.0f32),
                    6 => assert_eq!(activation, 0.0f32),
                    7 => assert_eq!(activation, 0.0f32),
                    8 => assert_eq!(activation, 1.0f32),
                    9 => assert_eq!(activation, 0.0f32),
                    _ => {}
                };
            }
            let result = graph.deactivate(&5, false, false);
            assert!(result.is_ok());
            for (i, element) in graph.into_iter().enumerate() {
                let activation = element.borrow().activation();
                let n = i + 1;
                if n == 8 { assert_eq!(activation, 1.0f32) } else { assert_eq!(activation, 0.0f32) }
            }
        }
    }
}