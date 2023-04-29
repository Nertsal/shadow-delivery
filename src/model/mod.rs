use super::*;

mod collider;
mod level;
mod lights;
mod logic;

pub use collider::*;
pub use level::*;
pub use lights::*;

const PLAYER_SIZE: vec2<f32> = vec2(0.6, 0.2);

pub type Coord = R32;
pub type Time = R32;

pub struct World {
    pub player: Player,
    pub obstacles: StructOf<Vec<Obstacle>>,
    pub camera: Camera2d,
    pub global_light: GlobalLight,
}

#[derive(StructOf, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Obstacle {
    pub collider: Collider,
    /// In relative coordinates.
    pub lights: Vec<Spotlight>,
}

pub struct Player {
    pub collider: Collider,
    pub speed: Coord,
}

pub struct PlayerControl {
    pub accelerate: Coord,
    pub turn: Coord,
}

impl World {
    pub fn new(level: Level) -> Self {
        let mut obstacles = StructOf::<Vec<Obstacle>>::new();
        for obstacle in level.obstacles {
            obstacles.insert(obstacle);
        }

        Self {
            player: Player {
                collider: Collider::new(Aabb2::ZERO.extend_symmetric(PLAYER_SIZE.map(Coord::new))),
                speed: Coord::ZERO,
            },
            obstacles,
            camera: Camera2d {
                center: vec2::ZERO,
                rotation: 0.0,
                fov: 30.0,
            },
            global_light: GlobalLight {
                color: Rgba::WHITE,
                intensity: 0.1,
            },
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
