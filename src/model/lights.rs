use super::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GlobalLight {
    pub color: Rgba<f32>,
    pub intensity: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Spotlight {
    pub position: vec2<Coord>,
    pub angle: f32,
    pub angle_range: f32,
    #[serde(default = "default_gradient")]
    pub angle_gradient: f32,
    pub color: Rgba<f32>,
    pub intensity: f32,
    pub max_distance: Coord,
    #[serde(default = "default_gradient")]
    pub distance_gradient: f32,
    pub volume: f32,
}

fn default_gradient() -> f32 {
    1.0
}

impl Default for GlobalLight {
    fn default() -> Self {
        Self {
            color: Rgba::WHITE,
            intensity: 1.0,
        }
    }
}

impl Default for Spotlight {
    fn default() -> Self {
        Self {
            position: vec2::ZERO,
            angle: 0.0,
            angle_range: 1.0,
            angle_gradient: 1.0,
            color: Rgba::WHITE,
            intensity: 0.5,
            max_distance: Coord::new(5.0),
            distance_gradient: 1.0,
            volume: 0.5,
        }
    }
}
