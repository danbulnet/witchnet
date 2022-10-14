use std::{
    rc::{ Rc, Weak },
    cell::RefCell,
    fmt::{ Display, Formatter, Result as FmtResult },
    marker::PhantomData
};

use anyhow::Result;

use witchnet_common::{
    neuron::{ 
        Neuron, 
        NeuronID, 
        NeuronConnect,
        NeuronConnectBilateral
    }, 
    connection::{
        ConnectionKind,
        collective::{
            CollectiveConnections,
            defining::DefiningConnections
        },
        collective::explanatory::ExplanatoryConnections
    },
    sensor::SensorData,
    data::{ DataDeductor, DataTypeValue, DataType }
};

use asa_graphs::neural::element::Element;

pub struct SimpleNeuron {
    pub id: NeuronID,
    pub activation: f32,
    pub(crate) self_ptr: Weak<RefCell<SimpleNeuron>>,
    pub(crate) defined_neurons: DefiningConnections,
    pub(crate) defining_neurons: ExplanatoryConnections,
    pub(crate) defining_sensors: ExplanatoryConnections
}

impl SimpleNeuron {
    pub fn new(id: NeuronID) -> Rc<RefCell<SimpleNeuron>> {
        let neuron_ptr = Rc::new(
            RefCell::new(
                SimpleNeuron {
                    id,
                    activation: 0.0f32,
                    self_ptr: Weak::new(), 
                    defined_neurons: DefiningConnections::new(),
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
}

impl NeuronConnect for SimpleNeuron {
    fn connect_to<Other: Neuron + NeuronConnect + 'static>(
        &mut self, to: Rc<RefCell<Other>>, kind: ConnectionKind
    ) -> Result<()> {
        match kind {
            ConnectionKind::Defining => {
                let to_borrowed = to.borrow();
                if to_borrowed.is_sensor() {
                    anyhow::bail!("only defining connection from sensor to neuron can be created")
                }
                self.defined_neurons.add(to.clone());
                Ok(())
            },
            ConnectionKind::Explanatory => {
                let to_borrowed = to.borrow();
                if to_borrowed.is_sensor() {
                    self.defining_sensors.add(to.clone());
                } else {
                    self.defining_neurons.add(to.clone());
                }
                Ok(())
            },
            _ => { anyhow::bail!("only defining connection to SimpleNeuron can be created") }
        }
    }
}

impl<Key, const ORDER: usize> NeuronConnectBilateral<Element<Key, ORDER>> for SimpleNeuron 
where 
    Key: SensorData, 
    [(); ORDER + 1]:, 
    PhantomData<Key>: DataDeductor,
    DataTypeValue: From<Key>
{
    fn connect_bilateral(
        _from: Rc<RefCell<Self>>, _to: Rc<RefCell<Element<Key, ORDER>>>, _kind: ConnectionKind
    ) -> Result<()> {
        anyhow::bail!("connections from SimpleNeuron to Element are not allowed")
    }
}

impl NeuronConnectBilateral<SimpleNeuron> for SimpleNeuron {
    fn connect_bilateral(
        from: Rc<RefCell<Self>>, to: Rc<RefCell<SimpleNeuron>>, kind: ConnectionKind
    ) -> Result<()> {
        match kind {
            ConnectionKind::Defining => {
                from.borrow_mut().connect_to(to.clone(), kind)?;
                to.borrow_mut().connect_to(from, ConnectionKind::Explanatory)?;
                Ok(())
            }
            _ => {
                anyhow::bail!("only defining connection from Element to SimpleNeuron can be created")
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
        neuron::{ NeuronConnect, NeuronConnectBilateral, NeuronID },
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
            neuron_2.clone(), ConnectionKind::Defining
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
            neuron_2.clone(), ConnectionKind::Defining
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
            neuron_2.clone(), ConnectionKind::Defining
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

        let connection_1 = SimpleNeuron::connect_bilateral(
            neuron_1.clone(), neuron_2.clone(), ConnectionKind::Defining
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

        SimpleNeuron::connect_bilateral(
            neuron_1.clone(), neuron_2.clone(), ConnectionKind::Defining
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

        let connection_1 = SimpleNeuron::connect_bilateral(
            neuron_1.clone(), neuron_2.clone(), ConnectionKind::Inhibitory
        );
        assert!(connection_1.is_err());

        let connection_1 = SimpleNeuron::connect_bilateral(
            neuron_1.clone(), sensor.clone(), ConnectionKind::Defining
        );
        assert!(connection_1.is_err());

        let connection_1 = SimpleNeuron::connect_bilateral(
            neuron_1.clone(), sensor.clone(), ConnectionKind::Explanatory
        );
        assert!(connection_1.is_err());

        let connection_1 = neuron_1.borrow_mut().connect_to(
            neuron_2.clone(), ConnectionKind::Inhibitory
        );
        assert!(connection_1.is_err());

        let connection_1 = neuron_1.borrow_mut().connect_to(
            sensor.clone(), ConnectionKind::Inhibitory
        );
        assert!(connection_1.is_err());
    }
}