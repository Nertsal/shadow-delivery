use super::*;

const CAMERA_INTERPOLATION: f32 = 0.5;

const WAYPOINT_DISTANCE_MIN: f32 = 5.0;
const WAYPOINT_DISTANCE_MAX: f32 = 20.0;

const DEATH_PENALTY: Score = 1000;
const DELIVER_SCORE: Score = 500;
const SHADOW_BONUS: Score = 1000;
const SHADOW_MAX_VIS: f32 = 0.05;

// const HEALTH_RESTORE: f32 = 10.0;
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
        self.update_particles(delta_time);
        self.update_player(player_visibility, delta_time);
        self.control_player(player_control, delta_time);
        self.obstacles_movement(delta_time);
        self.player_movement(delta_time);
        self.collisions();
        self.waypoints();
        self.update_lamps(delta_time);
        self.update_camera(delta_time);
    }

    fn update_lamps(&mut self, delta_time: Time) {
        #[derive(StructQuery)]
        struct LampRef<'a> {
            state: &'a mut LampState,
            up_time: &'a Time,
            down_time: &'a Time,
        }
        let mut query = query_lamp_ref!(self.level.lamps);
        let mut iter = query.iter_mut();
        while let Some((_, lamp)) = iter.next() {
            let (LampState::Up(time) | LampState::Down(time)) = lamp.state;
            *time -= delta_time;
            if *time <= Time::ZERO {
                *lamp.state = match lamp.state {
                    LampState::Up(_) => {
                        if *lamp.down_time > Time::ZERO {
                            LampState::Down(*lamp.down_time)
                        } else {
                            LampState::Up(Time::ZERO)
                        }
                    }
                    LampState::Down(_) => LampState::Up(*lamp.up_time),
                };
            }
        }
    }

    fn update_particles(&mut self, delta_time: Time) {
        #[derive(StructQuery)]
        struct ParticleRef<'a> {
            position: &'a mut vec2<Coord>,
            velocity: &'a vec2<Coord>,
            lifetime: &'a mut Time,
            radius: &'a mut Coord,
        }
        let mut dead = Vec::new();
        let mut query = query_particle_ref!(self.particles);
        let mut iter = query.iter_mut();
        while let Some((id, particle)) = iter.next() {
            *particle.lifetime -= delta_time;
            if *particle.lifetime <= Time::ZERO {
                dead.push(id);
                continue;
            }
            *particle.position += *particle.velocity * delta_time;
            let time = Time::new(0.2);
            *particle.radius = (*particle.lifetime).min(time) / time * Coord::new(0.1);
        }
        dead.sort();
        for id in dead.into_iter().rev() {
            self.particles.remove(id);
        }
    }

    fn update_player(&mut self, visibility: R32, delta_time: Time) {
        // if visibility == R32::ZERO {
        //     self.player.health = (self.player.health + Health::new(HEALTH_RESTORE) * delta_time)
        //         .min(Health::new(100.0));
        // }

        if visibility.as_f32() < SHADOW_MAX_VIS {
            return;
        }
        self.player.shadow_bonus = false;

        // Particles
        let p = f64::from(visibility.as_f32()) * 0.5;
        let mut rng = thread_rng();
        if rng.gen_bool(p) {
            let position = rng.gen_circle(self.player.collider.pos(), Coord::new(0.1));
            let speed = 1.0;
            let angle = rng.gen_range(0.0..f32::PI * 2.0);
            let velocity = (Angle::new_radians(angle).unit_direction() * speed).map(Coord::new);
            self.particles.insert(Particle {
                position,
                velocity,
                lifetime: Time::new(0.5),
                radius: Coord::new(0.1),
                color: Rgba::WHITE,
            });
        }

        self.player.health =
            (self.player.health - visibility * Health::new(200.0) * delta_time).max(Health::ZERO);
        if self.player.health <= Health::ZERO {
            self.kill_player();
        }
    }

    fn kill_player(&mut self) {
        self.player.shadow_bonus = true;
        self.player.score = self.player.score.saturating_sub(DEATH_PENALTY);
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

            let speed = item.path.move_speed;
            let angular_speed = item.path.angular_speed;

            let delta = target - item.collider.pos();
            let len = delta.len();
            let max_len = speed * delta_time;
            if len < max_len {
                item.path.next_point += 1;
            }

            let target_angle = Angle::new_radians(delta.arg().as_f32());
            let max_delta =
                Angle::new_radians((angular_speed * delta_time).as_f32().clamp_abs(f32::PI));
            let angle_delta = (target_angle - item.collider.rotation).clamp_abs(max_delta);

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
        let query = query_waypoint_ref!(self.level.waypoints);

        let active_id = self.active_waypoint;
        let Some(active) = query.get(active_id) else {
            self.next_waypoint();
            return;
        };

        if player.collider.check(active.collider) {
            player.score += DELIVER_SCORE;
            if player.shadow_bonus {
                player.score += SHADOW_BONUS;
            }
            player.shadow_bonus = true;
            self.next_waypoint();
        }
    }

    fn next_waypoint(&mut self) {
        #[derive(StructQuery)]
        struct WaypointRef<'a> {
            collider: &'a Collider,
        }
        let query = query_waypoint_ref!(self.level.waypoints);

        let Some(last) = query.get(self.active_waypoint) else {
            self.active_waypoint = 0;
            return;
        };

        let mut rng = thread_rng();

        let next = self
            .level
            .waypoints
            .ids()
            .filter(|&id| id != self.active_waypoint)
            .filter_map(|id| query.get(id).map(|item| (id, item)))
            .filter(|(_, item)| {
                let delta = item.collider.pos() - last.collider.pos();
                let distance = delta.len().as_f32();
                (WAYPOINT_DISTANCE_MIN..=WAYPOINT_DISTANCE_MAX).contains(&distance)
            })
            .map(|(id, _)| id)
            .choose(&mut rng);

        let next = next.or_else(|| self.level.waypoints.ids().choose(&mut rng));
        self.active_waypoint = next.unwrap_or(0);
    }

    fn update_camera(&mut self, delta_time: Time) {
        let target = self.player.collider.pos();
        self.camera.center += ((target - self.camera.center.map(Coord::new))
            / Coord::new(CAMERA_INTERPOLATION)
            * delta_time)
            .map(Coord::as_f32);
    }
}
