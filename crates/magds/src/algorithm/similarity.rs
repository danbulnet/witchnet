use std::collections::HashSet;

use anyhow::{ Result, Context };

use log::{ log_enabled, Level};

use witchnet_common::{
    data::{DataTypeValue, DataCategory},
    sensor::Sensor,
    distances::{ Distance, DistanceChecked },
    neuron::NeuronID
};

pub fn mutual_information<S1: Sensor<DataTypeValue>, S2: Sensor<DataTypeValue>>(
    s1: &S1, s2: &S2
) -> Result<f64> {
    let mut mutual_information = 0.0;

    let mut s1_total_counter = 0;
    for sensor_neuron in s1.neurons() {
        s1_total_counter += sensor_neuron.borrow().counter();
    }
    let mut s2_total_counter = 0;
    for sensor_neuron in s2.neurons() {
        s2_total_counter += sensor_neuron.borrow().counter();
    }
    let total_counter = usize::max(s1_total_counter, s2_total_counter) as f64;

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
                let p_x_y = coincidences as f64 / total_counter;
                let p_x = s1_sensor_neuron.borrow().counter() as f64;
                let p_y = s2_sensor_neuron.borrow().counter() as f64;
                let mi = p_x_y * f64::log2(p_x_y / (p_x * p_y));
                mutual_information += mi;
                if log_enabled!(Level::Debug) {
                    println!("p_x_y {p_x_y}, p_x {p_x}, p_y {p_y}, mi {mi}");
                }
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
    use env_logger::{ self, Env };

    use crate::simple::{
        parser,
        sensor::SensorConatiner
    };

    #[test]
    fn mutual_information() {
        env_logger::Builder::from_env(Env::default().default_filter_or("debug")).is_test(true).init();

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

        println!(
            "mutual_information sepal_length-petal_length {:?}", 
            super::mutual_information(
                &*petal_length_sensor.borrow(), &*petal_width_sensor.borrow()
            )
        );
    }
}