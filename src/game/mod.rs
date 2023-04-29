use crate::{model::World, render::GameRender};

use super::*;

pub struct Game {
    geng: Geng,
    assets: Rc<Assets>,
    render: GameRender,
    world: World,
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            render: GameRender::new(geng, assets),
            world: World::new(),
        }
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        self.render.draw(&self.world, framebuffer);
    }
}
