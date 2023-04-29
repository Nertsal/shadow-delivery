use crate::{
    model::{Coord, Level, PlayerControl, Time, World},
    render::{GameRender, RenderCache},
};

use super::*;

const KEYS_ACC: [geng::Key; 2] = [geng::Key::W, geng::Key::Up];
const KEYS_DEC: [geng::Key; 2] = [geng::Key::S, geng::Key::Down];
const KEYS_LEFT: [geng::Key; 2] = [geng::Key::A, geng::Key::Left];
const KEYS_RIGHT: [geng::Key; 2] = [geng::Key::D, geng::Key::Right];

pub struct Game {
    geng: Geng,
    assets: Rc<Assets>,
    render: GameRender,
    render_cache: RenderCache,
    world: World,
    draw_hitboxes: bool,
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>, level: Level) -> Self {
        let world = World::new(level);
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            render: GameRender::new(geng, assets),
            render_cache: RenderCache::calculate(&world, geng, assets),
            world,
            draw_hitboxes: cfg!(debug_assertions),
        }
    }

    fn get_player_control(&mut self) -> PlayerControl {
        let mut control = PlayerControl {
            accelerate: Coord::ZERO,
            turn: Coord::ZERO,
        };
        let window = self.geng.window();
        let pressed = |keys: &[geng::Key]| keys.iter().any(|key| window.is_key_pressed(*key));
        if pressed(&KEYS_ACC) {
            control.accelerate += Coord::ONE;
        }
        if pressed(&KEYS_DEC) {
            control.accelerate -= Coord::ONE;
        }
        if pressed(&KEYS_LEFT) {
            control.turn += Coord::ONE;
        }
        if pressed(&KEYS_RIGHT) {
            control.turn -= Coord::ONE;
        }
        control
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        self.render.draw(
            &self.world,
            self.draw_hitboxes,
            &self.render_cache,
            framebuffer,
        );
    }

    fn handle_event(&mut self, event: geng::Event) {
        if let geng::Event::KeyDown { key: geng::Key::F2 } = event {
            self.draw_hitboxes = !self.draw_hitboxes;
        }
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = Time::new(delta_time as f32);
        let player_control = self.get_player_control();
        self.world.update(player_control, delta_time);
    }
}
