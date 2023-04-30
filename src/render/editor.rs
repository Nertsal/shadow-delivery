use crate::editor::EditorMode;

use super::*;

pub struct EditorRender {
    geng: Geng,
    #[allow(dead_code)]
    assets: Rc<Assets>,
    world: WorldRender,
    lights: LightsRender,
    player: PlayerRender,
}

impl EditorRender {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            world: WorldRender::new(geng, assets),
            lights: LightsRender::new(geng, assets),
            player: PlayerRender::new(geng, assets),
        }
    }

    pub fn draw(
        &mut self,
        world: &World,
        mode: &EditorMode,
        cache: &RenderCache,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        // Lighting
        let (mut world_framebuffer, mut normal_framebuffer) =
            self.lights.start_render(Rgba::BLACK, framebuffer);
        // World
        self.world
            .draw(world, &mut world_framebuffer, &mut normal_framebuffer);
        // Lights
        // self.lights.render_normal_map(&world.camera, &cache.normal_geometry);
        self.lights
            .render_lights(world, &world.camera, &cache.light_geometry);
        // Finish
        self.lights.finish(framebuffer);

        self.world.draw_hitboxes(world, framebuffer);

        match mode {
            EditorMode::Spawn => {
                let mut collider = world.player.collider;
                collider.teleport(world.level.spawn_point);
                draw_collider(
                    &collider,
                    Rgba::new(0.0, 1.0, 0.0, 0.5),
                    &self.geng,
                    framebuffer,
                    &world.camera,
                );
            }
            EditorMode::Waypoint => {
                #[derive(StructQuery)]
                struct WaypointRef<'a> {
                    collider: &'a Collider,
                }
                for waypoint in query_waypoint_ref!(world.level.waypoints).values() {
                    draw_collider(
                        waypoint.collider,
                        Rgba::new(0.0, 1.0, 1.0, 0.5),
                        &self.geng,
                        framebuffer,
                        &world.camera,
                    );
                }
            }
            EditorMode::Obstacle => {}
        }
    }
}
