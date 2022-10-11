use std::collections::HashSet;

use anyhow::{ Result, Context };

use log::{ log_enabled, Level};

use witchnet_common::{
    data::{DataTypeValue, DataCategory},
    sensor::Sensor,
    distances::{ Distance, DistanceChecked },
    neuron::NeuronID
};

pub fn entropy<S: Sensor<DataTypeValue>>(sensor: &S) -> Result<f64> {
    let mut entropy = 0.0;

    let mut sensor_total_counter = 0f64;
    for sensor_neuron in sensor.neurons() {
        sensor_total_counter += sensor_neuron.borrow().counter() as f64;
    }

    for sensor_neuron in sensor.neurons() {
        let px = sensor_neuron.borrow().counter() as f64 / sensor_total_counter;
        entropy += px * f64::log2(px);
    }
    Ok(-1f64 * entropy)
}

pub fn mutual_information<S1: Sensor<DataTypeValue>, S2: Sensor<DataTypeValue>>(
    s1: &S1, s2: &S2, normalized: bool
) -> Result<f64> {
    let mut mutual_information = 0.0;

    let mut s1_total_counter = 0;
    let s1_neurons = s1.neurons();
    if s1_neurons.is_empty() { anyhow::bail!("s1 is empty") }
    for sensor_neuron in s1_neurons {
        s1_total_counter += sensor_neuron.borrow().counter();
    }
    
    let mut s2_total_counter = 0;
    let s2_neurons = s2.neurons();
    if s2_neurons.is_empty() { anyhow::bail!("s2 is empty") }
    for sensor_neuron in s2_neurons {
        s2_total_counter += sensor_neuron.borrow().counter();
    }
    let total_counter = usize::max(s1_total_counter, s2_total_counter) as f64;

    let mut s1_entropy = f64::NAN;
    let mut s2_entropy = f64::NAN;
    if normalized {
        s1_entropy = entropy(s1)?;
        s2_entropy = entropy(s2)?;
    }

    for s1_sensor_neuron in s1.neurons() {
        let (s1_defining_neurons, _) = 
            s1_sensor_neuron.borrow_mut().activate(1.0f32, false, true);
        
        let s1_neurons_ids: HashSet<NeuronID> = HashSet::from_iter(
            s1_defining_neurons.keys().cloned()
        );
        
        for s2_sensor_neuron in s2.neurons() {
            let (s2_defining_neurons, _) = 
                s2_sensor_neuron.borrow_mut().activate(1.0f32, false, true);

            let s2_neurons_ids: HashSet<NeuronID> = HashSet::from_iter(
                s2_defining_neurons.keys().cloned()
            );
            let coincidences = s1_neurons_ids.intersection(&s2_neurons_ids).count();
            
            if coincidences == 0 {
                continue
            } else {
                let p_xy = coincidences as f64 / total_counter;
                let p_x = s1_sensor_neuron.borrow().counter() as f64 / total_counter;
                let p_y = s2_sensor_neuron.borrow().counter() as f64 / total_counter;
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
        s1_sensor_neuron.borrow_mut().deactivate(false, true);
    }
    for s2_sensor_neuron in s2.neurons() {
        s2_sensor_neuron.borrow_mut().deactivate(false, true);
    }

    Ok(mutual_information)
}

mod tests {
    use crate::simple::parser;

    #[test]
    fn entropy() {
        let magds = parser::magds_from_csv("iris", "data/iris.csv", &vec![]).unwrap();

        let variety_sensor_id = *magds.sensor_ids("variety").unwrap().first().unwrap();
        let sepal_length_sensor_id = *magds.sensor_ids("sepal.length").unwrap().first().unwrap();
        let petal_length_sensor_id = *magds.sensor_ids("petal.length").unwrap().first().unwrap();

        let variety_sensor = magds.sensor(variety_sensor_id).unwrap();
        let sepal_length_sensor = magds.sensor(sepal_length_sensor_id).unwrap();
        let petal_length_sensor = magds.sensor(petal_length_sensor_id).unwrap();

        let variety_entropy = super::entropy(&*variety_sensor.borrow()).unwrap();
        assert!(variety_entropy > 0f64);
        println!("entropy variety_sensor {:?}", variety_entropy);
        
        let sepal_length_entropy = super::entropy(&*sepal_length_sensor.borrow()).unwrap();
        assert!(sepal_length_entropy > 0f64);
        println!("entropy sepal_length_sensor {:?}", sepal_length_entropy);
        
        let petal_length_entropy = super::entropy(&*petal_length_sensor.borrow()).unwrap();
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
            &*petal_width_sensor.borrow(), &*petal_length_sensor.borrow(), false
        ).unwrap();
        assert!(mi > 0f64);
        println!("mutual_information petal_width-petal_length {:?}", mi);

        let mi = super::mutual_information(
            &*petal_length_sensor.borrow(), &*petal_width_sensor.borrow(), false
        ).unwrap();
        assert!(mi > 0f64);
        println!("mutual_information petal_length-petal_width {:?}", mi);

        let mi = super::mutual_information(
            &*petal_width_sensor.borrow(), &*petal_length_sensor.borrow(), true
        ).unwrap();
        assert!(mi > 0f64);
        println!("mutual_information petal_width-petal_length weighted {:?}", mi);

        let mi = super::mutual_information(
            &*petal_length_sensor.borrow(), &*petal_width_sensor.borrow(), true
        ).unwrap();
        assert!(mi > 0f64);
        println!("mutual_information petal_length-petal_width weighted {:?}", mi);

        // sepal_width-petal_length
        let mi = super::mutual_information(
            &*sepal_width_sensor.borrow(), &*petal_length_sensor.borrow(), false
        ).unwrap();
        assert!(mi > 0f64);
        println!("mutual_information sepal_width-petal_length {:?}", mi);

        let mi = super::mutual_information(
            &*petal_length_sensor.borrow(), &*sepal_width_sensor.borrow(), false
        ).unwrap();
        assert!(mi > 0f64);
        println!("mutual_information petal_length-sepal_width {:?}", mi);

        let mi = super::mutual_information(
            &*sepal_width_sensor.borrow(), &*petal_length_sensor.borrow(), true
        ).unwrap();
        assert!(mi > 0f64);
        println!("mutual_information sepal_width-petal_length weighted {:?}", mi);

        let mi = super::mutual_information(
            &*petal_length_sensor.borrow(), &*sepal_width_sensor.borrow(), true
        ).unwrap();
        assert!(mi > 0f64);
        println!("mutual_information petal_length-sepal_width weighted {:?}", mi);

        // variety
        let mi = super::mutual_information(
            &*variety_sensor.borrow(), &*petal_length_sensor.borrow(), true
        ).unwrap();
        assert!(mi > 0f64);
        println!("mutual_information variety-petal_length {:?}", mi);

        let mi = super::mutual_information(
            &*variety_sensor.borrow(), &*petal_width_sensor.borrow(), true
        ).unwrap();
        assert!(mi > 0f64);
        println!("mutual_information variety-petal_width {:?}", mi);

        let mi = super::mutual_information(
            &*variety_sensor.borrow(), &*sepal_length_sensor.borrow(), true
        ).unwrap();
        assert!(mi > 0f64);
        println!("mutual_information variety-sepal_length {:?}", mi);

        let mi = super::mutual_information(
            &*variety_sensor.borrow(), &*sepal_width_sensor.borrow(), true
        ).unwrap();
        assert!(mi > 0f64);
        println!("mutual_information variety-sepal_width {:?}", mi);
    }
}