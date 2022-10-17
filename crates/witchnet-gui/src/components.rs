use bevy::prelude::*;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32
}

#[derive(Component)]
pub struct Activation(pub f32);