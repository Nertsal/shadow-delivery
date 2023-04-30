use crate::model::*;

use super::*;

mod cache;
mod game;
mod lights;
mod player;
mod util;
mod world;

pub use cache::*;
pub use game::*;
pub use lights::*;
pub use player::*;
pub use util::*;
pub use world::*;

#[derive(ugli::Vertex, Debug, Clone, Copy)]
pub struct NormalVertex {
    pub a_pos: vec2<f32>,
    pub a_normal: vec2<f32>,
}

#[derive(ugli::Vertex, Debug, Clone, Copy)]
pub struct Vertex {
    pub a_pos: vec2<f32>,
    pub a_uv: vec2<f32>,
}

#[derive(ugli::Vertex, Debug, Clone, Copy)]
pub struct MaskedVertex {
    pub a_pos: vec2<f32>,
    pub a_uv: vec2<f32>,
    pub a_mask_uv: vec2<f32>,
}
