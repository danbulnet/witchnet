use std::{
    fmt::{ Display, Formatter, Result as FmtResult },
    sync::{ Arc, RwLock, Weak },
    marker::PhantomData,
};

use anyhow::Result;

use witchnet_common::{
    neuron::{ 
        NeuronAsync, NeuronID, NeuronConnectAsync, NeuronConnectBilateralAsync
    },
    connection::{
        ConnectionKind,
        collective::{
            CollectiveConnectionsAsync,
            defining::DefiningConnectionsAsync
        }
    },
    sensor::SensorData,
    data::{ DataDeductor, DataCategory, DataTypeValue, DataType }
};

#[derive(Clone)]
pub struct Element<Key, const ORDER: usize>
where Key: SensorData, [(); ORDER + 1]: {
    pub id: u32,
    pub parent_id: u32,
    pub key: Key,
    pub counter: usize,
    pub activation: f32,
    pub next: Option<(Weak<RwLock<Element<Key, ORDER>>>, f32)>,
    pub prev: Option<(Weak<RwLock<Element<Key, ORDER>>>, f32)>,
    pub definitions: DefiningConnectionsAsync,
    pub(crate) data_type: PhantomData<Key>
}

impl<Key, const ORDER: usize> Element<Key, ORDER> 
where 
    Key: SensorData, [(); ORDER + 1]:, 
    PhantomData<Key>: DataDeductor, 
    DataTypeValue: From<Key> 
{
    pub const INTERELEMENT_ACTIVATION_THRESHOLD: f32 = 0.998;

    pub fn new(key: &Key, id: u32, parent_id: u32)
    -> Arc<RwLock<Element<Key, ORDER>>> {
        let element_ptr = Arc::new(
            RwLock::new(
                Element {
                    id,
                    parent_id,
                    key: *dyn_clone::clone_box(key),
                    counter: 1,
                    activation: 0.0f32,
                    next: None,
                    prev: None,
                    definitions: DefiningConnectionsAsync::new(),
                    data_type: PhantomData
                }
            )
        );

        element_ptr
    }

    pub(crate) fn set_connections(
        element_ptr: &Arc<RwLock<Element<Key, ORDER>>>,
        prev_opt: Option<&Arc<RwLock<Element<Key, ORDER>>>>,
        next_opt: Option<&Arc<RwLock<Element<Key, ORDER>>>>,
        range: f32
    ) {
        let mut element = &mut *element_ptr.write().unwrap();
        
        if prev_opt.is_some() {
            let prev_ptr = prev_opt.unwrap();
            let weight = element.weight(&prev_ptr.read().unwrap(), range);
            element.prev = Some((Arc::downgrade(prev_ptr), weight));
            prev_ptr.write().unwrap().next = Some((Arc::downgrade(element_ptr), weight));
        } else { 
            element.prev = None; 
        }

        if next_opt.is_some() {
            let next_ptr = next_opt.unwrap();
            let weight = element.weight(&next_ptr.read().unwrap(), range);
            element.next = Some((Arc::downgrade(next_ptr), weight));
            next_ptr.write().unwrap().prev = Some((Arc::downgrade(&element_ptr), weight));
        } else { 
            element.next = None; 
        }
    }

    pub fn weight(&self, other: &Self, range: f32) -> f32 {
        1.0f32 - (other.key.distance(&self.key) as f32).abs() / range
    }

    pub fn weight_to_key(&self, other_key: &Key, range: f32) -> f32 {
        1.0f32 - (other_key.distance(&self.key) as f32).abs() / range
    }

    pub fn fuzzy_activate(&mut self, signal: f32) -> Vec<(Arc<RwLock<dyn NeuronAsync>>, f32)> {
        self.activation += signal;

        let mut neurons: Vec<(Arc<RwLock<dyn NeuronAsync>>, f32)> = self
            .defining_neurons()
            .into_iter()
            .map(|neuron| (
                neuron.clone(),
                self.activation() * self.definitions.common_weight()
            ))
            .collect();

        let mut element_activation = self.activation;
        if let Some(next) = &self.next {
            let mut element = next.0.upgrade().unwrap();
            let mut weight = next.1;
            while element_activation > Self::INTERELEMENT_ACTIVATION_THRESHOLD {
                let new_element;
                {
                    {
                        let element_borrowed = &mut *element.write().unwrap();
                        element_borrowed.activate(element_activation * weight, false, false);
                    }
                    let element_borrowed = &*element.read().unwrap();
                    neurons.append(
                        &mut (&*element.read().unwrap())
                            .defining_neurons()
                            .into_iter()
                            .cloned()
                            .into_iter().map(
                                |neuron| (
                                    neuron,
                                    element_borrowed.definitions.output_signal(element.clone())
                                )
                            )
                            .collect()
                    );

                    new_element = match &element_borrowed.next {
                        Some(next) => {
                            weight = next.1;
                            next.0.upgrade().unwrap()
                        },
                        None => break
                    };
                    element_activation = element_borrowed.activation;
                }
                element = new_element;
            }
        }
        
        element_activation = self.activation;
        if let Some(prev) = &self.prev {
            let mut element = prev.0.upgrade().unwrap();
            let mut weight = prev.1;
            while element_activation > Self::INTERELEMENT_ACTIVATION_THRESHOLD {
                let new_element;
                {
                    {
                        let element_borrowed = &mut *element.write().unwrap();
                        element_borrowed.activate(element_activation * weight, false, false);
                    }
                    let element_borrowed = &*element.read().unwrap();
                    neurons.append(
                        &mut element_borrowed
                            .defining_neurons()
                            .into_iter()
                            .cloned()
                            .into_iter().map(
                                |neuron| (
                                    neuron, 
                                    element_borrowed.definitions.output_signal(element.clone())
                                )
                            )
                            .collect()
                    );
    
                    new_element = match &element_borrowed.prev {
                        Some(prev) => {
                            weight = prev.1;
                            prev.0.upgrade().unwrap()
                        },
                        None => break
                    };
                    element_activation = element_borrowed.activation;
                }
                element = new_element;
            }
        }

        neurons
    }

    
    pub(crate) fn simple_activate(
        &mut self, signal: f32
    )-> Vec<(Arc<RwLock<dyn NeuronAsync>>, f32)> {
        self.activation += signal;
        self.defining_neurons().into_iter()
            .cloned()
            .into_iter().map(|x| (
                x.clone(), 
                self.activation() * self.definitions.common_weight()
            ))
            .collect()
    }

    pub fn defining_neurons(&self) -> &[Arc<RwLock<dyn NeuronAsync>>] {
        self.definitions.connected_neurons()
    }
}

impl<Key, const ORDER: usize> NeuronAsync for Element<Key, ORDER> 
where Key: SensorData, [(); ORDER + 1]:, PhantomData<Key>: DataDeductor, DataTypeValue: From<Key> {
    fn id(&self) -> NeuronID {
        NeuronID {
            id: self.id,
            parent_id: self.parent_id
        }
    }

    fn value(&self) -> DataTypeValue { (*dyn_clone::clone_box(&self.key)).into() }

    fn activation(&self) -> f32 { self.activation }

    fn is_sensor(&self) -> bool { true }

    fn data_type(&self) -> DataType { self.data_type.data_type() }

    fn counter(&self) -> usize { self.counter }
    
    fn explain(&self) -> &[Arc<RwLock<dyn NeuronAsync>>] { &[] }

    fn explain_one(&self, _parent: u32) -> Option<DataTypeValue> {
        Some((*dyn_clone::clone_box(&self.key)).into())
    }

    fn defined_neurons(&self) -> &[Arc<RwLock<dyn NeuronAsync>>] {
        &self.definitions.connected_neurons()
    }

    fn activate(
        &mut self, signal: f32, propagate_horizontal: bool, propagate_vertical: bool
    ) -> f32 {
        let data_category: DataCategory = self.data_type.data_category();
        let is_fuzzy_ok = match data_category {
            DataCategory::Numerical | DataCategory::Ordinal => true,
            _ => false
        };
        let neurons_activation = if propagate_horizontal && is_fuzzy_ok {
            self.fuzzy_activate(signal)
        } else {
            self.simple_activate(signal)
        };

        let mut max_activation = 0.0f32;
        if propagate_vertical {
            for (neuron, activation) in &neurons_activation {
                max_activation = f32::max(max_activation, *activation);
                neuron.write().unwrap().activate(
                    *activation, propagate_horizontal, propagate_vertical
                );
            }
        }

        max_activation
    }

    fn deactivate(&mut self, propagate_horizontal: bool, propagate_vertical: bool) {
        self.activation = 0.0f32;

        let mut neurons: Vec<Arc<RwLock<dyn NeuronAsync>>> = Vec::new();
        if propagate_vertical {
            neurons = self.defining_neurons().into_iter().cloned().collect();
        }

        if propagate_horizontal{
            if let Some(next) = &self.next {
                let mut element = next.0.upgrade().unwrap();
                loop {
                    element.write().unwrap().activation = 0.0f32;
                    if propagate_vertical {
                        neurons.append(
                            &mut element.read().unwrap().defining_neurons().into_iter().cloned().collect()
                        );
                    }
                    let new_element = match &element.read().unwrap().next {
                        Some(next) => next.0.upgrade().unwrap(),
                        None => break
                    };
                    element = new_element;
                }
            }
            
            if let Some(prev) = &self.prev {
                let mut element = prev.0.upgrade().unwrap();
                loop {
                    element.write().unwrap().activation = 0.0f32;
                    if propagate_vertical {
                        neurons.append(
                            &mut element.read().unwrap().defining_neurons().into_iter().cloned().collect()
                        );
                    };
                    let new_element = match &element.read().unwrap().prev {
                        Some(prev) => prev.0.upgrade().unwrap(),
                        None => break
                    };
                    element = new_element;
                }
            }
        }
        
        if propagate_vertical {
            for neuron in neurons {
                neuron.write().unwrap().deactivate(propagate_horizontal, propagate_vertical)
            }
        }
    }
}

impl<Key, const ORDER: usize> NeuronConnectAsync for Element<Key, ORDER> 
where 
    Key: SensorData, 
    [(); ORDER + 1]:, 
    PhantomData<Key>: DataDeductor,
    DataTypeValue: From<Key>
{
    fn connect_to<Other: NeuronAsync + NeuronConnectAsync + 'static>(
        &mut self, to: Arc<RwLock<Other>>, kind: ConnectionKind
    ) -> Result<()> {
        match kind {
            ConnectionKind::Defining => {
                if to.read().unwrap().is_sensor() {
                    anyhow::bail!("only defining connection from sensor to neuron can be created")
                }
                self.definitions.add(to);
                Ok(())
            }
            _ => {
                anyhow::bail!("only defining connection to element can be created for asa-graphs")
            }
        }
    }
}

impl<Key, const ORDER: usize, Other: NeuronAsync + NeuronConnectAsync + 'static> 
NeuronConnectBilateralAsync<Other> for Element<Key, ORDER>
where 
    Key: SensorData, 
    [(); ORDER + 1]:, 
    PhantomData<Key>: DataDeductor,
    DataTypeValue: From<Key>
{
    fn connect_bilateral(
        from: Arc<RwLock<Self>>, to: Arc<RwLock<Other>>, kind: ConnectionKind
    ) -> Result<()> {
        match kind {
            ConnectionKind::Defining => {
                if !to.read().unwrap().is_sensor() {
                    from.write().unwrap().connect_to(to.clone(), kind)?;
                    to.write().unwrap().connect_to(from, ConnectionKind::Explanatory)?;
                    Ok(())
                } else {
                    anyhow::bail!("connections between sensors are not allowed")    
                }
            }
            _ => {
                anyhow::bail!("only defining connection from Element to SimpleNeuron can be created")
            }
        }
    }
}

impl<Key, const ORDER: usize> Display for Element<Key, ORDER> 
where Key: SensorData, [(); ORDER + 1]: {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "[{}:{}]", &self.key, &self.counter)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{ Arc, RwLock };

    use witchnet_common::{
        neuron::{ NeuronAsync, NeuronConnectAsync },
        connection::ConnectionKind
    };

    use super::super::{
        element::Element,
        graph::ASAGraph
    };

    #[test]
    fn set_connections() {
        let graph = Arc::new(
            RwLock::new(ASAGraph::<i32, 3>::new(1))
        );
        let graph_id = graph.read().unwrap().id;

        let element_1_ptr: Arc<RwLock<Element<i32, 3>>> = Element::new(&1, 1, graph_id);
        let element_2_ptr: Arc<RwLock<Element<i32, 3>>> = Element::new(&2, 2, graph_id);
        let element_3_ptr: Arc<RwLock<Element<i32, 3>>> = Element::new(&3, 3, graph_id);

        assert!(element_1_ptr.read().unwrap().prev.is_none());
        assert!(element_1_ptr.read().unwrap().next.is_none());
        assert!(element_2_ptr.read().unwrap().prev.is_none());
        assert!(element_2_ptr.read().unwrap().next.is_none());
        assert!(element_3_ptr.read().unwrap().prev.is_none());
        assert!(element_3_ptr.read().unwrap().next.is_none());
        
        Element::set_connections(&element_2_ptr, Some(&element_1_ptr), None, 2f32);

        assert!(element_1_ptr.read().unwrap().prev.is_none());
        assert_eq!(
            element_1_ptr.read().unwrap().next.as_ref().unwrap().0.upgrade().unwrap().read().unwrap().key,
            element_2_ptr.read().unwrap().key
        );
        assert!(element_2_ptr.read().unwrap().next.is_none());
        assert!(element_3_ptr.read().unwrap().prev.is_none());
        assert!(element_3_ptr.read().unwrap().next.is_none());

        Element::set_connections(&element_2_ptr, None, Some(&element_3_ptr), 2f32);

        assert!(element_1_ptr.read().unwrap().prev.is_none());
        assert_eq!(
            element_1_ptr.read().unwrap().next.as_ref().unwrap().0.upgrade().unwrap().read().unwrap().key,
            element_2_ptr.read().unwrap().key
        );
        assert!(element_2_ptr.read().unwrap().prev.is_none());
        assert_eq!(
            element_2_ptr.read().unwrap().next.as_ref().unwrap().0.upgrade().unwrap().read().unwrap().key,
            element_3_ptr.read().unwrap().key
        );
        assert_eq!(
            element_3_ptr.read().unwrap().prev.as_ref().unwrap().0.upgrade().unwrap().read().unwrap().key, 
            element_2_ptr.read().unwrap().key
        );
        assert!(element_3_ptr.read().unwrap().next.is_none());

        Element::set_connections(&element_1_ptr, None, None, 2f32);
        Element::set_connections(&element_2_ptr, None, None, 2f32);
        Element::set_connections(&element_3_ptr, None, None, 2f32);

        assert!(element_1_ptr.read().unwrap().prev.is_none());
        assert!(element_1_ptr.read().unwrap().next.is_none());
        assert!(element_2_ptr.read().unwrap().prev.is_none());
        assert!(element_2_ptr.read().unwrap().next.is_none());
        assert!(element_3_ptr.read().unwrap().prev.is_none());
        assert!(element_3_ptr.read().unwrap().next.is_none());
    }

    #[test]
    fn parent_id() {
        let graph = Arc::new(RwLock::new(ASAGraph::<i32, 3>::new(1)));
        let graph_id = graph.read().unwrap().id;

        let element_1_ptr: Arc<RwLock<Element<i32, 3>>> = Element::new(&1, 1, graph_id);
        let id = element_1_ptr.read().unwrap().id;
        let parent_id = element_1_ptr.read().unwrap().parent_id;
        assert_eq!(id, 1);
        assert_eq!(parent_id, 1);
    }

    #[test]
    fn as_neuron() {
        let graph = Arc::new(RwLock::new(ASAGraph::<i32, 3>::new(1)));
        let graph_id = graph.read().unwrap().id;

        let element_1_ptr: Arc<RwLock<Element<i32, 3>>> = Element::new(&1, 1, graph_id);
        let mut element_1 = element_1_ptr.write().unwrap();
        let element_2_ptr: Arc<RwLock<Element<i32, 3>>> = Element::new(&2, 2, graph_id);

        let element_1_id = element_1.id();
        assert_eq!(element_1_id.id.to_string(), 1.to_string());
        assert_eq!(element_1_id.parent_id.to_string(), graph.read().unwrap().id.to_string());
        let element_2_id = element_2_ptr.read().unwrap().id();
        assert_eq!(element_2_id.id.to_string(),2.to_string());
        assert_eq!(element_2_id.parent_id.to_string(), graph.read().unwrap().id.to_string());

        assert_eq!(element_1.is_sensor(), true);

        assert_eq!(element_1.activation(), 0.0f32);

        assert_eq!(element_1.counter(), 1usize);

        let activated = element_1.activate(1.0f32, true, true);
        assert_eq!(activated, 0.0f32);
        assert_eq!(element_1.activation(), 1.0f32);
        assert_eq!(element_2_ptr.read().unwrap().activation(), 0.0f32);

        element_1.activate(1.0f32, false, true);
        assert_eq!(element_1.activation(), 2.0f32);
        element_1.deactivate(true, true);
        assert_eq!(element_1.activation(), 0.0f32);
        assert_eq!(element_2_ptr.read().unwrap().activation(), 0.0f32);

        element_1.activate(1.0f32, true, false);
        assert_eq!(element_1.activation(), 1.0f32);
        element_1.deactivate(false, true);
        assert_eq!(element_1.activation(), 0.0f32);
        assert_eq!(element_2_ptr.read().unwrap().activation(), 0.0f32);

        element_1.activate(1.0f32, false, false);
        assert_eq!(element_1.activation(), 1.0f32);
        element_1.deactivate(true, false);
        assert_eq!(element_1.activation(), 0.0f32);
        assert_eq!(element_2_ptr.read().unwrap().activation(), 0.0f32);

        element_1.activate(1.0f32, false, false);
        assert_eq!(element_1.activation(), 1.0f32);
        element_1.deactivate(false, false);
        assert_eq!(element_1.activation(), 0.0f32);

        let exp_1 = element_1.explain();
        assert_eq!(exp_1.len(), 0);
    }

    #[test]
    fn fuzzy_activate_deactivate() {
        let threshold = Element::<i32, 3>::INTERELEMENT_ACTIVATION_THRESHOLD;

        let graph = Arc::new(
            RwLock::new(ASAGraph::<i32, 3>::new(1))
        );
        for i in 1..=9 { graph.write().unwrap().insert(&i); }
        {
            let mid_element = graph.read().unwrap().search(&5).unwrap();
            mid_element.write().unwrap().fuzzy_activate(1.0f32);
            assert_eq!(mid_element.read().unwrap().activation(), 1.0f32);
            let mid_element_ref =  mid_element.read().unwrap();

            if threshold == 0.8f32 {
                let (left_neighbour_ptr, left_neighbour_weight) = mid_element_ref.prev.as_ref().unwrap();
                let left_neighbour = left_neighbour_ptr.upgrade().unwrap();
                assert_eq!(*left_neighbour_weight, 0.875f32);
                assert_eq!(left_neighbour.read().unwrap().activation(), 0.875f32);
                let left_neighbour_ref =  left_neighbour.read().unwrap();
    
                let (left_left_neighbour_ptr, left_left_neighbour_weight) = left_neighbour_ref.prev.as_ref().unwrap();
                let left_left_neighbour = left_left_neighbour_ptr.upgrade().unwrap();
                assert_eq!(*left_left_neighbour_weight, 0.875f32);
                assert_eq!(left_left_neighbour.read().unwrap().activation(), 0.765625f32);
    
                let (right_neighbour_ptr, right_neighbour_weight) = mid_element_ref.next.as_ref().unwrap();
                let right_neighbour = right_neighbour_ptr.upgrade().unwrap();
                assert_eq!(*right_neighbour_weight, 0.875f32);
                assert_eq!(right_neighbour.read().unwrap().activation(), 0.875f32);
                let right_neighbour_ref =  right_neighbour.read().unwrap();
    
                let (right_right_neighbour_ptr, right_right_neighbour_weight) = right_neighbour_ref.next.as_ref().unwrap();
                let right_right_neighbour = right_right_neighbour_ptr.upgrade().unwrap();
                assert_eq!(*right_right_neighbour_weight, 0.875f32);
                assert_eq!(right_right_neighbour.read().unwrap().activation(), 0.765625f32);
    
                let second_element = graph.read().unwrap().search(&2).unwrap();
                assert_eq!(second_element.read().unwrap().activation(), 0.0f32);
                let eight_element = graph.read().unwrap().search(&8).unwrap();
                assert_eq!(eight_element.read().unwrap().activation(), 0.0f32);
    
                let element_min = graph.read().unwrap().element_min.as_ref().unwrap().clone();
                assert_eq!(element_min.read().unwrap().activation(), 0.0f32);
    
                let element_max = graph.read().unwrap().element_max.as_ref().unwrap().clone();
                assert_eq!(element_max.read().unwrap().activation(), 0.0f32);
            }
        }

        let mid_element = graph.read().unwrap().search(&5).unwrap();
        mid_element.write().unwrap().deactivate(true, true);
        assert_eq!(mid_element.read().unwrap().activation(), 0.0f32);
        let mid_element_ref =  mid_element.read().unwrap();
        
        let (left_neighbour_ptr, _) = mid_element_ref.prev.as_ref().unwrap();
        let left_neighbour = left_neighbour_ptr.upgrade().unwrap();
        assert_eq!(left_neighbour.read().unwrap().activation(), 0.0f32);
        let left_neighbour_ref =  left_neighbour.read().unwrap();

        let (left_left_neighbour_ptr, _) = left_neighbour_ref.prev.as_ref().unwrap();
        let left_left_neighbour = left_left_neighbour_ptr.upgrade().unwrap();
        assert_eq!(left_left_neighbour.read().unwrap().activation(), 0.0f32);

        let (right_neighbour_ptr, _) = mid_element_ref.next.as_ref().unwrap();
        let right_neighbour = right_neighbour_ptr.upgrade().unwrap();
        assert_eq!(right_neighbour.read().unwrap().activation(), 0.0f32);
        let right_neighbour_ref =  right_neighbour.read().unwrap();

        let (right_right_neighbour_ptr, _) = right_neighbour_ref.next.as_ref().unwrap();
        let right_right_neighbour = right_right_neighbour_ptr.upgrade().unwrap();
        assert_eq!(right_right_neighbour.read().unwrap().activation(), 0.0f32);

        let second_element = graph.read().unwrap().search(&2).unwrap();
        assert_eq!(second_element.read().unwrap().activation(), 0.0f32);
        let eight_element = graph.read().unwrap().search(&8).unwrap();
        assert_eq!(eight_element.read().unwrap().activation(), 0.0f32);

        let element_min = graph.read().unwrap().element_min.as_ref().unwrap().clone();
        assert_eq!(element_min.read().unwrap().activation(), 0.0f32);

        let element_max = graph.read().unwrap().element_max.as_ref().unwrap().clone();
        assert_eq!(element_max.read().unwrap().activation(), 0.0f32);
    }

    #[test]
    fn simple_activate() {
        let graph = Arc::new(
            RwLock::new(ASAGraph::<i32, 3>::new(1))
        );
        for i in 1..=9 { graph.write().unwrap().insert(&i); }

        let mid_element = graph.read().unwrap().search(&5).unwrap();
        mid_element.write().unwrap().simple_activate(1.0f32);
        assert_eq!(mid_element.read().unwrap().activation(), 1.0f32);
        let mid_element_ref =  mid_element.read().unwrap();

        let (left_neighbour_ptr, left_neighbour_weight) = mid_element_ref.prev.as_ref().unwrap();
        let left_neighbour = left_neighbour_ptr.upgrade().unwrap();
        assert_eq!(*left_neighbour_weight, 0.875f32);
        assert_eq!(left_neighbour.read().unwrap().activation(), 0.0f32);
        let left_neighbour_ref =  left_neighbour.read().unwrap();

        let (left_left_neighbour_ptr, left_left_neighbour_weight) = left_neighbour_ref.prev.as_ref().unwrap();
        let left_left_neighbour = left_left_neighbour_ptr.upgrade().unwrap();
        assert_eq!(*left_left_neighbour_weight, 0.875f32);
        assert_eq!(left_left_neighbour.read().unwrap().activation(), 0.0f32);

        let (right_neighbour_ptr, right_neighbour_weight) = mid_element_ref.next.as_ref().unwrap();
        let right_neighbour = right_neighbour_ptr.upgrade().unwrap();
        assert_eq!(*right_neighbour_weight, 0.875f32);
        assert_eq!(right_neighbour.read().unwrap().activation(), 0.0f32);
        let right_neighbour_ref =  right_neighbour.read().unwrap();

        let (right_right_neighbour_ptr, right_right_neighbour_weight) = right_neighbour_ref.next.as_ref().unwrap();
        let right_right_neighbour = right_right_neighbour_ptr.upgrade().unwrap();
        assert_eq!(*right_right_neighbour_weight, 0.875f32);
        assert_eq!(right_right_neighbour.read().unwrap().activation(), 0.0f32);

        let second_element = graph.read().unwrap().search(&2).unwrap();
        assert_eq!(second_element.read().unwrap().activation(), 0.0f32);
        let eight_element = graph.read().unwrap().search(&8).unwrap();
        assert_eq!(eight_element.read().unwrap().activation(), 0.0f32);

        let element_min = graph.read().unwrap().element_min.as_ref().unwrap().clone();
        assert_eq!(element_min.read().unwrap().activation(), 0.0f32);

        let element_max = graph.read().unwrap().element_max.as_ref().unwrap().clone();
        assert_eq!(element_max.read().unwrap().activation(), 0.0f32);
    }

    #[test]
    fn connections_trait() {
        let element_1: Arc<RwLock<Element<i32, 3>>> = Element::new(&1, 1, 1);
        let element_2: Arc<RwLock<Element<i32, 3>>> = Element::new(&2, 2, 1);

        let ok = element_1.write().unwrap().connect_to(element_2.clone(), ConnectionKind::Defining);
        assert!(ok.is_err());
        assert_eq!(element_1.read().unwrap().defining_neurons().len(), 0);
    }
}