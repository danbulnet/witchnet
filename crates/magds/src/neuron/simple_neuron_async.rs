use std::{
    sync::{ Arc, Weak, RwLock },
    fmt::{ Display, Formatter, Result as FmtResult }
};

use anyhow::Result;

use witchnet_common::{
    neuron::{ NeuronAsync, NeuronID }, 
    connection::{
        ConnectionKind,
        collective::{
            CollectiveConnectionsAsync,
            defining::DefiningConnectionsAsync,
            explanatory::ExplanatoryConnectionsAsync
        }
    },
    data::{ DataTypeValue, DataType }
};

pub struct SimpleNeuron {
    pub id: NeuronID,
    pub activation: f32,
    pub(crate) self_ptr: Weak<RwLock<SimpleNeuron>>,
    pub(crate) defined_neurons: DefiningConnectionsAsync,
    pub(crate) defining_neurons: ExplanatoryConnectionsAsync,
    pub(crate) defining_sensors: ExplanatoryConnectionsAsync
}

impl SimpleNeuron {
    pub fn new(id: NeuronID) -> Arc<RwLock<SimpleNeuron>> {
        let neuron_ptr = Arc::new(
            RwLock::new(
                SimpleNeuron {
                    id,
                    activation: 0.0f32,
                    self_ptr: Weak::new(), 
                    defined_neurons: DefiningConnectionsAsync::new(),
                    defining_neurons: ExplanatoryConnectionsAsync::new(),
                    defining_sensors: ExplanatoryConnectionsAsync::new()
                }
            )
        );

        neuron_ptr.write().unwrap().self_ptr = Arc::downgrade(&neuron_ptr);
        neuron_ptr
    }

    pub(crate) fn defined_neurons(&self) -> &[Arc<RwLock<dyn NeuronAsync>>] {
        self.defined_neurons.connected_neurons()
    }

    pub(crate) fn defining_sensors(&self) -> &[Arc<RwLock<dyn NeuronAsync>>] {
        self.defining_sensors.connected_neurons()
    }

    #[allow(dead_code)]
    pub(crate) fn defining_neurons(&self) -> &[Arc<RwLock<dyn NeuronAsync>>] {
        self.defining_neurons.connected_neurons()
    }

    pub fn id(&self) -> NeuronID { self.id.clone() }

    pub fn value(&self) -> DataTypeValue { self.id().id.into() }

    pub fn activation(&self) -> f32 { self.activation }

    pub fn is_sensor(&self) -> bool { false }

    pub fn data_type(&self) -> DataType { DataType::Unknown }

    pub fn counter(&self) -> usize { 1usize }

    pub fn explain(&self) -> &[Arc<RwLock<dyn NeuronAsync>>] {
        self.defining_sensors()
    }

    fn explain_one(&self, parent: u32) -> Option<DataTypeValue> {
        let defining_sensors = self.defining_sensors();
        for sensor in defining_sensors.into_iter() {
            let sensor = sensor.read().unwrap();
            if sensor.id().parent_id == parent { return Some(sensor.value()) }
        }
        None
    }

    pub fn activate(
        &mut self, signal: f32, propagate_horizontal: bool, propagate_vertical: bool
    ) -> f32 {
        self.activation += signal;

        let mut max_activation = 0.0f32;
        // let neurons_activation: Vec<Arc<RwLock<dyn NeuronAsync>>> = self.defined_neurons()
        //     .into_iter()
        //     .cloned()
        //     .collect();
        
        if propagate_vertical {
            for neuron in self.defined_neurons() {
                if !neuron.read().unwrap().is_sensor() {
                    let output_signal = self.activation / self.defined_neurons.common_weight();
                    // let output_signal = self.activation /
                    //     f32::max(self.defined_neurons().len() as f32, 1.0f32);
                    max_activation = f32::max(max_activation, output_signal);
                    neuron.write().unwrap().activate(
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
                neuron.write().unwrap().deactivate(propagate_horizontal, propagate_vertical);
            }
        }
    }
}

impl NeuronAsync for SimpleNeuron {
    fn id(&self) -> NeuronID { self.id() }

    fn value(&self) -> DataTypeValue { self.value() }

    fn activation(&self) -> f32 { self.activation() }

    fn is_sensor(&self) -> bool { self.is_sensor() }

    fn data_type(&self) -> DataType { self.data_type() }

    fn counter(&self) -> usize { self.counter() }

    fn explain(&self) -> &[Arc<RwLock<dyn NeuronAsync>>] {
        self.explain()
    }

    fn explain_one(&self, parent: u32) -> Option<DataTypeValue> {
        self.explain_one(parent)
    }

    fn defined_neurons(&self) -> &[Arc<RwLock<dyn NeuronAsync>>] {
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
        &mut self, to: Arc<RwLock<dyn NeuronAsync>>, is_to_sensor: bool, kind: ConnectionKind
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
        &mut self, to: Arc<RwLock<dyn NeuronAsync>>, is_to_sensor: bool, kind: ConnectionKind
    ) -> Result<()> {
        if is_to_sensor {
            anyhow::bail!("connections from SimpleNeuron to Element are not allowed")
        } else {
            match kind {
                ConnectionKind::Defining => {
                    self.connect_to(to.clone(), is_to_sensor, kind)?;
                    to.write().unwrap().connect_to(
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
    use std::sync::{ Arc, RwLock };

    use witchnet_common::{
        neuron::{ NeuronAsync, NeuronID },
        connection::ConnectionKind
    };

    use asa_graphs::neural_async::{
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

        let neuron_1_id = neuron_1.read().unwrap().id();
        assert_eq!(neuron_1_id.id, 1);
        assert_eq!(neuron_1_id.parent_id, parent_id);
        let neuron_2_id = neuron_2.read().unwrap().id();
        assert_eq!(neuron_2_id.id, 2);
        assert_eq!(neuron_2_id.parent_id, parent_id);

        assert_eq!(neuron_1.read().unwrap().is_sensor(), false);

        assert_eq!(neuron_1.read().unwrap().activation(), 0.0f32);
        assert_eq!(neuron_2.read().unwrap().activation(), 0.0f32);

        assert_eq!(neuron_1.read().unwrap().counter(), 1usize);
        
        neuron_1.write().unwrap().connect_to(
            neuron_2.clone(), false, ConnectionKind::Defining
        ).unwrap();

        let max_activation = neuron_1.write().unwrap().activate(1.0f32, true, true);
        assert!(max_activation > 0.0f32);
        assert_eq!(neuron_1.read().unwrap().activation(), 1.0f32);
        assert_eq!(neuron_2.read().unwrap().activation(), 1.0f32);

        neuron_1.write().unwrap().activate(1.0f32, false, true);
        assert_eq!(neuron_1.read().unwrap().activation(), 2.0f32);
        assert_eq!(neuron_2.read().unwrap().activation(), 3.0f32);
        neuron_1.write().unwrap().deactivate(true, true);
        assert_eq!(neuron_1.read().unwrap().activation(), 0.0f32);
        assert_eq!(neuron_2.read().unwrap().activation(), 0.0f32);

        neuron_1.write().unwrap().activate(1.0f32, true, false);
        assert_eq!(neuron_1.read().unwrap().activation(), 1.0f32);
        assert_eq!(neuron_2.read().unwrap().activation(), 0.0f32);
        neuron_1.write().unwrap().deactivate(false, true);
        assert_eq!(neuron_1.read().unwrap().activation(), 0.0f32);
        assert_eq!(neuron_2.read().unwrap().activation(), 0.0f32);

        neuron_1.write().unwrap().activate(1.0f32, false, false);
        assert_eq!(neuron_1.read().unwrap().activation(), 1.0f32);
        assert_eq!(neuron_2.read().unwrap().activation(), 0.0f32);
        neuron_1.write().unwrap().deactivate(true, false);
        assert_eq!(neuron_1.read().unwrap().activation(), 0.0f32);
        assert_eq!(neuron_2.read().unwrap().activation(), 0.0f32);

        neuron_1.write().unwrap().activate(1.0f32, true, true);
        assert_eq!(neuron_1.read().unwrap().activation(), 1.0f32);
        assert_eq!(neuron_2.read().unwrap().activation(), 1.0f32);
        neuron_1.write().unwrap().deactivate(false, false);
        assert_eq!(neuron_1.read().unwrap().activation(), 0.0f32);
        assert_eq!(neuron_2.read().unwrap().activation(), 1.0f32);

        let neuron_1_borrowed_mut = neuron_1.write().unwrap();
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

        let connection_1 = neuron_1.write().unwrap().connect_to(
            neuron_2.clone(), false, ConnectionKind::Defining
        );
        assert!(connection_1.is_ok());
        assert_eq!(neuron_1.read().unwrap().defined_neurons().len(), 1);
        assert_eq!(neuron_1.read().unwrap().defining_neurons().len(), 0);
        assert_eq!(neuron_1.read().unwrap().defining_sensors().len(), 0);
        assert_eq!(neuron_2.read().unwrap().defined_neurons().len(), 0);
        assert_eq!(neuron_2.read().unwrap().defining_neurons().len(), 0);
        assert_eq!(neuron_2.read().unwrap().defining_sensors().len(), 0);
        connection_1.unwrap();
    }

    #[test]
    fn connect_to_sensor() {
        let parent_id = 1u32;
        let neuron_1 = SimpleNeuron::new(
            NeuronID { id: 1, parent_id }
        );
        let neuron_2: Arc<RwLock<Element<i32, 3>>> = Element::new(&1, 1, 1);

        let connection_1 = neuron_1.write().unwrap().connect_to(
            neuron_2.clone(), true, ConnectionKind::Defining
        );
        assert!(connection_1.is_err());
        assert_eq!(neuron_1.read().unwrap().defined_neurons().len(), 0);
        assert_eq!(neuron_1.read().unwrap().defining_neurons().len(), 0);
        assert_eq!(neuron_1.read().unwrap().defining_sensors().len(), 0);
        assert_eq!(neuron_2.read().unwrap().defining_neurons().len(), 0);
    }

    #[test]
    fn connect_bilateral_to_sensor() {
        let parent_id = 1u32;
        let neuron_1 = SimpleNeuron::new(
            NeuronID { id: 1, parent_id }
        );
        let neuron_2: Arc<RwLock<Element<i32, 3>>> = Element::new(&1, 1, 1);

        let connection_1 = neuron_1.write().unwrap().connect_bilateral(
            neuron_2.clone(), true, ConnectionKind::Defining
        );
        assert!(connection_1.is_err());
        assert_eq!(neuron_1.read().unwrap().defined_neurons().len(), 0);
        assert_eq!(neuron_1.read().unwrap().defining_neurons().len(), 0);
        assert_eq!(neuron_1.read().unwrap().defining_sensors().len(), 0);
        assert_eq!(neuron_2.read().unwrap().defining_neurons().len(), 0);
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

        neuron_1.write().unwrap().connect_bilateral(
            neuron_2.clone(), false, ConnectionKind::Defining
        ).unwrap();
        assert_eq!(neuron_1.read().unwrap().defined_neurons().len(), 1);
        assert_eq!(neuron_1.read().unwrap().defining_neurons().len(), 0);
        assert_eq!(neuron_1.read().unwrap().defining_sensors().len(), 0);
        assert_eq!(neuron_2.read().unwrap().defined_neurons().len(), 0);
        assert_eq!(neuron_2.read().unwrap().defining_neurons().len(), 1);
        assert_eq!(neuron_2.read().unwrap().defining_sensors().len(), 0);
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
        let sensor: Arc<RwLock<Element<i32, 3>>> = Element::new(&1, 1, 1);

        let connection_1 = neuron_1.write().unwrap().connect_bilateral(
            neuron_2.clone(), false, ConnectionKind::Inhibitory
        );
        assert!(connection_1.is_err());

        let connection_1 = neuron_1.write().unwrap().connect_bilateral(
            sensor.clone(), true, ConnectionKind::Defining
        );
        assert!(connection_1.is_err());

        let connection_1 = neuron_1.write().unwrap().connect_bilateral(
            sensor.clone(), true, ConnectionKind::Explanatory
        );
        assert!(connection_1.is_err());

        let connection_1 = neuron_1.write().unwrap().connect_to(
            neuron_2.clone(), false, ConnectionKind::Inhibitory
        );
        assert!(connection_1.is_err());

        let connection_1 = neuron_1.write().unwrap().connect_to(
            sensor.clone(), true, ConnectionKind::Inhibitory
        );
        assert!(connection_1.is_err());
    }
}