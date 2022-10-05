use std::{
    collections::{ HashMap, BTreeMap },
    rc::Rc,
    cell::RefCell
};

use ordered_float::OrderedFloat;

use witchnet_common::{
    data::{ DataTypeValue, DataTypeValueStr, DataCategory },
    neuron::{ Neuron, NeuronID }, distances::Distance,
    sensor::Sensor,
    polars::{ self as polars_common, DataVecOption }
};
use polars::{
    prelude::*,
    export::num::ToPrimitive
};

use crate::simple::magds::MAGDS;

pub fn predict(
    magds: &mut MAGDS, 
    features: &Vec<(Rc<str>, DataTypeValue)>,
    target: Rc<str>,
    fuzzy: bool
) -> Option<(DataTypeValue, f64)> {
    let mut neurons: HashMap<NeuronID, Rc<RefCell<dyn Neuron>>> = HashMap::new();

    for (id, value) in features {
        let sensor = match magds.sensor_search(id.clone(), value) {
            Some(s) => s,
            None => {
                match magds.sensor_data_category(id.clone()) {
                    Some(DataCategory::Numerical) | Some(DataCategory::Ordinal) => {
                        if fuzzy {
                            log::info!("cannot find sensor {id} value {:?}, inserting", value);
                            match magds.sensor_insert(id.clone(), value) {
                                Some(s) => s,
                                None => {
                                    log::warn!("cannot insert {:?} to {id}, skipping", value);
                                    continue
                                }
                            }
                        } else {
                            log::warn!("cannot find sensor {id} for value {:?}, skipping", value);
                            continue
                        }
                    }
                    _ => {
                        log::warn!("cannot find sensor {id} for value {:?}, skipping", value);
                        continue
                    }
                }
            }
        };
        neurons.extend(sensor.borrow_mut().activate(1.0_f32, fuzzy, true));
    }

    if neurons.is_empty() { return None }

    let neurons_activations: Vec<OrderedFloat<f32>> = neurons.values()
        .cloned()
        .map(|neuron| OrderedFloat(neuron.borrow().activation()))
        .collect();
    let neurons: Vec<Rc<RefCell<dyn Neuron>>> = neurons.values().cloned().collect();

    let neurons_sorted: BTreeMap<OrderedFloat<f32>, Rc<RefCell<dyn Neuron>>> 
        = BTreeMap::from_iter(neurons_activations.into_iter().zip(neurons));

    let (winner_activation, winner) = neurons_sorted.into_iter().next_back()?;

    let max_activation = features.len() as f64;
    let proba = winner_activation.to_f32()? as f64 / max_activation;

    let predicted_value = winner.borrow().explain_one(target)?;

    Some((predicted_value, proba))
}

pub fn predict_weighted(
    magds: &mut MAGDS, 
    features: Vec<(Rc<str>, DataTypeValue, f32)>,
    target: Rc<str>,
    fuzzy: bool
) -> Option<(DataTypeValue, f64)> {
    let mut neurons: HashMap<NeuronID, Rc<RefCell<dyn Neuron>>> = HashMap::new();

    for (id, value, weight) in &features {
        let sensor = match magds.sensor_search(id.clone(), value) {
            Some(s) => s,
            None => {
                match magds.sensor_data_category(id.clone()) {
                    Some(DataCategory::Numerical) | Some(DataCategory::Ordinal) => {
                        if fuzzy {
                            log::info!("cannot find sensor {id} value {:?}, inserting", value);
                            match magds.sensor_insert(id.clone(), value) {
                                Some(s) => s,
                                None => {
                                    log::warn!("cannot insert {:?} to {id}, skipping", value);
                                    continue
                                }
                            }
                        } else {
                            log::warn!("cannot find sensor {id} for value {:?}, skipping", value);
                            continue
                        }
                    }
                    _ => {
                        log::warn!("cannot find sensor {id} for value {:?}, skipping", value);
                        continue
                    }
                }
            }
        };
        neurons.extend(sensor.borrow_mut().activate(*weight, fuzzy, true));
    }

    if neurons.is_empty() { return None }

    let neurons_activations: Vec<OrderedFloat<f32>> = neurons.values()
        .cloned()
        .map(|neuron| OrderedFloat(neuron.borrow().activation()))
        .collect();
    let neurons: Vec<Rc<RefCell<dyn Neuron>>> = neurons.values().cloned().collect();

    let neurons_sorted: BTreeMap<OrderedFloat<f32>, Rc<RefCell<dyn Neuron>>> 
        = BTreeMap::from_iter(neurons_activations.into_iter().zip(neurons));

    let (winner_activation, winner) = neurons_sorted.into_iter().next_back()?;

    let max_activation = features.len() as f64;
    let proba = winner_activation.to_f32()? as f64 / max_activation;

    let predicted_value = winner.borrow().explain_one(target)?;

    Some((predicted_value, proba))
}

pub fn prediction_score(
    train: &mut MAGDS, test: &mut MAGDS, target: Rc<str>, fuzzy: bool
) -> Option<(f64, f64)> {
    let mut total_proba = 0.0;
    let mut total_error = 0.0;

    let mut i = 1;
    for (neuron_id, neuron) in &mut test.neurons {
        if i % 100 == 0 { log::info!("prediction iteration: {i}"); }
        let mut features: Vec<(Rc<str>, DataTypeValue)> = Vec::new();
        let sensors = neuron.borrow().defining_sensors();
        let mut test_reference_value = DataTypeValue::Unknown;
        let mut should_skip = false;

        for (sensor_id, sensor) in sensors {
            let feature_name: Rc<str> = sensor_id.parent_id.clone();
            let feature_value_rcstr: Rc<str> = sensor_id.id.clone();
            let feaure_value_str = DataTypeValueStr(&feature_value_rcstr);
            let feature_data_type = sensor.borrow().data_type();
            let feature_value = feaure_value_str.data_type_value(feature_data_type);
            
            if *feature_name == *target {
                match feature_value {
                    Some(v) => test_reference_value = v,
                    None => {
                        log::warn!("target feature {target} is None for {neuron_id}, skipping");
                        should_skip = true;
                        break
                    }
                };
            } else {
                match feature_value {
                    Some(v) => features.push((feature_name, v)),
                    None => continue,
                };
            }
        }

        if should_skip { continue }
        if test_reference_value.is_unknown() { 
            panic!("test_reference_value shouldn't be unknown");
        }

        let (winner_value, winner_proba) = predict(train, &features, target.clone(), fuzzy)?;
        total_proba += winner_proba;
        log::debug!("winner_value {:?}, test_reference_value {:?}", winner_value, test_reference_value);
        total_error += winner_value.distance(&test_reference_value).powf(2.0);
        train.deactivate();

        i += 1;
    }

    let test_len = test.neurons.len() as f64;
    let final_proba = total_proba / test_len;

            
    let target_data_category = train.sensor(target.clone())?.borrow().data_category();
    match target_data_category {
        DataCategory::Numerical => {
            let rmse = (total_error as f64 / test_len).sqrt();
            Some((rmse, final_proba))
        }
        DataCategory::Categorical | DataCategory::Ordinal => {
            let accuracy = total_error as f64 / test_len;
            Some((accuracy, final_proba))
        }
    }
}

pub fn prediction_score_df(
    train: &mut MAGDS, test: &DataFrame, target: &str, fuzzy: bool
) -> Option<(f64, f64)> {
    let mut total_proba = 0.0;
    let mut total_error = 0.0;

    let mut i = 1;

    let mut feature_columns: HashMap<&str, DataVecOption> = HashMap::new();
    let mut target_column: Option<DataVecOption> = None;
    for column in test.get_columns() {
        let column_name = column.name();
        let datavec = match polars_common::series_to_datavec(column) {
            Ok(v) => v,
            Err(e) => { 
                log::error!("error convering {column_name} to datavec, error: {e}");
                continue
            }
        };
        if column_name == target {
            target_column = Some(datavec);
        } else {
            feature_columns.insert(column_name, datavec);
        }
    }
    let target_column = target_column.unwrap();
    let target_rc: Rc<str> = Rc::from(target);

    for i in 0..test.height() {
        if i % 100 == 0 { log::info!("prediction iteration: {i}"); }
        if let Some(reference_value) = target_column.get(i) {
            let mut features: Vec<(Rc<str>, DataTypeValue)> = Vec::new();
            for column_name in feature_columns.keys() {
                if let Some(f) = feature_columns[column_name].get(i) {
                    features.push((Rc::from(*column_name), f));
                }
            }

            let (winner_value, winner_proba) = predict(train, &features, target_rc.clone(), fuzzy)?;
            total_proba += winner_proba;
            log::debug!("winner_value {:?}, reference_value {:?}", winner_value, reference_value);
            total_error += winner_value.distance(&reference_value).powf(2.0);
            train.deactivate();
        } else {
            log::warn!("{target} is missing for row {i}, skipping");
            continue
        }
    }

    let test_len = test.height() as f64;
    let final_proba = total_proba / test_len;

            
    let target_data_category = train.sensor(target.into())?.borrow().data_category();
    match target_data_category {
        DataCategory::Numerical => {
            let rmse = (total_error as f64 / test_len).sqrt();
            Some((rmse, final_proba))
        }
        DataCategory::Categorical | DataCategory::Ordinal => {
            let accuracy = total_error as f64 / test_len;
            Some((accuracy, final_proba))
        }
    }
}

mod tests {
    use std::fs::File;
    
    use polars::prelude::*;
    
    use test_log::test;

    #[allow(unused_imports)]
    use crate::{
        algorithm::predict,
        simple::parser
    };

    #[test]
    fn prediction_score() {
        let train_file = "data/iris_train.csv";
        let test_file = "data/iris_test.csv";

        let mut magds_train = parser::magds_from_csv("iris_train", train_file).unwrap();
        let mut magds_test = parser::magds_from_csv("iris_test", test_file).unwrap();

        let (accuracy, proba) = predict::prediction_score(
            &mut magds_train, &mut magds_test, "variety".into(), false
        ).unwrap();
        println!("accuracy: {accuracy} proba: {proba}");
        assert!(accuracy > 0.95);
    }

    #[test]
    fn prediction_score_df() {
        let train_file = "data/iris_train.csv";
        let test_file = "data/iris_test.csv";

        let mut magds_train = parser::magds_from_csv("iris_train", train_file).unwrap();
        let test: DataFrame = CsvReader::new(File::open(test_file).unwrap())
            .infer_schema(None)
            .has_header(true)
            .finish()
            .unwrap();

        let (accuracy, proba) = predict::prediction_score_df(
            &mut magds_train, &test, "variety".into(), false
        ).unwrap();
        println!("accuracy: {accuracy} proba: {proba}");
        assert!(accuracy > 0.95);
    }
}