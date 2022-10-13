use std::{
    rc::Rc,
    cell::RefCell
};

use crate::{
    neuron::Neuron,
    connection::{
        standalone::StandaloneConnection,
        ConnectionKind, 
        ConnectionID 
    }
};

pub struct SequentialConnection<From: Neuron + ?Sized, To: Neuron + ?Sized> {
    from: Rc<RefCell<From>>,
    to: Rc<RefCell<To>>,
    weight: f32
}

impl<From: Neuron + ?Sized, To: Neuron + ?Sized> SequentialConnection<From, To> {
    pub fn new(
        from: Rc<RefCell<From>>, to: Rc<RefCell<To>>, weight: f32
    ) -> SequentialConnection<From, To> {
        SequentialConnection { from, to, weight }
    }
}

impl<From: Neuron + ?Sized, To: Neuron + ?Sized> StandaloneConnection for SequentialConnection<From, To> {
    type From = From;
    type To = To;

    fn id(&self) -> ConnectionID { 
        ConnectionID { from: self.from.borrow().id(), to: self.to.borrow().id() }
    }

    fn from(&self) -> Rc<RefCell<From>> { self.from.clone() }
    
    fn to(&self) -> Rc<RefCell<To>> { self.to.clone() }

    fn kind(&self) -> ConnectionKind { ConnectionKind::Sequential }
    
    fn weight(&self) -> f32 { self.weight }
}