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
        #[derive(StructQuery)]
        struct ObstacleRef<'a> {
            collider: &'a Collider,
        }
        for obstacle in query_obstacle_ref!(world.obstacles).values() {
            self.draw_collider(
                obstacle.collider,
                Rgba::new(0.3, 0.3, 0.3, 0.5),
                &world.camera,
                framebuffer,
            );
        }

        self.draw_collider(
            &world.player.collider,
            Rgba::new(0.0, 1.0, 0.0, 0.5),
            &world.camera,
            framebuffer,
        );
    }

    pub fn draw_collider(
        &self,
        collider: &Collider,
        color: Rgba<f32>,
        camera: &Camera2d,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        let rotation = collider.rotation.as_f32();
        let collider = collider.raw().map(Coord::as_f32);
        let center = collider.center();
        self.geng.draw2d().draw2d(
            framebuffer,
            camera,
            &draw2d::Quad::new(collider, color)
                .translate(-center)
                .rotate(rotation)
                .translate(center),
        );
    }
}
