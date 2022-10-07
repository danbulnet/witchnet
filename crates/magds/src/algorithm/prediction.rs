use std::{
    collections::{ HashMap, BTreeMap },
    rc::Rc,
    cell::RefCell
};

use ordered_float::OrderedFloat;

use witchnet_common::{
    data::{ DataTypeValue, DataCategory },
    neuron::{ Neuron, NeuronID },
    sensor::Sensor,
    polars::{ self as polars_common, DataVecOption },
    performance::{ SupervisedPerformance, DataProbability }
};
use polars::{
    prelude::*,
    export::num::ToPrimitive
};

use crate::simple::magds::MAGDS;

pub fn predict(
    magds: &mut MAGDS, 
    features: &Vec<(u32, DataTypeValue)>,
    target: u32,
    fuzzy: bool
) -> Option<DataProbability> {
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

    let max_activation = features.len() as f32;
    let proba = winner_activation.to_f32()? / max_activation;

    let predicted_value = winner.borrow().explain_one(target)?;

    Some(DataProbability(predicted_value, proba))
}

pub fn predict_weighted(
    magds: &mut MAGDS, 
    features: Vec<(u32, DataTypeValue, f32)>,
    target: u32,
    fuzzy: bool
) -> Option<DataProbability> {
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

    let max_activation = features.len() as f32;
    let proba = winner_activation.to_f32()? / max_activation;

    let predicted_value = winner.borrow().explain_one(target)?;

    Some(DataProbability(predicted_value, proba))
}

pub fn prediction_score(
    train: &mut MAGDS, test: &mut MAGDS, target: Rc<str>, fuzzy: bool
) -> anyhow::Result<SupervisedPerformance> {
    let y_len = test.neurons.len();
    let n_features = test.sensors.len();
    let mut references: Vec<DataTypeValue> = Vec::with_capacity(y_len);
    let mut predictions: Vec<DataTypeValue> = Vec::with_capacity(y_len);
    let mut probabilities: Vec<f32> = Vec::with_capacity(y_len);

    let target_id = *train.sensor_ids(&target).unwrap().first().unwrap();

    for (i, (_neuron_id, neuron)) in &mut test.neurons.iter().enumerate() {
        if i % 100 == 0 { log::info!("prediction iteration: {i}"); }

        let mut features: Vec<(u32, DataTypeValue)> = Vec::with_capacity(n_features);
        let sensors = neuron.borrow().explain();
        let mut test_reference_value = DataTypeValue::Unknown;
        let mut should_skip = true;

        for (sensor_id, sensor) in sensors {
            let feature_id = sensor_id.parent_id;
            let feature_name = test.sensor_name(feature_id).unwrap();
            let feature_value = sensor.borrow().value();
            let feature_id_train = *train.sensor_ids(feature_name).unwrap().first().unwrap();
            
            if *feature_name == *target {
                test_reference_value = feature_value;
                should_skip = false;
            } else {
                features.push((feature_id_train, feature_value));
            }
        }

        if should_skip { continue }
        if test_reference_value.is_unknown() { 
            anyhow::bail!("test_reference_value shouldn't be unknown");
        }

        let data_proba = match predict(train, &features, target_id, fuzzy) {
            Some(dp) => dp,
            None => { train.deactivate(); continue }
        };
        let (winner_value, winner_proba) = (data_proba.0, data_proba.1);
        log::debug!("winner_value {:?}, test_reference_value {:?}", winner_value, test_reference_value);
        train.deactivate();

        references.push(test_reference_value);
        predictions.push(winner_value);
        probabilities.push(winner_proba);
    }

    let target_data_category = match train.sensor(target_id) {
        Some(s) => s.borrow().data_category(),
        None => anyhow::bail!("error getting sensor {target}")
    };
    match target_data_category {
        DataCategory::Numerical => {
            SupervisedPerformance::regression(references, predictions, probabilities)
        }
        DataCategory::Categorical | DataCategory::Ordinal => {
            SupervisedPerformance::classification(references, predictions, probabilities)
        }
    }
}

pub fn prediction_score_df(
    train: &mut MAGDS, test: &DataFrame, target: &str, fuzzy: bool
) -> anyhow::Result<SupervisedPerformance> {
    let y_len = test.height();
    let n_features = test.width();
    let mut references: Vec<DataTypeValue> = Vec::with_capacity(y_len);
    let mut predictions: Vec<DataTypeValue> = Vec::with_capacity(y_len);
    let mut probabilities: Vec<f32> = Vec::with_capacity(y_len);

    let target_id = *train.sensor_ids(target).unwrap().first().unwrap();

    let mut feature_columns: HashMap<u32, DataVecOption> = HashMap::new();
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
            let feature_id_train = *train.sensor_ids(column_name).unwrap().first().unwrap();
            feature_columns.insert(feature_id_train, datavec);
        }
    }
    let target_column = target_column.unwrap();

    for i in 0..y_len {
        if i % 100 == 0 { log::info!("prediction iteration: {i}"); }
        
        if let Some(reference_value) = target_column.get(i) {
            let mut features: Vec<(u32, DataTypeValue)> = Vec::with_capacity(n_features);
            for feature_id in feature_columns.keys() {
                if let Some(f) = feature_columns[feature_id].get(i) {
                    features.push((*feature_id, f));
                }
            }

            let data_proba = match predict(train, &features, target_id, fuzzy) {
                Some(dp) => dp,
                None => { train.deactivate(); continue }
            };
            let (winner_value, winner_proba) = (data_proba.0, data_proba.1);
            log::debug!("winner_value {:?}, reference_value {:?}", winner_value, reference_value);
            train.deactivate();

            references.push(reference_value);
            predictions.push(winner_value);
            probabilities.push(winner_proba);
        } else {
            log::warn!("{target} is missing for row {i}, skipping");
            continue
        }
    }

    let target_data_category = match train.sensor(target_id) {
        Some(s) => s.borrow().data_category(),
        None => anyhow::bail!("error getting sensor {target}")
    };
    match target_data_category {
        DataCategory::Numerical => {
            SupervisedPerformance::regression(references, predictions, probabilities)
        }
        DataCategory::Categorical | DataCategory::Ordinal => {
            SupervisedPerformance::classification(references, predictions, probabilities)
        }
    }
}

mod tests {
    use std::fs::File;
    
    use polars::prelude::*;
    
    use test_log::test;

    #[allow(unused_imports)]
    use crate::{
        algorithm::prediction,
        simple::parser
    };

    #[test]
    fn prediction_score() {
        let train_file = "data/iris_train.csv";
        let test_file = "data/iris_test.csv";

        let mut magds_train = parser::magds_from_csv("iris_train", train_file).unwrap();
        let mut magds_test = parser::magds_from_csv("iris_test", test_file).unwrap();

        let performance = prediction::prediction_score(
            &mut magds_train, &mut magds_test, "variety".into(), false
        ).unwrap();
        let accuracy = performance.accuracy().unwrap();
        let proba = performance.mean_probability().unwrap();
        println!("accuracy: {accuracy} proba: {proba}");
        assert!(accuracy > 0.95);
        assert!(proba > 0.0);
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

        let performance = prediction::prediction_score_df(
            &mut magds_train, &test, "variety".into(), false
        ).unwrap();
        println!("performance.predictions() {:?}", performance.predictions());
        println!("performance.references() {:?}", performance.references());
        println!("performance.probabilities() {:?}", performance.probabilities());
        let accuracy = performance.accuracy().unwrap();
        let proba = performance.mean_probability().unwrap();
        println!("accuracy: {accuracy} proba: {proba}");
        assert!(accuracy > 0.95);
        assert!(proba > 0.0);
    }
}