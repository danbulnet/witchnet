use std::sync::Arc;

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
    mut magds_res: &mut MAGDSMain
) {
    #[allow(unused)]
    let &mut MAGDSMain { magds, appearance, loaded_datasets, positions } = &mut magds_res;
    let sensor_settings = &appearance.sensors[&Selector::All];
    let connection_settings = &appearance.connections[&Selector::All];

    let magds = magds.read().unwrap();
    let sensors = magds.sensors();

    for (group_id, neurons) in positions.group_ids_to_neurons.clone() {
        let group_name: Arc<str> = magds.neuron_group_name_from_id(group_id).unwrap().into();
        neuron_2d::neurons(
            ui,
            &magds,
            (group_id, &group_name), 
            &neurons, 
            positions,
            appearance
        );
    }

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
    }
}