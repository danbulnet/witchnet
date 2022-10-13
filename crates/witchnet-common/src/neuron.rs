use std::{
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

    fn counter(&self) -> usize;

    fn explain(&self) -> &[Rc<RefCell<dyn Neuron>>];

    fn explain_one(&self, parent: u32) -> Option<DataTypeValue>;

    // fn connected_neuron_ids(&self) -> &[u32];

    fn activate(
        &mut self, signal: f32, propagate_horizontal: bool, propagate_vertical: bool
    ) -> (Vec<Rc<RefCell<dyn Neuron>>>, f32);

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

pub trait NeuronConnect {
    fn connect_to<Other: Neuron + NeuronConnect + 'static>(
        &mut self, to: Rc<RefCell<Other>>, kind: ConnectionKind
    ) -> Result<()>;
}

pub trait NeuronConnectBilateral<Other: Neuron + NeuronConnect>: Neuron + NeuronConnect {   
    fn connect_bilateral(
        from: Rc<RefCell<Self>>, to: Rc<RefCell<Other>>, kind: ConnectionKind
    ) -> Result<()>;
}