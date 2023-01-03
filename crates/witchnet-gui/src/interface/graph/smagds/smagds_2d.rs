use witchnet_common::{
    sensor::SensorAsync
};

use crate::{
    resources::{
        appearance::Selector ,
        smagds::SMAGDSMain
    },
    interface::graph::smagds::{ sensor_2d, neuron_2d },
    widgets::plot::PlotUi
};

pub(crate) fn smagds(
    ui: &mut PlotUi,
    mut smagds_res: &mut SMAGDSMain
) {
    if smagds_res.smagds.is_some() {
        #[allow(unused)]
        let &mut SMAGDSMain { smagds, appearance, loaded_datasets, positions } = &mut smagds_res;
        let neuron_settings = &appearance.neurons[&Selector::All];
        let sensor_settings = &appearance.sensors[&Selector::All];
        let connection_settings = &appearance.connections[&Selector::All];

        let smagds = smagds.as_ref().unwrap().read().unwrap();

        let magds = &smagds.magds;
        let sensors = magds.sensors();
        let neurons = magds.neurons();

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
        }
    }
}