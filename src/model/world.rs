use super::*;

pub struct World {
    pub player: Player,
    pub level: Level,
    pub camera: Camera2d,
}

impl World {
    pub fn new(level: Level) -> Self {
        Self {
            player: Player {
                health: Health::new(100.0),
                collider: Collider::new(Aabb2::ZERO.extend_symmetric(PLAYER_SIZE.map(Coord::new))),
                velocity: vec2::ZERO,
            },
            level,
            camera: Camera2d {
                center: vec2::ZERO,
                rotation: 0.0,
                fov: 30.0,
            },
        }
    }

    pub fn calculate_light_geometry(
        &self,
        geng: &Geng,
        _assets: &Assets,
    ) -> ugli::VertexBuffer<render::NormalVertex> {
        let vertices = self
            .level
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
