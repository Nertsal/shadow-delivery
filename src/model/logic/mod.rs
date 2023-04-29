use super::*;

const PLAYER_TURN_SPEED: f32 = 3.0;
const PLAYER_ACCELERATION: f32 = 10.0;
const PLAYER_MAX_SPEED: f32 = 5.0;

impl World {
    pub fn update(&mut self, player_control: PlayerControl, delta_time: Time) {
        self.control_player(player_control, delta_time);
        self.movement(delta_time);
    }

    fn control_player(&mut self, control: PlayerControl, delta_time: Time) {
        self.player.rotation += control.turn * Coord::new(PLAYER_TURN_SPEED) * delta_time;
        let target_speed =
            self.player.speed + control.accelerate * Coord::new(PLAYER_ACCELERATION) * delta_time;
        self.player.speed = target_speed.clamp(Coord::ZERO, Coord::new(PLAYER_MAX_SPEED));
    }

    fn movement(&mut self, delta_time: Time) {
        let dir = vec2::UNIT_X.rotate(self.player.rotation);
        let delta = dir * self.player.speed * delta_time;
        self.player.collider.translate(delta);
    }
}
