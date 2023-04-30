use super::*;

mod collider;
mod level;
mod lights;
mod logic;
mod world;

pub use collider::*;
pub use level::*;
pub use lights::*;
pub use world::*;

const PLAYER_SIZE: vec2<f32> = vec2(0.6, 0.2);

pub type Coord = R32;
pub type Time = R32;

pub struct Player {
    pub collider: Collider,
    pub velocity: vec2<Coord>,
}

pub struct PlayerControl {
    pub accelerate: Coord,
    pub turn: Coord,
}