use rand::{
    prelude::*,
    rngs::StdRng
};

use asa_graphs::{
    simple::graph::ASAGraph as ASAGraphSimple,
    neural::graph::ASAGraph as ASAGraphNeural
};

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let mut rng = StdRng::seed_from_u64(35);

    let mut graph_simple = ASAGraphSimple::<i32, 3>::new("test");

    for _i in 0..10_000 {
        let number = rng.gen_range(0..58);
        graph_simple.insert(&number);
    }

    println!("graph_simple");
    graph_simple.print_graph();
    println!();

    let mut rng = StdRng::seed_from_u64(35);

    let mut graph_neural = ASAGraphNeural::<i32, 3>::new(1);

    for _i in 0..10_000 {
        let number = rng.gen_range(0..58);
        graph_neural.insert(&number);
    }

    println!("graph_neural");
    graph_neural.print_graph();

    fuzzy_activate()
}

use std::rc::Rc;
use std::cell::RefCell;
use witchnet_common::neuron::Neuron;
fn fuzzy_activate() {
    let graph = Rc::new(RefCell::new(ASAGraphNeural::<i32, 3>::new(1)));
    for i in 1..=9 { graph.borrow_mut().insert(&i); }

    let mid_element = graph.borrow().search(&5).unwrap();
    mid_element.borrow_mut().fuzzy_activate(1.0f32);
    assert_eq!(mid_element.borrow().activation(), 1.0f32);
    let mid_element_ref =  mid_element.borrow();

    let (left_neighbour_ptr, left_neighbour_weight) = mid_element_ref.prev.as_ref().unwrap();
    println!("left_neighbour_weight {left_neighbour_weight}");
    let left_neighbour = left_neighbour_ptr.upgrade().unwrap();
    println!("left_neighbour activation {}", left_neighbour.borrow().activation());
    let left_neighbour_ref =  left_neighbour.borrow();

    let (left_left_neighbour_ptr, left_left_neighbour_weight) = left_neighbour_ref.prev.as_ref().unwrap();
    println!("left_left_neighbour_weight {left_left_neighbour_weight}");
    let left_left_neighbour = left_left_neighbour_ptr.upgrade().unwrap();
    println!("left_left_neighbour activation {}", left_left_neighbour.borrow().activation());

    let (right_neighbour_ptr, right_neighbour_weight) = mid_element_ref.next.as_ref().unwrap();
    println!("right_neighbour_weight {right_neighbour_weight}");
    let right_neighbour = right_neighbour_ptr.upgrade().unwrap();
    println!("right_neighbour activation {}", right_neighbour.borrow().activation());
    let right_neighbour_ref =  right_neighbour.borrow();

    let (right_right_neighbour_ptr, right_right_neighbour_weight) = right_neighbour_ref.next.as_ref().unwrap();
    println!("right_right_neighbour_weight {right_right_neighbour_weight}");
    let right_right_neighbour = right_right_neighbour_ptr.upgrade().unwrap();
    println!("right_right_neighbour activation {}", right_right_neighbour.borrow().activation());

    let element_min = graph.borrow().element_min.as_ref().unwrap().clone();
    assert_eq!(element_min.borrow().activation(), 0.0f32);

    let element_max = graph.borrow().element_max.as_ref().unwrap().clone();
    assert_eq!(element_max.borrow().activation(), 0.0f32);
}