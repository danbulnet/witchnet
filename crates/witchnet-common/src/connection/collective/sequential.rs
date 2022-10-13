use std::{
    rc::Rc,
    cell::RefCell
};

use crate::neuron::Neuron;

#[derive(Debug, Clone)]
pub struct SequentialConnections {
    pub connections: Vec<Rc<RefCell<dyn Neuron>>>
}
