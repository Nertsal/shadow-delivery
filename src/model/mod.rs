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
        let vertices = self
            .obstacles
            .collider
            .iter()
            .flat_map(|(_, collider)| {
                let vs = collider.vertices().map(|v| v.map(Coord::as_f32));
                let sides = [
                    (vs[0], vs[1]),
                    (vs[1], vs[2]),
                    (vs[2], vs[3]),
                    (vs[3], vs[0]),
                ];

                sides.into_iter().flat_map(|(a, b)| {
                    let a_normal = (a - b).rotate_90().normalize_or_zero();
                    let [a, b] = [a, b].map(|v| render::NormalVertex { a_pos: v, a_normal });
                    let [a1, b1] = [a, b].map(|mut v| {
                        v.a_normal = vec2::ZERO;
                        v
                    });
                    [b1, a1, a, b1, a, b]
                })
            })
            .collect();
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
