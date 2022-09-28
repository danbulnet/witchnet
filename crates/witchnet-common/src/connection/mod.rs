pub mod universal_connection;
pub mod defining_connection;
pub mod similarity_connection;
pub mod inhibitory_connection;
pub mod sequential_connection;  

use std::{
    rc::Rc,
    cell::RefCell
};

use crate::neuron::{ Neuron, NeuronID };

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ConnectionID {
    pub from: NeuronID,
    pub to: NeuronID
}

#[derive(Copy, Clone, Debug)]
pub enum ConnectionKind {
    Defining,
    Similarity,
    Inhibitory,
    Sequential
}

pub trait Connection {
    type From: Neuron + ?Sized;
    type To: Neuron + ?Sized;

    fn id(&self) -> ConnectionID;

    fn from(&self) -> Rc<RefCell<Self::From>>;
    
    fn to(&self) -> Rc<RefCell<Self::To>>;

    fn kind(&self) -> ConnectionKind;
    
    fn weight(&self) -> f32;
}