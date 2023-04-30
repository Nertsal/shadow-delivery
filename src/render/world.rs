use super::*;

pub struct WorldRender {
    geng: Geng,
    #[allow(dead_code)]
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
        _world: &World,
        _framebuffer: &mut ugli::Framebuffer,
        _normal_framebuffer: &mut ugli::Framebuffer,
    ) {
    }

    pub fn draw_hitboxes(&mut self, world: &World, framebuffer: &mut ugli::Framebuffer) {
        #[derive(StructQuery)]
        struct ObstacleRef<'a> {
            collider: &'a Collider,
        }
        for obstacle in query_obstacle_ref!(world.level.obstacles).values() {
            draw_collider(
                obstacle.collider,
                Rgba::new(0.3, 0.3, 0.3, 0.5),
                &self.geng,
                framebuffer,
                &world.camera,
            );
        }

        draw_collider(
            &world.player.collider,
            Rgba::new(0.0, 1.0, 0.0, 0.5),
            &self.geng,
            framebuffer,
            &world.camera,
        );
    }
}
