use super::*;

pub struct LightsRender {
    geng: Geng,
    assets: Rc<Assets>,
    buffers: Buffers,
}

/// All buffers used in the lighting pipeline.
struct Buffers {
    /// Current size of all the buffers.
    framebuffer_size: vec2<usize>,
    /// Constant unit quad geometry.
    quad_geometry: ugli::VertexBuffer<draw2d::Vertex>,
    /// Texture for the render of the world ignoring lighting.
    world_texture: ugli::Texture,
    /// Texture of normal vectors.
    normal_texture: ugli::Texture,
    /// Stencil buffer to count shadow casters.
    shadow_stencil: ugli::Renderbuffer<ugli::DepthStencilValue>,
    /// Final texture available for postprocessing.
    postprocess_texture: ugli::Texture,
}

impl LightsRender {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            buffers: Buffers::new(geng),
        }
    }

    pub fn start_render(
        &mut self,
        background_color: Rgba<f32>,
        framebuffer: &mut ugli::Framebuffer,
    ) -> (ugli::Framebuffer, ugli::Framebuffer) {
        self.buffers.update(framebuffer.size(), &self.geng);

        // Clear the postprocess
        let mut post_framebuffer =
            attach_texture(&mut self.buffers.postprocess_texture, &self.geng);
        ugli::clear(&mut post_framebuffer, Some(background_color), None, Some(0));

        let mut world_framebuffer = attach_texture(&mut self.buffers.world_texture, &self.geng);
        let mut normal_framebuffer = attach_texture(&mut self.buffers.normal_texture, &self.geng);
        ugli::clear(&mut world_framebuffer, Some(background_color), None, None);
        ugli::clear(
            &mut normal_framebuffer,
            Some(Rgba::TRANSPARENT_BLACK),
            None,
            None,
        );

        (world_framebuffer, normal_framebuffer)
    }

    pub fn finish(&mut self, framebuffer: &mut ugli::Framebuffer) {
        // Draw the texture to the screen
        self.geng.draw2d().draw2d(
            framebuffer,
            &geng::PixelPerfectCamera,
            &draw2d::TexturedQuad::new(
                Aabb2::ZERO.extend_positive(framebuffer.size().map(|x| x as f32)),
                &self.buffers.postprocess_texture,
            ),
        );
    }

    /// Renders the world for each light separately onto the postprocessing texture.
    pub fn render_lights(
        &mut self,
        world: &World,
        camera: &Camera2d,
        geometry: &ugli::VertexBuffer<NormalVertex>,
    ) {
        self.render_global_light(world);
        self.render_spotlights(world, true, camera, geometry);
    }

    /// Renders the world for the global light onto the postprocessing texture.
    pub fn render_global_light(&mut self, world: &World) {
        let mut world_framebuffer =
            attach_texture(&mut self.buffers.postprocess_texture, &self.geng);
        let framebuffer_size = world_framebuffer.size();

        ugli::draw(
            &mut world_framebuffer,
            &self.assets.shaders.global_light,
            ugli::DrawMode::TriangleFan,
            &self.buffers.quad_geometry,
            ugli::uniforms! {
                u_framebuffer_size: framebuffer_size,
                u_source_texture: &self.buffers.world_texture,
                u_light_color: world.level.global_light.color,
                u_light_intensity: world.level.global_light.intensity,
            },
            ugli::DrawParameters {
                blend_mode: Some(ugli::BlendMode::combined(ugli::ChannelBlendMode {
                    src_factor: ugli::BlendFactor::One,
                    dst_factor: ugli::BlendFactor::One,
                    equation: ugli::BlendEquation::Add,
                })),
                ..Default::default()
            },
        );
    }

    pub fn render_spotlights(
        &mut self,
        world: &World,
        volumetric: bool,
        camera: &Camera2d,
        geometry: &ugli::VertexBuffer<NormalVertex>,
    ) {
        let spotlights = world.level.obstacles.iter().flat_map(|(_, obstacle)| {
            obstacle
                .lights
                .iter()
                .map(|(_, light)| (light, obstacle.collider.rotation, obstacle.collider.pos()))
        });
        for (spotlight, rotation, offset) in spotlights {
            let light_pos = spotlight
                .position
                .map(Coord::as_f32)
                .rotate(rotation.as_radians())
                + offset.map(Coord::as_f32);
            let light_angle = spotlight.angle + rotation.as_radians();

            // Using `world_texture` here but it is not actually used by the shader
            let mut light_framebuffer = ugli::Framebuffer::new(
                self.geng.ugli(),
                ugli::ColorAttachment::Texture(&mut self.buffers.world_texture),
                ugli::DepthAttachment::RenderbufferWithStencil(&mut self.buffers.shadow_stencil),
            );
            let framebuffer_size = light_framebuffer.size().map(|x| x as f32);
            ugli::clear(&mut light_framebuffer, None, None, Some(0));

            // Cast shadow
            ugli::draw(
                &mut light_framebuffer,
                &self.assets.shaders.point_light_shadow_map,
                ugli::DrawMode::Triangles,
                geometry,
                (
                    ugli::uniforms! {
                        u_model_matrix: mat3::identity(),
                        u_light_pos: light_pos,
                    },
                    camera.uniforms(framebuffer_size),
                ),
                ugli::DrawParameters {
                    // Just in case the shader writes something in the texture,
                    // discard it during blending.
                    blend_mode: Some(ugli::BlendMode::combined(ugli::ChannelBlendMode {
                        src_factor: ugli::BlendFactor::Zero,
                        dst_factor: ugli::BlendFactor::One,
                        equation: ugli::BlendEquation::Add,
                    })),
                    // Increment the shadow casters count
                    stencil_mode: Some(ugli::StencilMode::always(ugli::FaceStencilMode {
                        test: ugli::StencilTest {
                            condition: ugli::Condition::Always,
                            reference: 0,
                            mask: 0xFF,
                        },
                        op: ugli::StencilOp::always(ugli::StencilOpFunc::Increment),
                    })),
                    ..Default::default()
                },
            );

            // Render the world for that light
            let mut world_framebuffer = ugli::Framebuffer::new(
                self.geng.ugli(),
                ugli::ColorAttachment::Texture(&mut self.buffers.postprocess_texture),
                ugli::DepthAttachment::RenderbufferWithStencil(&mut self.buffers.shadow_stencil),
            );
            let framebuffer_size = world_framebuffer.size().map(|x| x as f32);
            ugli::draw(
                &mut world_framebuffer,
                &self.assets.shaders.spotlight,
                ugli::DrawMode::TriangleFan,
                &self.buffers.quad_geometry,
                (
                    ugli::uniforms! {
                        u_model_matrix: mat3::identity(),
                        u_light_pos: light_pos,
                        u_light_angle: light_angle,
                        u_light_angle_range: spotlight.angle_range,
                        u_light_angle_gradient: spotlight.angle_gradient,
                        u_light_color: spotlight.color,
                        u_light_intensity: spotlight.intensity,
                        u_light_max_distance: spotlight.max_distance.as_f32(),
                        u_light_distance_gradient: spotlight.distance_gradient,
                        u_light_volume: if volumetric { spotlight.volume } else { 0.0 },
                        u_normal_texture: &self.buffers.normal_texture,
                        u_source_texture: &self.buffers.world_texture,
                        u_framebuffer_size: self.buffers.normal_texture.size(),
                    },
                    camera.uniforms(framebuffer_size),
                ),
                ugli::DrawParameters {
                    blend_mode: Some(ugli::BlendMode::combined(ugli::ChannelBlendMode {
                        src_factor: ugli::BlendFactor::One,
                        dst_factor: ugli::BlendFactor::One,
                        equation: ugli::BlendEquation::Add,
                    })),
                    // Ignore the parts in shadow
                    stencil_mode: Some(ugli::StencilMode::always(ugli::FaceStencilMode {
                        test: ugli::StencilTest {
                            condition: ugli::Condition::Equal,
                            reference: 0, // 0 shadow casters means the point is lit up.
                            mask: 0xFF,
                        },
                        op: ugli::StencilOp::always(ugli::StencilOpFunc::Keep),
                    })),
                    ..Default::default()
                },
            );
        }
    }

    // pub fn render_normal_map(
    //     &mut self,
    //     camera: &Camera2d,
    //     geometry: &ugli::VertexBuffer<NormalVertex>,
    // ) {
    //     let mut normal_framebuffer = attach_texture(&mut self.buffers.normal_texture, &self.geng);
    //     let framebuffer_size = normal_framebuffer.size().map(|x| x as f32);

    //     ugli::draw(
    //         &mut normal_framebuffer,
    //         &self.assets.shaders.normal_map,
    //         ugli::DrawMode::Triangles,
    //         geometry,
    //         (
    //             ugli::uniforms! {
    //                 u_model_matrix: mat3::identity(),
    //                 u_normal_influence: 1.0,
    //             },
    //             camera.uniforms(framebuffer_size),
    //         ),
    //         ugli::DrawParameters {
    //             blend_mode: Some(ugli::BlendMode::straight_alpha()),
    //             ..Default::default()
    //         },
    //     );
    // }
}

impl Buffers {
    pub fn new(geng: &Geng) -> Self {
        Self {
            framebuffer_size: vec2(1, 1),
            world_texture: new_texture(geng),
            normal_texture: new_texture(geng),
            shadow_stencil: ugli::Renderbuffer::new(geng.ugli(), vec2(1, 1)),
            postprocess_texture: new_texture(geng),
            quad_geometry: ugli::VertexBuffer::new_static(
                geng.ugli(),
                vec![
                    draw2d::Vertex {
                        a_pos: vec2(-1.0, -1.0),
                    },
                    draw2d::Vertex {
                        a_pos: vec2(1.0, -1.0),
                    },
                    draw2d::Vertex {
                        a_pos: vec2(1.0, 1.0),
                    },
                    draw2d::Vertex {
                        a_pos: vec2(-1.0, 1.0),
                    },
                ],
            ),
        }
    }

    pub fn update(&mut self, framebuffer_size: vec2<usize>, geng: &Geng) {
        if self.framebuffer_size != framebuffer_size {
            // Framebuffer size has changed -> update textures
            for texture in [
                &mut self.world_texture,
                &mut self.normal_texture,
                &mut self.postprocess_texture,
            ] {
                update_texture_size(texture, framebuffer_size, geng);
            }
            self.shadow_stencil = ugli::Renderbuffer::new(geng.ugli(), framebuffer_size);

            self.framebuffer_size = framebuffer_size;
        }
    }
}
