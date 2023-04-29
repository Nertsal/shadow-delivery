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
        }
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        self.render
            .draw(&self.world, &self.render_cache, framebuffer);
    }
}
