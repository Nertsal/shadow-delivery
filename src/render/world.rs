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
        struct ColliderRef<'a> {
            collider: &'a Collider,
        }

        let obstacles = query_collider_ref!(world.level.obstacles);
        let waypoints = query_collider_ref!(world.level.waypoints);
        let colliders = obstacles
            .values()
            .map(|item| (item, Rgba::new(0.3, 0.3, 0.3, 0.5)))
            .chain(
                waypoints
                    .values()
                    .map(|item| (item, Rgba::new(0.0, 1.0, 1.0, 0.5))),
            )
            .chain(std::iter::once((
                ColliderRef {
                    collider: &world.player.collider,
                },
                Rgba::new(0.0, 1.0, 0.0, 0.5),
            )));

        for (item, color) in colliders {
            draw_collider(item.collider, color, &self.geng, framebuffer, &world.camera);
        }
    }
}
