use witchnet_common::{
    data::{ DataTypeValue, DataCategory },
    neuron::Neuron
};

use crate::synchronous::magds::MAGDS;

pub fn recommend(
    magds: &mut MAGDS, 
    features: &Vec<(u32, DataTypeValue)>,
    target: u32,
    fuzzy: bool
) -> Option<Vec<(DataTypeValue, f32)>> {
    let features: Vec<(u32, DataTypeValue, f32)> = features.into_iter()
        .map(|(id, value)| (*id, value.clone(), 1.0f32))
        .collect();
    recommend_weighted(magds, &features, target, fuzzy)
}

pub fn recommend_weighted(
    magds: &mut MAGDS, 
    features: &Vec<(u32, DataTypeValue, f32)>,
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

    let neurons = magds.neurons.values();

    let neurons_len = neurons.len();
    if neurons_len == 0 { return None }

    let mut values_sorted: Vec<(DataTypeValue, f32)> = neurons
        .map(|neuron| (neuron.borrow().explain_one(target), neuron.borrow().activation() / max_activation_sum))
        .filter(|(target, _activation)| target.is_some())
        .map(|(target, activation)| (target.unwrap(), activation))
        .collect();
    values_sorted.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    Some(values_sorted.into_iter().rev().collect())
}

#[allow(unused_imports)]
mod tests {
    use witchnet_common::data::DataTypeValue;

    use crate::{
        synchronous::{
            parser,
            algorithm::{ recommendation, prediction }
        }
    };

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
        
        let pred = prediction::predict(&mut magds, &features, variety_sensor_id, true);
        println!("pred {:?}", pred);
    }
}