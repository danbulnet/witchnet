#![allow(incomplete_features)] // TODO: check if stable versions allows for const generic parameters not only standalone in constexpr
#![feature(generic_const_exprs)] // TODO: check if stable versions allows for const generic parameters not only standalone in constexpr
#![feature(map_try_insert)]

pub mod neuron;
pub mod dynamic;
pub mod simple;
pub mod algorithm;

#[cfg(test)]
mod tests {
    
}
