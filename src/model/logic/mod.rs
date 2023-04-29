use super::*;

const PLAYER_DRAG: f32 = 0.1;
const PLAYER_MAX_SPEED: f32 = 5.0;
const PLAYER_TURN_SPEED: f32 = 3.0;
const PLAYER_ACCELERATION: f32 = 10.0;

impl World {
    pub fn update(&mut self, player_control: PlayerControl, delta_time: Time) {
        self.control_player(player_control, delta_time);
        self.movement(delta_time);
        self.collisions();
    }

    fn control_player(&mut self, control: PlayerControl, delta_time: Time) {
        self.player.collider.rotation += control.turn * Coord::new(PLAYER_TURN_SPEED) * delta_time;
        self.player.speed -= self.player.speed * Coord::new(1.0 - PLAYER_DRAG) * delta_time;
        let target_speed =
            self.player.speed + control.accelerate * Coord::new(PLAYER_ACCELERATION) * delta_time;
        self.player.speed = target_speed.clamp(Coord::ZERO, Coord::new(PLAYER_MAX_SPEED));
    }

    fn movement(&mut self, delta_time: Time) {
        let dir = vec2::UNIT_X.rotate(self.player.collider.rotation);
        let delta = dir * self.player.speed * delta_time;
        self.player.collider.translate(delta);
    }

    fn collisions(&mut self) {
        #[derive(StructQuery)]
        struct ObstacleRef<'a> {
            collider: &'a Collider,
        }

        let player = &mut self.player;

        for obstacle in query_obstacle_ref!(self.obstacles).values() {
            if let Some(collision) = player.collider.collide(obstacle.collider) {
                player
                    .collider
                    .translate(-collision.normal * collision.penetration);
                // TODO: fix velocity
            }
        }
    }
}
