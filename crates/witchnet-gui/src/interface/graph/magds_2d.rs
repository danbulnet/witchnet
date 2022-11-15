use std::collections::HashMap;

use bevy::prelude::*;

use witchnet_common::{
    neuron::NeuronID,
    sensor::SensorAsync
};

use crate::{
    resources::{
        appearance::{ Appearance, Selector },
        magds::{ MainMAGDS, PositionXY, BIG_GAP_FACTOR }
    },
    interface::graph::{ sensor_2d, neuron_2d },
    widgets::plot::PlotUi
};

pub(crate) fn magds(
    ui: &mut PlotUi,
    magds_res: &mut ResMut<MainMAGDS>,
    position_xy_res: &mut ResMut<PositionXY>,
    appearance_res: &mut ResMut<Appearance>,
) {
    let neuron_settings = &appearance_res.neurons[&Selector::All];
    let sensor_settings = &appearance_res.sensors[&Selector::All];
    let connection_settings = &appearance_res.connections[&Selector::All];

    let magds = magds_res.0.read().unwrap();
    let sensors = magds.sensors();
    let neurons = magds.neurons();

    let mut current_top_x = 0.0f64;
    let mut current_bottom_x = 0.0f64;
    let mut sensor_point_map: HashMap<NeuronID, [f64; 2]> = HashMap::new();

    for sensor in sensors {
        let sensor_id = sensor.read().unwrap().id();
        sensor_2d::sensory_field(
            ui, 
            &magds.sensor_name(sensor_id).unwrap(),
            position_xy_res.sensors[&sensor_id], 
            sensor.clone(),
            position_xy_res,
            sensor_settings,
            connection_settings,
            false
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
    neuron_2d::neurons(
        ui, 
        "neurons", 
        neurons, 
        position_xy_res,
        neuron_settings, 
        connection_settings,
    );
}