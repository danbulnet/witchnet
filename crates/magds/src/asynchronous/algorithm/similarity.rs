use std::collections::{ HashMap, HashSet };

use anyhow::{ Result, Context };

use witchnet_common::{
    data::DataTypeValue,
    sensor::SensorAsync,
    neuron::NeuronID
};

use crate::asynchronous::magds::MAGDS;

pub fn entropy<S: SensorAsync<DataTypeValue>>(sensor: &S) -> Result<f64> {
    let mut entropy = 0.0;

    let mut sensor_total_counter = 0f64;
    for sensor_neuron in sensor.neurons() {
        sensor_total_counter += sensor_neuron.read().unwrap().counter() as f64;
    }

    for sensor_neuron in sensor.neurons() {
        let px = sensor_neuron.read().unwrap().counter() as f64 / sensor_total_counter;
        entropy += px * f64::log2(px);
    }
    Ok(-1f64 * entropy)
}

pub fn mutual_information<S1: SensorAsync<DataTypeValue>, S2: SensorAsync<DataTypeValue>>(
    s1: &S1, s2: &S2, normalized: bool
) -> Result<f64> {
    let mut mutual_information = 0.0;

    let mut s1_total_counter = 0;
    let s1_neurons = s1.neurons();
    if s1_neurons.is_empty() { anyhow::bail!("s1 is empty") }
    for sensor_neuron in s1_neurons {
        s1_total_counter += sensor_neuron.read().unwrap().counter();
    }
    
    let mut s2_total_counter = 0;
    let s2_neurons = s2.neurons();
    if s2_neurons.is_empty() { anyhow::bail!("s2 is empty") }
    for sensor_neuron in s2_neurons {
        s2_total_counter += sensor_neuron.read().unwrap().counter();
    }
    let total_counter = usize::max(s1_total_counter, s2_total_counter) as f64;

    let mut s1_entropy = f64::NAN;
    let mut s2_entropy = f64::NAN;
    if normalized {
        s1_entropy = entropy(s1)?;
        s2_entropy = entropy(s2)?;
    }

    for s1_sensor_neuron in s1.neurons() {
        let s1_sensor_neuron_borrowed = s1_sensor_neuron.read().unwrap();
        let s1_defining_neurons = s1_sensor_neuron_borrowed.defined_neurons();
        
        let s1_neurons_ids: HashSet<NeuronID> = HashSet::from_iter(
            s1_defining_neurons.into_iter().map(|x| x.read().unwrap().id())
        );
        
        for s2_sensor_neuron in s2.neurons() {
            let s2_sensor_neuron_borrowed = s2_sensor_neuron.read().unwrap();
            let s2_defining_neurons = s2_sensor_neuron_borrowed.defined_neurons();

            let s2_neurons_ids: HashSet<NeuronID> = HashSet::from_iter(
                s2_defining_neurons.into_iter().map(|x| x.read().unwrap().id())
            );
            let coincidences = s1_neurons_ids.intersection(&s2_neurons_ids).count();
            
            if coincidences == 0 {
                continue
            } else {
                let p_xy = coincidences as f64 / total_counter;
                let p_x = s1_sensor_neuron.read().unwrap().counter() as f64 / total_counter;
                let p_y = s2_sensor_neuron.read().unwrap().counter() as f64 / total_counter;
                let mi = if normalized {
                    2f64 * (p_xy * f64::log2(p_xy / (p_x * p_y))) / (s1_entropy * s2_entropy)
                } else {
                    p_xy * f64::log2(p_xy / (p_x * p_y))
                };
                mutual_information += mi;
            }
        }
    }

    for s1_sensor_neuron in s1.neurons() {
        s1_sensor_neuron.write().unwrap().deactivate(false, true);
    }
    for s2_sensor_neuron in s2.neurons() {
        s2_sensor_neuron.write().unwrap().deactivate(false, true);
    }

    Ok(mutual_information)
}


pub fn features_target_weights(magds: &MAGDS, target_id: u32) -> Result<HashMap<u32, f64>> {
    let target_sensor = magds.sensor(target_id).context("error getting target sensor")?;
    
    let features_ids: Vec<u32> = magds.sensors.keys()
        .map(|id| *id)
        .filter(|id| *id != target_id)
        .collect();

    let mut ret = HashMap::new();
    for id in features_ids {
        let sensor = magds.sensor(id).context("error getting sensor id {id}")?;
        let similarity = if sensor.read().unwrap().data_category().is_sortable() {
            1.0f64
        } else {
            mutual_information(
                &*sensor.read().unwrap(), &*target_sensor.read().unwrap(), true
            )?
        };
        ret.insert(id, similarity);
    }

    log::info!("features_target_weights for target {target_id} {:?}", ret);
    Ok(ret)
}

#[allow(unused_imports)]
mod tests {
    use crate::asynchronous::parser;

    #[test]
    fn features_target_weights() {
        let magds = parser::magds_from_csv("iris", "data/iris.csv", &vec![]).unwrap();
        let variety_sensor_id = *magds.sensor_ids("variety").unwrap().first().unwrap();
        let weights = super::features_target_weights(&magds, variety_sensor_id).unwrap();
        println!("{:?}", weights);
        for (_id, weight) in weights.into_iter() { assert!(weight > 0f64); }
    }

    #[test]
    fn entropy() {
        let magds = parser::magds_from_csv("iris", "data/iris.csv", &vec![]).unwrap();

        let variety_sensor_id = *magds.sensor_ids("variety").unwrap().first().unwrap();
        let sepal_length_sensor_id = *magds.sensor_ids("sepal.length").unwrap().first().unwrap();
        let petal_length_sensor_id = *magds.sensor_ids("petal.length").unwrap().first().unwrap();

        let variety_sensor = magds.sensor(variety_sensor_id).unwrap();
        let sepal_length_sensor = magds.sensor(sepal_length_sensor_id).unwrap();
        let petal_length_sensor = magds.sensor(petal_length_sensor_id).unwrap();

        let variety_entropy = super::entropy(&*variety_sensor.read().unwrap()).unwrap();
        assert!(variety_entropy > 0f64);
        println!("entropy variety_sensor {:?}", variety_entropy);
        
        let sepal_length_entropy = super::entropy(&*sepal_length_sensor.read().unwrap()).unwrap();
        assert!(sepal_length_entropy > 0f64);
        println!("entropy sepal_length_sensor {:?}", sepal_length_entropy);
        
        let petal_length_entropy = super::entropy(&*petal_length_sensor.read().unwrap()).unwrap();
        assert!(petal_length_entropy > 0f64);
        println!("entropy petal_length_sensor {:?}", petal_length_entropy);
    }

    #[test]
    fn mutual_information() {
        let magds = parser::magds_from_csv("iris", "data/iris.csv", &vec![]).unwrap();

        let variety_sensor_id = *magds.sensor_ids("variety").unwrap().first().unwrap();
        let sepal_length_sensor_id = *magds.sensor_ids("sepal.length").unwrap().first().unwrap();
        let petal_length_sensor_id = *magds.sensor_ids("petal.length").unwrap().first().unwrap();
        let petal_width_sensor_id = *magds.sensor_ids("petal.width").unwrap().first().unwrap();
        let sepal_width_sensor_id = *magds.sensor_ids("sepal.width").unwrap().first().unwrap();

        let variety_sensor = magds.sensor(variety_sensor_id).unwrap();
        let sepal_length_sensor = magds.sensor(sepal_length_sensor_id).unwrap();
        let petal_length_sensor = magds.sensor(petal_length_sensor_id).unwrap();
        let petal_width_sensor = magds.sensor(petal_width_sensor_id).unwrap();
        let sepal_width_sensor = magds.sensor(sepal_width_sensor_id).unwrap();

        // petal_width-petal_length
        let mi = super::mutual_information(
            &*petal_width_sensor.read().unwrap(), &*petal_length_sensor.read().unwrap(), false
        ).unwrap();
        assert!(mi > 0f64);
        println!("mutual_information petal_width-petal_length {:?}", mi);

        let mi = super::mutual_information(
            &*petal_length_sensor.read().unwrap(), &*petal_width_sensor.read().unwrap(), false
        ).unwrap();
        assert!(mi > 0f64);
        println!("mutual_information petal_length-petal_width {:?}", mi);

        let mi = super::mutual_information(
            &*petal_width_sensor.read().unwrap(), &*petal_length_sensor.read().unwrap(), true
        ).unwrap();
        assert!(mi > 0f64);
        println!("mutual_information petal_width-petal_length weighted {:?}", mi);

        let mi = super::mutual_information(
            &*petal_length_sensor.read().unwrap(), &*petal_width_sensor.read().unwrap(), true
        ).unwrap();
        assert!(mi > 0f64);
        println!("mutual_information petal_length-petal_width weighted {:?}", mi);

        // sepal_width-petal_length
        let mi = super::mutual_information(
            &*sepal_width_sensor.read().unwrap(), &*petal_length_sensor.read().unwrap(), false
        ).unwrap();
        assert!(mi > 0f64);
        println!("mutual_information sepal_width-petal_length {:?}", mi);

        let mi = super::mutual_information(
            &*petal_length_sensor.read().unwrap(), &*sepal_width_sensor.read().unwrap(), false
        ).unwrap();
        assert!(mi > 0f64);
        println!("mutual_information petal_length-sepal_width {:?}", mi);

        let mi = super::mutual_information(
            &*sepal_width_sensor.read().unwrap(), &*petal_length_sensor.read().unwrap(), true
        ).unwrap();
        assert!(mi > 0f64);
        println!("mutual_information sepal_width-petal_length weighted {:?}", mi);

        let mi = super::mutual_information(
            &*petal_length_sensor.read().unwrap(), &*sepal_width_sensor.read().unwrap(), true
        ).unwrap();
        assert!(mi > 0f64);
        println!("mutual_information petal_length-sepal_width weighted {:?}", mi);

        // variety
        let mi = super::mutual_information(
            &*variety_sensor.read().unwrap(), &*petal_length_sensor.read().unwrap(), true
        ).unwrap();
        assert!(mi > 0f64);
        println!("mutual_information variety-petal_length {:?}", mi);

        let mi = super::mutual_information(
            &*variety_sensor.read().unwrap(), &*petal_width_sensor.read().unwrap(), true
        ).unwrap();
        assert!(mi > 0f64);
        println!("mutual_information variety-petal_width {:?}", mi);

        let mi = super::mutual_information(
            &*variety_sensor.read().unwrap(), &*sepal_length_sensor.read().unwrap(), true
        ).unwrap();
        assert!(mi > 0f64);
        println!("mutual_information variety-sepal_length {:?}", mi);

        let mi = super::mutual_information(
            &*variety_sensor.read().unwrap(), &*sepal_width_sensor.read().unwrap(), true
        ).unwrap();
        assert!(mi > 0f64);
        println!("mutual_information variety-sepal_width {:?}", mi);
    }
}