use std::{
    sync::Arc,
    rc::Rc,
    cell::RefCell,
    marker::PhantomData,
    path::Path
};

use polars::prelude::*;

use rand::{ thread_rng, seq::SliceRandom };

use asa_graphs::neural::graph::ASAGraph;

use witchnet_common::{
    polars::{ self as polars_common, DataVec, DataVecOption },
    neuron::{ Neuron, NeuronID },
    connection::{
        ConnectionKind,
        collective::defining::{ DefiningWeightingStrategy, ConstantOneWeight }
    },
    sensor::{ Sensor, SensorData },
    data::{ DataDeductor, DataTypeValue, DataType }
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
    magds: &mut MAGDS, 
    name: &str, 
    data: &DataVec,
    weighting_strategy: Rc<dyn DefiningWeightingStrategy>,
    interelement_activation_threshold: f32,
    interelement_activation_exponent: i32
) -> (Rc<RefCell<SensorConatiner>>, u32) {
    let new_id: u32 = *magds.sensors.keys().max().unwrap_or(&0) + 1;
    match data {
        DataVec::Unknown => {
            panic!("can't parse vec data type for sensor {name}")
        }
        DataVec::BoolVec(vec) => {
            let graph = 
                ASAGraph::<_>::new_box_from_vec_custom(
                    new_id, 
                    vec, 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                ) as Box<dyn Sensor<bool>>;
            magds.add_sensor(name, Rc::new(RefCell::new(graph.into())))
        }
        DataVec::UInt8Vec(vec) => {
            let graph = 
                ASAGraph::<_>::new_box_from_vec_custom(
                    new_id, 
                    vec, 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                ) as Box<dyn Sensor<u8>>;
            magds.add_sensor(name, Rc::new(RefCell::new(graph.into())))
        }
        DataVec::UInt16Vec(vec) => {
            let graph = 
                ASAGraph::<_>::new_box_from_vec_custom(
                    new_id, 
                    vec, 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                ) as Box<dyn Sensor<u16>>;
            magds.add_sensor(name, Rc::new(RefCell::new(graph.into())))
        }
        DataVec::UInt32Vec(vec) => {
            let graph = 
                ASAGraph::<_>::new_box_from_vec_custom(
                    new_id, 
                    vec, 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                ) as Box<dyn Sensor<u32>>;
            magds.add_sensor(name, Rc::new(RefCell::new(graph.into())))
        }
        DataVec::UInt64Vec(vec) => {
            let graph = 
                ASAGraph::<_>::new_box_from_vec_custom(
                    new_id, 
                    vec, 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                ) as Box<dyn Sensor<u64>>;
            magds.add_sensor(name, Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Int8Vec(vec) => {
            let graph = 
                ASAGraph::<_>::new_box_from_vec_custom(
                    new_id, 
                    vec, 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                ) as Box<dyn Sensor<i8>>;
            magds.add_sensor(name, Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Int16Vec(vec) => {
            let graph = 
                ASAGraph::<_>::new_box_from_vec_custom(
                    new_id, 
                    vec, 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                ) as Box<dyn Sensor<i16>>;
            magds.add_sensor(name, Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Int32Vec(vec) => {
            let graph = 
                ASAGraph::<_>::new_box_from_vec_custom(
                    new_id, 
                    vec, 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                ) as Box<dyn Sensor<i32>>;
            magds.add_sensor(name, Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Int64Vec(vec) => {
            let graph = 
                ASAGraph::<_>::new_box_from_vec_custom(
                    new_id, 
                    vec, 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                ) as Box<dyn Sensor<i64>>;
            magds.add_sensor(name, Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Float32Vec(vec) => {
            let graph = 
                ASAGraph::<_>::new_box_from_vec_custom(
                    new_id, 
                    vec, 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                ) as Box<dyn Sensor<f32>>;
            magds.add_sensor(name, Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Float64Vec(vec) => {
            let graph = 
                ASAGraph::<_>::new_box_from_vec_custom(
                    new_id, 
                    vec, 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                ) as Box<dyn Sensor<f64>>;
            magds.add_sensor(name, Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Utf8Vec(vec) => {
            let graph = 
                ASAGraph::<Arc<str>>::new_box_from_vec_custom(
                    new_id, 
                    vec, 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                ) as Box<dyn Sensor<Arc<str>>>;
            magds.add_sensor(name, Rc::new(RefCell::new(graph.into())))
        }
    }
}

pub(crate) fn connected_sensor_from_datavec(
    mut magds: &mut MAGDS, 
    name: &str, 
    data: &DataVecOption, 
    neurons: &[Rc<RefCell<SimpleNeuron>>],
    weighting_strategy: Rc<dyn DefiningWeightingStrategy>,
    interelement_activation_threshold: f32,
    interelement_activation_exponent: i32
) -> (Rc<RefCell<SensorConatiner>>, u32) {
    match data {
        DataVecOption::Unknown => {
            panic!("can't parse vec data type for sensor {name}")
        }
        DataVecOption::BoolVec(vec) => {
            connector(
                &mut magds, 
                name, 
                DataType::Bool, 
                vec, 
                neurons, 
                weighting_strategy,
                interelement_activation_threshold,
                interelement_activation_exponent
            )
        }
        DataVecOption::UInt8Vec(vec) => {
            connector(
                &mut magds, 
                name, 
                DataType::U8, 
                vec, 
                neurons, 
                weighting_strategy,
                interelement_activation_threshold,
                interelement_activation_exponent
            )
        }
        DataVecOption::UInt16Vec(vec) => {
            connector(
                &mut magds, 
                name, 
                DataType::U16, 
                vec, 
                neurons, 
                weighting_strategy,
                interelement_activation_threshold,
                interelement_activation_exponent
            )
        }
        DataVecOption::UInt32Vec(vec) => {
            connector(
                &mut magds, 
                name, 
                DataType::U32, 
                vec, 
                neurons, 
                weighting_strategy,
                interelement_activation_threshold,
                interelement_activation_exponent
            )
        }
        DataVecOption::UInt64Vec(vec) => {
            connector(
                &mut magds, 
                name, 
                DataType::U64, 
                vec, 
                neurons, 
                weighting_strategy,
                interelement_activation_threshold,
                interelement_activation_exponent
            )
        }
        DataVecOption::Int8Vec(vec) => {
            connector(
                &mut magds, 
                name, 
                DataType::I8, 
                vec, 
                neurons, 
                weighting_strategy,
                interelement_activation_threshold,
                interelement_activation_exponent
            )
        }
        DataVecOption::Int16Vec(vec) => {
            connector(
                &mut magds, 
                name, 
                DataType::I16, 
                vec, 
                neurons, 
                weighting_strategy,
                interelement_activation_threshold,
                interelement_activation_exponent
            )
        }
        DataVecOption::Int32Vec(vec) => {
            connector(
                &mut magds, 
                name, 
                DataType::I32, 
                vec, 
                neurons, 
                weighting_strategy,
                interelement_activation_threshold,
                interelement_activation_exponent
            )
        }
        DataVecOption::Int64Vec(vec) => {
            connector(
                &mut magds, 
                name, 
                DataType::I64, 
                vec, 
                neurons, 
                weighting_strategy,
                interelement_activation_threshold,
                interelement_activation_exponent
            )
        }
        DataVecOption::Float32Vec(vec) => {
            connector(
                &mut magds, 
                name, 
                DataType::F32, 
                vec, 
                neurons, 
                weighting_strategy,
                interelement_activation_threshold,
                interelement_activation_exponent
            )
        }
        DataVecOption::Float64Vec(vec) => {
            connector(
                &mut magds, 
                name, 
                DataType::F64, 
                vec, 
                neurons, 
                weighting_strategy,
                interelement_activation_threshold,
                interelement_activation_exponent
            )
        }
        DataVecOption::Utf8Vec(vec) => {
            connector_string(
                &mut magds, 
                name, 
                DataType::ArcStr, 
                vec, 
                neurons, 
                weighting_strategy,
                interelement_activation_threshold,
                interelement_activation_exponent
            )
        }
    }
}

fn connector_string(
    magds: &mut MAGDS, 
    name: &str,
    data_type: DataType,
    vec: &[Option<Arc<str>>], 
    neurons: &[Rc<RefCell<SimpleNeuron>>],
    weighting_strategy: Rc<dyn DefiningWeightingStrategy>,
    interelement_activation_threshold: f32,
    interelement_activation_exponent: i32
) -> (Rc<RefCell<SensorConatiner>>, u32) 
where 
    PhantomData<String>: DataDeductor, 
    SensorConatiner: From<Box<dyn Sensor<Arc<str>>>>,
    DataTypeValue: From<Arc<str>>
{
    assert_eq!(neurons.len(), vec.len());
    let (sensor, id) = if let Some(ids) = magds.sensor_ids(name) {
        if let Some(id) = ids.first() {
            if let Some(sensor) = magds.sensor(*id) {
                (sensor.clone(), *id)
            } else { magds.create_sensor(name, data_type) }
        } else { magds.create_sensor(name, data_type) }
    } else { magds.create_sensor(name, data_type) };

    for (i, key) in vec.into_iter().enumerate() {
        if let Some(key) = key {
            if key.as_ref() == "" { continue }
            
            let neuron_ptr = neurons[i].clone();

            let key_vec: Vec<_> = polars_common::string_to_vec(key)
                .into_iter()
                .map(|x| Arc::<str>::from(x))
                .collect();
            for key in key_vec {
                let element = sensor.borrow_mut().insert_custom(
                    &key.into(), 
                    weighting_strategy.clone(),
                    interelement_activation_threshold,
                    interelement_activation_exponent
                );
                let mut element = element.borrow_mut();
                if let Err(e) = element.connect_bilateral(
                    neuron_ptr.clone(), false, ConnectionKind::Defining
                ) {
                    log::error!(
                        "error connecting neuron {} with sensor {}, error: {e}", 
                        neuron_ptr.borrow(), 
                        element
                    );
                }
            }
        } else {
            continue
        }
    }
    
    (sensor.clone(), id)
}

fn connector<T: SensorData>(
    magds: &mut MAGDS, 
    name: &str,
    data_type: DataType,
    vec: &[Option<T>], 
    neurons: &[Rc<RefCell<SimpleNeuron>>],
    weighting_strategy: Rc<dyn DefiningWeightingStrategy>,
    interelement_activation_threshold: f32,
    interelement_activation_exponent: i32
) -> (Rc<RefCell<SensorConatiner>>, u32)
where 
    PhantomData<T>: DataDeductor, 
    SensorConatiner: From<Box<dyn Sensor<T>>>,
    DataTypeValue: From<T>
{
    assert_eq!(neurons.len(), vec.len());
    let (sensor, id) = if let Some(ids) = magds.sensor_ids(name) {
        if let Some(id) = ids.first() {
            if let Some(sensor) = magds.sensor(*id) {
                (sensor.clone(), *id)
            } else { magds.create_sensor(name, data_type) }
        } else { magds.create_sensor(name, data_type) }
    } else { magds.create_sensor(name, data_type) };
    
    for (i, key) in vec.into_iter().enumerate() {
        if let Some(key) = key {
            let key_converted = &(*dyn_clone::clone_box(key)).into();
            let element = sensor.borrow_mut().insert_custom(
                key_converted, 
                weighting_strategy.clone(),
                interelement_activation_threshold,
                interelement_activation_exponent
            );
            let mut element = element.borrow_mut();
            let neuron_ptr = neurons[i].clone();
            if let Err(e) = element.connect_bilateral(
                neuron_ptr.clone(), false, ConnectionKind::Defining
            ) {
                log::error!(
                    "error connecting neuron {} with sensor {}, error: {e}", 
                    neuron_ptr.borrow(), 
                    element
                );
            }
        } else {
            continue
        }
    }
    
    (sensor.clone(), id)
}

pub fn magds_from_df(
    df_name: &str, 
    df: &DataFrame
) -> MAGDS {
    let mut magds = MAGDS::new();
    add_df_to_magds(
        &mut magds, 
        df_name, 
        df, 
        &vec![], 
        0, 
        false,
        Rc::new(ConstantOneWeight),
        0.00001,
        1
    );
    magds
}

pub fn magds_from_df_custom(
    df_name: &str, 
    df: &DataFrame, 
    skip_columns: &[&str],
    limit: usize, 
    random: bool, 
    weighting_strategy: Rc<dyn DefiningWeightingStrategy>,
    interelement_activation_threshold: f32,
    interelement_activation_exponent: i32
) -> MAGDS {
    let mut magds = MAGDS::new();
    add_df_to_magds(
        &mut magds,
        df_name,
        df,
        skip_columns,
        limit,
        random,
        weighting_strategy,
        interelement_activation_threshold,
        interelement_activation_exponent
    );
    magds
}

pub fn add_df_to_magds(
    magds: &mut MAGDS, 
    df_name: &str, 
    df: &DataFrame,
    skip_columns: &[&str],
    limit: usize, 
    random: bool,
    weighting_strategy: Rc<dyn DefiningWeightingStrategy>,
    interelement_activation_threshold: f32,
    interelement_activation_exponent: i32
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
        let neuron = SimpleNeuron::new_custom(
            NeuronID{ id: i as u32, parent_id: neuron_group_id },
            weighting_strategy.clone()
        );
        neurons.push(neuron.clone());
        magds.add_neuron(neuron as Rc<RefCell<dyn Neuron>>);
    }
    magds.add_neuron_group(df_name, Some(neuron_group_id));

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
            connected_sensor_from_datavec(
                magds,
                column_name,
                &datavec,
                &neurons,
                weighting_strategy.clone(),
                interelement_activation_threshold,
                interelement_activation_exponent
            );
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

pub fn magds_from_csv_custom(
    name: &str, 
    file_path: &str, 
    skip: &[&str],
    weighting_strategy: Rc<dyn DefiningWeightingStrategy>,
    interelement_activation_threshold: f32,
    interelement_activation_exponent: i32
) -> Option<MAGDS> {
    let path = Path::new(file_path);
    if !path.is_file() || !file_path.ends_with(".csv") { return None }
    let df = polars_common::csv_to_dataframe(file_path, &skip).ok()?;
    let magds = magds_from_df_custom(
        name,
        &df,
        &vec![],
        0,
        false, weighting_strategy,
        interelement_activation_threshold,
        interelement_activation_exponent
    );
    Some(magds)
}

#[cfg(test)]
mod tests {
    use std::{
        sync::Arc,
        rc::Rc
    };
    
    use polars::datatypes::DataType;

    use witchnet_common::{
        polars as polars_common,
        sensor::Sensor,
        data::DataTypeValue, 
        connection::collective::defining::ConstantOneWeight
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

        let iris_neuron_group_id = *magds.neuron_group_ids_from_name("iris").unwrap().first().unwrap();
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

        let weighting_strategy = Rc::new(ConstantOneWeight);

        let variety_df = df.column("variety").unwrap();
        assert_eq!(*variety_df.dtype(), DataType::Utf8);
        let variety_df_datavec = polars_common::series_to_datavec_skipna(variety_df).unwrap();
        let variety_graph = super::sensor_from_datavec(
            &mut magds, "variety", &variety_df_datavec, weighting_strategy.clone(), 0.00001, 1
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
            &mut magds, "sepal.length", &sepal_length_df_datavec, weighting_strategy.clone(), 0.00001, 1
        );

        let sepal_length_sensor_id = *magds.sensor_ids("sepal.length").unwrap().first().unwrap();
        println!("{}", sepal_length_graph.0.borrow());
        let sepal_length_graph_from_magds = magds.sensor(sepal_length_sensor_id).unwrap();
        let sepal_length_result = sepal_length_graph_from_magds.borrow().search(&5.8_f64.into());
        assert!(sepal_length_result.is_some());
        assert_eq!(sepal_length_result.unwrap().borrow().counter(), 7);
    }
}