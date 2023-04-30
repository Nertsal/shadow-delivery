use super::*;

const PLAYER_DRAG: f32 = 0.2;
const PLAYER_MAX_SPEED: f32 = 5.0;
const PLAYER_TURN_SPEED: f32 = 3.0;
const PLAYER_ACCELERATION: f32 = 10.0;

impl World {
    pub fn update(&mut self, player_control: PlayerControl, delta_time: Time) {
        self.control_player(player_control, delta_time);
        self.movement(delta_time);
        self.collisions();
        self.waypoints();
    }

    fn control_player(&mut self, control: PlayerControl, delta_time: Time) {
        self.player.collider.rotation += control.turn * Coord::new(PLAYER_TURN_SPEED) * delta_time;

        let mut speed = self.player.velocity.len();
        speed -= speed * Coord::new(1.0 - PLAYER_DRAG) * delta_time;
        let target_speed =
            speed + control.accelerate * Coord::new(PLAYER_ACCELERATION) * delta_time;
        speed = target_speed.clamp(Coord::ZERO, Coord::new(PLAYER_MAX_SPEED));

        let target_velocity = vec2::UNIT_X.rotate(self.player.collider.rotation) * speed;
        self.player.velocity += (target_velocity - self.player.velocity)
            .clamp_len(..=Coord::new(PLAYER_ACCELERATION) * delta_time);
    }

    fn movement(&mut self, delta_time: Time) {
        let delta = self.player.velocity * delta_time;
        self.player.collider.translate(delta);
    }

    fn collisions(&mut self) {
        #[derive(StructQuery)]
        struct ObstacleRef<'a> {
            collider: &'a Collider,
        }

        let player = &mut self.player;
        for obstacle in query_obstacle_ref!(self.level.obstacles).values() {
            if let Some(collision) = player.collider.collide(obstacle.collider) {
                player
                    .collider
                    .translate(-collision.normal * collision.penetration);
                let bounciness = Coord::new(0.8);
                player.velocity -= collision.normal
                    * vec2::dot(player.velocity, collision.normal)
                    * (Coord::ONE + bounciness);
            }
        }
    }

    fn waypoints(&mut self) {
        #[derive(StructQuery)]
        struct WaypointRef<'a> {
            collider: &'a Collider,
        }

        let player = &mut self.player;
        let mut hits = query_waypoint_ref!(self.level.waypoints)
            .iter()
            .filter(|(_, waypoint)| player.collider.check(waypoint.collider))
            .map(|(id, _)| id)
            .collect::<Vec<_>>();
        hits.sort();
        for id in hits.into_iter().rev() {
            self.level.waypoints.remove(id);
        }
    }
}
