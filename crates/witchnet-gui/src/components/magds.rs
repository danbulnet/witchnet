use std::{
    default::Default,
    sync::{ Arc, RwLock }
};

use bevy::prelude::*;

use magds::synchronous::magds::MAGDS;

// #[derive(Component)]
// pub struct MAGDSComponent(pub Arc<RwLock<MAGDS>>);

// impl Default for MAGDSComponent {
//     fn default() -> MAGDSComponent { 
//         Arc::new(RwLock::new(MAGDSComponent(MAGDS::new())))
//     }
// }