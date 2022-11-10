use std::f64::consts::PI;

use bevy::prelude::*;

use witchnet_common::{
    sensor::SensorAsync
};

use magds::asynchronous::{ magds::MAGDS };

use crate::{
    resources::{
        appearance::{ Appearance, Selector },
        magds::{ 
            PositionXY,
            BIG_GAP_FACTOR,
            SMALL_GAP_FACTOR
        }
    }
};

pub(crate) fn set_positions(
    magds: &MAGDS,
    origin: (f64, f64),
    position_xy_res: &mut ResMut<PositionXY>,
    appearance_res: &mut ResMut<Appearance>
) {
    let sensors = magds.sensors();
    let sensor_points = &mut position_xy_res.sensors;
    let sensor_size = appearance_res.sensors[&Selector::All].size;

    let mut current_top_x = 0.0f64;
    let mut current_bottom_x = 0.0f64;
    for sensor in sensors {
        let sensor = sensor.read().unwrap();
        let sensor_id = sensor.id();
        if current_top_x < current_bottom_x {
            sensor_points.insert(sensor_id, (current_top_x, origin.1 + 10.0));
            current_top_x += (BIG_GAP_FACTOR * sensor_size) as f64
        } else {
            sensor_points.insert(sensor_id, (current_bottom_x, origin.1 - 10.0));
            current_bottom_x += (BIG_GAP_FACTOR * sensor_size) as f64
        }
    }
    
    let neurons = magds.neurons();
    let neuron_points = &mut position_xy_res.neurons;
    let neuron_size = appearance_res.neurons[&Selector::All].size;

    let (neuron_points_vec, _neuron_r) = neuron_positions(
        origin,
        neurons.len(),
        neuron_size as f64,
        SMALL_GAP_FACTOR as f64
    );

    for (i, neuron) in neurons.into_iter().enumerate() {
        let neuron_id = neuron.read().unwrap().id();
        neuron_points.insert(neuron_id, neuron_points_vec[i]);
    }
}

pub(crate) fn neuron_positions(
    origin: (f64, f64), n: usize, size: f64, gap: f64
) -> (Vec<(f64, f64)>, f64) {
    if n == 0 {
        (vec![], 0.0)
    } else if n == 1 {
        (vec![origin], 0.0)
    } else {
        let mut points = Vec::new();

        let distance = size + gap;
        let mut r = distance;
        let mut current_position = 0;
        while current_position < n {
            let l_total = circler_to_l(r);
            let circle_count = usize::min(
                (l_total / distance).trunc() as usize, 
                n - current_position
            );
            for i in 0..circle_count {
                let l_current = i as f64 * l_total / circle_count as f64;
                let (x, y, alpha) = circle_geometry(l_current, l_total);
                points.push((x + origin.0, y + origin.1));
                current_position += 1;
            }
            r += distance;
        }
        (points, r)
    }
}

pub fn circler_to_l(r: f64) -> f64 { 2.0 * PI * r }

pub fn circlel_to_r(l: f64) -> f64 { 0.5 * l / 2.0 * PI }

pub fn circle_y(x: f64, r: f64) -> f64 { (r.powi(2) - x.powi(2)).sqrt() }

pub(crate) fn circle_geometry(l_current: f64, l_total: f64) -> (f64, f64, f64) {
    let r = circlel_to_r(l_total);
    let l_norm = l_current / l_total;
    let alpha = l_norm * 2.0 * PI;
    let x = alpha.cos() * r;
    let y_sign = if l_norm > 0.5 && l_norm < 1.0 { -1.0 } else { 1.0 };
    (x, y_sign * circle_y(x, r), alpha)
}