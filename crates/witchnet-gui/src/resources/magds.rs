use std::{
    default::Default,
    sync::{ Arc, RwLock }
};

use magds::asynchronous::magds::MAGDS;

pub struct MainMAGDS(pub Arc<RwLock<MAGDS>>);

impl Default for MainMAGDS {
    fn default() -> Self { MainMAGDS(MAGDS::new_arc()) }
}