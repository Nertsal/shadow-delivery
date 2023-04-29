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
    }

    pub fn draw_hitboxes(&mut self, world: &World, framebuffer: &mut ugli::Framebuffer) {
        self.geng.draw2d().draw2d(
            framebuffer,
            &world.camera,
            &draw2d::Quad::new(
                world.player.collider.raw().map(Coord::as_f32),
                Rgba::new(0.0, 1.0, 0.0, 0.5),
            ),
        );
    }
}
