use std::{
    collections::HashMap,
    sync::{ Arc, RwLock },
    str::FromStr
};

use witchnet_common::{
    data::{ DataTypeValue, DataCategory, DataType },
    neuron::NeuronAsync,
    sensor::SensorAsync,
    polars::{ self as polars_common, DataVecOption },
    performance::{ SupervisedPerformance, DataProbability }
};
use polars::{
    prelude::*,
    export::num::ToPrimitive
};

use crate::{
    asynchronous::{
        algorithm::similarity,
        magds::MAGDS
    }
};

pub fn predict(
    magds: &mut MAGDS, 
    features: &Vec<(u32, DataTypeValue)>,
    target: u32,
    fuzzy: bool
) -> Option<DataProbability> {
    let features: Vec<(u32, DataTypeValue, f32)> = features.into_iter()
        .map(|(id, value)| (*id, value.clone(), 1.0f32))
        .collect();
    predict_weighted(magds, &features, target, fuzzy)
}

pub fn predict_weighted(
    magds: &mut MAGDS, 
    features: &Vec<(u32, DataTypeValue, f32)>,
    target: u32,
    fuzzy: bool
) -> Option<DataProbability> {
    let mut max_activation_sum = 0.0f32;

    for (id, value, weight) in features {
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
        let max_activation = sensor.write().unwrap().activate(*weight, fuzzy, true);
        max_activation_sum += max_activation;
    }

    let neurons = &magds.neurons;

    let neurons_len = neurons.len();
    if neurons_len == 0 { return None }

    let winners_limit = usize::min(12usize, neurons_len);

    let mut neurons_sorted: Vec<(f32, Arc<RwLock<dyn NeuronAsync>>)> = neurons.into_iter()
        .map(
            |neuron| (
                neuron.read().unwrap().activation(), 
                neuron.clone() as Arc<RwLock<dyn NeuronAsync>>
            )
        )
        .collect();
    neurons_sorted.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let neurons_sorted = &neurons_sorted[(neurons_len - winners_limit)..neurons_len];

    let target_data_category = match magds.sensor(target) {
        Some(s) => s.read().unwrap().data_category(),
        None => { log::error!("error getting sensor {target}"); return None }
    };
    let target_data_type = magds.sensor(target).unwrap().read().unwrap().data_type();

    let weight_ratio = f32::ln(features.len() as f32);

    match target_data_category {
        DataCategory::Numerical => {
            let mut targets_weighted: Vec<f64> = Vec::new();
            let mut probas: Vec<f32> = Vec::new();
            let mut weights = 0.0f32;
            let mut current_weight = 1.0f32;
            let mut winners_counter = 0;
            for (neuron_activation, neuron) in (neurons_sorted).into_iter().rev() {
                if let Some(target_value) = neuron.read().unwrap().explain_one(target) {
                    targets_weighted.push(target_value.to_f64().unwrap() * current_weight as f64);
                    weights += current_weight;
                    probas.push((neuron_activation.to_f32().unwrap() / max_activation_sum) * current_weight);
                    current_weight /= weight_ratio;

                    winners_counter += 1;
                    if winners_counter >= winners_limit { break }
                }
            }
            let predicted_value_f64: f64 = targets_weighted.iter().sum::<f64>() / weights as f64;
            let predicted_value: DataTypeValue = match target_data_type {
                DataType::U8 => (predicted_value_f64 as u8).into(),
                DataType::U16 => (predicted_value_f64 as u16).into(),
                DataType::U32 => (predicted_value_f64 as u32).into(),
                DataType::U64 => (predicted_value_f64 as u64).into(),
                DataType::U128 => (predicted_value_f64 as u128).into(),
                DataType::USize => (predicted_value_f64 as usize).into(),
                DataType::I8 => (predicted_value_f64 as i8).into(),
                DataType::I16 => (predicted_value_f64 as i16).into(),
                DataType::I32 => (predicted_value_f64 as i32).into(),
                DataType::I64 => (predicted_value_f64 as i64).into(),
                DataType::I128 => (predicted_value_f64 as i128).into(),
                DataType::ISize => (predicted_value_f64 as isize).into(),
                DataType::F32 => (predicted_value_f64 as f32).into(),
                DataType::F64 => (predicted_value_f64 as f64).into(),
                _ => { log::error!("classified as numerical data so shouldn't be here"); return None }
            };
            let proba: f32 = probas.iter().sum::<f32>() / weights;
            Some(DataProbability(predicted_value.into(), proba))
        }
        DataCategory::Categorical | DataCategory::Ordinal => {
            let mut values: HashMap<String, f32> = HashMap::new();
            let mut probas: Vec<f32> = Vec::new();
            let mut weights = 0.0f32;
            let mut current_weight = 1.0f32;
            let mut winners_counter = 0;
            for (neuron_activation, neuron) in (neurons_sorted).into_iter().rev() {
                if let Some(target_value) = neuron.read().unwrap().explain_one(target) {
                    let target_value = target_value.to_string();
                    if values.contains_key(&target_value) {
                        let current_value = values.get_mut(&target_value).unwrap();
                        *current_value += current_weight;
                    } else {
                        values.insert(target_value, current_weight);
                    }
                    weights += current_weight;
                    probas.push((neuron_activation.to_f32().unwrap() / max_activation_sum) * current_weight);
                    current_weight /= weight_ratio;

                    winners_counter += 1;
                    if winners_counter >= winners_limit { break }
                }
            }

            let mut values_sorted: Vec<(String, f32)> = values.into_iter()
                .map(|(name, value)| (name, value))
                .collect();
            values_sorted.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            let predicted_value_str = values_sorted.into_iter().next_back()?.0;
            let predicted_value: DataTypeValue = match target_data_type {
                DataType::Bool => bool::from_str(&predicted_value_str).ok()?.into(),
                DataType::ArcStr => Arc::<str>::from(predicted_value_str).into(),
                DataType::String => predicted_value_str.to_string().into(),
                _ => { log::error!("classified as not numerical data so shouldn't be here"); return None }
            };
            let proba: f32 = probas.iter().sum::<f32>() / weights;
            Some(DataProbability(predicted_value.into(), proba))
        }
    }
}

pub fn prediction_score(
    train: &mut MAGDS, 
    test: &mut MAGDS, 
    target: Arc<str>, 
    fuzzy: bool,
    weighted: bool
) -> anyhow::Result<SupervisedPerformance> {
    let y_len = test.neurons.len();
    let n_features = test.sensors.len();
    let mut references: Vec<DataTypeValue> = Vec::with_capacity(y_len);
    let mut predictions: Vec<DataTypeValue> = Vec::with_capacity(y_len);
    let mut probabilities: Vec<f32> = Vec::with_capacity(y_len);

    let target_id = *train.sensor_ids(&target).unwrap().first().unwrap();

    let mut similarities: HashMap<u32, f64> = HashMap::new();
    if weighted {
        similarities = similarity::features_target_weights(train, target_id)?;
    }

    for (i, neuron) in &mut test.neurons.iter().enumerate() {
        if i % 100 == 0 { log::info!("prediction iteration: {i}"); }

        let mut features: Vec<(u32, DataTypeValue, f32)> = Vec::with_capacity(n_features);
        let neuron_borrowed = neuron.read().unwrap();
        let sensors = neuron_borrowed.explain();
        let mut test_reference_value = DataTypeValue::Unknown;
        let mut should_skip = true;

        for sensor in sensors {
            let sensor_borrowed = sensor.read().unwrap();
            let sensor_id = sensor_borrowed.id();
            let feature_id = sensor_id.parent_id;
            let feature_name = test.sensor_name(feature_id).unwrap();
            let feature_value = sensor_borrowed.value();
            let feature_id_train = *train.sensor_ids(feature_name).unwrap().first().unwrap();
            
            if *feature_name == *target {
                test_reference_value = feature_value;
                should_skip = false;
            } else {
                let weight = if weighted { similarities[&feature_id] as f32 } else { 1.0f32 };
                features.push((feature_id_train, feature_value, weight));
            }
        }

        if should_skip { continue }
        if test_reference_value.is_unknown() { 
            anyhow::bail!("test_reference_value shouldn't be unknown");
        }

        let data_proba = match predict_weighted(train, &features, target_id, fuzzy) {
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
        Some(s) => s.read().unwrap().data_category(),
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
    train: &mut MAGDS, 
    test: &DataFrame, 
    target: &str, 
    fuzzy: bool,
    weighted: bool
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

    let mut similarities: HashMap<u32, f64> = HashMap::new();
    if weighted {
        similarities = similarity::features_target_weights(train, target_id)?;
    }

    for i in 0..y_len {
        if i % 1000 == 0 { println!("prediction iteration: {i}"); }
        
        if let Some(reference_value) = target_column.get(i) {
            let mut features: Vec<(u32, DataTypeValue, f32)> = Vec::with_capacity(n_features);
            for feature_id in feature_columns.keys() {
                let weight = if weighted { similarities[feature_id] as f32 } else { 1.0f32 };
                if let Some(feature_raw) = feature_columns[feature_id].get(i) {
                    for feature in  feature_raw.to_vec() {
                        features.push((*feature_id, feature, weight));
                    }
                }
            }

            let data_proba = match predict_weighted(train, &features, target_id, fuzzy) {
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
        Some(s) => s.read().unwrap().data_category(),
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

#[allow(unused_imports)]
mod tests {
    use std::fs::File;
    
    use polars::prelude::*;
    
    use test_log::test;

    #[allow(unused_imports)]
    use crate::asynchronous::{
        algorithm::prediction,
        parser
    };

    #[test]
    fn prediction_score() {
        let train_file = "data/iris_original_train.csv";
        let test_file = "data/iris_original_test.csv";

        let mut magds_train = parser::magds_from_csv("iris_train", train_file, &vec![]).unwrap();
        let mut magds_test = parser::magds_from_csv("iris_test", test_file, &vec![]).unwrap();

        let performance = prediction::prediction_score(
            &mut magds_train, &mut magds_test, "variety".into(), true, false
        ).unwrap();
        let accuracy = performance.accuracy().unwrap();
        let proba = performance.mean_probability().unwrap();
        println!("accuracy: {accuracy} proba: {proba}");
        assert!(accuracy > 0.90);
        assert!(proba > 0.0);

        let performance = prediction::prediction_score(
            &mut magds_train, &mut magds_test, "variety".into(), true, true
        ).unwrap();
        let accuracy = performance.accuracy().unwrap();
        let proba = performance.mean_probability().unwrap();
        println!("accuracy: {accuracy} proba: {proba}");
        assert!(accuracy > 0.90);
        assert!(proba > 0.0);
    }

    #[test]
    fn prediction_score_df() {
        let train_file = "data/iris_original_train.csv";
        let test_file = "data/iris_original_test.csv";

        let mut magds_train = parser::magds_from_csv("iris_train", train_file, &vec![]).unwrap();
        let test: DataFrame = CsvReader::new(File::open(test_file).unwrap())
            .infer_schema(None)
            .has_header(true)
            .finish()
            .unwrap();

        let performance = prediction::prediction_score_df(
            &mut magds_train, &test, "variety".into(), true, false
        ).unwrap();
        println!("performance.predictions() {:?}", performance.predictions());
        println!("performance.references() {:?}", performance.references());
        println!("performance.probabilities() {:?}", performance.probabilities());
        let accuracy = performance.accuracy().unwrap();
        let proba = performance.mean_probability().unwrap();
        println!("accuracy: {accuracy} proba: {proba}");
        assert!(accuracy > 0.90);
        assert!(proba > 0.0);

        let performance = prediction::prediction_score_df(
            &mut magds_train, &test, "variety".into(), true, true
        ).unwrap();
        let accuracy = performance.accuracy().unwrap();
        let proba = performance.mean_probability().unwrap();
        println!("accuracy: {accuracy} proba: {proba}");
        assert!(accuracy > 0.80);
        assert!(proba > 0.0);
    }
}