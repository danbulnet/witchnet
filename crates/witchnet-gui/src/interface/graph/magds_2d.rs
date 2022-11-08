use bevy::prelude::*;

use witchnet_common::sensor::SensorAsync;

use crate::{
    resources::{
        appearance::{ Appearance, Selector },
        magds::MainMAGDS
    },
    interface::graph::{ sensor_2d, neuron_2d },
    widgets::plot::PlotUi
};

pub fn magds(
    ui: &mut PlotUi,
    magds_res: &mut ResMut<MainMAGDS>,
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
    for sensor in sensors {
        let sensor_id = sensor.read().unwrap().id();
        let sensor_name = magds.sensor_name(sensor_id).unwrap();

        if current_top_x < current_bottom_x {
            current_top_x += sensor_2d::sensor(
                ui, 
                &sensor_name,
                (current_top_x, 20.0), 
                sensor.clone(), 
                sensor_settings,
                connection_settings,
                false
            ) + 2.0 * sensor_settings.size as f64;
        } else {
            current_bottom_x += sensor_2d::sensor(
                ui, 
                &sensor_name,
                (current_bottom_x, 0.0), 
                sensor.clone(), 
                sensor_settings,
                connection_settings,
                true
            ) + 2.0 * sensor_settings.size as f64;
        }
    }

    neuron_2d::neurons(
        ui, 
        "neurons", 
        (0.0, 10.0), 
        neurons, 
        neuron_settings, 
        connection_settings,
        f64::max(current_top_x, current_bottom_x) - 2.0 * sensor_settings.size as f64
    );
}