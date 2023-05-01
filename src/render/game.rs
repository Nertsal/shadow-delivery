use super::*;

const PLAYER_BACKGROUND_COLOR: Rgba<f32> = Rgba {
    r: 0.05,
    g: 0.05,
    b: 0.05,
    a: 0.9,
};
const PLAYER_RESOLUTION: usize = 50;
const VISIBILTY_THRESHOLD: f32 = 0.1;

pub struct GameRender {
    geng: Geng,
    #[allow(dead_code)]
    assets: Rc<Assets>,
    world: WorldRender,
    lights: LightsRender,
    player: PlayerRender,
    player_texture: ugli::Texture,
}

impl GameRender {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            world: WorldRender::new(geng, assets),
            lights: LightsRender::new(geng, assets),
            player: PlayerRender::new(geng, assets),
            player_texture: {
                let mut texture = ugli::Texture::new_with(
                    geng.ugli(),
                    vec2(PLAYER_RESOLUTION, PLAYER_RESOLUTION),
                    |_| Rgba::BLACK,
                );
                texture.set_filter(ugli::Filter::Nearest);
                texture
            },
        }
    }

    /// Returns player visibilty.
    pub fn draw(
        &mut self,
        world: &World,
        draw_hitboxes: bool,
        cache: &RenderCache,
        framebuffer: &mut ugli::Framebuffer,
    ) -> f32 {
        let framebuffer_size = framebuffer.size().map(|x| x as f32);
        let unit_geometry = ugli::VertexBuffer::new_dynamic(self.geng.ugli(), unit_quad());

        // Lighting
        {
            let (mut world_framebuffer, mut normal_framebuffer) =
                self.lights.start_render(Rgba::BLACK, framebuffer);
            // World
            self.world
                .draw(world, &mut world_framebuffer, &mut normal_framebuffer);
            // Lights
            // self.lights.render_normal_map(&world.camera, &cache.normal_geometry);
            let geometry = cache
                .light_geometry
                .as_slice()
                .iter()
                .copied()
                .chain(world.calculate_dynamic_light_geometry())
                .collect();
            let geometry = ugli::VertexBuffer::new_dynamic(self.geng.ugli(), geometry);
            self.lights.render_lights(world, &world.camera, &geometry);
            let player_light = Spotlight {
                color: Rgba::opaque(0.8, 0.8, 1.0),
                position: world.player.collider.pos(),
                angle_range: f32::PI * 2.0,
                max_distance: Coord::new(3.0),
                volume: 0.2,
                intensity: 0.1,
                ..default()
            };
            self.lights
                .render_spotlight(&player_light, true, &world.camera, &geometry);
            // Finish
            self.lights.finish(framebuffer);
        }

        // Waypoint arrow
        {
            let collider = world.player.collider.raw().map(Coord::as_f32);
            let size = collider.size();
            let radius = size.x.max(size.y) * 0.5 * 3.0;
            let aabb = Aabb2::point(collider.center()).extend_uniform(radius);

            let target = world
                .level
                .waypoints
                .collider
                .get(world.active_waypoint)
                .unwrap()
                .pos()
                .map(Coord::as_f32);
            let rotation = (target - collider.center()).arg();

            let matrix = mat3::translate(aabb.center())
                * mat3::scale(aabb.size() / 2.0)
                * mat3::rotate(rotation);
            ugli::draw(
                framebuffer,
                &self.assets.shaders.texture,
                ugli::DrawMode::TriangleFan,
                &unit_geometry,
                (
                    ugli::uniforms! {
                        u_model_matrix: matrix,
                        u_texture: &self.assets.sprites.arrow,
                        u_color: Rgba::new(1.0, 1.0, 1.0, 0.5),
                    },
                    world.camera.uniforms(framebuffer_size),
                ),
                ugli::DrawParameters {
                    blend_mode: Some(ugli::BlendMode::straight_alpha()),
                    ..default()
                },
            );
        }

        // Hitboxes
        if draw_hitboxes {
            self.world.draw_paths(world, framebuffer);
            self.world.draw_hitboxes(world, framebuffer);
        } else {
            #[derive(StructQuery)]
            struct WaypointRef<'a> {
                collider: &'a Collider,
            }
            let query = query_waypoint_ref!(world.level.waypoints);
            if let Some(item) = query.get(world.active_waypoint) {
                let size = item.collider.size();
                let radius = size.x.max(size.y).as_f32();
                self.geng.draw2d().draw2d(
                    framebuffer,
                    &world.camera,
                    &draw2d::Ellipse::circle_with_cut(
                        item.collider.pos().map(Coord::as_f32),
                        radius,
                        radius * 1.1,
                        Rgba::new(0.0, 0.7, 0.7, 0.2),
                    ),
                );
            }
        }

        let visibility = {
            // Player overlay
            let mut player_framebuffer = ugli::Framebuffer::new_color(
                self.geng.ugli(),
                ugli::ColorAttachment::Texture(&mut self.player_texture),
            );
            ugli::clear(
                &mut player_framebuffer,
                Some(Rgba::TRANSPARENT_BLACK),
                None,
                None,
            );
            self.player.draw(world, cache, &mut player_framebuffer);

            // Count the visibility
            let read = ugli::FramebufferRead::new_color(
                self.geng.ugli(),
                ugli::ColorAttachmentRead::Texture(&self.player_texture),
            );
            let data = read.read_color();
            let mut total = 0.0;
            let mut total_alpha = 0.0;
            for x in 0..read.size().x {
                for y in 0..read.size().y {
                    let color = data.get(x, y);
                    let value = color.r.max(color.g).max(color.b) as f32 / 255.0;
                    total += if value >= VISIBILTY_THRESHOLD {
                        1.0
                    } else {
                        value
                    };
                    total_alpha += color.a as f32 / 255.0;
                }
            }
            let visibility = total / total_alpha;

            // Render

            // World
            {
                let pos = world.player.collider.pos().map(Coord::as_f32);
                let scale = 3.0;
                let matrix = mat3::translate(pos) * mat3::scale_uniform(scale / 2.0);
                ugli::draw(
                    framebuffer,
                    &self.assets.shaders.visibility,
                    ugli::DrawMode::TriangleFan,
                    &unit_geometry,
                    (
                        ugli::uniforms! {
                            u_model_matrix: matrix,
                            u_texture: &self.player_texture,
                            u_alpha: 0.9,
                        },
                        world.camera.uniforms(framebuffer_size),
                    ),
                    ugli::DrawParameters {
                        blend_mode: Some(ugli::BlendMode::straight_alpha()),
                        ..default()
                    },
                );
            }

            // UI
            let size = 0.2 * framebuffer_size.y;
            let aabb =
                Aabb2::point(vec2(0.1, 0.1) * framebuffer_size).extend_positive(vec2(size, size));
            self.geng.draw2d().draw2d(
                framebuffer,
                &geng::PixelPerfectCamera,
                &draw2d::Ellipse::circle(aabb.center(), size / 2.0, PLAYER_BACKGROUND_COLOR),
            );
            let matrix = mat3::translate(aabb.center()) * mat3::scale(aabb.size() / 2.0);
            ugli::draw(
                framebuffer,
                &self.assets.shaders.visibility,
                ugli::DrawMode::TriangleFan,
                &unit_geometry,
                (
                    ugli::uniforms! {
                        u_model_matrix: matrix,
                        u_texture: &self.player_texture,
                        u_alpha: 1.0,
                    },
                    geng::PixelPerfectCamera.uniforms(framebuffer_size),
                ),
                ugli::DrawParameters {
                    blend_mode: Some(ugli::BlendMode::straight_alpha()),
                    ..default()
                },
            );

            visibility
        };

        // Health
        {
            let health = world.player.health.as_f32() / 100.0;
            let time = world.time.as_f32();
            ugli::draw(
                framebuffer,
                &self.assets.shaders.health,
                ugli::DrawMode::TriangleFan,
                &unit_geometry,
                ugli::uniforms! {
                    u_health: health,
                    u_time: time,
                },
                ugli::DrawParameters {
                    blend_mode: Some(ugli::BlendMode::straight_alpha()),
                    ..default()
                },
            );
        }

        visibility
    }
}
