use super::*;

const HEALTH_RESTORE: f32 = 10.0;
const PLAYER_DRAG: f32 = 0.2;
const PLAYER_MAX_SPEED: f32 = 5.0;
const PLAYER_TURN_SPEED: f32 = 3.0;
const PLAYER_ACCELERATION: f32 = 10.0;

impl World {
    pub fn update(
        &mut self,
        player_control: PlayerControl,
        player_visibility: R32,
        delta_time: Time,
    ) {
        self.update_player(player_visibility, delta_time);
        self.control_player(player_control, delta_time);
        self.obstacles_movement(delta_time);
        self.player_movement(delta_time);
        self.collisions();
        self.waypoints();
    }

    fn update_player(&mut self, visibility: R32, delta_time: Time) {
        if visibility == R32::ZERO {
            self.player.health = (self.player.health + Health::new(HEALTH_RESTORE) * delta_time)
                .min(Health::new(100.0));
        }

        self.player.health =
            (self.player.health - visibility * Health::new(100.0) * delta_time).max(Health::ZERO);
        if self.player.health <= Health::ZERO {
            self.kill_player();
        }
    }

    fn kill_player(&mut self) {
        self.player.velocity = vec2::ZERO;
        self.player.health = Health::new(100.0);
        self.player.collider.teleport(self.level.spawn_point);
        self.player.collider.rotation = Angle::ZERO;
    }

    fn control_player(&mut self, control: PlayerControl, delta_time: Time) {
        self.player.collider.rotation +=
            Angle::new_radians(control.turn.as_f32() * PLAYER_TURN_SPEED * delta_time.as_f32());

        let mut speed = self.player.velocity.len();
        speed -= speed * Coord::new(1.0 - PLAYER_DRAG) * delta_time;
        let target_speed =
            speed + control.accelerate * Coord::new(PLAYER_ACCELERATION) * delta_time;
        speed = target_speed.clamp(Coord::ZERO, Coord::new(PLAYER_MAX_SPEED));

        let target_velocity = self
            .player
            .collider
            .rotation
            .unit_direction()
            .map(Coord::new)
            * speed;
        self.player.velocity += (target_velocity - self.player.velocity)
            .clamp_len(..=Coord::new(PLAYER_ACCELERATION) * delta_time);
    }

    fn obstacles_movement(&mut self, delta_time: Time) {
        #[derive(StructQuery)]
        struct ObstacleRef<'a> {
            collider: &'a mut Collider,
            #[query(component = "Option<Path>")]
            path: &'a mut Path,
        }
        let mut query = query_obstacle_ref!(self.level.obstacles);
        let mut iter = query.iter_mut();
        while let Some((_, item)) = iter.next() {
            let Some(&target) = item.path.points.get(item.path.next_point) else {
                item.path.next_point = 0;
                continue;
            };

            let speed = Coord::new(5.0);
            let angular_speed = Coord::new(2.0);

            let delta = target - item.collider.pos();
            let len = delta.len();
            let max_len = speed * delta_time;
            if len < max_len {
                item.path.next_point += 1;
            }

            let target_angle = Angle::new_radians(delta.arg().as_f32());
            let angle_delta = (target_angle - item.collider.rotation)
                .clamp_abs(Angle::new_radians((angular_speed * delta_time).as_f32()));

            item.collider.rotation += angle_delta;
            item.collider.translate(
                item.collider.rotation.unit_direction().map(Coord::new) * speed * delta_time,
            );
        }
    }

    fn player_movement(&mut self, delta_time: Time) {
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
