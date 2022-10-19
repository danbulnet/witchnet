#![allow(incomplete_features)] // TODO: check if stable versions allows for const generic parameters not only standalone in constexpr
#![feature(generic_const_exprs)] // TODO: check if stable versions allows for const generic parameters not only standalone in constexpr
#![feature(map_try_insert)]

pub mod neuron;
pub mod synchronous;
// pub mod asynchronous;

#[cfg(test)]
mod tests {
    
}
