use super::*;

pub struct WorldRender {
    geng: Geng,
    assets: Rc<Assets>,
}

impl WorldRender {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
        }
    }

    pub fn draw(
        &mut self,
        world: &World,
        framebuffer: &mut ugli::Framebuffer,
        normal_framebuffer: &mut ugli::Framebuffer,
    ) {
        // TODO
    }
}
