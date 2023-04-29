use crate::model::*;

use super::*;

pub struct GameRender {
    geng: Geng,
    assets: Rc<Assets>,
}

impl GameRender {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
        }
    }

    pub fn draw(&mut self, world: &World, framebuffer: &mut ugli::Framebuffer) {
        // TODO
    }
}
