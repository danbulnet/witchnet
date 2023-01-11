use std::{
    rc::Rc,
    cell::RefCell,
    marker::PhantomData
};

use anyhow::Result;

use witchnet_common::{
    data::{ DataCategory, DataType, DataDeductor, DataTypeValue },
    neuron::Neuron,
    sensor::{ Sensor, SensorData }, 
    connection::collective::defining::DefiningWeightingStrategy
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

    fn insert_custom(
        &mut self, 
        item: &Key, 
        weighting_strategy: Rc<dyn DefiningWeightingStrategy>,
        interelement_activation_threshold: f32,
        interelement_activation_exponent: i32
    ) -> Rc<RefCell<dyn Neuron>> {
        self.insert_custom(
            item, 
            weighting_strategy,
            interelement_activation_threshold,
            interelement_activation_exponent
        )
    }

    fn search(&self, item: &Key) -> Option<Rc<RefCell<dyn Neuron>>> { 
        match self.search(item) {
            Some(n) => Some(n as Rc<RefCell<dyn Neuron>>),
            None => None
        }
    }

    fn fuzzy_search(
        &mut self, item: &Key, threshold: f32, perserve_inserted_neuron: bool
    ) -> Option<(Rc<RefCell<dyn Neuron>>, f32)> {
        match self.fuzzy_search(item, threshold, perserve_inserted_neuron) {
            Some(n) => Some((n.0 as Rc<RefCell<dyn Neuron>>, n.1)),
            None => None
        }
    }

    fn remove(&mut self, item: &Key) -> bool { self.remove(item) }

    fn activate(
        &mut self, 
        item: &Key, 
        signal: f32, 
        propagate_horizontal: bool, 
        propagate_vertical: bool
    ) -> Result<f32> {
        self.activate(item, signal, propagate_horizontal, propagate_vertical)
    }

    fn deactivate(
        &mut self, 
        item: &Key, 
        propagate_horizontal: bool, 
        propagate_vertical: bool
    ) -> Result<()> {
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
        let threshold = Element::<i32, 3>::new(&1, 0, 0)
            .borrow()
            .interelement_activation_threshold;

        let mut graph = ASAGraph::<i32, 3>::new(1);
        for i in (1..=9).rev() { graph.insert(&i); }
        
        assert_eq!(graph.id(), 1);
        assert_eq!(graph.data_category(), DataCategory::Continuous);

        let max_activation = graph.activate(&5, 1.0f32, true, true);
        assert!(max_activation.is_ok());
        assert_eq!(max_activation.unwrap(), 0.0f32);
        
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

            let max_activation = graph.activate(&5, 1.0f32, false, false);
            assert!(max_activation.is_ok());
            let max_activation = graph.activate(&8, 1.0f32, false, false);
            assert!(max_activation.is_ok());
            assert!(max_activation.unwrap() > 0.0f32);
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