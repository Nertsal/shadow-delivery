use super::*;

pub(super) const CAMERA_DEAD_FOV: f32 = 20.0;
pub(super) const CAMERA_ALIVE_FOV: f32 = 30.0;

pub struct World {
    pub assets: Rc<Assets>,
    pub time: Time,
    pub death_time: Option<Time>,
    pub player: Player,
    pub active_waypoint: usize,
    pub level: Level,
    pub obstacles: StructOf<Vec<Obstacle>>,
    pub particles: StructOf<Vec<Particle>>,
    pub camera: Camera2d,
    pub bounced: bool,
    pub hurt_sfx_timeout: Time,
}

impl World {
    pub fn new(assets: &Rc<Assets>, level: Level) -> Self {
        Self {
            assets: assets.clone(),
            time: Time::ZERO,
            death_time: None,
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
            obstacles: StructOf::new(),
            particles: StructOf::new(),
            camera: Camera2d {
                center: vec2::ZERO,
                rotation: 0.0,
                fov: CAMERA_DEAD_FOV,
            },
            bounced: false,
            hurt_sfx_timeout: Time::ZERO,
        }
    }
}
