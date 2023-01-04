use std::{
    sync::{ Arc, RwLock },
    marker::PhantomData
};

use anyhow::Result;

use witchnet_common::{
    data::{ DataCategory, DataType, DataDeductor, DataTypeValue },
    neuron::NeuronAsync,
    sensor::{ SensorAsync, SensorData }, 
    connection::collective::defining::DefiningWeightingStrategyAsync
};

use super::graph::ASAGraph;

impl<Key, const ORDER: usize> SensorAsync<Key> for ASAGraph<Key, ORDER> 
where 
    Key: SensorData + Sync + Send, 
    [(); ORDER + 1]:, 
    PhantomData<Key>: DataDeductor,
    DataTypeValue: From<Key>
{
    fn id(&self) -> u32 { self.id() }

    fn data_type(&self) -> DataType { self.data_type() }

    fn data_category(&self) -> DataCategory { self.data_category() }

    fn insert(&mut self, item: &Key) -> Arc<RwLock<dyn NeuronAsync>> {
        self.insert(item)
    }

    fn insert_custom(
        &mut self, 
        item: &Key, 
        weighting_strategy: Arc<dyn DefiningWeightingStrategyAsync>,
        interelement_activation_threshold: f32,
        interelement_activation_exponent: i32
    ) -> Arc<RwLock<dyn NeuronAsync>> {
        self.insert_custom(
            item, 
            weighting_strategy,
            interelement_activation_threshold,
            interelement_activation_exponent
        )
    }

    fn search(&self, item: &Key) -> Option<Arc<RwLock<dyn NeuronAsync>>> { 
        match self.search(item) {
            Some(n) => Some(n as Arc<RwLock<dyn NeuronAsync>>),
            None => None
        }
    }

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

    fn neurons(&self) -> Vec<Arc<RwLock<dyn NeuronAsync>>> {
        self.into_iter().map(|x| x.clone() as Arc<RwLock<dyn NeuronAsync>>).collect()
    }

    fn values(&self) -> Vec<Key> {
        self.into_iter().map(|e| *dyn_clone::clone_box(&e.read().unwrap().key)).collect()
    }
}

#[cfg(test)]
mod tests {
    use witchnet_common::{
        data::DataCategory,
        neuron::NeuronAsync
    };

    use super::super::element::Element;
    use super::super::graph::ASAGraph;
    
    #[test]
    fn sensor() {
        let threshold = Element::<i32, 3>::new(&1, 0, 0)
            .read().unwrap()
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
                let activation = element.read().unwrap().activation();
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
                let activation = element.read().unwrap().activation();
                assert_eq!(activation, 0.0f32)
            }

            let neurons = graph.activate(&5, 1.0f32, true, true);
            assert!(neurons.is_ok());
            for (i, element) in graph.into_iter().enumerate() {
                let activation = element.read().unwrap().activation();
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
                let activation = element.read().unwrap().activation();
                assert_eq!(activation, 0.0f32)
            }

            let max_activation = graph.activate(&5, 1.0f32, false, false);
            assert!(max_activation.is_ok());
            let max_activation = graph.activate(&8, 1.0f32, false, false);
            assert!(max_activation.is_ok());
            assert!(max_activation.unwrap() > 0.0f32);
            for (i, element) in graph.into_iter().enumerate() {
                let activation = element.read().unwrap().activation();
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
                let activation = element.read().unwrap().activation();
                let n = i + 1;
                if n == 8 { assert_eq!(activation, 1.0f32) } else { assert_eq!(activation, 0.0f32) }
            }
        }
    }
}