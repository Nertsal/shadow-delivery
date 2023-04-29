use super::*;

#[derive(Serialize, Deserialize)]
pub struct Level {
    pub obstacles: Vec<Obstacle>,
}
