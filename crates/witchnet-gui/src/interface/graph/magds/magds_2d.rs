use bevy::prelude::*;

use witchnet_common::{
    sensor::SensorAsync
};

use crate::{
    resources::{
        appearance::Selector,
        magds::MAGDSMain
    },
    interface::graph::magds::{ sensor_2d, neuron_2d },
    widgets::plot::PlotUi
};

pub(crate) fn magds(
    ui: &mut PlotUi,
    magds_res: &mut ResMut<MAGDSMain>
) {
    #[allow(unused)]
    let &mut MAGDSMain { magds, appearance, loaded_datasets, positions } = &mut magds_res.as_mut();

    let neuron_settings = &appearance.neurons[&Selector::All];
    let sensor_settings = &appearance.sensors[&Selector::All];
    let connection_settings = &appearance.connections[&Selector::All];

    let magds = magds.read().unwrap();
    let sensors = magds.sensors();
    let neurons = magds.neurons();

    // let mut current_top_x = 0.0f64;
    // let mut current_bottom_x = 0.0f64;
    // let mut sensor_point_map: HashMap<NeuronID, [f64; 2]> = HashMap::new();

    neuron_2d::neurons(
        ui, 
        "neurons", 
        neurons, 
        positions,
        neuron_settings, 
        connection_settings,
    );

    for sensor in sensors {
        let sensor_id = sensor.read().unwrap().id();
        sensor_2d::sensory_field(
            ui, 
            &magds.sensor_name(sensor_id).unwrap(),
            sensor.clone(),
            positions,
            sensor_settings,
            connection_settings
        );
        // current_top_x += x + 2.0 * sensor_settings.size as f64;

        // if current_top_x < current_bottom_x {
        //     let (current_map, x) = sensor_2d::sensor(
        //         ui, 
        //         &sensor_name,
        //         position, 
        //         sensor.clone(), 
        //         sensor_settings,
        //         connection_settings,
        //         false
        //     );
        //     current_top_x += x + 2.0 * sensor_settings.size as f64;
        //     sensor_point_map.extend(current_map);
        // } else {
        //     let (current_map, x) = sensor_2d::sensor(
        //         ui, 
        //         &sensor_name,
        //         (current_bottom_x, 0.0), 
        //         sensor.clone(), 
        //         sensor_settings,
        //         connection_settings,
        //         true
        //     );
        //     current_bottom_x += x + 2.0 * sensor_settings.size as f64;
        //     sensor_point_map.extend(current_map);
        // }
    }

    // neuron_2d::neurons(
    //     ui, 
    //     "neurons", 
    //     (0.0, 10.0), 
    //     neurons, 
    //     neuron_settings, 
    //     connection_settings,
    //     f64::max(current_top_x, current_bottom_x) - 2.0 * sensor_settings.size as f64,
    //     sensor_point_map
    // );
}