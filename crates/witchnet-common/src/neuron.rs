use std::{
    sync::{ Arc, RwLock },
    rc::Rc, 
    cell::RefCell,
    hash::Hash,
    fmt::{ Debug, Display, Formatter, Result as FmtResult }
};

use anyhow::Result;

use crate::{
    connection::ConnectionKind, 
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

    fn increment_counter(&mut self) -> usize;

    fn decrement_counter(&mut self) -> usize;

    fn counter(&self) -> usize;

    fn explain(&self) -> &[Rc<RefCell<dyn Neuron>>];

    fn explain_one(&self, parent: u32) -> Option<DataTypeValue>;

    fn defined_neurons(&self) -> &[Rc<RefCell<dyn Neuron>>];

    fn activate(
        &mut self, signal: f32, propagate_horizontal: bool, propagate_vertical: bool
    ) -> f32;

    fn deactivate(&mut self, propagate_horizontal: bool, propagate_vertical: bool);

    fn connect_to(
        &mut self, to: Rc<RefCell<dyn Neuron>>, is_to_sensor: bool, kind: ConnectionKind
    ) -> Result<()>;

    fn connect_bilateral(
        &mut self, to: Rc<RefCell<dyn Neuron>>, is_to_sensor: bool, kind: ConnectionKind
    ) -> Result<()>;
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

impl Debug for dyn Neuron {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f, "[{}|c:{}|a:{}]",
            self.id(), 
            self.counter(), 
            self.activation()
        )
    }
}

pub trait NeuronAsync: Sync + Send {
    fn id(&self) -> NeuronID;

    fn value(&self) -> DataTypeValue;

    fn activation(&self) -> f32;

    fn is_sensor(&self) -> bool;

    fn data_type(&self) -> DataType;

    fn increment_counter(&mut self) -> usize;
    
    fn decrement_counter(&mut self) -> usize;

    fn counter(&self) -> usize;

    fn explain(&self) -> &[Arc<RwLock<dyn NeuronAsync>>];

    fn explain_one(&self, parent: u32) -> Option<DataTypeValue>;

    fn defined_neurons(&self) -> &[Arc<RwLock<dyn NeuronAsync>>];

    fn activate(
        &mut self, signal: f32, propagate_horizontal: bool, propagate_vertical: bool
    ) -> f32;

    fn deactivate(&mut self, propagate_horizontal: bool, propagate_vertical: bool);

    fn connect_to(
        &mut self, to: Arc<RwLock<dyn NeuronAsync>>, is_to_sensor: bool, kind: ConnectionKind
    ) -> Result<()>;

    fn connect_bilateral(
        &mut self, to: Arc<RwLock<dyn NeuronAsync>>, is_to_sensor: bool, kind: ConnectionKind
    ) -> Result<()>;
}

impl Display for dyn NeuronAsync {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f, "[{}|c:{}|a:{}]",
            self.id(), 
            self.counter(), 
            self.activation()
        )
    }
}

impl Debug for dyn NeuronAsync {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f, "[{}|c:{}|a:{}]",
            self.id(), 
            self.counter(), 
            self.activation()
        )
    }
}