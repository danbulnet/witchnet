use std::{
    rc::Rc,
    cell::RefCell
};

use crate::{
    connection::collective::CollectiveConnections,
    neuron::Neuron
};

#[derive(Debug, Clone)]
pub struct ExplanatoryConnections {
    pub connections: Vec<Rc<RefCell<dyn Neuron>>>
}

impl CollectiveConnections for ExplanatoryConnections {  
    fn add(&mut self, other: Rc<RefCell<dyn Neuron>>) {
        self.connections.push(other);
    }

    fn common_weight(&self) -> f32 { 1.0f32 }
    
    fn connected_neurons(&self) -> &[Rc<RefCell<dyn Neuron>>] { &self.connections }
}

impl ExplanatoryConnections {
    pub fn new() -> ExplanatoryConnections { 
        ExplanatoryConnections { connections: Vec::new() } 
    }

    pub fn output_signal(&self, neuron: Rc<RefCell<dyn Neuron>>) -> f32 { 
        neuron.borrow().activation()
    }
}