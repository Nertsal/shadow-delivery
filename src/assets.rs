use super::*;

#[derive(geng::Load)]
pub struct Assets {
    pub shaders: Shaders,
}

#[derive(geng::Load)]
pub struct Shaders {
    pub color: ugli::Program,
    // pub texture: ugli::Program,
    // pub texture_mask: ugli::Program,
    pub global_light: ugli::Program,
    pub spotlight: ugli::Program,
    pub point_light_shadow_map: ugli::Program,
    pub normal_map: ugli::Program,
    // pub normal_texture: ugli::Program,
}
