use super::*;

pub struct PlayerRender {
    geng: Geng,
    assets: Rc<Assets>,
    lights: LightsRender,
}

impl PlayerRender {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            lights: LightsRender::new(geng, assets),
        }
    }

    pub fn draw(
        &mut self,
        world: &World,
        cache: &RenderCache,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        let camera = Camera2d {
            center: world.player.collider.pos().map(Coord::as_f32),
            rotation: 0.0, // world.player.collider.rotation.as_f32(),
            fov: 3.0,
        };

        let collider = &world.player.collider;
        let rotation = collider.rotation.as_f32();
        let collider = collider.raw().map(Coord::as_f32);
        let center = collider.center();
        let matrix = mat3::translate(center) * mat3::rotate(rotation) * mat3::translate(-center);
        let vertices = ugli::VertexBuffer::new_dynamic(
            self.geng.ugli(),
            collider
                .corners()
                .map(|v| draw2d::Vertex { a_pos: v })
                .into_iter()
                .collect(),
        );
        let draw_player = |color: Rgba<f32>, framebuffer: &mut ugli::Framebuffer| {
            ugli::draw(
                framebuffer,
                &self.assets.shaders.color,
                ugli::DrawMode::TriangleFan,
                &vertices,
                (
                    ugli::uniforms! {
                        u_model_matrix: matrix,
                        u_color: color,
                    },
                    camera.uniforms(framebuffer.size().map(|x| x as f32)),
                ),
                ugli::DrawParameters {
                    blend_mode: Some(ugli::BlendMode::combined(ugli::ChannelBlendMode {
                        src_factor: ugli::BlendFactor::One,
                        dst_factor: ugli::BlendFactor::One,
                        equation: ugli::BlendEquation::Add,
                    })),
                    ..default()
                },
            );
        };

        let (mut world_framebuffer, _) = self.lights.start_render(Rgba::BLACK, framebuffer);
        draw_player(Rgba::WHITE, &mut world_framebuffer);

        self.lights
            .render_spotlights(world, false, &camera, &cache.light_geometry);
        self.lights.finish(framebuffer);

        // Clear alpha from the framebuffer
        let vertices = ugli::VertexBuffer::new_dynamic(
            self.geng.ugli(),
            [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
                .map(|(x, y)| draw2d::Vertex { a_pos: vec2(x, y) })
                .into_iter()
                .collect(),
        );
        ugli::draw(
            framebuffer,
            &self.assets.shaders.color,
            ugli::DrawMode::TriangleFan,
            &vertices,
            (
                ugli::uniforms! {
                    u_model_matrix: mat3::identity(),
                    u_color: Rgba::TRANSPARENT_BLACK,
                },
                geng::PixelPerfectCamera.uniforms(vec2(1.0, 1.0)),
            ),
            ugli::DrawParameters {
                blend_mode: Some(ugli::BlendMode {
                    rgb: ugli::ChannelBlendMode {
                        src_factor: ugli::BlendFactor::Zero,
                        dst_factor: ugli::BlendFactor::One, // Keep old rgb
                        equation: ugli::BlendEquation::Add,
                    },
                    alpha: ugli::ChannelBlendMode {
                        src_factor: ugli::BlendFactor::Zero,
                        dst_factor: ugli::BlendFactor::Zero, // Clear old alpha
                        equation: ugli::BlendEquation::Add,
                    },
                }),
                ..default()
            },
        );

        // Add the player as alpha for calculating its visibilty
        draw_player(Rgba::new(0.0, 0.0, 0.0, 1.0), framebuffer);
    }
}
