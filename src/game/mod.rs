use crate::{
    model::World,
    render::{GameRender, RenderCache},
};

use super::*;

pub struct Game {
    geng: Geng,
    assets: Rc<Assets>,
    render: GameRender,
    render_cache: RenderCache,
    world: World,
    draw_hitboxes: bool,
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        let world = World::new();
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            render: GameRender::new(geng, assets),
            render_cache: RenderCache::calculate(&world, geng, assets),
            world,
            draw_hitboxes: cfg!(debug_assertions),
        }
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
}
