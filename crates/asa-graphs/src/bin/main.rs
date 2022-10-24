use std::sync::{ Arc, RwLock };

// use rand::{
//     prelude::*,
//     rngs::StdRng
// };

use asa_graphs::{
    // simple::graph::ASAGraph as ASAGraphSimple,
    neural_async::graph::ASAGraph as ASAGraphNeuralAsync
};

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // let mut rng = StdRng::seed_from_u64(35);

    // let mut graph_simple = ASAGraphSimple::<i32, 3>::new("test");

    // for _i in 0..10_000 {
    //     let number = rng.gen_range(0..58);
    //     graph_simple.insert(&number);
    // }

    // println!("graph_simple");
    // graph_simple.print_graph();
    // println!();

    // let mut rng = StdRng::seed_from_u64(35);

    // let mut graph_neural = ASAGraphNeural::<i32, 3>::new(1);

    // for _i in 0..10_000 {
    //     let number = rng.gen_range(0..58);
    //     graph_neural.insert(&number);
    // }

    // println!("graph_neural");
    // graph_neural.print_graph();

    activate()
}

fn activate() {
    let graph = Arc::new(
        RwLock::new(ASAGraphNeuralAsync::<i32, 3>::new(1))
    );
    for i in 1..=9 { 
        println!("i {i}");
        graph.write().unwrap().insert(&i);
    }
}