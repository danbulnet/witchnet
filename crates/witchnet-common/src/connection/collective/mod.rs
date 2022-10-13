pub mod defining;
pub mod explanatory;
pub mod similarity;
pub mod inhibitory;
pub mod sequential;

use std::{
    rc::Rc,
    cell::RefCell
};

use crate::neuron::Neuron;

pub trait CollectiveConnections {
    fn add(&mut self, other: Rc<RefCell<dyn Neuron>>);

    fn common_weight(&self) -> f32;

    fn connected_neurons(&self) -> &[Rc<RefCell<dyn Neuron>>];
}