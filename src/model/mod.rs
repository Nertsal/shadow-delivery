use super::*;

mod collider;
mod lights;

pub use collider::*;
pub use lights::*;

const PLAYER_SIZE: vec2<f32> = vec2(0.2, 0.6);

pub type Coord = R32;

pub struct World {
    pub player: Player,
    pub camera: Camera2d,
    pub global_light: GlobalLight,
    pub spotlights: Vec<Spotlight>,
}

pub struct Player {
    pub collider: Collider,
}

impl World {
    pub fn new() -> Self {
        Self {
            player: Player {
                collider: Collider::new(Aabb2::ZERO.extend_symmetric(PLAYER_SIZE.map(Coord::new))),
            },
            camera: Camera2d {
                center: vec2::ZERO,
                rotation: 0.0,
                fov: 30.0,
            },
            global_light: GlobalLight::default(),
            spotlights: vec![Spotlight::default()],
        }
    }

    pub fn calculate_light_geometry(
        &self,
        geng: &Geng,
        assets: &Assets,
    ) -> ugli::VertexBuffer<render::NormalVertex> {
        let mut vertices = vec![];
        ugli::VertexBuffer::new_dynamic(geng.ugli(), vertices)
    }

    // pub fn calculate_normal_geometry(
    //     &self,
    //     geng: &Geng,
    //     assets: &Assets,
    // ) -> ugli::VertexBuffer<render::NormalVertex> {
    //     todo!()
    // }
}
