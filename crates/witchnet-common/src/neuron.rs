use std::{
    rc::Rc, 
    cell::RefCell,
    hash::Hash,
    collections::HashMap,
    fmt::{ Display, Formatter, Result as FmtResult }
};

use crate::{
    connection::{ Connection, ConnectionKind }, 
    data::{ DataTypeValue, DataType }
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct NeuronID {
    pub id: u32,
    pub parent_id: u32
}

impl NeuronID {
    pub fn new(id: u32, parent_id: u32) -> NeuronID {
        NeuronID { id, parent_id }
    }
}

impl Display for NeuronID {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}_{}", self.parent_id, self.id)
    }
}

pub trait Neuron {
    fn id(&self) -> NeuronID;

    fn value(&self) -> DataTypeValue;

    fn activation(&self) -> f32;

    fn is_sensor(&self) -> bool;

    fn data_type(&self) -> DataType;

    fn counter(&self) -> usize;

    fn explain(&self) -> HashMap<NeuronID, Rc<RefCell<dyn Neuron>>>;

    fn explain_one(&self, parent: u32) -> Option<DataTypeValue>;

    fn activate(
        &mut self, signal: f32, propagate_horizontal: bool, propagate_vertical: bool
    ) -> HashMap<NeuronID, Rc<RefCell<dyn Neuron>>>;

    fn deactivate(&mut self, propagate_horizontal: bool, propagate_vertical: bool);
}

impl Display for dyn Neuron {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f, "[{}|c:{}|a:{}]",
            self.id(), 
            self.counter(), 
            self.activation()
        )
    }
}

pub trait NeuronConnect {
    fn connect_to(
        &mut self, to: Rc<RefCell<dyn Neuron>>, kind: ConnectionKind
    ) -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>, String>;

    fn connect_to_connection(
        &mut self, to_connection: Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>
    ) -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>, String>;

    fn connect_from(
        &mut self, from: Rc<RefCell<dyn Neuron>>, kind: ConnectionKind
    ) -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>, String>;

    fn connect_from_connection(
        &mut self, from_connection: Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>
    ) -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>, String>;
}

pub trait NeuronConnectBilateral<Other: Neuron + NeuronConnect>: Neuron + NeuronConnect {
    fn connect_bilateral_to(&mut self, to: Rc<RefCell<Other>>, kind: ConnectionKind) 
    -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>, String>;

    fn connect_bilateral_from(&mut self, from: Rc<RefCell<Other>>, kind: ConnectionKind) 
    -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>, String>;
}