use super::*;

mod lights;

pub use lights::*;

pub type Coord = R32;

pub struct World {
    pub camera: Camera2d,
    pub global_light: GlobalLight,
    pub spotlights: Vec<Spotlight>,
}

impl World {
    pub fn new() -> Self {
        Self {
            camera: Camera2d {
                center: vec2::ZERO,
                rotation: 0.0,
                fov: 30.0,
            },
            global_light: GlobalLight::default(),
            spotlights: vec![Spotlight::default()],
        }
    }

    pub fn calculate_light_geometry(
        &self,
        geng: &Geng,
        assets: &Assets,
    ) -> ugli::VertexBuffer<render::NormalVertex> {
        let mut vertices = vec![];
        ugli::VertexBuffer::new_dynamic(geng.ugli(), vertices)
    }

    // pub fn calculate_normal_geometry(
    //     &self,
    //     geng: &Geng,
    //     assets: &Assets,
    // ) -> ugli::VertexBuffer<render::NormalVertex> {
    //     todo!()
    // }
}
