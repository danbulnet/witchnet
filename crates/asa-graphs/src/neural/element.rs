use std::{
    fmt::{ Display, Formatter, Result as FmtResult },
    rc::{ Rc, Weak },
    cell::RefCell,
    marker::PhantomData,
};

use anyhow::Result;

use witchnet_common::{
    neuron::{ Neuron, NeuronID },
    connection::{
        ConnectionKind,
        collective::{
            CollectiveConnections,
            WeightingStrategy,
            defining::{
                DefiningConnections,
                ConstantOneWeight, 
                DefiningWeightingStrategy
            }
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
    pub(crate) self_ptr: Weak<RefCell<Element<Key, ORDER>>>,
    pub next: Option<(Weak<RefCell<Element<Key, ORDER>>>, f32)>,
    pub prev: Option<(Weak<RefCell<Element<Key, ORDER>>>, f32)>,
    pub definitions: DefiningConnections,
    pub(crate) data_type: PhantomData<Key>,
    pub interelement_activation_threshold: f32,
    pub interelement_activation_exponent: i32
}

impl<Key, const ORDER: usize> Element<Key, ORDER> 
where 
    Key: SensorData, [(); ORDER + 1]:, 
    PhantomData<Key>: DataDeductor, 
    DataTypeValue: From<Key> 
{
    pub fn new(key: &Key, id: u32, parent_id: u32)
    -> Rc<RefCell<Element<Key, ORDER>>> {
        let weighting_strategy = Rc::new(ConstantOneWeight);
        let element_ptr = Rc::new(
            RefCell::new(
                Element {
                    id,
                    parent_id,
                    key: *dyn_clone::clone_box(key),
                    counter: 1,
                    activation: 0.0f32,
                    self_ptr: Weak::new(),
                    next: None,
                    prev: None,
                    definitions: DefiningConnections::new(weighting_strategy),
                    data_type: PhantomData,
                    interelement_activation_threshold: 1.0,
                    interelement_activation_exponent: 1
                }
            )
        );
        element_ptr.borrow_mut().self_ptr = Rc::downgrade(&element_ptr);
        element_ptr
    }
    
    pub fn new_custom(
        key: &Key, 
        id: u32, 
        parent_id: u32,
        weighting_strategy: Rc<dyn DefiningWeightingStrategy>,
        interelement_activation_threshold: f32,
        interelement_activation_exponent: i32
    ) -> Rc<RefCell<Element<Key, ORDER>>> {
        let element_ptr = Rc::new(
            RefCell::new(
                Element {
                    id,
                    parent_id,
                    key: *dyn_clone::clone_box(key),
                    counter: 1,
                    activation: 0.0f32,
                    self_ptr: Weak::new(),
                    next: None,
                    prev: None,
                    definitions: DefiningConnections::new(weighting_strategy),
                    data_type: PhantomData,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                }
            )
        );
        element_ptr.borrow_mut().self_ptr = Rc::downgrade(&element_ptr);
        element_ptr
    }

    pub(crate) fn set_connections(
        element_ptr: &Rc<RefCell<Element<Key, ORDER>>>,
        prev_opt: Option<&Rc<RefCell<Element<Key, ORDER>>>>,
        next_opt: Option<&Rc<RefCell<Element<Key, ORDER>>>>,
        range: f32
    ) {
        let mut element = element_ptr.borrow_mut();
        
        if prev_opt.is_some() {
            let prev_ptr = prev_opt.unwrap();
            let weight = (&*element).weight(&*prev_ptr.borrow(), range);
            element.prev = Some((Rc::downgrade(prev_ptr), weight));
            prev_ptr.borrow_mut().next = Some((Rc::downgrade(element_ptr), weight));
        } else { 
            element.prev = None; 
        }

        if next_opt.is_some() {
            let next_ptr = next_opt.unwrap();
            let weight = (&*element).weight(&*next_ptr.borrow(), range);
            element.next = Some((Rc::downgrade(next_ptr), weight));
            next_ptr.borrow_mut().prev = Some((Rc::downgrade(&element_ptr), weight));
        } else { 
            element.next = None; 
        }
    }

    pub fn weight(&self, other: &Self, range: f32) -> f32 {
        1.0f32 - (other.key.distance(&self.key) as f32).abs() / range
    }

    pub fn fuzzy_activate(&mut self, signal: f32) -> Vec<(Rc<RefCell<dyn Neuron>>, f32)> {
        self.activation += signal;

        let mut neurons: Vec<(Rc<RefCell<dyn Neuron>>, f32)> = self
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
            while element_activation > self.interelement_activation_threshold {
                element.borrow_mut().activate(
                    element_activation * weight.powi(self.interelement_activation_exponent), 
                    false, 
                    false
                );
                neurons.append(
                    &mut element.borrow()
                        .defining_neurons()
                        .into_iter()
                        .cloned()
                        .into_iter().map(
                            |neuron| (
                                neuron,
                                element.borrow().definitions.output_signal(element.clone())
                            )
                        )
                        .collect()
                );

                let new_element = match &element.borrow().next {
                    Some(next) => {
                        weight = next.1;
                        next.0.upgrade().unwrap()
                    },
                    None => break
                };
                element_activation = element.borrow().activation;
                element = new_element;
            }
        }
        
        element_activation = self.activation;
        if let Some(prev) = &self.prev {
            let mut element = prev.0.upgrade().unwrap();
            let mut weight = prev.1;
            while element_activation > self.interelement_activation_threshold {
                element.borrow_mut().activate(
                    element_activation * weight.powi(self.interelement_activation_exponent), 
                    false, 
                    false
                );
                neurons.append(
                    &mut element.borrow()
                        .defining_neurons()
                        .into_iter()
                        .cloned()
                        .into_iter().map(
                            |neuron| (
                                neuron, 
                                element.borrow().definitions.output_signal(element.clone())
                            )
                        )
                        .collect()
                );

                let new_element = match &element.borrow().prev {
                    Some(prev) => {
                        weight = prev.1;
                        prev.0.upgrade().unwrap()
                    },
                    None => break
                };
                element_activation = element.borrow().activation;
                element = new_element;
            }
        }

        neurons
    }

    
    pub(crate) fn simple_activate(
        &mut self, signal: f32
    )-> Vec<(Rc<RefCell<dyn Neuron>>, f32)> {
        self.activation += signal;
        self.defining_neurons().into_iter()
            .cloned()
            .into_iter().map(|x| (
                x.clone(), 
                self.activation() * self.definitions.common_weight()
            ))
            .collect()
    }

    pub fn defining_neurons(&self) -> &[Rc<RefCell<dyn Neuron>>] {
        self.definitions.connected_neurons()
    }
}

impl<Key, const ORDER: usize> Neuron for Element<Key, ORDER> 
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
    
    fn explain(&self) -> &[Rc<RefCell<dyn Neuron>>] { &[] }

    fn explain_one(&self, _parent: u32) -> Option<DataTypeValue> {
        Some((*dyn_clone::clone_box(&self.key)).into())
    }

    fn defined_neurons(&self) -> &[Rc<RefCell<dyn Neuron>>] {
        &self.definitions.connected_neurons()
    }

    fn activate(
        &mut self, signal: f32, propagate_horizontal: bool, propagate_vertical: bool
    ) -> f32 {
        let data_category: DataCategory = self.data_type.data_category();
        let is_fuzzy_ok = match data_category {
            DataCategory::Continuous | DataCategory::Ordinal => true,
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
                neuron.borrow_mut().activate(
                    *activation, propagate_horizontal, propagate_vertical
                );
            }
        }

        max_activation
    }

    fn deactivate(&mut self, propagate_horizontal: bool, propagate_vertical: bool) {
        self.activation = 0.0f32;

        let mut neurons: Vec<Rc<RefCell<dyn Neuron>>> = Vec::new();
        if propagate_vertical {
            neurons = self.defining_neurons().into_iter().cloned().collect();
        }

        if propagate_horizontal{
            if let Some(next) = &self.next {
                let mut element = next.0.upgrade().unwrap();
                loop {
                    element.borrow_mut().activation = 0.0f32;
                    if propagate_vertical {
                        neurons.append(
                            &mut element.borrow().defining_neurons().into_iter().cloned().collect()
                        );
                    }
                    let new_element = match &element.borrow().next {
                        Some(next) => next.0.upgrade().unwrap(),
                        None => break
                    };
                    element = new_element;
                }
            }
            
            if let Some(prev) = &self.prev {
                let mut element = prev.0.upgrade().unwrap();
                loop {
                    element.borrow_mut().activation = 0.0f32;
                    if propagate_vertical {
                        neurons.append(
                            &mut element.borrow().defining_neurons().into_iter().cloned().collect()
                        );
                    };
                    let new_element = match &element.borrow().prev {
                        Some(prev) => prev.0.upgrade().unwrap(),
                        None => break
                    };
                    element = new_element;
                }
            }
        }
        
        if propagate_vertical {
            for neuron in neurons {
                neuron.borrow_mut().deactivate(propagate_horizontal, propagate_vertical)
            }
        }
    }

    fn connect_to(
        &mut self, to: Rc<RefCell<dyn Neuron>>, is_to_sensor: bool, kind: ConnectionKind
    ) -> Result<()> {
        match kind {
            ConnectionKind::Defining => {
                if is_to_sensor {
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

    fn connect_bilateral(
        &mut self, to: Rc<RefCell<dyn Neuron>>, is_to_sensor: bool, kind: ConnectionKind
    ) -> Result<()> {
        match kind {
            ConnectionKind::Defining => {
                if !is_to_sensor {
                    self.connect_to(to.clone(), is_to_sensor, kind)?;
                    to.borrow_mut().connect_to(
                        self.self_ptr.upgrade().unwrap(), true, ConnectionKind::Explanatory
                    )?;
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
    use std::{
        rc::Rc,
        cell::RefCell
    };

    use witchnet_common::{
        neuron::Neuron,
        connection::ConnectionKind
    };

    use super::super::{
        element::Element,
        graph::ASAGraph
    };

    #[test]
    fn set_connections() {
        let graph = Rc::new(
            RefCell::new(ASAGraph::<i32, 3>::new(1))
        );
        let graph_id = graph.borrow().id;

        let element_1_ptr: Rc<RefCell<Element<i32, 3>>> = Element::new(&1, 1, graph_id);
        let element_2_ptr: Rc<RefCell<Element<i32, 3>>> = Element::new(&2, 2, graph_id);
        let element_3_ptr: Rc<RefCell<Element<i32, 3>>> = Element::new(&3, 3, graph_id);

        assert!(element_1_ptr.borrow().prev.is_none());
        assert!(element_1_ptr.borrow().next.is_none());
        assert!(element_2_ptr.borrow().prev.is_none());
        assert!(element_2_ptr.borrow().next.is_none());
        assert!(element_3_ptr.borrow().prev.is_none());
        assert!(element_3_ptr.borrow().next.is_none());
        
        Element::set_connections(&element_2_ptr, Some(&element_1_ptr), None, 2f32);

        assert!(element_1_ptr.borrow().prev.is_none());
        assert_eq!(
            element_1_ptr.borrow().next.as_ref().unwrap().0.upgrade().unwrap().borrow().key,
            element_2_ptr.borrow().key
        );
        assert!(element_2_ptr.borrow().next.is_none());
        assert!(element_3_ptr.borrow().prev.is_none());
        assert!(element_3_ptr.borrow().next.is_none());

        Element::set_connections(&element_2_ptr, None, Some(&element_3_ptr), 2f32);

        assert!(element_1_ptr.borrow().prev.is_none());
        assert_eq!(
            element_1_ptr.borrow().next.as_ref().unwrap().0.upgrade().unwrap().borrow().key,
            element_2_ptr.borrow().key
        );
        assert!(element_2_ptr.borrow().prev.is_none());
        assert_eq!(
            element_2_ptr.borrow().next.as_ref().unwrap().0.upgrade().unwrap().borrow().key,
            element_3_ptr.borrow().key
        );
        assert_eq!(
            element_3_ptr.borrow().prev.as_ref().unwrap().0.upgrade().unwrap().borrow().key, 
            element_2_ptr.borrow().key
        );
        assert!(element_3_ptr.borrow().next.is_none());

        Element::set_connections(&element_1_ptr, None, None, 2f32);
        Element::set_connections(&element_2_ptr, None, None, 2f32);
        Element::set_connections(&element_3_ptr, None, None, 2f32);

        assert!(element_1_ptr.borrow().prev.is_none());
        assert!(element_1_ptr.borrow().next.is_none());
        assert!(element_2_ptr.borrow().prev.is_none());
        assert!(element_2_ptr.borrow().next.is_none());
        assert!(element_3_ptr.borrow().prev.is_none());
        assert!(element_3_ptr.borrow().next.is_none());
    }

    #[test]
    fn parent_id() {
        let graph = Rc::new(RefCell::new(ASAGraph::<i32, 3>::new(1)));
        let graph_id = graph.borrow().id;

        let element_1_ptr: Rc<RefCell<Element<i32, 3>>> = Element::new(&1, 1, graph_id);
        let id = element_1_ptr.borrow().id;
        let parent_id = element_1_ptr.borrow().parent_id;
        assert_eq!(id, 1);
        assert_eq!(parent_id, 1);
    }

    #[test]
    fn as_neuron() {
        let graph = Rc::new(RefCell::new(ASAGraph::<i32, 3>::new(1)));
        let graph_id = graph.borrow().id;

        let element_1_ptr: Rc<RefCell<Element<i32, 3>>> = Element::new(&1, 1, graph_id);
        let mut element_1 = element_1_ptr.borrow_mut();
        let element_2_ptr: Rc<RefCell<Element<i32, 3>>> = Element::new(&2, 2, graph_id);

        let element_1_id = element_1.id();
        assert_eq!(element_1_id.id.to_string(), 1.to_string());
        assert_eq!(element_1_id.parent_id.to_string(), graph.borrow().id.to_string());
        let element_2_id = element_2_ptr.borrow().id();
        assert_eq!(element_2_id.id.to_string(),2.to_string());
        assert_eq!(element_2_id.parent_id.to_string(), graph.borrow().id.to_string());

        assert_eq!(element_1.is_sensor(), true);

        assert_eq!(element_1.activation(), 0.0f32);

        assert_eq!(element_1.counter(), 1usize);

        let activated = element_1.activate(1.0f32, true, true);
        assert_eq!(activated, 0.0f32);
        assert_eq!(element_1.activation(), 1.0f32);
        assert_eq!(element_2_ptr.borrow().activation(), 0.0f32);

        element_1.activate(1.0f32, false, true);
        assert_eq!(element_1.activation(), 2.0f32);
        element_1.deactivate(true, true);
        assert_eq!(element_1.activation(), 0.0f32);
        assert_eq!(element_2_ptr.borrow().activation(), 0.0f32);

        element_1.activate(1.0f32, true, false);
        assert_eq!(element_1.activation(), 1.0f32);
        element_1.deactivate(false, true);
        assert_eq!(element_1.activation(), 0.0f32);
        assert_eq!(element_2_ptr.borrow().activation(), 0.0f32);

        element_1.activate(1.0f32, false, false);
        assert_eq!(element_1.activation(), 1.0f32);
        element_1.deactivate(true, false);
        assert_eq!(element_1.activation(), 0.0f32);
        assert_eq!(element_2_ptr.borrow().activation(), 0.0f32);

        element_1.activate(1.0f32, false, false);
        assert_eq!(element_1.activation(), 1.0f32);
        element_1.deactivate(false, false);
        assert_eq!(element_1.activation(), 0.0f32);

        let exp_1 = element_1.explain();
        assert_eq!(exp_1.len(), 0);
    }

    #[test]
    fn fuzzy_activate_deactivate() {
        let threshold = Element::<i32, 3>::new(&1, 0, 0).borrow().interelement_activation_threshold;

        let graph = Rc::new(
            RefCell::new(ASAGraph::<i32, 3>::new(1))
        );
        for i in 1..=9 { graph.borrow_mut().insert(&i); }
        {
            let mid_element = graph.borrow().search(&5).unwrap();
            mid_element.borrow_mut().fuzzy_activate(1.0f32);
            assert_eq!(mid_element.borrow().activation(), 1.0f32);
            let mid_element_ref =  mid_element.borrow();

            if threshold == 0.8f32 {
                let (left_neighbour_ptr, left_neighbour_weight) = mid_element_ref.prev.as_ref().unwrap();
                let left_neighbour = left_neighbour_ptr.upgrade().unwrap();
                assert_eq!(*left_neighbour_weight, 0.875f32);
                assert_eq!(left_neighbour.borrow().activation(), 0.875f32);
                let left_neighbour_ref =  left_neighbour.borrow();
    
                let (left_left_neighbour_ptr, left_left_neighbour_weight) = left_neighbour_ref.prev.as_ref().unwrap();
                let left_left_neighbour = left_left_neighbour_ptr.upgrade().unwrap();
                assert_eq!(*left_left_neighbour_weight, 0.875f32);
                assert_eq!(left_left_neighbour.borrow().activation(), 0.765625f32);
    
                let (right_neighbour_ptr, right_neighbour_weight) = mid_element_ref.next.as_ref().unwrap();
                let right_neighbour = right_neighbour_ptr.upgrade().unwrap();
                assert_eq!(*right_neighbour_weight, 0.875f32);
                assert_eq!(right_neighbour.borrow().activation(), 0.875f32);
                let right_neighbour_ref =  right_neighbour.borrow();
    
                let (right_right_neighbour_ptr, right_right_neighbour_weight) = right_neighbour_ref.next.as_ref().unwrap();
                let right_right_neighbour = right_right_neighbour_ptr.upgrade().unwrap();
                assert_eq!(*right_right_neighbour_weight, 0.875f32);
                assert_eq!(right_right_neighbour.borrow().activation(), 0.765625f32);
    
                let second_element = graph.borrow().search(&2).unwrap();
                assert_eq!(second_element.borrow().activation(), 0.0f32);
                let eight_element = graph.borrow().search(&8).unwrap();
                assert_eq!(eight_element.borrow().activation(), 0.0f32);
    
                let element_min = graph.borrow().element_min.as_ref().unwrap().clone();
                assert_eq!(element_min.borrow().activation(), 0.0f32);
    
                let element_max = graph.borrow().element_max.as_ref().unwrap().clone();
                assert_eq!(element_max.borrow().activation(), 0.0f32);
            }
        }

        let mid_element = graph.borrow().search(&5).unwrap();
        mid_element.borrow_mut().deactivate(true, true);
        assert_eq!(mid_element.borrow().activation(), 0.0f32);
        let mid_element_ref =  mid_element.borrow();
        
        let (left_neighbour_ptr, _) = mid_element_ref.prev.as_ref().unwrap();
        let left_neighbour = left_neighbour_ptr.upgrade().unwrap();
        assert_eq!(left_neighbour.borrow().activation(), 0.0f32);
        let left_neighbour_ref =  left_neighbour.borrow();

        let (left_left_neighbour_ptr, _) = left_neighbour_ref.prev.as_ref().unwrap();
        let left_left_neighbour = left_left_neighbour_ptr.upgrade().unwrap();
        assert_eq!(left_left_neighbour.borrow().activation(), 0.0f32);

        let (right_neighbour_ptr, _) = mid_element_ref.next.as_ref().unwrap();
        let right_neighbour = right_neighbour_ptr.upgrade().unwrap();
        assert_eq!(right_neighbour.borrow().activation(), 0.0f32);
        let right_neighbour_ref =  right_neighbour.borrow();

        let (right_right_neighbour_ptr, _) = right_neighbour_ref.next.as_ref().unwrap();
        let right_right_neighbour = right_right_neighbour_ptr.upgrade().unwrap();
        assert_eq!(right_right_neighbour.borrow().activation(), 0.0f32);

        let second_element = graph.borrow().search(&2).unwrap();
        assert_eq!(second_element.borrow().activation(), 0.0f32);
        let eight_element = graph.borrow().search(&8).unwrap();
        assert_eq!(eight_element.borrow().activation(), 0.0f32);

        let element_min = graph.borrow().element_min.as_ref().unwrap().clone();
        assert_eq!(element_min.borrow().activation(), 0.0f32);

        let element_max = graph.borrow().element_max.as_ref().unwrap().clone();
        assert_eq!(element_max.borrow().activation(), 0.0f32);
    }

    #[test]
    fn simple_activate() {
        let graph = Rc::new(
            RefCell::new(ASAGraph::<i32, 3>::new(1))
        );
        for i in 1..=9 { graph.borrow_mut().insert(&i); }

        let mid_element = graph.borrow().search(&5).unwrap();
        mid_element.borrow_mut().simple_activate(1.0f32);
        assert_eq!(mid_element.borrow().activation(), 1.0f32);
        let mid_element_ref =  mid_element.borrow();

        let (left_neighbour_ptr, left_neighbour_weight) = mid_element_ref.prev.as_ref().unwrap();
        let left_neighbour = left_neighbour_ptr.upgrade().unwrap();
        assert_eq!(*left_neighbour_weight, 0.875f32);
        assert_eq!(left_neighbour.borrow().activation(), 0.0f32);
        let left_neighbour_ref =  left_neighbour.borrow();

        let (left_left_neighbour_ptr, left_left_neighbour_weight) = left_neighbour_ref.prev.as_ref().unwrap();
        let left_left_neighbour = left_left_neighbour_ptr.upgrade().unwrap();
        assert_eq!(*left_left_neighbour_weight, 0.875f32);
        assert_eq!(left_left_neighbour.borrow().activation(), 0.0f32);

        let (right_neighbour_ptr, right_neighbour_weight) = mid_element_ref.next.as_ref().unwrap();
        let right_neighbour = right_neighbour_ptr.upgrade().unwrap();
        assert_eq!(*right_neighbour_weight, 0.875f32);
        assert_eq!(right_neighbour.borrow().activation(), 0.0f32);
        let right_neighbour_ref =  right_neighbour.borrow();

        let (right_right_neighbour_ptr, right_right_neighbour_weight) = right_neighbour_ref.next.as_ref().unwrap();
        let right_right_neighbour = right_right_neighbour_ptr.upgrade().unwrap();
        assert_eq!(*right_right_neighbour_weight, 0.875f32);
        assert_eq!(right_right_neighbour.borrow().activation(), 0.0f32);

        let second_element = graph.borrow().search(&2).unwrap();
        assert_eq!(second_element.borrow().activation(), 0.0f32);
        let eight_element = graph.borrow().search(&8).unwrap();
        assert_eq!(eight_element.borrow().activation(), 0.0f32);

        let element_min = graph.borrow().element_min.as_ref().unwrap().clone();
        assert_eq!(element_min.borrow().activation(), 0.0f32);

        let element_max = graph.borrow().element_max.as_ref().unwrap().clone();
        assert_eq!(element_max.borrow().activation(), 0.0f32);
    }

    #[test]
    fn connections_trait() {
        let element_1: Rc<RefCell<Element<i32, 3>>> = Element::new(&1, 1, 1);
        let element_2: Rc<RefCell<Element<i32, 3>>> = Element::new(&2, 2, 1);

        let ok = element_1.borrow_mut().connect_to(
            element_2.clone(), true, ConnectionKind::Defining
        );
        assert!(ok.is_err());
        assert_eq!(element_1.borrow().defining_neurons().len(), 0);
    }
}