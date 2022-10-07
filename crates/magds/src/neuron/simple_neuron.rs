use std::{
    rc::{ Rc, Weak },
    cell::RefCell,
    collections::HashMap,
    fmt::{ Display, Formatter, Result as FmtResult },
    marker::PhantomData
};

use witchnet_common::{
    neuron::{ 
        Neuron, 
        NeuronID, 
        NeuronConnect,
        NeuronConnectBilateral
    }, 
    connection::{
        Connection,
        ConnectionKind,
        ConnectionID,
        defining_connection::DefiningConnection
    },
    sensor::SensorData,
    data::{ DataDeductor, DataTypeValue, DataType, DataTypeValueStr }
};

use asa_graphs::neural::element::Element;

pub struct SimpleNeuron {
    pub id: NeuronID,
    pub activation: f32,
    pub(crate) self_ptr: Weak<RefCell<SimpleNeuron>>,
    pub(crate) definitions_from_self: 
        HashMap<ConnectionID, Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>>,
    pub(crate) definitions_to_self: 
        HashMap<ConnectionID, Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>>
}

impl SimpleNeuron {
    pub fn new(id: NeuronID) -> Rc<RefCell<SimpleNeuron>> {
        let neuron_ptr = Rc::new(
            RefCell::new(
                SimpleNeuron {
                    id,
                    activation: 0.0f32,
                    self_ptr: Weak::new(), 
                    definitions_from_self: HashMap::new(),
                    definitions_to_self: HashMap::new()
                }
            )
        );

        neuron_ptr.borrow_mut().self_ptr = Rc::downgrade(&neuron_ptr);
        neuron_ptr
    }

    pub(crate) fn defined_neurons(&self) -> HashMap<NeuronID, Rc<RefCell<dyn Neuron>>> {
        let mut neurons = HashMap::new();
        for (_id, definition) in &self.definitions_from_self {
            let neuron = definition.borrow().to();
            if !neuron.borrow().is_sensor() {
                neurons.insert(neuron.borrow().id(), neuron.clone());
            }
        }
        neurons
    }

    pub(crate) fn defining_sensors(&self) -> HashMap<NeuronID, Rc<RefCell<dyn Neuron>>> {
        let mut sensors = HashMap::new();
        for (_id, definition) in &self.definitions_to_self {
            let neuron = definition.borrow().from();
            if neuron.borrow().is_sensor() {
                sensors.insert(neuron.borrow().id(), neuron.clone());
            }
        }
        sensors
    }

    #[allow(dead_code)]
    pub(crate) fn defining_neurons(&self) -> HashMap<NeuronID, Rc<RefCell<dyn Neuron>>> {
        let mut neurons = HashMap::new();
        for (_id, definition) in &self.definitions_to_self {
            let neuron = definition.borrow().from();
            if !neuron.borrow().is_sensor() {
                neurons.insert(neuron.borrow().id(), neuron.clone());
            }
        }
        neurons
    }

    pub fn id(&self) -> NeuronID { self.id.clone() }

    pub fn activation(&self) -> f32 { self.activation }

    pub fn is_sensor(&self) -> bool { false }

    pub fn data_type(&self) -> DataType { DataType::Unknown }

    pub fn counter(&self) -> usize { 1usize }

    pub fn explain(&self) -> HashMap<NeuronID, Rc<RefCell<dyn Neuron>>> {
        self.defining_sensors()
    }

    fn explain_one(&self, parent: Rc<str>) -> Option<DataTypeValue> {
        let defining_sensors = self.defining_sensors();
        let sensor = defining_sensors.into_iter()
            .filter(|(id, _sensor)| id.parent_id == parent)
            .next()?.1;

        let sensor_id = sensor.borrow().id().id;
        let value_str = DataTypeValueStr(&sensor_id);
        let sensor_data_type = sensor.borrow().data_type();
        value_str.data_type_value(sensor_data_type)
    }

    pub fn activate(
        &mut self, signal: f32, propagate_horizontal: bool, propagate_vertical: bool
    ) -> HashMap<NeuronID, Rc<RefCell<dyn Neuron>>> {
        self.activation += signal;

        let mut neurons = self.defined_neurons();
        if propagate_vertical {
            for (_id, neuron) in &neurons.clone() {
                if !neuron.borrow().is_sensor() {
                    let output_signal = self.activation / self.defined_neurons().len() as f32;
                    neurons.extend(
                        neuron.borrow_mut().activate(
                            output_signal, propagate_horizontal, propagate_vertical
                        )
                    );
                }
            }
        }

        neurons
    }

    pub fn deactivate(&mut self, propagate_horizontal: bool, propagate_vertical: bool) {
        self.activation = 0.0f32;

        if propagate_vertical {
            for (_id, neuron) in &self.defined_neurons() {
                neuron.borrow_mut().deactivate(propagate_horizontal, propagate_vertical);
            }
        }
    }
}

impl Neuron for SimpleNeuron {
    fn id(&self) -> NeuronID { self.id() }

    fn activation(&self) -> f32 { self.activation() }

    fn is_sensor(&self) -> bool { self.is_sensor() }

    fn data_type(&self) -> DataType { self.data_type() }

    fn counter(&self) -> usize { self.counter() }

    fn explain(&self) -> HashMap<NeuronID, Rc<RefCell<dyn Neuron>>> {
        self.explain()
    }

    fn explain_one(&self, parent: Rc<str>) -> Option<DataTypeValue> {
        self.explain_one(parent)
    }

    fn activate(
        &mut self, signal: f32, propagate_horizontal: bool, propagate_vertical: bool
    ) -> HashMap<NeuronID, Rc<RefCell<dyn Neuron>>> {
        self.activate(signal, propagate_horizontal, propagate_vertical)
    }

    fn deactivate(&mut self, propagate_horizontal: bool, propagate_vertical: bool) {
        self.deactivate(propagate_horizontal, propagate_vertical)
    }
}

impl NeuronConnect for SimpleNeuron {
    fn connect_to(
        &mut self, to: Rc<RefCell<dyn Neuron>>, kind: ConnectionKind
    ) -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>, String> {
        match kind {
            ConnectionKind::Defining => {
                if to.borrow().is_sensor() {
                    let msg = "only defining connection from sensor to neuron can be created";
                    log::error!("{}", msg);
                    return Err(msg.to_string())
                }

                let connection = Rc::new(RefCell::new(DefiningConnection::new(
                    self.self_ptr.upgrade().unwrap() as Rc<RefCell<dyn Neuron>>, 
                    to.clone()
                )));
                let connection_id = ConnectionID { from: self.id(), to: to.borrow().id() };
                self.definitions_from_self.insert(connection_id, connection.clone());

                Ok(connection)
            },
            _ => {
                let msg = "only defining connection to SimpleNeuron can be created";
                log::error!("{}", msg);
                Err(msg.to_string())
            }
        }
    }

    fn connect_to_connection(
        &mut self, to_connection: Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>
    ) -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>, String> {
        match to_connection.borrow().kind() {
            ConnectionKind::Defining => {
                let to_neuron_ptr = to_connection.borrow().to().as_ptr();

                if unsafe { (&*to_neuron_ptr).is_sensor() } {
                    let msg = "only defining connection from sensor to neuron can be created";
                    log::error!("{}", msg);
                    return Err(msg.to_string())
                }

                let to_neuorn_id = unsafe { (&*to_neuron_ptr).id() };
                let connection_id = ConnectionID { from: self.id(), to: to_neuorn_id };
                self.definitions_from_self.insert(connection_id, to_connection.clone());
                Ok(to_connection.clone())
            },
            _ => {
                let msg = "only defining connection to SimpleNeuron can be created";
                log::error!("{}", msg);
                Err(msg.to_string())
            }
        }
    }

    fn connect_from(
        &mut self, from: Rc<RefCell<dyn Neuron>>, kind: ConnectionKind
    ) -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>, String> {
        match kind {
            ConnectionKind::Defining => {
                let connection = Rc::new(RefCell::new(DefiningConnection::new(
                    from.clone(),
                    self.self_ptr.upgrade().unwrap() as Rc<RefCell<dyn Neuron>>
                )));
                let connection_id = ConnectionID { from: from.borrow().id(), to: self.id() };
                self.definitions_to_self.insert(connection_id, connection.clone());
                Ok(connection)
            },
            _ => {
                let msg = "only defining connection to SimpleNeuron can be created";
                log::error!("{}", msg);
                Err(msg.to_string())
            }
        }
    }

    fn connect_from_connection(
        &mut self, from_connection: Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>
    ) -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>, String> {
        match from_connection.borrow().kind() {
            ConnectionKind::Defining => {
                let from_neuron_ptr = from_connection.borrow().from().as_ptr();
                let from_neuorn_id = unsafe { (&*from_neuron_ptr).id() };
                let connection_id = ConnectionID { from: from_neuorn_id, to: self.id() };
                self.definitions_to_self.insert(connection_id, from_connection.clone());
                Ok(from_connection.clone())
            },
            _ => {
                let msg = "only defining connection to SimpleNeuron can be created";
                log::error!("{}", msg);
                Err(msg.to_string())
            }
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
    fn connect_bilateral_to(&mut self, _to: Rc<RefCell<Element<Key, ORDER>>>, _kind: ConnectionKind) 
    -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>, String> {
        let msg = "only defining connection from Element to SimpleNeuron can be created";
        log::error!("{}", msg);
        Err(msg.to_string())
    }

    fn connect_bilateral_from(&mut self, from: Rc<RefCell<Element<Key, ORDER>>>, kind: ConnectionKind) 
    -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>, String> {
        match kind {
            ConnectionKind::Defining => {
                match self.connect_from(from.clone(), kind) {
                    Ok(connection) => {
                        match from.borrow_mut().connect_to_connection(connection) {
                            Ok(second_connection) => Ok(second_connection),
                            Err(e) => {
                                let msg = format!("error while creating second connection: {}", e);
                                log::error!("{}", msg);
                                Err(msg)
                            }        
                        }
                    }
                    Err(e) => {
                        let msg = format!("error while creating connection: {}", e);
                        log::error!("{}", msg);
                        Err(msg)
                    }
                }
            },
            _ => {
                let msg = "only defining connection from Element to SimpleNeuron can be created";
                log::error!("{}", msg);
                Err(msg.to_string())
            }
        }
    }
}

impl NeuronConnectBilateral<SimpleNeuron> for SimpleNeuron {
    fn connect_bilateral_to(&mut self, to: Rc<RefCell<SimpleNeuron>>, kind: ConnectionKind) 
    -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>, String> {
        match kind {
            ConnectionKind::Defining => {
                match self.connect_to(to.clone(), kind) {
                    Ok(connection) => {
                        match to.borrow_mut().connect_from_connection(connection) {
                            Ok(second_connection) => Ok(second_connection),
                            Err(e) => {
                                let msg = format!("error while creating second connection: {}", e);
                                log::error!("{}", msg);
                                Err(msg)
                            }        
                        }
                    }
                    Err(e) => {
                        let msg = format!("error while creating connection: {}", e);
                        log::error!("{}", msg);
                        Err(msg)
                    }
                }
            },
            _ => {
                let msg = "only defining connection Element -> SimpleNeuron can be created";
                log::error!("{}", msg);
                Err(msg.to_string())
            }
        }
    }

    fn connect_bilateral_from(&mut self, from: Rc<RefCell<SimpleNeuron>>, kind: ConnectionKind) 
    -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>, String> {
        match kind {
            ConnectionKind::Defining => {
                match self.connect_from(from.clone(), kind) {
                    Ok(connection) => {
                        match from.borrow_mut().connect_to_connection(connection) {
                            Ok(second_connection) => Ok(second_connection),
                            Err(e) => {
                                let msg = format!("error while creating second connection: {}", e);
                                log::error!("{}", msg);
                                Err(msg)
                            }        
                        }
                    }
                    Err(e) => {
                        let msg = format!("error while creating connection: {}", e);
                        log::error!("{}", msg);
                        Err(msg)
                    }
                }
            },
            _ => {
                let msg = "only defining connection Element -> SimpleNeuron can be created";
                log::error!("{}", msg);
                Err(msg.to_string())
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
        let parent_name: Rc<str> = Rc::from("test");
        let neuron_1 = SimpleNeuron::new(
            NeuronID { id: "neuron_1".into(), parent_id: parent_name.clone() }
        );
        let neuron_2 = SimpleNeuron::new(
            NeuronID { id: "neuron_2".into(), parent_id: parent_name.clone() }
        );

        let neuron_1_id = neuron_1.borrow().id();
        assert_eq!(neuron_1_id.id.to_string(), "neuron_1".to_string());
        assert_eq!(neuron_1_id.parent_id.to_string(), parent_name.to_string());
        let neuron_2_id = neuron_2.borrow().id();
        assert_eq!(neuron_2_id.id.to_string(), "neuron_2".to_string());
        assert_eq!(neuron_2_id.parent_id.to_string(), parent_name.to_string());

        assert_eq!(neuron_1.borrow().is_sensor(), false);

        assert_eq!(neuron_1.borrow().activation(), 0.0f32);
        assert_eq!(neuron_2.borrow().activation(), 0.0f32);

        assert_eq!(neuron_1.borrow().counter(), 1usize);
        
        let connection = neuron_1
            .borrow_mut().connect_to(neuron_2.clone(), ConnectionKind::Defining).unwrap();
        assert_eq!(
            connection.borrow().from().as_ptr() as *const () as usize, 
            neuron_1.as_ptr() as *const () as usize
        );
        assert_eq!(
            connection.borrow().to().as_ptr() as *const () as usize, 
            neuron_2.as_ptr() as *const () as usize
        );

        let activated = neuron_1.borrow_mut().activate(1.0f32, true, true);
        assert_eq!(activated.len(), 1);
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

        let exp_1 = neuron_1.borrow_mut().explain();
        assert_eq!(exp_1.len(), 0);
    }

    #[test]
    fn connect_from_neuron() {
        let parent_name: Rc<str> = Rc::from("test");
        let neuron_1 = SimpleNeuron::new(
            NeuronID { id: "neuron_1".into(), parent_id: parent_name.clone() }
        );
        let neuron_2 = SimpleNeuron::new(
            NeuronID { id: "neuron_2".into(), parent_id: parent_name.clone() }
        );

        let connection_1 = neuron_1.borrow_mut().connect_from(
            neuron_2.clone(), ConnectionKind::Defining
        );
        assert!(connection_1.is_ok());
        assert_eq!(neuron_1.borrow().defined_neurons().len(), 0);
        assert_eq!(neuron_1.borrow().defining_neurons().len(), 1);
        assert_eq!(neuron_1.borrow().defining_sensors().len(), 0);
        assert_eq!(neuron_2.borrow().defined_neurons().len(), 0);
        assert_eq!(neuron_2.borrow().defining_neurons().len(), 0);
        assert_eq!(neuron_2.borrow().defining_sensors().len(), 0);
        let connection_1 = connection_1.unwrap();
        let connection_2 = neuron_2.borrow_mut().connect_to_connection(connection_1.clone());
        assert!(connection_2.is_ok());
        assert_eq!(neuron_1.borrow().defined_neurons().len(), 0);
        assert_eq!(neuron_1.borrow().defining_neurons().len(), 1);
        assert_eq!(neuron_1.borrow().defining_sensors().len(), 0);
        assert_eq!(neuron_2.borrow().defined_neurons().len(), 1);
        assert_eq!(neuron_2.borrow().defining_neurons().len(), 0);
        assert_eq!(neuron_2.borrow().defining_sensors().len(), 0);
        assert_eq!(
            connection_1.borrow().from().as_ptr() as *const () as usize, 
            neuron_2.as_ptr() as *const () as usize
        );
        assert_eq!(
            connection_1.borrow().to().as_ptr() as *const () as usize, 
            neuron_1.as_ptr() as *const () as usize
        );
    }

    #[test]
    fn connect_to_neuron() {
        let parent_name: Rc<str> = Rc::from("test");
        let neuron_1 = SimpleNeuron::new(
            NeuronID { id: "neuron_1".into(), parent_id: parent_name.clone() }
        );
        let neuron_2 = SimpleNeuron::new(
            NeuronID { id: "neuron_2".into(), parent_id: parent_name.clone() }
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
        let connection_1 = connection_1.unwrap();
        let connection_2 = neuron_2.borrow_mut().connect_from_connection(connection_1.clone());
        assert!(connection_2.is_ok());
        assert_eq!(neuron_1.borrow().defined_neurons().len(), 1);
        assert_eq!(neuron_1.borrow().defining_neurons().len(), 0);
        assert_eq!(neuron_1.borrow().defining_sensors().len(), 0);
        assert_eq!(neuron_2.borrow().defined_neurons().len(), 0);
        assert_eq!(neuron_2.borrow().defining_neurons().len(), 1);
        assert_eq!(neuron_2.borrow().defining_sensors().len(), 0);
        assert_eq!(
            connection_1.borrow().from().as_ptr() as *const () as usize, 
            neuron_1.as_ptr() as *const () as usize
        );
        assert_eq!(
            connection_1.borrow().to().as_ptr() as *const () as usize, 
            neuron_2.as_ptr() as *const () as usize
        );
    }

    #[test]
    fn connect_from_sensor() {
        let parent_name:Rc<str> = Rc::from("test");
        let neuron_1 = SimpleNeuron::new(
            NeuronID { id: "neuron_1".into(), parent_id: parent_name.clone() }
        );
        let neuron_2: Rc<RefCell<Element<i32, 3>>> = Element::new(&1, &Rc::from("test"));

        let connection_1 = neuron_1.borrow_mut().connect_from(
            neuron_2.clone(), ConnectionKind::Defining
        );
        assert!(connection_1.is_ok());
        assert_eq!(neuron_1.borrow().defined_neurons().len(), 0);
        assert_eq!(neuron_1.borrow().defining_neurons().len(), 0);
        assert_eq!(neuron_1.borrow().defining_sensors().len(), 1);
        assert_eq!(neuron_2.borrow().defining_neurons().len(), 0);
        let connection_1 = connection_1.unwrap();
        let connection_2 = neuron_2.borrow_mut().connect_to_connection(connection_1.clone());
        assert!(connection_2.is_ok());
        assert_eq!(neuron_1.borrow().defined_neurons().len(), 0);
        assert_eq!(neuron_1.borrow().defining_neurons().len(), 0);
        assert_eq!(neuron_1.borrow().defining_sensors().len(), 1);
        assert_eq!(neuron_2.borrow().defining_neurons().len(), 1);
        assert_eq!(
            connection_1.borrow().from().as_ptr() as *const () as usize, 
            neuron_2.as_ptr() as *const () as usize
        );
        assert_eq!(
            connection_1.borrow().to().as_ptr() as *const () as usize, 
            neuron_1.as_ptr() as *const () as usize
        );
    }

    #[test]
    fn connect_to_sensor() {
        let parent_name: Rc<str> = Rc::from("test");
        let neuron_1 = SimpleNeuron::new(
            NeuronID { id: "neuron_1".into(), parent_id: parent_name.clone() }
        );
        let neuron_2: Rc<RefCell<Element<i32, 3>>> = Element::new(&1, &Rc::from("test"));

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
        let parent_name: Rc<str> = Rc::from("test");
        let neuron_1 = SimpleNeuron::new(
            NeuronID { id: "neuron_1".into(), parent_id: parent_name.clone() }
        );
        let neuron_2: Rc<RefCell<Element<i32, 3>>> = Element::new(&1, &Rc::from("test"));

        let connection_1 = neuron_1.borrow_mut().connect_bilateral_to(
            neuron_2.clone(), ConnectionKind::Defining
        );
        assert!(connection_1.is_err());
        assert_eq!(neuron_1.borrow().defined_neurons().len(), 0);
        assert_eq!(neuron_1.borrow().defining_neurons().len(), 0);
        assert_eq!(neuron_1.borrow().defining_sensors().len(), 0);
        assert_eq!(neuron_2.borrow().defining_neurons().len(), 0);
    }

    #[test]
    fn connect_bilateral_from_sensor() {
        let parent_name: Rc<str> = Rc::from("test");
        let neuron_1 = SimpleNeuron::new(
            NeuronID { id: "neuron_1".into(), parent_id: parent_name.clone() }
        );
        let neuron_2: Rc<RefCell<Element<i32, 3>>> = Element::new(&1, &Rc::from("test"));

        let connection_1 = neuron_1.borrow_mut().connect_bilateral_from(
            neuron_2.clone(), ConnectionKind::Defining
        );
        assert!(connection_1.is_ok());
        assert_eq!(neuron_1.borrow().defined_neurons().len(), 0);
        assert_eq!(neuron_1.borrow().defining_neurons().len(), 0);
        assert_eq!(neuron_1.borrow().defining_sensors().len(), 1);
        assert_eq!(neuron_2.borrow().defining_neurons().len(), 1);
    }

    #[test]
    fn connect_bilateral_to_neuron() {
        let parent_name: Rc<str> = Rc::from("test");
        let neuron_1 = SimpleNeuron::new(
            NeuronID { id: "neuron_1".into(), parent_id: parent_name.clone() }
        );
        let neuron_2 = SimpleNeuron::new(
            NeuronID { id: "neuron_2".into(), parent_id: parent_name.clone() }
        );

        let connection_1 = neuron_1.borrow_mut().connect_bilateral_to(
            neuron_2.clone(), ConnectionKind::Defining
        );
        assert!(connection_1.is_ok());
        assert_eq!(neuron_1.borrow().defined_neurons().len(), 1);
        assert_eq!(neuron_1.borrow().defining_neurons().len(), 0);
        assert_eq!(neuron_1.borrow().defining_sensors().len(), 0);
        assert_eq!(neuron_2.borrow().defined_neurons().len(), 0);
        assert_eq!(neuron_2.borrow().defining_neurons().len(), 1);
        assert_eq!(neuron_2.borrow().defining_sensors().len(), 0);
    }

    #[test]
    fn connect_bilateral_from_neuron() {
        let parent_name: Rc<str> = Rc::from("test");
        let neuron_1 = SimpleNeuron::new(
            NeuronID { id: "neuron_1".into(), parent_id: parent_name.clone() }
        );
        let neuron_2 = SimpleNeuron::new(
            NeuronID { id: "neuron_2".into(), parent_id: parent_name.clone() }
        );

        let connection_1 = neuron_1.borrow_mut().connect_bilateral_from(
            neuron_2.clone(), ConnectionKind::Defining
        );
        assert!(connection_1.is_ok());
        assert_eq!(neuron_1.borrow().defined_neurons().len(), 0);
        assert_eq!(neuron_1.borrow().defining_neurons().len(), 1);
        assert_eq!(neuron_1.borrow().defining_sensors().len(), 0);
        assert_eq!(neuron_2.borrow().defined_neurons().len(), 1);
        assert_eq!(neuron_2.borrow().defining_neurons().len(), 0);
        assert_eq!(neuron_2.borrow().defining_sensors().len(), 0);
    }

    #[test]
    fn connect_wrong() {
        let parent_name: Rc<str> = Rc::from("test");
        let neuron_1 = SimpleNeuron::new(
            NeuronID { id: "neuron_1".into(), parent_id: parent_name.clone() }
        );
        let neuron_2 = SimpleNeuron::new(
            NeuronID { id: "neuron_2".into(), parent_id: parent_name.clone() }
        );
        let sensor: Rc<RefCell<Element<i32, 3>>> = Element::new(&1, &Rc::from("test"));

        let connection_1 = neuron_1.borrow_mut().connect_bilateral_from(
            neuron_2.clone(), ConnectionKind::Inhibitory
        );
        assert!(connection_1.is_err());

        let connection_1 = neuron_1.borrow_mut().connect_bilateral_to(
            neuron_2.clone(), ConnectionKind::Inhibitory
        );
        assert!(connection_1.is_err());

        let connection_1 = neuron_1.borrow_mut().connect_to(
            neuron_2.clone(), ConnectionKind::Inhibitory
        );
        assert!(connection_1.is_err());

        let connection_1 = neuron_1.borrow_mut().connect_from(
            neuron_2.clone(), ConnectionKind::Inhibitory
        );
        assert!(connection_1.is_err());

        let connection_1 = neuron_1.borrow_mut().connect_from(
            sensor.clone(), ConnectionKind::Inhibitory
        );
        assert!(connection_1.is_err());

        let connection_1 = neuron_1.borrow_mut().connect_to(
            sensor.clone(), ConnectionKind::Inhibitory
        );
        assert!(connection_1.is_err());

        let connection_1 = neuron_1.borrow_mut().connect_bilateral_to(
            sensor.clone(), ConnectionKind::Inhibitory
        );
        assert!(connection_1.is_err());
    }
}