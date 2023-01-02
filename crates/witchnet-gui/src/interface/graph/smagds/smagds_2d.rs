use std::collections::HashMap;

use bevy::prelude::*;

use witchnet_common::{
    neuron::NeuronID,
    sensor::SensorAsync
};

use crate::{
    resources::{
        appearance::{ Appearance, Selector },
        smagds::{ SMAGDSMain, SMAGDSPositions, BIG_GAP_FACTOR }
    },
    interface::graph::smagds::{ sensor_2d, neuron_2d },
    widgets::plot::PlotUi
};

pub(crate) fn smagds(
    ui: &mut PlotUi,
    smagds_res: &mut ResMut<SMAGDSMain>,
    position_xy_res: &mut ResMut<SMAGDSPositions>,
    appearance_res: &mut ResMut<Appearance>,
) {
    let neuron_settings = &appearance_res.neurons[&Selector::All];
    let sensor_settings = &appearance_res.sensors[&Selector::All];
    let connection_settings = &appearance_res.connections[&Selector::All];

    if let Some(smagds) = &smagds_res.smagds {
        let smagds = smagds.read().unwrap();

        let magds = &smagds.magds;
        let sensors = magds.sensors();
        let neurons = magds.neurons();

        neuron_2d::neurons(
            ui, 
            "neurons", 
            neurons, 
            position_xy_res,
            neuron_settings, 
            connection_settings,
        );

        for sensor in sensors {
            let sensor_id = sensor.read().unwrap().id();
            sensor_2d::sensory_field(
                ui, 
                &magds.sensor_name(sensor_id).unwrap(),
                sensor.clone(),
                position_xy_res,
                sensor_settings,
                connection_settings
            );
        }
    }
}