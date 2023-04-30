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

    pub fn draw_paths(&mut self, world: &World, framebuffer: &mut ugli::Framebuffer) {
        #[derive(StructQuery)]
        struct PathRef<'a> {
            #[query(component = "Option<Path>")]
            path: &'a Path,
        }
        for item in query_path_ref!(world.level.obstacles).values() {
            let mut points = item
                .path
                .points
                .iter()
                .map(|(_, point)| point.map(Coord::as_f32))
                .collect::<Vec<_>>();
            if let Some(&point) = points.first() {
                points.push(point);
            }
            let chain = Chain::new(points);
            self.geng.draw2d().draw2d(
                framebuffer,
                &world.camera,
                &draw2d::Chain::new(chain, 0.1, Rgba::new(0.4, 0.4, 0.4, 0.5), 2),
            );
        }
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
                    .get(world.active_waypoint)
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
