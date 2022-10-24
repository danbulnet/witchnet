use std::{
    rc::Rc,
    cell::RefCell,
    sync::{ Arc, RwLock }
};

use crate::neuron::{ Neuron, NeuronAsync };

#[derive(Debug, Clone)]
pub struct SequentialConnections {
    pub connections: Vec<Rc<RefCell<dyn Neuron>>>
}

#[derive(Debug, Clone)]
pub struct SequentialConnectionsAsync {
    pub connections: Vec<Arc<RwLock<dyn NeuronAsync>>>
}