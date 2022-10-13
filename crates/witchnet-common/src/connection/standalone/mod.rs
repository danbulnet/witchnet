pub mod universal;
pub mod defining;
pub mod similarity;
pub mod inhibitory;
pub mod sequential;

use std::{
    rc::Rc,
    cell::RefCell
};

use crate::{
    neuron::Neuron,
    connection::{ ConnectionID, ConnectionKind }
};

pub trait StandaloneConnection {
    type From: Neuron + ?Sized;
    type To: Neuron + ?Sized;

    fn id(&self) -> ConnectionID;

    fn from(&self) -> Rc<RefCell<Self::From>>;
    
    fn to(&self) -> Rc<RefCell<Self::To>>;

    fn kind(&self) -> ConnectionKind;
    
    fn weight(&self) -> f32;
}