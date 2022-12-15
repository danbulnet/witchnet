use std::{
    rc::Rc,
    cell::RefCell,
    sync::{ Arc, RwLock },
    fmt::{ Debug, Formatter, Result as FmtResult }
};

use crate::{
    connection::collective::{ 
        CollectiveConnections, 
        CollectiveConnectionsAsync,
        WeightingStrategy
},
    neuron::{ Neuron, NeuronAsync }
};

pub trait DefiningWeightingStrategy {
    fn weight(&self, defining_connections: &DefiningConnections) -> f32;
}

impl Debug for dyn DefiningWeightingStrategy {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { write!(f, "strategy {:?}", self) }
}

pub struct ConstantZeroWeight;

impl DefiningWeightingStrategy for ConstantZeroWeight {
    fn weight(&self, _defining_connections: &DefiningConnections) -> f32 { 0.0 }
}

impl Debug for ConstantZeroWeight {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { write!(f, "const(0.0)") }
}

pub struct ConstantOneWeight;

impl DefiningWeightingStrategy for ConstantOneWeight {
    fn weight(&self, _defining_connections: &DefiningConnections) -> f32 { 1.0 }
}

impl Debug for ConstantOneWeight {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { write!(f, "const(1.0)") }
}

pub struct OneOverOuts;

impl DefiningWeightingStrategy for OneOverOuts {
    fn weight(&self, defining_connections: &DefiningConnections) -> f32 {
        1.0f32 / f32::max(defining_connections.connections.len() as f32, 1.0f32)
    }
}

impl Debug for OneOverOuts {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { write!(f, "one_over_outs") }
}

pub struct OneOverOutsUpperHalf;

impl DefiningWeightingStrategy for OneOverOutsUpperHalf {
    fn weight(&self, defining_connections: &DefiningConnections) -> f32 {
        0.5f32 + (1.0f32 / f32::max(defining_connections.connections.len() as f32, 1.0f32) / 2f32)
    }
}

impl Debug for OneOverOutsUpperHalf {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { write!(f, "one_over_outs_upper_half") }
}

pub struct OneOverOutsUpperQuarter;

impl DefiningWeightingStrategy for OneOverOutsUpperQuarter {
    fn weight(&self, defining_connections: &DefiningConnections) -> f32 {
        0.75f32 + (1.0f32 / f32::max(defining_connections.connections.len() as f32, 1.0f32) / 4f32)
    }
}

impl Debug for OneOverOutsUpperQuarter {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { write!(f, "one_over_outs_upper_half") }
}

#[derive(Debug, Clone)]
pub struct DefiningConnections {
    pub connections: Vec<Rc<RefCell<dyn Neuron>>>,
    pub weighting_strategy: Rc<dyn DefiningWeightingStrategy>
}

impl CollectiveConnections for DefiningConnections {  
    fn add(&mut self, other: Rc<RefCell<dyn Neuron>>) {
        self.connections.push(other);
    }

    fn connected_neurons(&self) -> &[Rc<RefCell<dyn Neuron>>] { &self.connections }
}

impl WeightingStrategy for DefiningConnections {  
    fn common_weight(&self) -> f32 { self.weighting_strategy.weight(&self) }
}

impl DefiningConnections {
    pub fn new(weighting_strategy: Rc<dyn DefiningWeightingStrategy>) -> DefiningConnections { 
        DefiningConnections { connections: Vec::new(), weighting_strategy }
    }

    pub fn output_signal(&self, neuron: Rc<RefCell<dyn Neuron>>) -> f32 {
        neuron.borrow().activation() * self.common_weight()
    }
}

////////
// Async
////////

pub trait DefiningWeightingStrategyAsync: Sync + Send {
    fn weight(&self, defining_connections: &DefiningConnectionsAsync) -> f32;
}

impl Debug for dyn DefiningWeightingStrategyAsync {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { write!(f, "strategy {:?}", self) }
}

pub struct ConstantZeroWeightAsync;

impl DefiningWeightingStrategyAsync for ConstantZeroWeightAsync {
    fn weight(&self, _defining_connections: &DefiningConnectionsAsync) -> f32 { 0.0 }
}

impl Debug for ConstantZeroWeightAsync {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { write!(f, "const(0.0)") }
}

pub struct ConstantOneWeightAsync;

impl DefiningWeightingStrategyAsync for ConstantOneWeightAsync {
    fn weight(&self, _defining_connections: &DefiningConnectionsAsync) -> f32 { 1.0 }
}

impl Debug for ConstantOneWeightAsync {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { write!(f, "const(1.0)") }
}

pub struct OneOverOutsAsync;

impl DefiningWeightingStrategyAsync for OneOverOutsAsync {
    fn weight(&self, defining_connections: &DefiningConnectionsAsync) -> f32 {
        1.0f32 / f32::max(defining_connections.connections.len() as f32, 1.0f32)
    }
}

impl Debug for OneOverOutsAsync {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { write!(f, "one_over_outs") }
}

pub struct OneOverOutsUpperHalfAsync;

impl DefiningWeightingStrategyAsync for OneOverOutsUpperHalfAsync {
    fn weight(&self, defining_connections: &DefiningConnectionsAsync) -> f32 {
        0.5f32 + (1.0f32 / f32::max(defining_connections.connections.len() as f32, 1.0f32) / 2f32)
    }
}

impl Debug for OneOverOutsUpperHalfAsync {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { write!(f, "one_over_outs_upper_half") }
}

#[derive(Debug, Clone)]
pub struct DefiningConnectionsAsync {
    pub connections: Vec<Arc<RwLock<dyn NeuronAsync>>>,
    pub weighting_strategy: Arc<dyn DefiningWeightingStrategyAsync>
}

impl CollectiveConnectionsAsync for DefiningConnectionsAsync {  
    fn add(&mut self, other: Arc<RwLock<dyn NeuronAsync>>) {
        self.connections.push(other);
    }

    fn connected_neurons(&self) -> &[Arc<RwLock<dyn NeuronAsync>>] { &self.connections }
}

impl WeightingStrategy for DefiningConnectionsAsync {  
    fn common_weight(&self) -> f32 { self.weighting_strategy.weight(&self) }
}

impl DefiningConnectionsAsync {
    pub fn new(
        weighting_strategy: Arc<dyn DefiningWeightingStrategyAsync>
    ) -> DefiningConnectionsAsync { 
        DefiningConnectionsAsync { connections: Vec::new(), weighting_strategy }
    }

    pub fn output_signal(&self, neuron: Arc<RwLock<dyn NeuronAsync>>) -> f32 { 
        neuron.read().unwrap().activation() * self.common_weight()
    }
}