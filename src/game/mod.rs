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
    #[allow(dead_code)]
    assets: Rc<Assets>,
    render: GameRender,
    render_cache: RenderCache,
    framebuffer_size: vec2<usize>,
    world: World,
    draw_hitboxes: bool,
    player_visibilty: f32,
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>, level: Level) -> Self {
        let world = World::new(level);
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            render: GameRender::new(geng, assets),
            render_cache: RenderCache::calculate(&world, geng, assets),
            framebuffer_size: vec2(1, 1),
            world,
            draw_hitboxes: cfg!(debug_assertions),
            player_visibilty: 0.0,
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
        let delta_time = Time::new(delta_time as f32);
        let player_control = self.get_player_control();
        self.world
            .update(player_control, r32(self.player_visibilty), delta_time);
    }

    fn ui<'a>(&'a mut self, _cx: &'a geng::ui::Controller) -> Box<dyn geng::ui::Widget + 'a> {
        use geng::ui::*;

        let framebuffer_size = self.framebuffer_size.map(|x| x as f32);

        let color = Rgba::lerp(Rgba::GREEN, Rgba::RED, self.player_visibilty);
        let visibility = geng::ui::Text::new(
            format!("Visibility: {:.0}%", self.player_visibilty * 100.0),
            self.geng.default_font(),
            30.0,
            color,
        )
        .align(vec2(0.5, 0.9))
        .fixed_size(framebuffer_size.map(|x| x.into()) * 0.1)
        .align(vec2(0.0, 0.0))
        .padding_left(framebuffer_size.y as f64 * 0.1);

        let color = Rgba::lerp(
            Rgba::RED,
            Rgba::GREEN,
            self.world.player.health.as_f32() / 100.0,
        );
        let health = geng::ui::Text::new(
            format!("Health: {:.0}", self.world.player.health.as_f32()),
            self.geng.default_font(),
            30.0,
            color,
        )
        .align(vec2(0.5, 0.5))
        .fixed_size(framebuffer_size.map(|x| x.into()) * 0.1)
        .align(vec2(0.0, 0.0))
        .padding_left(framebuffer_size.y as f64 * 0.1);

        let score = geng::ui::Text::new(
            format!("Score: {}", self.world.player.score),
            self.geng.default_font(),
            50.0,
            Rgba::WHITE,
        )
        .fixed_size(framebuffer_size.map(|x| x.into()) * 0.1)
        .align(vec2(0.5, 1.0));

        geng::ui::stack![visibility, health, score].boxed()
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
