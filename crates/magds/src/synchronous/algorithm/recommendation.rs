use std::{ rc::Rc, cell::RefCell };

use witchnet_common::{
    data::{ DataTypeValue, DataCategory },
    neuron::Neuron
};

use crate::synchronous::magds::MAGDS;

#[derive(Debug, Clone)]
pub enum DataValueFilter {
    Empty,
    One(DataTypeValue),
    Range((DataTypeValue, DataTypeValue)),
    Many(Vec<DataTypeValue>)
}

pub fn recommend(
    magds: &mut MAGDS, 
    features: &Vec<(u32, DataTypeValue)>,
    target: u32,
    fuzzy: bool
) -> Option<Vec<(DataTypeValue, f32)>> {
    let features: Vec<(u32, DataTypeValue, f32)> = features.into_iter()
        .map(|(id, value)| (*id, value.clone(), 1.0f32))
        .collect();
    recommend_weighted(magds, &features, &vec![], target, fuzzy)
}

pub fn recommend_filter(
    magds: &mut MAGDS, 
    features: &[(u32, DataTypeValue)],
    filters: &[(u32, DataValueFilter)],
    target: u32,
    fuzzy: bool
) -> Option<Vec<(DataTypeValue, f32)>> {
    let features: Vec<(u32, DataTypeValue, f32)> = features.into_iter()
        .map(|(id, value)| (*id, value.clone(), 1.0f32))
        .collect();
    recommend_weighted(magds, &features, filters, target, fuzzy)
}

pub fn recommend_weighted(
    magds: &mut MAGDS, 
    features: &Vec<(u32, DataTypeValue, f32)>,
    filters: &[(u32, DataValueFilter)],
    target: u32,
    fuzzy: bool
) -> Option<Vec<(DataTypeValue, f32)>> {
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
        let max_activation = sensor.borrow_mut().activate(*weight, fuzzy, true);
        max_activation_sum += max_activation;
    }

    let neurons = &magds.neurons;

    let neurons_len = neurons.len();
    if neurons_len == 0 { return None }

    let mut values_sorted: Vec<(DataTypeValue, f32)> = neurons.into_iter()
        .filter(|neuron| {
            if filters.is_empty() { true } else { filter_neuron(neuron, filters) }
        })
        .map(|neuron| (
            neuron.borrow().explain_one(target), 
            if max_activation_sum == 0.0f32 {
                0.0f32
            } else {
                neuron.borrow().activation() / max_activation_sum
            }
        ))
        .filter(|(target, _activation)| target.is_some())
        .map(|(target, activation)| (target.unwrap(), activation))
        .collect();
    values_sorted.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    Some(values_sorted.into_iter().rev().collect())
}

fn filter_neuron(
    neuron: &Rc<RefCell<dyn Neuron>>, filters: &[(u32, DataValueFilter)]
) -> bool {
    let neuron = neuron.borrow();
    for (filter_id, filter) in filters {
        if let Some(neuron_value) = &neuron.explain_one(*filter_id) {
            let is_ok = match filter {
                DataValueFilter::Empty => true,
                DataValueFilter::One(value) => neuron_value == value,
                DataValueFilter::Range((min, max)) => {
                    neuron_value >= min && neuron_value <= max
                }
                DataValueFilter::Many(values) => values.contains(neuron_value),
            };
            if !is_ok { return false }
        }
    }
    true
}

#[allow(unused_imports)]
mod tests {
    use std::sync::Arc;

    use witchnet_common::data::DataTypeValue;

    use crate::{
        synchronous::{
            parser,
            algorithm::{ recommendation, prediction }
        }
    };

    use super::*;

    #[test]
    fn recommend() {
        let iris_file = "data/iris_original.csv";
        let mut magds = parser::magds_from_csv("iris", iris_file, &vec![]).unwrap();
        let variety_sensor_id = *magds.sensor_ids("variety").unwrap().first().unwrap();
        let sepal_length_sensor_id = *magds.sensor_ids("sepal.length").unwrap().first().unwrap();
        let petal_length_sensor_id = *magds.sensor_ids("petal.length").unwrap().first().unwrap();
        let petal_width_sensor_id = *magds.sensor_ids("petal.width").unwrap().first().unwrap();
        let sepal_width_sensor_id = *magds.sensor_ids("sepal.width").unwrap().first().unwrap();

        let features: Vec<(u32, DataTypeValue)> = vec![
            (sepal_length_sensor_id, 5.8.into()),
            (sepal_width_sensor_id, 3.8.into()),
            (petal_length_sensor_id, 1.5.into()),
            (petal_width_sensor_id, 2.0.into()),
        ];

        let recommendations = recommendation::recommend(
            &mut magds, &features, variety_sensor_id, true
        ).unwrap();

        assert!(!recommendations.is_empty());
        assert!(recommendations.first().unwrap().1 > 0f32);

        println!("recommendations {:?}", recommendations);

        let setosa_filters = vec![
            (variety_sensor_id, DataValueFilter::One(Arc::<str>::from("setosa").into()))
        ];
        let recommendations = recommendation::recommend_filter(
            &mut magds, &features, &setosa_filters, variety_sensor_id, true
        ).unwrap();
        assert!(recommendations.len() == 50);
        assert!(recommendations.first().unwrap().1 > 0f32);
        println!("recommendations filtered by setosa {:?}", recommendations);

        let empty_filters = vec![(variety_sensor_id, DataValueFilter::Empty)];
        let recommendations = recommendation::recommend_filter(
            &mut magds, &features, &empty_filters, variety_sensor_id, true
        ).unwrap();
        assert!(recommendations.len() == 150);
        assert!(recommendations.first().unwrap().1 > 0f32);
        println!("recommendations filtered by empty filter {:?}", recommendations);

        let range_filters = vec![
            (sepal_length_sensor_id, DataValueFilter::Range((5.0.into(), 10.0.into())))
        ];
        let recommendations = recommendation::recommend_filter(
            &mut magds, &features, &range_filters, variety_sensor_id, true
        ).unwrap();
        assert!(recommendations.len() == 128);
        assert!(recommendations.first().unwrap().1 > 0f32);
        println!("recommendations filtered by range filter {:?}", recommendations);

        let many_filters = vec![(
            variety_sensor_id, DataValueFilter::Many(
                vec![Arc::<str>::from("setosa").into(), Arc::<str>::from("virginica").into()]
            )
        )];
        let recommendations = recommendation::recommend_filter(
            &mut magds, &features, &many_filters, variety_sensor_id, true
        ).unwrap();
        assert!(recommendations.len() == 100);
        assert!(recommendations.first().unwrap().1 > 0f32);
        println!("recommendations filtered by setosa and versicolor {:?}", recommendations);
        
        let pred = prediction::predict(&mut magds, &features, variety_sensor_id, true);
        println!("pred {:?}", pred);
    }
}