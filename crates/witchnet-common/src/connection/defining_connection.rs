use std::{
    rc::Rc,
    cell::RefCell
};

use crate::{
    neuron::Neuron,
    connection::{ Connection, ConnectionKind, ConnectionID }
};

pub struct DefiningConnection<From: Neuron + ?Sized, To: Neuron + ?Sized> {
    from: Rc<RefCell<From>>,
    to: Rc<RefCell<To>>
}

impl<From: Neuron + ?Sized, To: Neuron + ?Sized> DefiningConnection<From, To> {
    pub fn new(from: Rc<RefCell<From>>, to: Rc<RefCell<To>>) -> DefiningConnection<From, To> {
        DefiningConnection { from, to }
    }
}

impl<From: Neuron + ?Sized, To: Neuron + ?Sized> Connection for DefiningConnection<From, To> {
    type From = From;
    type To = To;

    fn id(&self) -> ConnectionID { 
        ConnectionID { from: self.from.borrow().id(), to: self.to.borrow().id() }
    }

    fn from(&self) -> Rc<RefCell<From>> { self.from.clone() }
    
    fn to(&self) -> Rc<RefCell<To>> { self.to.clone() }

    fn kind(&self) -> ConnectionKind { ConnectionKind::Defining }
    
    fn weight(&self) -> f32 { 1.0f32 / self.from.borrow().counter() as f32 }
}