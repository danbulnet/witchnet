use std::{
    rc::Rc,
    cell::RefCell
};

use crate::neuron::Neuron;

#[derive(Debug, Clone)]
pub struct SimilarityConnections {
    pub connections: Vec<Rc<RefCell<dyn Neuron>>>
}
