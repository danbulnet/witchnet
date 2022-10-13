pub mod collective;
pub mod standalone;

use crate::neuron::NeuronID;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ConnectionID {
    pub from: NeuronID,
    pub to: NeuronID
}

#[derive(Copy, Clone, Debug)]
pub enum ConnectionKind {
    Defining,
    Explanatory,
    Similarity,
    Inhibitory,
    Sequential
}