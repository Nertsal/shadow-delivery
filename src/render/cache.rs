use super::*;

pub struct RenderCache {
    pub light_geometry: ugli::VertexBuffer<NormalVertex>,
    // pub normal_geometry: ugli::VertexBuffer<NormalVertex>,
}

impl RenderCache {
    pub fn calculate(world: &World, geng: &Geng, assets: &Assets) -> Self {
        // let normal_geometry = world.calculate_normal_geometry(geng, assets);
        Self {
            light_geometry: world.calculate_light_geometry(geng, assets),
            // normal_geometry,
        }
    }
}
