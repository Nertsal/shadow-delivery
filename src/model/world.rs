use super::*;

pub struct World {
    pub time: Time,
    pub player: Player,
    pub active_waypoint: usize,
    pub level: Level,
    pub particles: StructOf<Vec<Particle>>,
    pub camera: Camera2d,
}

impl World {
    pub fn new(level: Level) -> Self {
        Self {
            time: Time::ZERO,
            player: Player {
                shadow_bonus: true,
                score: 0,
                health: Health::new(100.0),
                collider: Collider::new(
                    Aabb2::point(level.spawn_point).extend_symmetric(PLAYER_SIZE.map(Coord::new)),
                ),
                velocity: vec2::ZERO,
            },
            active_waypoint: 0,
            level,
            particles: StructOf::new(),
            camera: Camera2d {
                center: vec2::ZERO,
                rotation: 0.0,
                fov: 30.0,
            },
        }
    }
}
