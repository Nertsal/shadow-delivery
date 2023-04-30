use crate::render::{draw_collider, LightsRender, WorldRender};

use super::*;

pub struct EditorRender {
    world: WorldRender,
    lights: LightsRender,
}

impl EditorRender {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            world: WorldRender::new(geng, assets),
            lights: LightsRender::new(geng, assets),
        }
    }
}

impl Editor {
    pub fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        // Lighting
        let (mut world_framebuffer, mut normal_framebuffer) =
            self.render.lights.start_render(Rgba::BLACK, framebuffer);
        // World
        self.render
            .world
            .draw(&self.world, &mut world_framebuffer, &mut normal_framebuffer);
        // Lights
        // self.render.lights.render_normal_map(&self.world.camera, &self.cache.normal_geometry);
        self.render.lights.render_lights(
            &self.world,
            &self.world.camera,
            &self.render_cache.light_geometry,
        );
        // Finish
        self.render.lights.finish(framebuffer);

        self.render.world.draw_hitboxes(&self.world, framebuffer);

        match self.mode {
            EditorMode::Spawn => {
                let mut collider = self.world.player.collider;
                collider.teleport(self.world.level.spawn_point);
                draw_collider(
                    &collider,
                    Rgba::new(0.0, 1.0, 0.0, 0.5),
                    &self.geng,
                    framebuffer,
                    &self.world.camera,
                );
            }
            EditorMode::Waypoint => {
                #[derive(StructQuery)]
                struct WaypointRef<'a> {
                    collider: &'a Collider,
                }
                for waypoint in query_waypoint_ref!(self.world.level.waypoints).values() {
                    draw_collider(
                        waypoint.collider,
                        Rgba::new(0.0, 1.0, 1.0, 0.5),
                        &self.geng,
                        framebuffer,
                        &self.world.camera,
                    );
                }
            }
            EditorMode::Obstacle => {}
        }

        if let Some(drag) = &self.drag {
            if let DragTarget::NewObstacle = drag.target {
                let aabb = Aabb2::from_corners(drag.from, self.cursor_pos);
                let collider = Collider::new(aabb);
                draw_collider(
                    &collider,
                    Rgba::new(0.4, 0.4, 0.4, 0.5),
                    &self.geng,
                    framebuffer,
                    &self.world.camera,
                );
            }
        }
    }
}
