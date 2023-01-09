use std::{
    rc::{ Rc, Weak },
    cell::RefCell,
    fmt::{ Display, Formatter, Result as FmtResult }
};

use anyhow::Result;

use witchnet_common::{
    neuron::{ Neuron, NeuronID }, 
    connection::{
        ConnectionKind,
        collective::{
            CollectiveConnections,
            WeightingStrategy,
            defining::{
                DefiningConnections,
                ConstantOneWeight, 
                DefiningWeightingStrategy
            }
        },
        collective::explanatory::ExplanatoryConnections
    },
    data::{ DataTypeValue, DataType }
};

pub struct SimpleNeuron {
    pub id: NeuronID,
    pub activation: f32,
    pub counter: usize,
    pub(crate) self_ptr: Weak<RefCell<SimpleNeuron>>,
    pub(crate) defined_neurons: DefiningConnections,
    pub(crate) defining_neurons: ExplanatoryConnections,
    pub(crate) defining_sensors: ExplanatoryConnections
}

impl SimpleNeuron {
    pub fn new(id: NeuronID) -> Rc<RefCell<SimpleNeuron>> {
        let weighting_strategy = Rc::new(ConstantOneWeight);
        let neuron_ptr = Rc::new(
            RefCell::new(
                SimpleNeuron {
                    id,
                    activation: 0.0f32,
                    counter: 0,
                    self_ptr: Weak::new(), 
                    defined_neurons: DefiningConnections::new(weighting_strategy),
                    defining_neurons: ExplanatoryConnections::new(),
                    defining_sensors: ExplanatoryConnections::new()
                }
            )
        );

        neuron_ptr.borrow_mut().self_ptr = Rc::downgrade(&neuron_ptr);
        neuron_ptr
    }
    
    pub fn new_custom(
        id: NeuronID,
        weighting_strategy: Rc<dyn DefiningWeightingStrategy>
    ) -> Rc<RefCell<SimpleNeuron>> {
        let neuron_ptr = Rc::new(
            RefCell::new(
                SimpleNeuron {
                    id,
                    activation: 0.0f32,
                    counter: 0,
                    self_ptr: Weak::new(), 
                    defined_neurons: DefiningConnections::new(weighting_strategy),
                    defining_neurons: ExplanatoryConnections::new(),
                    defining_sensors: ExplanatoryConnections::new()
                }
            )
        );

        neuron_ptr.borrow_mut().self_ptr = Rc::downgrade(&neuron_ptr);
        neuron_ptr
    }

    pub(crate) fn defined_neurons(&self) -> &[Rc<RefCell<dyn Neuron>>] {
        self.defined_neurons.connected_neurons()
    }

    pub(crate) fn defining_sensors(&self) -> &[Rc<RefCell<dyn Neuron>>] {
        self.defining_sensors.connected_neurons()
    }

    #[allow(dead_code)]
    pub(crate) fn defining_neurons(&self) -> &[Rc<RefCell<dyn Neuron>>] {
        self.defining_neurons.connected_neurons()
    }

    pub fn id(&self) -> NeuronID { self.id.clone() }

    pub fn value(&self) -> DataTypeValue { self.id().id.into() }

    pub fn activation(&self) -> f32 { self.activation }

    pub fn is_sensor(&self) -> bool { false }

    pub fn data_type(&self) -> DataType { DataType::Unknown }

    pub fn counter(&self) -> usize { 1usize }

    pub fn explain(&self) -> &[Rc<RefCell<dyn Neuron>>] {
        self.defining_sensors()
    }

    fn explain_one(&self, parent: u32) -> Option<DataTypeValue> {
        let defining_sensors = self.defining_sensors();
        for sensor in defining_sensors.into_iter() {
            let sensor = sensor.borrow();
            if sensor.id().parent_id == parent { return Some(sensor.value()) }
        }
        None
    }

    pub fn activate(
        &mut self, signal: f32, propagate_horizontal: bool, propagate_vertical: bool
    ) -> f32 {
        self.activation += signal;

        let mut max_activation = 0.0f32;
        // let neurons_activation: Vec<Rc<RefCell<dyn Neuron>>> = self.defined_neurons()
        //     .into_iter()
        //     .cloned()
        //     .collect();
        
        if propagate_vertical {
            for neuron in self.defined_neurons() {
                if !neuron.borrow().is_sensor() {
                    let output_signal = self.activation / self.defined_neurons.common_weight();
                    // let output_signal = self.activation /
                    //     f32::max(self.defined_neurons().len() as f32, 1.0f32);
                    max_activation = f32::max(max_activation, output_signal);
                    neuron.borrow_mut().activate(
                        output_signal, propagate_horizontal, propagate_vertical
                    );
                }
            }
        }

        max_activation
    }

    pub fn deactivate(&mut self, propagate_horizontal: bool, propagate_vertical: bool) {
        self.activation = 0.0f32;

        if propagate_vertical {
            for neuron in self.defined_neurons() {
                neuron.borrow_mut().deactivate(propagate_horizontal, propagate_vertical);
            }
        }
    }
}

impl Neuron for SimpleNeuron {
    fn id(&self) -> NeuronID { self.id() }

    fn value(&self) -> DataTypeValue { self.value() }

    fn activation(&self) -> f32 { self.activation() }

    fn is_sensor(&self) -> bool { self.is_sensor() }

    fn data_type(&self) -> DataType { self.data_type() }

    fn increment_counter(&mut self) -> usize {
        self.counter += 1;
        self.counter
    }

    fn counter(&self) -> usize { self.counter() }

    fn explain(&self) -> &[Rc<RefCell<dyn Neuron>>] {
        self.explain()
    }

    fn explain_one(&self, parent: u32) -> Option<DataTypeValue> {
        self.explain_one(parent)
    }

    fn defined_neurons(&self) -> &[Rc<RefCell<dyn Neuron>>] {
        &self.defined_neurons.connected_neurons()
    }

    fn activate(
        &mut self, signal: f32, propagate_horizontal: bool, propagate_vertical: bool
    ) -> f32 {
        self.activate(signal, propagate_horizontal, propagate_vertical)
    }

    fn deactivate(&mut self, propagate_horizontal: bool, propagate_vertical: bool) {
        self.deactivate(propagate_horizontal, propagate_vertical)
    }

    fn connect_to(
        &mut self, to: Rc<RefCell<dyn Neuron>>, is_to_sensor: bool, kind: ConnectionKind
    ) -> Result<()> {
        match kind {
            ConnectionKind::Defining => {
                if is_to_sensor {
                    anyhow::bail!("only defining connection from sensor to neuron can be created")
                }
                self.defined_neurons.add(to.clone());
                Ok(())
            },
            ConnectionKind::Explanatory => {
                if is_to_sensor {
                    self.defining_sensors.add(to.clone());
                } else {
                    self.defining_neurons.add(to.clone());
                }
                Ok(())
            },
            _ => { anyhow::bail!("only defining connection to SimpleNeuron can be created") }
        }
    }

    fn connect_bilateral(
        &mut self, to: Rc<RefCell<dyn Neuron>>, is_to_sensor: bool, kind: ConnectionKind
    ) -> Result<()> {
        if is_to_sensor {
            anyhow::bail!("connections from SimpleNeuron to Element are not allowed")
        } else {
            match kind {
                ConnectionKind::Defining => {
                    self.connect_to(to.clone(), is_to_sensor, kind)?;
                    to.borrow_mut().connect_to(
                        self.self_ptr.upgrade().unwrap(), false, ConnectionKind::Explanatory
                    )?;
                    Ok(())
                }
                _ => {
                    anyhow::bail!(
                        "only defining connection from Element to SimpleNeuron can be created"
                    )
                }
            }
        }
    }
}

impl Display for SimpleNeuron {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f, "[{}|c:{}|a:{}]",
            self.id(), 
            self.counter(), 
            self.activation()
        )
    }
}

#[cfg(test)]
mod tests {
    use std::{
        rc::Rc,
        cell::RefCell
    };

    use witchnet_common::{
        neuron::{ Neuron, NeuronID },
        connection::ConnectionKind
    };

    use asa_graphs::neural::{
        element::Element,
    };

    use super::SimpleNeuron;

    #[test]
    fn as_neuron() {
        let parent_id = 1u32;
        let neuron_1 = SimpleNeuron::new(
            NeuronID { id: 1, parent_id }
        );
        let neuron_2 = SimpleNeuron::new(
            NeuronID { id: 2, parent_id }
        );

        let neuron_1_id = neuron_1.borrow().id();
        assert_eq!(neuron_1_id.id, 1);
        assert_eq!(neuron_1_id.parent_id, parent_id);
        let neuron_2_id = neuron_2.borrow().id();
        assert_eq!(neuron_2_id.id, 2);
        assert_eq!(neuron_2_id.parent_id, parent_id);

        assert_eq!(neuron_1.borrow().is_sensor(), false);

        assert_eq!(neuron_1.borrow().activation(), 0.0f32);
        assert_eq!(neuron_2.borrow().activation(), 0.0f32);

        assert_eq!(neuron_1.borrow().counter(), 1usize);
        
        neuron_1.borrow_mut().connect_to(
            neuron_2.clone(), false, ConnectionKind::Defining
        ).unwrap();

        let max_activation = neuron_1.borrow_mut().activate(1.0f32, true, true);
        assert!(max_activation > 0.0f32);
        assert_eq!(neuron_1.borrow().activation(), 1.0f32);
        assert_eq!(neuron_2.borrow().activation(), 1.0f32);

        neuron_1.borrow_mut().activate(1.0f32, false, true);
        assert_eq!(neuron_1.borrow().activation(), 2.0f32);
        assert_eq!(neuron_2.borrow().activation(), 3.0f32);
        neuron_1.borrow_mut().deactivate(true, true);
        assert_eq!(neuron_1.borrow().activation(), 0.0f32);
        assert_eq!(neuron_2.borrow().activation(), 0.0f32);

        neuron_1.borrow_mut().activate(1.0f32, true, false);
        assert_eq!(neuron_1.borrow().activation(), 1.0f32);
        assert_eq!(neuron_2.borrow().activation(), 0.0f32);
        neuron_1.borrow_mut().deactivate(false, true);
        assert_eq!(neuron_1.borrow().activation(), 0.0f32);
        assert_eq!(neuron_2.borrow().activation(), 0.0f32);

        neuron_1.borrow_mut().activate(1.0f32, false, false);
        assert_eq!(neuron_1.borrow().activation(), 1.0f32);
        assert_eq!(neuron_2.borrow().activation(), 0.0f32);
        neuron_1.borrow_mut().deactivate(true, false);
        assert_eq!(neuron_1.borrow().activation(), 0.0f32);
        assert_eq!(neuron_2.borrow().activation(), 0.0f32);

        neuron_1.borrow_mut().activate(1.0f32, true, true);
        assert_eq!(neuron_1.borrow().activation(), 1.0f32);
        assert_eq!(neuron_2.borrow().activation(), 1.0f32);
        neuron_1.borrow_mut().deactivate(false, false);
        assert_eq!(neuron_1.borrow().activation(), 0.0f32);
        assert_eq!(neuron_2.borrow().activation(), 1.0f32);

        let neuron_1_borrowed_mut = neuron_1.borrow_mut();
        let exp_1 = neuron_1_borrowed_mut.explain();
        assert_eq!(exp_1.len(), 0);
    }

    #[test]
    fn connect_to_neuron() {
        let parent_id = 1u32;
        let neuron_1 = SimpleNeuron::new(
            NeuronID { id: 1, parent_id }
        );
        let neuron_2 = SimpleNeuron::new(
            NeuronID { id: 2, parent_id }
        );

        let connection_1 = neuron_1.borrow_mut().connect_to(
            neuron_2.clone(), false, ConnectionKind::Defining
        );
        assert!(connection_1.is_ok());
        assert_eq!(neuron_1.borrow().defined_neurons().len(), 1);
        assert_eq!(neuron_1.borrow().defining_neurons().len(), 0);
        assert_eq!(neuron_1.borrow().defining_sensors().len(), 0);
        assert_eq!(neuron_2.borrow().defined_neurons().len(), 0);
        assert_eq!(neuron_2.borrow().defining_neurons().len(), 0);
        assert_eq!(neuron_2.borrow().defining_sensors().len(), 0);
        connection_1.unwrap();
    }

    #[test]
    fn connect_to_sensor() {
        let parent_id = 1u32;
        let neuron_1 = SimpleNeuron::new(
            NeuronID { id: 1, parent_id }
        );
        let neuron_2: Rc<RefCell<Element<i32, 3>>> = Element::new(&1, 1, 1);

        let connection_1 = neuron_1.borrow_mut().connect_to(
            neuron_2.clone(), true, ConnectionKind::Defining
        );
        assert!(connection_1.is_err());
        assert_eq!(neuron_1.borrow().defined_neurons().len(), 0);
        assert_eq!(neuron_1.borrow().defining_neurons().len(), 0);
        assert_eq!(neuron_1.borrow().defining_sensors().len(), 0);
        assert_eq!(neuron_2.borrow().defining_neurons().len(), 0);
    }

    #[test]
    fn connect_bilateral_to_sensor() {
        let parent_id = 1u32;
        let neuron_1 = SimpleNeuron::new(
            NeuronID { id: 1, parent_id }
        );
        let neuron_2: Rc<RefCell<Element<i32, 3>>> = Element::new(&1, 1, 1);

        let connection_1 = neuron_1.borrow_mut().connect_bilateral(
            neuron_2.clone(), true, ConnectionKind::Defining
        );
        assert!(connection_1.is_err());
        assert_eq!(neuron_1.borrow().defined_neurons().len(), 0);
        assert_eq!(neuron_1.borrow().defining_neurons().len(), 0);
        assert_eq!(neuron_1.borrow().defining_sensors().len(), 0);
        assert_eq!(neuron_2.borrow().defining_neurons().len(), 0);
    }

    #[test]
    fn connect_bilateral_to_neuron() {
        let parent_id = 1u32;
        let neuron_1 = SimpleNeuron::new(
            NeuronID { id: 1, parent_id }
        );
        let neuron_2 = SimpleNeuron::new(
            NeuronID { id: 2, parent_id }
        );

        neuron_1.borrow_mut().connect_bilateral(
            neuron_2.clone(), false, ConnectionKind::Defining
        ).unwrap();
        assert_eq!(neuron_1.borrow().defined_neurons().len(), 1);
        assert_eq!(neuron_1.borrow().defining_neurons().len(), 0);
        assert_eq!(neuron_1.borrow().defining_sensors().len(), 0);
        assert_eq!(neuron_2.borrow().defined_neurons().len(), 0);
        assert_eq!(neuron_2.borrow().defining_neurons().len(), 1);
        assert_eq!(neuron_2.borrow().defining_sensors().len(), 0);
    }

    #[test]
    fn connect_wrong() {
        let parent_id = 1u32;
        let neuron_1 = SimpleNeuron::new(
            NeuronID { id: 1, parent_id }
        );
        let neuron_2 = SimpleNeuron::new(
            NeuronID { id: 2, parent_id }
        );
        let sensor: Rc<RefCell<Element<i32, 3>>> = Element::new(&1, 1, 1);

        let connection_1 = neuron_1.borrow_mut().connect_bilateral(
            neuron_2.clone(), false, ConnectionKind::Inhibitory
        );
        assert!(connection_1.is_err());

        let connection_1 = neuron_1.borrow_mut().connect_bilateral(
            sensor.clone(), false, ConnectionKind::Defining
        );
        assert!(connection_1.is_err());

        let connection_1 = neuron_1.borrow_mut().connect_bilateral(
            sensor.clone(), false, ConnectionKind::Explanatory
        );
        assert!(connection_1.is_err());

        let connection_1 = neuron_1.borrow_mut().connect_to(
            neuron_2.clone(), false, ConnectionKind::Inhibitory
        );
        assert!(connection_1.is_err());

        let connection_1 = neuron_1.borrow_mut().connect_to(
            sensor.clone(), false, ConnectionKind::Inhibitory
        );
        assert!(connection_1.is_err());
    }
}