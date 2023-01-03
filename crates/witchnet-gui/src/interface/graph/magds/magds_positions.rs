use std::{
    f64::consts::PI,
    collections::HashMap,
    sync::Arc,
    marker::PhantomData
};

use rand::{thread_rng, seq::SliceRandom};

use witchnet_common::{
    sensor::{SensorAsync, SensorData},
    neuron::{NeuronID, NeuronAsync}, 
    data::{ DataType, DataTypeValue, DataDeductor }
};

use asa_graphs::neural_async::{
    graph::ASAGraph
};

use magds::asynchronous::{ magds::MAGDS, sensor::SensorConatiner };

use crate::{
    resources::{
        appearance::{ Appearance, Selector },
        magds::{ 
            MAGDSPositions,
            BIG_GAP_FACTOR,
            SMALL_GAP_FACTOR,
            SENSOR_NEURON_GAP_R_FRACTION
        }
    }
};

pub(crate) fn set_positions(
    magds: &MAGDS,
    origin: (f64, f64),
    position_xy_res: &mut MAGDSPositions,
    appearance_res: &mut Appearance
) {
    let radius = neuron_positions(magds, origin, position_xy_res, appearance_res);
    
    sensor_positions(
        magds, 
        origin, 
        radius * SENSOR_NEURON_GAP_R_FRACTION as f64, 
        position_xy_res, 
        appearance_res
    );
}

fn sensor_positions(
    magds: &MAGDS,
    origin: (f64, f64),
    radius: f64,
    mut position_xy_res: &mut MAGDSPositions,
    appearance_res: &mut Appearance
) {
    let sensors = magds.sensors();

    let sensor_size = appearance_res.sensors[&Selector::All].size;
    let level_gap = appearance_res.sensors[&Selector::All].level_gap;

    let sensor_points_vec = empty_circle_positions(
        origin,
        radius,
        sensors.len(),
        sensor_size as f64,
        BIG_GAP_FACTOR as f64
    );

    for (i, sensor) in sensors.into_iter().enumerate() {
        let sensor = sensor.read().unwrap();
        let sensor_id = sensor.id();

        let (position, angle) = sensor_points_vec[i];
        let angle = angle - PI / 2.0;
        let title_pos = sensor_neurons_positions(
            magds, position, angle, level_gap as f64, &sensor, &mut position_xy_res
        );
        
        position_xy_res.sensors.insert(sensor_id, (title_pos, angle));
    }
}

fn sensor_neurons_positions(
    magds: &MAGDS,
    origin: (f64, f64),
    angle: f64,
    level_gap: f64,
    sensor: &SensorConatiner,
    position_xy_res: &mut MAGDSPositions
) -> (f64, f64) {
    let sensor_levels = sensor_to_asa_3_levels(magds, &sensor);
    let gap = SMALL_GAP_FACTOR as f64;

    let mut unrotated_points: HashMap<NeuronID, (f64, f64)> = HashMap::new();

    let mut y = origin.1 + gap;
    for level in sensor_levels.into_iter() {
        let level_width: f64 = (&level).into_iter().map(
            |n| n.len() as f64 * (gap / 2.0)
        ).map(|nw| nw + 0.5 * gap).sum();
        let mut x = origin.0 - level_width / 2.0;
        let level_y = y;
        for (li, node) in (&level).into_iter().enumerate() {
            let node_y = y;
            if li < level.len() / 2 {
                for (i, neuron_id) in (&node).into_iter().enumerate() {
                    if i % 2 == 0 { y = level_y; } else { y = level_y - gap / 2.0 };
                    // position_xy_res.sensor_neurons.insert(neuron_id.clone(), (x, y));
                    unrotated_points.insert(neuron_id.clone(), (x, y));
                    x += gap / 2.0;
                }
            } else {
                for (i, neuron_id) in (&node).into_iter().enumerate() {
                    if node.len() % 2 == 1 {
                        if i % 2 == 0 {
                            y = level_y; 
                        } else {
                            y = level_y - gap / 2.0; 
                        }
                    } else {
                        if i % 2 == 0 {
                            y = level_y - gap / 2.0; 
                        } else {
                            y = level_y; 
                        }
                    }
                    // position_xy_res.sensor_neurons.insert(neuron_id.clone(), (x, y));
                    unrotated_points.insert(neuron_id.clone(), (x, y));
                    x += gap / 2.0;
                }
            }
            x += 0.5 * gap;
            y = node_y;
        }
        y = level_y;
        y += level_gap * gap;
    }

    rotate_by_angle(&unrotated_points, angle, origin, position_xy_res);
    
    rotate_point_around_origin((origin.0, y), origin, angle)
}

fn rotate_by_angle(
    points: &HashMap<NeuronID, (f64, f64)>,
    angle_in_radians: f64,
    origin: (f64, f64),
    position_xy_res: &mut MAGDSPositions
) {
    for (id, point) in &mut points.into_iter() {
        let point = rotate_point_around_origin(*point, origin, angle_in_radians);
        position_xy_res.sensor_neurons.insert(id.clone(), point);
    }
}

pub fn rotate_point_around_origin(
    mut point: (f64, f64), origin: (f64, f64), angle_in_radians: f64
) -> (f64, f64) {
    point = (point.0 - origin.0, point.1 - origin.1);

    let s = f64::sin(angle_in_radians);
    let c = f64::cos(angle_in_radians);

    point = (point.0 * c - point.1 * s, point.0 * s + point.1 * c);

    point = (point.0 + origin.0, point.1 + origin.1);

    point
}

fn levels<T: SensorData + Send + Sync>(
    magds: &MAGDS,
    graph: &ASAGraph<T, 3>
) -> Vec<Vec<Vec<NeuronID>>> where 
    PhantomData<T>: DataDeductor, 
    SensorConatiner: From<Box<dyn SensorAsync<T>>>,
    DataTypeValue: From<T>
{
    graph.levels().into_iter().map(
        |v| v.into_iter().map(
            |n| n.into_iter().map(
                |e| {
                    let element = e.read().unwrap();
                    let sensor = magds.sensor(graph.id()).unwrap().read().unwrap();
                    let value = element.value();
                    sensor.search(&value).unwrap().read().unwrap().id()
                }
            ).collect()
        ).collect()
    ).collect()
}

fn sensor_to_asa_3_levels(
    magds: &MAGDS,
    sensor: &SensorConatiner
) -> Vec<Vec<Vec<NeuronID>>> {
    let sensor_id = sensor.id();
    let mut data = sensor.values();
    data.shuffle(&mut thread_rng());
    match sensor.data_type() {
        DataType::Bool => {
            let data: Vec<bool> = data.into_iter().map(|x| *x.as_bool().unwrap()).collect();
            let graph = ASAGraph::<bool, 3>::new_box_from_vec(sensor_id, &data);
            levels(magds, &graph)
        }
        DataType::U8 => {
            let data: Vec<u8> = data.into_iter().map(|x| *x.as_u8().unwrap()).collect();
            let graph = ASAGraph::<u8, 3>::new_box_from_vec(sensor_id, &data);
            levels(magds, &graph)
        }
        DataType::U16 => {
            let data: Vec<u16> = data.into_iter().map(|x| *x.as_u16().unwrap()).collect();
            let graph = ASAGraph::<u16, 3>::new_box_from_vec(sensor_id, &data);
            levels(magds, &graph)
        }
        DataType::U32 => {
            let data: Vec<u32> = data.into_iter().map(|x| *x.as_u32().unwrap()).collect();
            let graph = ASAGraph::<u32, 3>::new_box_from_vec(sensor_id, &data);
            levels(magds, &graph)
        }
        DataType::U64 => {
            let data: Vec<u64> = data.into_iter().map(|x| *x.as_u64().unwrap()).collect();
            let graph = ASAGraph::<u64, 3>::new_box_from_vec(sensor_id, &data);
            levels(magds, &graph)
        }
        DataType::U128 => {
            let data: Vec<u128> = data.into_iter().map(|x| *x.as_u128().unwrap()).collect();
            let graph = ASAGraph::<u128, 3>::new_box_from_vec(sensor_id, &data);
            levels(magds, &graph)
        }
        DataType::USize => {
            let data: Vec<usize> = data.into_iter().map(|x| *x.as_u_size().unwrap()).collect();
            let graph = ASAGraph::<usize, 3>::new_box_from_vec(sensor_id, &data);
            levels(magds, &graph)
        }
        DataType::I8 => {
            let data: Vec<i8> = data.into_iter().map(|x| *x.as_i8().unwrap()).collect();
            let graph = ASAGraph::<i8, 3>::new_box_from_vec(sensor_id, &data);
            levels(magds, &graph)
        }
        DataType::I16 => {
            let data: Vec<i16> = data.into_iter().map(|x| *x.as_i16().unwrap()).collect();
            let graph = ASAGraph::<i16, 3>::new_box_from_vec(sensor_id, &data);
            levels(magds, &graph)
        }
        DataType::I32 => {
            let data: Vec<i32> = data.into_iter().map(|x| *x.as_i32().unwrap()).collect();
            let graph = ASAGraph::<i32, 3>::new_box_from_vec(sensor_id, &data);
            levels(magds, &graph)
        }
        DataType::I64 => {
            let data: Vec<i64> = data.into_iter().map(|x| *x.as_i64().unwrap()).collect();
            let graph = ASAGraph::<i64, 3>::new_box_from_vec(sensor_id, &data);
            levels(magds, &graph)
        }
        DataType::I128 => {
            let data: Vec<i128> = data.into_iter().map(|x| *x.as_i128().unwrap()).collect();
            let graph = ASAGraph::<i128, 3>::new_box_from_vec(sensor_id, &data);
            levels(magds, &graph)
        }
        DataType::ISize => {
            let data: Vec<isize> = data.into_iter().map(|x| *x.as_i_size().unwrap()).collect();
            let graph = ASAGraph::<isize, 3>::new_box_from_vec(sensor_id, &data);
            levels(magds, &graph)
        }
        DataType::F32 => {
            let data: Vec<f32> = data.into_iter().map(|x| *x.as_f32().unwrap()).collect();
            let graph = ASAGraph::<f32, 3>::new_box_from_vec(sensor_id, &data);
            levels(magds, &graph)
        }
        DataType::F64 => {
            let data: Vec<f64> = data.into_iter().map(|x| *x.as_f64().unwrap()).collect();
            let graph = ASAGraph::<f64, 3>::new_box_from_vec(sensor_id, &data);
            levels(magds, &graph)
        }
        DataType::ArcStr => {
            let data: Vec<Arc<str>> = data.into_iter().map(
                |x| x.as_arc_str().unwrap().clone()
            ).collect();
            let graph = ASAGraph::<Arc<str>, 3>::new_box_from_vec(sensor_id, &data);
            levels(magds, &graph)
        }
        DataType::String => {
            let data: Vec<String> = data.into_iter().map(
                |x| x.as_string().unwrap().clone()
            ).collect();
            let graph = ASAGraph::<String, 3>::new_box_from_vec(sensor_id, &data);
            levels(magds, &graph)
        }
        DataType::Unknown => panic!("unknown data type not allowed here")
    }
}

fn neuron_positions(
    magds: &MAGDS,
    origin: (f64, f64),
    position_xy_res: &mut MAGDSPositions,
    appearance_res: &mut Appearance
) -> f64 {
    let neurons = magds.neurons();

    let neuron_size = appearance_res.neurons[&Selector::All].size;
    let neuron_points = &mut position_xy_res.neurons;

    let (neuron_points_vec, r) = full_circle_positions(
        origin,
        neurons.len(),
        neuron_size as f64,
        SMALL_GAP_FACTOR as f64
    );

    for (i, neuron) in neurons.into_iter().enumerate() {
        let neuron_id = neuron.read().unwrap().id();
        neuron_points.insert(neuron_id, neuron_points_vec[i]);
    }

    r
}

pub(crate) fn empty_circle_positions(
    origin: (f64, f64), 
    r: f64, 
    n: usize, 
    size: f64, 
    gap: f64
) -> Vec<((f64, f64), f64)> {
    if n == 0 {
        vec![]
    } else if n == 1 {
        let l_total = circle_r_to_l(r);
        let (x, y, _alpha) = circle_geometry(0f64, l_total);
        vec![((x + origin.0, y + origin.1), 0.0)]
    } else {
        let mut points = Vec::new();

        let distance = size + gap;
        let mut current_position = 0;

        while current_position < n {
            let l_total = f64::max(circle_r_to_l(r), distance * n as f64);
            let circle_space = l_total / n as f64;
            for i in 0..n {
                let l_current = i as f64 * circle_space;
                let (x, y, alpha) = circle_geometry(l_current, l_total);
                points.push(((x + origin.0, y + origin.1), alpha));
                current_position += 1;
            }
        }

        points
    }
}

pub(crate) fn full_circle_positions(
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
            let l_total = circle_r_to_l(r);
            let circle_count = usize::min(
                (l_total / distance).trunc() as usize, 
                n - current_position
            );
            for i in 0..circle_count {
                let l_current = i as f64 * l_total / circle_count as f64;
                let (x, y, _alpha) = circle_geometry(l_current, l_total);
                points.push((x + origin.0, y + origin.1));
                current_position += 1;
            }
            r += distance;
        }
        (points, r)
    }
}

pub fn circle_r_to_l(r: f64) -> f64 { 2.0 * PI * r }

// pub fn circle_l_to_r(l: f64) -> f64 { 0.5 * l / 2.0 * PI }
pub fn circle_l_to_r(l: f64) -> f64 { l / (2.0 * PI) }

pub fn circle_y(x: f64, r: f64) -> f64 { (r.powi(2) - x.powi(2)).sqrt() }

pub(crate) fn circle_geometry(l_current: f64, l_total: f64) -> (f64, f64, f64) {
    let r = circle_l_to_r(l_total);
    let l_norm = l_current / l_total;
    let alpha = l_norm * 2.0 * PI;
    let x = alpha.cos() * r;
    let y_sign = if l_norm > 0.5 && l_norm < 1.0 { -1.0 } else { 1.0 };
    (x, y_sign * circle_y(x, r), alpha)
}