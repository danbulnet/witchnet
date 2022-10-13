use std::{
    rc::Rc,
    cell::RefCell
};

use crate::{
    connection::collective::CollectiveConnections,
    neuron::Neuron
};

#[derive(Debug, Clone)]
pub struct DefiningConnections {
    pub connections: Vec<Rc<RefCell<dyn Neuron>>>
}

impl CollectiveConnections for DefiningConnections {  
    fn add(&mut self, other: Rc<RefCell<dyn Neuron>>) {
        self.connections.push(other);
    }

    // fn common_weight(&self) -> f32 { 1.0f32 / f32::max(self.connections.len() as f32, 1.0f32) }
    // fn common_weight(&self) -> f32 { 
    //     0.5f32 + (1.0f32 / f32::max(self.connections.len() as f32, 1.0f32) / 2f32)
    // }
    fn common_weight(&self) -> f32 { 1.0f32 }
    
    fn connected_neurons(&self) -> &[Rc<RefCell<dyn Neuron>>] { &self.connections }
}

impl DefiningConnections {
    pub fn new() -> DefiningConnections { 
        DefiningConnections { connections: Vec::new() } 
    }

    pub fn output_signal(&self, neuron: Rc<RefCell<dyn Neuron>>) -> f32 { 
        neuron.borrow().activation() * self.common_weight()
    }
}