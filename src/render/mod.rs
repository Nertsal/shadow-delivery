use crate::model::*;

use super::*;

mod cache;
mod lights;
mod util;
mod world;

pub use cache::*;
pub use lights::*;
pub use util::*;
pub use world::*;

#[derive(ugli::Vertex, Debug, Clone, Copy)]
pub struct NormalVertex {
    pub a_pos: vec2<f32>,
    pub a_normal: vec2<f32>,
}

#[derive(ugli::Vertex, Debug, Clone, Copy)]
pub struct Vertex {
    pub a_pos: vec2<f32>,
    pub a_uv: vec2<f32>,
}

#[derive(ugli::Vertex, Debug, Clone, Copy)]
pub struct MaskedVertex {
    pub a_pos: vec2<f32>,
    pub a_uv: vec2<f32>,
    pub a_mask_uv: vec2<f32>,
}

pub struct GameRender {
    geng: Geng,
    assets: Rc<Assets>,
    world: WorldRender,
    lights: LightsRender,
}

impl GameRender {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            world: WorldRender::new(geng, assets),
            lights: LightsRender::new(geng, assets),
        }
    }

    pub fn draw(
        &mut self,
        world: &World,
        draw_hitboxes: bool,
        cache: &RenderCache,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        // Lighting
        let (mut world_framebuffer, mut normal_framebuffer) = self.lights.start_render(framebuffer);
        self.world
            .draw(world, &mut world_framebuffer, &mut normal_framebuffer);
        self.lights
            .finish_render(world, cache, &world.camera, framebuffer);

        // Hitboxes
        if draw_hitboxes {
            self.world.draw_hitboxes(world, framebuffer);
        }
    }
}
