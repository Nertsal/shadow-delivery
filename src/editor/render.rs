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
        self.world.obstacles = self.world.level.obstacles.clone();

        // Lighting
        let (mut world_framebuffer, mut normal_framebuffer) =
            self.render.lights.start_render(Rgba::BLACK, framebuffer);
        // World
        self.render
            .world
            .draw(&self.world, &mut world_framebuffer, &mut normal_framebuffer);
        // Lights
        self.render
            .lights
            .render_normal_map(&self.world.camera, &self.render_cache.normal_geometry);
        let geometry = ugli::VertexBuffer::new_dynamic(
            self.geng.ugli(),
            self.render_cache.light_geometry.clone(),
        );
        self.render
            .lights
            .render_lights(&self.world, &self.world.camera, &geometry);
        // Finish
        self.render.lights.finish(framebuffer);

        if self.draw_hitboxes {
            self.render.world.draw_paths(&self.world, framebuffer);
            self.render.world.draw_hitboxes(&self.world, framebuffer);
        }

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
            EditorMode::Lamp => {}
            EditorMode::Prop(prop) => {
                if let Some(prop) = self.props.get(prop) {
                    let framebuffer_size = framebuffer.size().map(|x| x as f32);
                    self.geng.draw2d().draw2d(
                        framebuffer,
                        &geng::PixelPerfectCamera,
                        &draw2d::Text::unit(
                            &**self.geng.default_font(),
                            format!("Prop: {prop}"),
                            Rgba::WHITE,
                        )
                        .scale_uniform(20.0)
                        .align_bounding_box(vec2(0.0, 1.0))
                        .translate(vec2(0.05, 0.85) * framebuffer_size),
                    );
                }
            }
        }

        if let Some(drag) = &self.drag {
            match drag.target {
                DragTarget::NewObstacle => {
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
                DragTarget::NewProp(prop) => {
                    let prop = self.props.get(prop).unwrap();
                    let texture = self.assets.sprites.props.get(prop).unwrap();
                    let aabb = Aabb2::from_corners(drag.from, self.cursor_pos).map(Coord::as_f32);
                    self.geng.draw2d().draw2d(
                        framebuffer,
                        &self.world.camera,
                        &draw2d::TexturedQuad::new(aabb, texture.texture()),
                    );
                }
                _ => (),
            }
        }
    }
}
