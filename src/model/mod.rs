use super::*;

mod angle;
mod collider;
mod geometry;
mod level;
mod lights;
mod logic;
mod world;

pub use angle::*;
pub use collider::*;
pub use level::*;
pub use lights::*;
pub use world::*;

const PLAYER_SIZE: vec2<f32> = vec2(0.6, 0.2);

pub type Coord = R32;
pub type Time = R32;
pub type Health = R32;
pub type Score = u64;
pub type Color = Rgba<f32>;

pub struct Player {
    pub shadow_bonus: bool,
    pub score: Score,
    pub health: Health,
    pub collider: Collider,
    pub velocity: vec2<Coord>,
}

pub struct PlayerControl {
    pub accelerate: Coord,
    pub turn: Coord,
}

#[derive(StructOf)]
pub struct Particle {
    pub position: vec2<Coord>,
    pub velocity: vec2<Coord>,
    pub lifetime: Time,
    pub radius: Coord,
    pub color: Color,
}
