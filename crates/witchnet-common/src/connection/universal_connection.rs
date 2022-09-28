use std::{
    rc::Rc,
    cell::RefCell
};

use crate::{
    neuron::Neuron,
    connection::{ Connection, ConnectionKind, ConnectionID }
};

pub struct UniversalConnection<From: Neuron + ?Sized, To: Neuron + ?Sized> {
    from: Rc<RefCell<From>>,
    to: Rc<RefCell<To>>,
    kind: ConnectionKind,
    weight: f32
}

impl<From: Neuron + ?Sized, To: Neuron + ?Sized> UniversalConnection<From, To> {
    pub fn new(
        from: Rc<RefCell<From>>, to: Rc<RefCell<To>>, kind: ConnectionKind, weight: f32
    ) -> UniversalConnection<From, To> { 
        UniversalConnection { from, to, kind, weight }
    }
}

impl<From: Neuron + ?Sized, To: Neuron + ?Sized> Connection for UniversalConnection<From, To> {
    type From = From;
    type To = To;

    fn id(&self) -> ConnectionID { 
        ConnectionID { from: self.from.borrow().id(), to: self.to.borrow().id() }
    }

    fn from(&self) -> Rc<RefCell<From>> { self.from.clone() }
    
    fn to(&self) -> Rc<RefCell<To>> { self.to.clone() }

    fn kind(&self) -> ConnectionKind { self.kind }

    fn weight(&self) -> f32 { self.weight }
}