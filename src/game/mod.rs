use crate::{
    model::{Coord, Level, PlayerControl, Time, World},
    render::{GameRender, RenderCache},
};

use super::*;

mod ui;

const KEYS_ACC: [geng::Key; 2] = [geng::Key::W, geng::Key::Up];
const KEYS_DEC: [geng::Key; 2] = [geng::Key::S, geng::Key::Down];
const KEYS_LEFT: [geng::Key; 2] = [geng::Key::A, geng::Key::Left];
const KEYS_RIGHT: [geng::Key; 2] = [geng::Key::D, geng::Key::Right];

pub struct Game {
    geng: Geng,
    assets: Rc<Assets>,
    render: GameRender,
    render_cache: RenderCache,
    framebuffer_size: vec2<usize>,
    world: World,
    level: Level,
    draw_hitboxes: bool,
    player_visibilty: f32,
    reset: bool,
    music: geng::SoundEffect,
    master_volume: f64,
    music_volume: f64,
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>, level: Level) -> Self {
        let world = World::new(assets, level.clone());
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            render: GameRender::new(geng, assets),
            render_cache: RenderCache::calculate(&world, geng, assets),
            framebuffer_size: vec2(1, 1),
            world,
            level,
            draw_hitboxes: cfg!(debug_assertions),
            player_visibilty: 0.0,
            reset: false,
            music: assets.music.play(),
            master_volume: 1.0,
            music_volume: 0.7,
        }
    }

    fn reset(&mut self) {
        self.world = World::new(&self.assets, self.level.clone());
        self.reset = false;
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
        self.framebuffer_size = framebuffer.size();
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        self.player_visibilty = self.render.draw(
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
        self.geng.audio().set_volume(self.master_volume);
        self.music.set_volume(self.music_volume);

        if self.reset {
            self.reset();
        }

        let delta_time = Time::new(delta_time as f32);
        let player_control = self.get_player_control();
        self.world
            .update(player_control, r32(self.player_visibilty), delta_time);
    }

    fn ui<'a>(&'a mut self, cx: &'a geng::ui::Controller) -> Box<dyn geng::ui::Widget + 'a> {
        self.ui(cx)
    }
}

pub fn run(geng: &Geng) -> impl Future<Output = impl geng::State> {
    let geng = geng.clone();
    async move {
        let assets: Assets = geng::Load::load(geng.asset_manager(), &run_dir().join("assets"))
            .await
            .expect("Failed to load assets");

        let level: model::Level = file::load_json(run_dir().join("assets").join("level.json"))
            .await
            .expect("Failed to load level");

        Game::new(&geng, &Rc::new(assets), level)
    }
}
