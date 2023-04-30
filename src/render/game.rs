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

        // Hitboxes
        if draw_hitboxes {
            self.world.draw_hitboxes(world, framebuffer);
        }

        {
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
            let framebuffer_size = framebuffer.size().map(|x| x as f32);
            let size = 0.2 * framebuffer_size.y;
            let aabb =
                Aabb2::point(vec2(0.1, 0.1) * framebuffer_size).extend_positive(vec2(size, size));
            self.geng.draw2d().draw2d(
                framebuffer,
                &geng::PixelPerfectCamera,
                &draw2d::Ellipse::circle(aabb.center(), size / 2.0, PLAYER_BACKGROUND_COLOR),
            );
            self.geng.draw2d().draw2d(
                framebuffer,
                &geng::PixelPerfectCamera,
                &draw2d::TexturedQuad::new(aabb, &self.player_texture),
            );

            visibility
        }
    }
}
