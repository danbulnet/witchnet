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

pub trait WeightingStrategy {
    fn common_weight(&self) -> f32;
}

pub trait CollectiveConnections: WeightingStrategy {
    fn add(&mut self, other: Rc<RefCell<dyn Neuron>>);

    fn connected_neurons(&self) -> &[Rc<RefCell<dyn Neuron>>];
}

pub trait CollectiveConnectionsAsync: WeightingStrategy {
    fn add(&mut self, other: Arc<RwLock<dyn NeuronAsync>>);

    fn connected_neurons(&self) -> &[Arc<RwLock<dyn NeuronAsync>>];
}