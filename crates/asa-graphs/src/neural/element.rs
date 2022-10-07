use std::{
    fmt::{ Display, Formatter, Result as FmtResult },
    rc::{ Rc, Weak },
    cell::RefCell,
    collections::HashMap,
    marker::PhantomData
};

use witchnet_common::{
    neuron::{ Neuron, NeuronConnect, NeuronID },
    connection::{ 
        Connection, 
        ConnectionKind,
        ConnectionID,
        defining_connection::DefiningConnection
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
    pub definitions: HashMap<ConnectionID, Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>>,
    pub(crate) data_type: PhantomData<Key>
}

impl<Key, const ORDER: usize> Element<Key, ORDER> 
where 
    Key: SensorData, [(); ORDER + 1]:, 
    PhantomData<Key>: DataDeductor, 
    DataTypeValue: From<Key> 
{
    pub const INTERELEMENT_ACTIVATION_THRESHOLD: f32 = 0.8;

    pub fn new(key: &Key, id: u32, parent_id: u32)
    -> Rc<RefCell<Element<Key, ORDER>>> {
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
                    definitions: HashMap::new(),
                    data_type: PhantomData
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

        let defining_neurons_len = self.defining_neurons().len();
        let mut neurons: Vec<(Rc<RefCell<dyn Neuron>>, f32)> = self
            .defining_neurons()
            .values()
            .map(|neuron| (neuron.clone(), self.activation() / defining_neurons_len as f32))
            .collect();

        let mut element_activation = self.activation;
        if let Some(next) = &self.next {
            let mut element = next.0.upgrade().unwrap();
            let mut weight = next.1;
            while element_activation > Self::INTERELEMENT_ACTIVATION_THRESHOLD {
                element.borrow_mut().activate(element_activation * weight, false, false);
                let defining_neurons_len = element.borrow().defining_neurons().len();
                neurons.append(
                    &mut element.borrow()
                        .defining_neurons()
                        .values()
                        .cloned()
                        .into_iter().map(
                            |neuron| (
                                neuron.clone(), 
                                element.borrow().activation() / defining_neurons_len as f32
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
            while element_activation > Self::INTERELEMENT_ACTIVATION_THRESHOLD {
                element.borrow_mut().activate(element_activation * weight, false, false);
                let defining_neurons_len = element.borrow().defining_neurons().len();
                neurons.append(
                    &mut element.borrow()
                        .defining_neurons()
                        .values()
                        .cloned()
                        .into_iter().map(
                            |neuron| (
                                neuron.clone(), 
                                element.borrow().activation() / defining_neurons_len as f32
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
        let defining_neurons_len = self.defining_neurons().len();
        self.defining_neurons()
            .values()
            .cloned()
            .into_iter().map(|x| (x.clone(), self.activation() / defining_neurons_len as f32))
            .collect()
    }

    pub fn defining_neurons(&self) -> HashMap<NeuronID, Rc<RefCell<dyn Neuron>>> {
        let mut neurons = HashMap::new();
        for (_id, definition) in &self.definitions {
            let neuron = definition.borrow().to();
            if !neuron.borrow().is_sensor() {
                neurons.insert(neuron.borrow().id(), neuron.clone());
            }
        }
        neurons
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
    
    fn explain(&self) -> HashMap<NeuronID, Rc<RefCell<dyn Neuron>>> { 
        HashMap::from(
            [(self.id(), self.self_ptr.upgrade().unwrap() as Rc<RefCell<dyn Neuron>>)]
        ) 
    }

    fn explain_one(&self, _parent: u32) -> Option<DataTypeValue> {
        Some((*dyn_clone::clone_box(&self.key)).into())
    }

    fn activate(
        &mut self, signal: f32, propagate_horizontal: bool, propagate_vertical: bool
    ) -> HashMap<NeuronID, Rc<RefCell<dyn Neuron>>> {
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

        let mut neurons: HashMap<NeuronID, Rc<RefCell<dyn Neuron>>> = HashMap::new();

        if propagate_vertical {
            for (neuron, activation) in &neurons_activation {
                neurons.insert(neuron.borrow().id(), neuron.clone());
                if !neuron.borrow().is_sensor() {
                    neurons.extend(
                        neuron.borrow_mut().activate(
                            *activation, propagate_horizontal, propagate_vertical
                        )
                    );
                }
            }
        }

        neurons
    }

    fn deactivate(&mut self, propagate_horizontal: bool, propagate_vertical: bool) {
        self.activation = 0.0f32;

        let mut neurons: Vec<Rc<RefCell<dyn Neuron>>> = Vec::new();
        if propagate_vertical {
            neurons = self.defining_neurons().values().cloned().collect();
        }

        if propagate_horizontal{
            if let Some(next) = &self.next {
                let mut element = next.0.upgrade().unwrap();
                loop {
                    element.borrow_mut().activation = 0.0f32;
                    if propagate_vertical {
                        neurons.append(
                            &mut element.borrow().defining_neurons().values().cloned().collect()
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
                            &mut element.borrow().defining_neurons().values().cloned().collect()
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
}

impl<Key, const ORDER: usize> NeuronConnect for Element<Key, ORDER> 
where 
    Key: SensorData, 
    [(); ORDER + 1]:, 
    PhantomData<Key>: DataDeductor,
    DataTypeValue: From<Key>
{
    fn connect_to(
        &mut self, to: Rc<RefCell<dyn Neuron>>, kind: ConnectionKind
    ) -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>, String> {
        match kind {
            ConnectionKind::Defining => {
                let connection = Rc::new(RefCell::new(DefiningConnection::new(
                    self.self_ptr.upgrade().unwrap() as Rc<RefCell<dyn Neuron>>, 
                    to.clone()
                )));
                let connection_id = ConnectionID { from: self.id(), to: to.borrow().id() };
                self.definitions.insert(connection_id, connection.clone());
                Ok(connection)
            },
            _ => {
                let msg = "only defining connection to element can be created for asa-graphs";
                log::error!("{}", msg);
                Err(msg.to_string())
            }
        }
    }

    fn connect_to_connection(
        &mut self, to_connection: Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>
    ) -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>, String> {
        match to_connection.borrow().kind() {
            ConnectionKind::Defining => {
                let to_neuron_ptr = to_connection.borrow().to().as_ptr();
                let to_neuorn_id = unsafe { (&*to_neuron_ptr).id() };
                let connection_id = ConnectionID { from: self.id(), to: to_neuorn_id };
                self.definitions.insert(connection_id, to_connection.clone());
                Ok(to_connection.clone())
            },
            _ => {
                let msg = "only defining connection to element can be created for asa-graphs";
                log::error!("{}", msg);
                Err(msg.to_string())
            }
        }
    }

    fn connect_from(
        &mut self, _from: Rc<RefCell<dyn Neuron>>, _kind: ConnectionKind
    ) -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>, String> {
        let msg = "only defining connection to neuron can be created for asa-graphs element";
        log::error!("{}", msg);
        Err(msg.to_string())
    }

    fn connect_from_connection(
        &mut self, _from_connection: Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>
    ) -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>, String> {
        let msg = "only defining connection to neuron can be created for asa-graphs element";
        log::error!("{}", msg);
        Err(msg.to_string())
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
        neuron::{ Neuron, NeuronConnect },
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
        let element_2_ptr: Rc<RefCell<Element<i32, 3>>> = Element::new(&2, 2, graph_id);

        let element_1_id = element_1_ptr.borrow().id();
        assert_eq!(element_1_id.id.to_string(), 1.to_string());
        assert_eq!(element_1_id.parent_id.to_string(), graph.borrow().id.to_string());
        let element_2_id = element_2_ptr.borrow().id();
        assert_eq!(element_2_id.id.to_string(),2.to_string());
        assert_eq!(element_2_id.parent_id.to_string(), graph.borrow().id.to_string());

        assert_eq!(element_1_ptr.borrow().is_sensor(), true);

        assert_eq!(element_1_ptr.borrow().activation(), 0.0f32);

        assert_eq!(element_1_ptr.borrow().counter(), 1usize);
        
        let connection = element_1_ptr
            .borrow_mut().connect_to(element_2_ptr.clone(), ConnectionKind::Defining).unwrap();
        
        assert_eq!(
            connection.borrow().from().as_ptr() as *const () as usize, 
            element_1_ptr.as_ptr() as *const () as usize
        );
        assert_eq!(
            connection.borrow().to().as_ptr() as *const () as usize, 
            element_2_ptr.as_ptr() as *const () as usize
        );

        let activated = element_1_ptr.borrow_mut().activate(1.0f32, true, true);
        assert_eq!(activated.len(), 0);
        assert_eq!(element_1_ptr.borrow().activation(), 1.0f32);
        assert_eq!(element_2_ptr.borrow().activation(), 0.0f32);

        element_1_ptr.borrow_mut().activate(1.0f32, false, true);
        assert_eq!(element_1_ptr.borrow().activation(), 2.0f32);
        element_1_ptr.borrow_mut().deactivate(true, true);
        assert_eq!(element_1_ptr.borrow().activation(), 0.0f32);
        assert_eq!(element_2_ptr.borrow().activation(), 0.0f32);

        element_1_ptr.borrow_mut().activate(1.0f32, true, false);
        assert_eq!(element_1_ptr.borrow().activation(), 1.0f32);
        element_1_ptr.borrow_mut().deactivate(false, true);
        assert_eq!(element_1_ptr.borrow().activation(), 0.0f32);
        assert_eq!(element_2_ptr.borrow().activation(), 0.0f32);

        element_1_ptr.borrow_mut().activate(1.0f32, false, false);
        assert_eq!(element_1_ptr.borrow().activation(), 1.0f32);
        element_1_ptr.borrow_mut().deactivate(true, false);
        assert_eq!(element_1_ptr.borrow().activation(), 0.0f32);
        assert_eq!(element_2_ptr.borrow().activation(), 0.0f32);

        element_1_ptr.borrow_mut().activate(1.0f32, false, false);
        assert_eq!(element_1_ptr.borrow().activation(), 1.0f32);
        element_1_ptr.borrow_mut().deactivate(false, false);
        assert_eq!(element_1_ptr.borrow().activation(), 0.0f32);

        let exp_1 = element_1_ptr.borrow_mut().explain();
        assert_eq!(exp_1.len(), 1);
        assert_eq!(exp_1.keys().into_iter().next().unwrap(), &element_1_ptr.borrow().id());
    }

    #[test]
    fn fuzzy_activate_deactivate() {
        assert_eq!(Element::<i32, 3>::INTERELEMENT_ACTIVATION_THRESHOLD, 0.8f32);

        let graph = Rc::new(
            RefCell::new(ASAGraph::<i32, 3>::new(1))
        );
        for i in 1..=9 { graph.borrow_mut().insert(&i); }
        {
            let mid_element = graph.borrow().search(&5).unwrap();
            mid_element.borrow_mut().fuzzy_activate(1.0f32);
            assert_eq!(mid_element.borrow().activation(), 1.0f32);
            let mid_element_ref =  mid_element.borrow();

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

        let er = element_1.borrow_mut().connect_from(element_2.clone(), ConnectionKind::Defining);
        assert!(er.is_err());

        let ok = element_1.borrow_mut().connect_to(element_2.clone(), ConnectionKind::Defining);
        assert!(ok.is_ok());
        assert_eq!(element_1.borrow().defining_neurons().len(), 0);
        let connection = ok.unwrap();
        assert!(Rc::ptr_eq(&connection.borrow().to(), &(element_2 as Rc<RefCell<dyn Neuron>>)));

        let er = element_1.borrow_mut().connect_to_connection(connection);
        assert!(er.is_ok());

        assert_eq!(element_1.borrow().defining_neurons().len(), 0);
    }
}