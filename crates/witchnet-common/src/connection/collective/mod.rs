pub mod defining;
pub mod explanatory;
pub mod similarity;
pub mod inhibitory;
pub mod sequential;

use std::{
    rc::Rc,
    cell::RefCell,
    sync::{ Arc, RwLock }
};

use crate::neuron::{ Neuron, NeuronAsync };

pub trait CollectiveConnections {
    fn add(&mut self, other: Rc<RefCell<dyn Neuron>>);

    fn common_weight(&self) -> f32;

    fn connected_neurons(&self) -> &[Rc<RefCell<dyn Neuron>>];
}

pub trait CollectiveConnectionsAsync {
    fn add(&mut self, other: Arc<RwLock<dyn NeuronAsync>>);

    fn common_weight(&self) -> f32;

    fn connected_neurons(&self) -> &[Arc<RwLock<dyn NeuronAsync>>];
}