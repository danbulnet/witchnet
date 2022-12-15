use std::{
    rc::Rc,
    cell::RefCell,
    sync::{ Arc, RwLock }
};

use crate::{
    connection::collective::{ 
        CollectiveConnections,
        CollectiveConnectionsAsync,
        WeightingStrategy
    },
    neuron::{ Neuron, NeuronAsync }
};

#[derive(Debug, Clone)]
pub struct ExplanatoryConnections {
    pub connections: Vec<Rc<RefCell<dyn Neuron>>>
}

impl CollectiveConnections for ExplanatoryConnections {  
    fn add(&mut self, other: Rc<RefCell<dyn Neuron>>) {
        self.connections.push(other);
    }
    
    fn connected_neurons(&self) -> &[Rc<RefCell<dyn Neuron>>] { &self.connections }
}

impl WeightingStrategy for ExplanatoryConnections {  
    fn common_weight(&self) -> f32 { 1.0f32 }
}

impl ExplanatoryConnections {
    pub fn new() -> ExplanatoryConnections { 
        ExplanatoryConnections { connections: Vec::new() } 
    }

    pub fn output_signal(&self, neuron: Rc<RefCell<dyn Neuron>>) -> f32 { 
        neuron.borrow().activation()
    }
}


#[derive(Debug, Clone)]
pub struct ExplanatoryConnectionsAsync {
    pub connections: Vec<Arc<RwLock<dyn NeuronAsync>>>
}

impl CollectiveConnectionsAsync for ExplanatoryConnectionsAsync {  
    fn add(&mut self, other: Arc<RwLock<dyn NeuronAsync>>) {
        self.connections.push(other);
    }
    
    fn connected_neurons(&self) -> &[Arc<RwLock<dyn NeuronAsync>>] { &self.connections }
}

impl WeightingStrategy for ExplanatoryConnectionsAsync {  
    fn common_weight(&self) -> f32 { 1.0f32 }
}

impl ExplanatoryConnectionsAsync {
    pub fn new() -> ExplanatoryConnectionsAsync { 
        ExplanatoryConnectionsAsync { connections: Vec::new() } 
    }

    pub fn output_signal(&self, neuron: Arc<RwLock<dyn NeuronAsync>>) -> f32 { 
        neuron.read().unwrap().activation()
    }
}