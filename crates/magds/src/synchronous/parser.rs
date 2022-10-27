use std::{
    sync::Arc,
    rc::Rc,
    cell::RefCell,
    marker::PhantomData,
    path::Path
};

use regex::Regex;

use polars::prelude::*;

use rand::{ thread_rng, seq::SliceRandom };

use asa_graphs::neural::{
    element::Element,
    graph::ASAGraph
};

use witchnet_common::{
    polars::{ self as polars_common, DataVec, DataVecOption },
    neuron::{ Neuron, NeuronID, NeuronConnectBilateral },
    connection::ConnectionKind,
    sensor::{ Sensor, SensorData },
    data::{ DataDeductor, DataTypeValue }
};

use crate::{
    neuron::simple_neuron::SimpleNeuron,
    synchronous::{
        magds::MAGDS,
        sensor::SensorConatiner
    }
};

#[allow(dead_code)]
pub(crate) fn sensor_from_datavec(
    magds: &mut MAGDS, name: &str, data: &DataVec
) -> (Rc<RefCell<SensorConatiner>>, u32) {
    let new_id: u32 = *magds.sensors.keys().max().unwrap_or(&0) + 1;
    match data {
        DataVec::Unknown => {
            panic!("can't parse vec data type for sensor {name}")
        }
        DataVec::BoolVec(vec) => {
            let graph = 
                ASAGraph::<_>::new_box_from_vec(new_id, vec) as Box<dyn Sensor<bool>>;
            magds.add_sensor(name, Rc::new(RefCell::new(graph.into())))
        }
        DataVec::UInt8Vec(vec) => {
            let graph = 
                ASAGraph::<_>::new_box_from_vec(new_id, vec) as Box<dyn Sensor<u8>>;
            magds.add_sensor(name, Rc::new(RefCell::new(graph.into())))
        }
        DataVec::UInt16Vec(vec) => {
            let graph = 
                ASAGraph::<_>::new_box_from_vec(new_id, vec) as Box<dyn Sensor<u16>>;
            magds.add_sensor(name, Rc::new(RefCell::new(graph.into())))
        }
        DataVec::UInt32Vec(vec) => {
            let graph = 
                ASAGraph::<_>::new_box_from_vec(new_id, vec) as Box<dyn Sensor<u32>>;
            magds.add_sensor(name, Rc::new(RefCell::new(graph.into())))
        }
        DataVec::UInt64Vec(vec) => {
            let graph = 
                ASAGraph::<_>::new_box_from_vec(new_id, vec) as Box<dyn Sensor<u64>>;
            magds.add_sensor(name, Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Int8Vec(vec) => {
            let graph = 
                ASAGraph::<_>::new_box_from_vec(new_id, vec) as Box<dyn Sensor<i8>>;
            magds.add_sensor(name, Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Int16Vec(vec) => {
            let graph = 
                ASAGraph::<_>::new_box_from_vec(new_id, vec) as Box<dyn Sensor<i16>>;
            magds.add_sensor(name, Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Int32Vec(vec) => {
            let graph = 
                ASAGraph::<_>::new_box_from_vec(new_id, vec) as Box<dyn Sensor<i32>>;
            magds.add_sensor(name, Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Int64Vec(vec) => {
            let graph = 
                ASAGraph::<_>::new_box_from_vec(new_id, vec) as Box<dyn Sensor<i64>>;
            magds.add_sensor(name, Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Float32Vec(vec) => {
            let graph = 
                ASAGraph::<_>::new_box_from_vec(new_id, vec) as Box<dyn Sensor<f32>>;
            magds.add_sensor(name, Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Float64Vec(vec) => {
            let graph = 
                ASAGraph::<_>::new_box_from_vec(new_id, vec) as Box<dyn Sensor<f64>>;
            magds.add_sensor(name, Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Utf8Vec(vec) => {
            let graph = 
                ASAGraph::<Arc<str>>::new_box_from_vec(new_id, vec) as Box<dyn Sensor<Arc<str>>>;
            magds.add_sensor(name, Rc::new(RefCell::new(graph.into())))
        }
    }
}

pub(crate) fn connected_sensor_from_datavec(
    mut magds: &mut MAGDS, name: &str, data: &DataVecOption, neurons: &[Rc<RefCell<SimpleNeuron>>]
) -> (Rc<RefCell<SensorConatiner>>, u32) {
    let new_id: u32 = *magds.sensors.keys().max().unwrap_or(&0) + 1;
    match data {
        DataVecOption::Unknown => {
            panic!("can't parse vec data type for sensor {name}")
        }
        DataVecOption::BoolVec(vec) => { connector(&mut magds, name, new_id, vec, neurons) }
        DataVecOption::UInt8Vec(vec) => { connector(&mut magds, name, new_id, vec, neurons) }
        DataVecOption::UInt16Vec(vec) => { connector(&mut magds, name, new_id, vec, neurons) }
        DataVecOption::UInt32Vec(vec) => { connector(&mut magds, name, new_id, vec, neurons) }
        DataVecOption::UInt64Vec(vec) => { connector(&mut magds, name, new_id, vec, neurons) }
        DataVecOption::Int8Vec(vec) => { connector(&mut magds, name, new_id, vec, neurons) }
        DataVecOption::Int16Vec(vec) => { connector(&mut magds, name, new_id, vec, neurons) }
        DataVecOption::Int32Vec(vec) => { connector(&mut magds, name, new_id, vec, neurons) }
        DataVecOption::Int64Vec(vec) => { connector(&mut magds, name, new_id, vec, neurons) }
        DataVecOption::Float32Vec(vec) => { connector(&mut magds, name, new_id, vec, neurons) }
        DataVecOption::Float64Vec(vec) => { connector(&mut magds, name, new_id, vec, neurons) }
        DataVecOption::Utf8Vec(vec) => { connector_string(&mut magds, name, new_id, vec, neurons) }
    }
}

fn connector_string(
    magds: &mut MAGDS, 
    name: &str,
    id: u32,
    vec: &[Option<Arc<str>>], 
    neurons: &[Rc<RefCell<SimpleNeuron>>]
) -> (Rc<RefCell<SensorConatiner>>, u32) 
where 
    PhantomData<String>: DataDeductor, 
    SensorConatiner: From<Box<dyn Sensor<Arc<str>>>>,
    DataTypeValue: From<Arc<str>>
{
    assert_eq!(neurons.len(), vec.len());
    let mut sensor = ASAGraph::<Arc<str>>::new_box(id);
    for (i, key) in vec.into_iter().enumerate() {
        if let Some(key) = key {
            if key.as_ref() == "" { continue }
            
            let neuron_ptr = neurons[i].clone();

            if key.starts_with("[") && key.ends_with("]") {
                let key = key.strip_prefix("[").unwrap().strip_suffix("]").unwrap();
                let key_vec: Vec<_> = Regex::new(r"\s*,\s*")
                    .unwrap()
                    .split(key)
                    .map(|x| {
                        Arc::<str>::from(
                            Regex::new(r#"["']+"#).unwrap()
                                .split(x)
                                .filter(|x| *x != "")
                                .next().unwrap()
                        )
                    }).collect();
                for key in key_vec {
                    let element = sensor.insert(&key);
                    if let Err(e) = Element::connect_bilateral(
                        element.clone(), neuron_ptr.clone(), ConnectionKind::Defining
                    ) {
                        log::error!(
                            "error connecting neuron {} with sensor {}, error: {e}", 
                            neuron_ptr.borrow(), 
                            element.borrow()
                        );
                    }
                }
            } else {
                let element = sensor.insert(key);
                if let Err(e) = Element::connect_bilateral(
                    element.clone(), neuron_ptr.clone(), ConnectionKind::Defining
                ) {
                    log::error!(
                        "error connecting neuron {} with sensor {}, error: {e}", 
                        neuron_ptr.borrow(), 
                        element.borrow()
                    );
                }
            }
        } else {
            continue
        }
    }
    magds.add_sensor(
        name, Rc::new(RefCell::new((sensor as Box<dyn Sensor<Arc<str>>>).into()))
    )
}

fn connector<T: SensorData>(
    magds: &mut MAGDS, 
    name: &str,
    id: u32,
    vec: &[Option<T>], 
    neurons: &[Rc<RefCell<SimpleNeuron>>]
) -> (Rc<RefCell<SensorConatiner>>, u32)
where 
    PhantomData<T>: DataDeductor, 
    SensorConatiner: From<Box<dyn Sensor<T>>>,
    DataTypeValue: From<T>
{
    assert_eq!(neurons.len(), vec.len());
    let mut sensor = ASAGraph::<T>::new_box(id);
    for (i, key) in vec.into_iter().enumerate() {
        if let Some(key) = key {
            let element = sensor.insert(key);
            let neuron_ptr = neurons[i].clone();
            if let Err(e) = Element::connect_bilateral(
                element.clone(), neuron_ptr.clone(), ConnectionKind::Defining
            ) {
                log::error!(
                    "error connecting neuron {} with sensor {}, error: {e}", 
                    neuron_ptr.borrow(), 
                    element.borrow()
                );
            }
        } else {
            continue
        }
    }
    magds.add_sensor(
        name, Rc::new(RefCell::new((sensor as Box<dyn Sensor<T>>).into()))
    )
}

pub fn magds_from_df(df_name: &str, df: &DataFrame) -> MAGDS {
    let mut magds = MAGDS::new();
    add_df_to_magds(&mut magds, df_name, df, &vec![], 0, false);
    magds
}

pub fn magds_from_df_limit(
    df_name: &str, 
    df: &DataFrame, 
    skip_columns: &[&str],
    limit: usize, 
    random: bool
) -> MAGDS {
    let mut magds = MAGDS::new();
    add_df_to_magds(&mut magds, df_name, df, skip_columns, limit, random);
    magds
}

pub fn add_df_to_magds(
    magds: &mut MAGDS, 
    df_name: &str, 
    df: &DataFrame,
    skip_columns: &[&str],
    limit: usize, 
    random: bool
) {
    log::info!("magds_from_df: df size: {} (cols) x {} (rows)", df.width(), df.height());
    log::info!("magds_from_df: df columns: {:?}", df.get_column_names());
    
    let neuron_group_id: u32 = *magds.neuron_group_names.keys().max().unwrap_or(&0) + 1;
    let mut neurons: Vec<Rc<RefCell<SimpleNeuron>>> = Vec::new();

    let mut all_indices: Vec<usize> = (0..df.height()).collect();
    if random { all_indices.shuffle(&mut thread_rng()) };
    let limit = if limit == 0 { df.height() } else { usize::min(limit, df.height()) };
    let indices = &all_indices[0..limit];
    
    for i in indices {
        let i = i + 1;
        let neuron = SimpleNeuron::new(
            NeuronID{ id: i as u32, parent_id: neuron_group_id }
        );
        neurons.push(neuron.clone());
        magds.add_neuron(neuron as Rc<RefCell<dyn Neuron>>);
    }
    magds.add_neuron_group(df_name, neuron_group_id);

    for column in df.get_columns() {
        if !skip_columns.contains(&column.name()) {
            let column = column.take_iter(&mut indices.into_iter().map(|x| *x)).unwrap();
            let column_name = column.name();
            let datavec = match polars_common::series_to_datavec(&column) {
                Ok(v) => v,
                Err(e) => { 
                    log::error!("error convering {column_name} to datavec, error: {e}");
                    continue
                }
            };
            connected_sensor_from_datavec(magds, column_name, &datavec, &neurons);
        }
    }
}

pub fn magds_from_csv(name: &str, file_path: &str, skip: &[&str]) -> Option<MAGDS> {
    let path = Path::new(file_path);
    if !path.is_file() || !file_path.ends_with(".csv") { return None }
    let df = polars_common::csv_to_dataframe(file_path, &skip).ok()?;
    let magds = magds_from_df(name, &df);
    Some(magds)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    
    use polars::datatypes::DataType;

    use witchnet_common::{
        polars as polars_common,
        sensor::Sensor,
        data::DataTypeValue
    };

    use crate::synchronous::magds::MAGDS;

    #[test]
    fn vec_parse() {
        let magds = super::magds_from_csv("lists", "data/lists.csv", &vec![]).unwrap();
        let x_sensor_id = *magds.sensor_ids("x").unwrap().first().unwrap();
        let y_sensor_id = *magds.sensor_ids("y").unwrap().first().unwrap();
        let z_sensor_id = *magds.sensor_ids("z").unwrap().first().unwrap();
        assert!(magds.sensor_search(x_sensor_id, &DataTypeValue::ArcStr("a".into())).is_some());
        assert!(magds.sensor_search(x_sensor_id, &DataTypeValue::ArcStr("b".into())).is_some());
        assert!(magds.sensor_search(y_sensor_id, &DataTypeValue::ArcStr("a".into())).is_some());
        assert!(magds.sensor_search(y_sensor_id, &DataTypeValue::ArcStr("b".into())).is_some());
        assert!(magds.sensor_search(z_sensor_id, &DataTypeValue::ArcStr("a".into())).is_some());
        assert!(magds.sensor_search(z_sensor_id, &DataTypeValue::ArcStr("b".into())).is_some());
        println!("{magds}");
    }

    #[test]
    fn csv_to_magds() {
        let magds = super::magds_from_csv("iris", "data/iris.csv", &vec![]).unwrap();
        println!("{magds}");

        let variety_sensor_id = *magds.sensor_ids("variety").unwrap().first().unwrap();
        let versicolor = 
            magds.sensor_search(variety_sensor_id, &Arc::<str>::from("Versicolor").into()).unwrap();
        let setosa = 
            magds.sensor_search(variety_sensor_id, &Arc::<str>::from("Setosa").into()).unwrap();
        let virginica = 
            magds.sensor_search(variety_sensor_id, &Arc::<str>::from("Virginica").into()).unwrap();
        assert_eq!(setosa.borrow().counter(), 49);
        assert_eq!(versicolor.borrow().counter(), 50);
        assert_eq!(virginica.borrow().counter(), 50);
    }

    #[test]
    fn df_to_magds() {
        let df = polars_common::csv_to_dataframe("data/iris.csv", &vec![]).unwrap();
        let magds = super::magds_from_df("iris", &df);
        println!("{magds}");

        let variety_sensor_id = *magds.sensor_ids("variety").unwrap().first().unwrap();
        let sepal_length_sensor_id = *magds.sensor_ids("sepal.length").unwrap().first().unwrap();
        let petal_length_sensor_id = *magds.sensor_ids("petal.length").unwrap().first().unwrap();
        let petal_width_sensor_id = *magds.sensor_ids("petal.width").unwrap().first().unwrap();
        let sepal_width_sensor_id = *magds.sensor_ids("sepal.width").unwrap().first().unwrap();

        let versicolor = 
            magds.sensor_search(variety_sensor_id, &Arc::<str>::from("Versicolor").into()).unwrap();
        let setosa = 
            magds.sensor_search(variety_sensor_id, &Arc::<str>::from("Setosa").into()).unwrap();
        let virginica = 
            magds.sensor_search(variety_sensor_id, &Arc::<str>::from("Virginica").into()).unwrap();
        assert_eq!(setosa.borrow().counter(), 49);
        assert_eq!(versicolor.borrow().counter(), 50);
        assert_eq!(virginica.borrow().counter(), 50);

        let sl58 = magds.sensor_search(sepal_length_sensor_id, &5.8_f64.into()).unwrap();
        assert_eq!(sl58.borrow().counter(), 7);

        let iris_neuron_group_id = *magds.neuron_group_ids("iris").unwrap().first().unwrap();
        let neuron_1 = magds.neuron(1, iris_neuron_group_id).unwrap();
        println!("neuron_1 {}", neuron_1.borrow());

        for sensor in neuron_1.borrow().explain() {
            let id = sensor.borrow().id();
            println!("sensor {id} {}", sensor.borrow().value());
            if id.parent_id == petal_length_sensor_id {
                assert_eq!(sensor.borrow().value(), 1.4.into());
            } else if id.parent_id == petal_width_sensor_id {
                assert_eq!(sensor.borrow().value(), 0.2.into());
            } else if id.parent_id == sepal_width_sensor_id {
                assert_eq!(sensor.borrow().value(), 3.5.into());
            } else if id.parent_id == variety_sensor_id {
                assert_eq!(sensor.borrow().value(), Arc::<str>::from("Setosa").into());
            } else if id.parent_id == sepal_length_sensor_id {
                panic!()
            } 
        }

        let neuron_2 = magds.neuron(2, iris_neuron_group_id).unwrap();
        println!("neuron_2 {}", neuron_2.borrow());
        for sensor in neuron_2.borrow().explain() {
            let id = sensor.borrow().id();
            println!("sensor {id} {}", sensor.borrow().value());
            if id.parent_id == petal_length_sensor_id {
                assert_eq!(sensor.borrow().value(), 1.4.into());
            } else if id.parent_id == petal_width_sensor_id {
                assert_eq!(sensor.borrow().value(), 0.2.into());
            } else if id.parent_id == sepal_width_sensor_id {
                assert_eq!(sensor.borrow().value(), 3.0.into());
            } else if id.parent_id == variety_sensor_id {
                assert_eq!(sensor.borrow().value(), Arc::<str>::from("Setosa").into());
            } else if id.parent_id == sepal_length_sensor_id {
                assert_eq!(sensor.borrow().value(), 4.9.into());
            } 
        }
    }

    #[test]
    fn csv_to_sensors() {
        let mut magds = MAGDS::new();

        let df = polars_common::csv_to_dataframe("data/iris.csv", &vec![]);
        assert!(df.is_ok());
        let df = df.unwrap();
        println!("{}", df);

        let variety_df = df.column("variety").unwrap();
        assert_eq!(*variety_df.dtype(), DataType::Utf8);
        let variety_df_datavec = polars_common::series_to_datavec_skipna(variety_df).unwrap();
        let variety_graph = super::sensor_from_datavec(
            &mut magds, "variety", &variety_df_datavec
        );
        let variety_sensor_id = *magds.sensor_ids("variety").unwrap().first().unwrap();
        println!("{}", variety_graph.0.borrow());
        let variety_from_magds = magds.sensor(variety_sensor_id).unwrap();
        let versicolor_result = variety_from_magds.borrow().search(
            &Arc::<str>::from("Versicolor").into()
        );
        assert!(versicolor_result.is_some());
        assert_eq!(versicolor_result.unwrap().borrow().counter(), 50);
        
        let sepal_length_df = df.column("sepal.length").unwrap();
        assert_eq!(*sepal_length_df.dtype(), DataType::Float64);
        let sepal_length_df_datavec = 
            polars_common::series_to_datavec_skipna(sepal_length_df).unwrap();
        let sepal_length_graph = super::sensor_from_datavec(
            &mut magds, "sepal.length", &sepal_length_df_datavec
        );

        let sepal_length_sensor_id = *magds.sensor_ids("sepal.length").unwrap().first().unwrap();
        println!("{}", sepal_length_graph.0.borrow());
        let sepal_length_graph_from_magds = magds.sensor(sepal_length_sensor_id).unwrap();
        let sepal_length_result = sepal_length_graph_from_magds.borrow().search(&5.8_f64.into());
        assert!(sepal_length_result.is_some());
        assert_eq!(sepal_length_result.unwrap().borrow().counter(), 7);
    }
}