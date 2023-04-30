use super::*;

#[derive(Clone, Serialize, Deserialize)]
#[serde(into = "LevelSerde", from = "LevelSerde")]
pub struct Level {
    pub spawn_point: vec2<Coord>,
    pub global_light: GlobalLight,
    pub waypoints: StructOf<Vec<Waypoint>>,
    pub obstacles: StructOf<Vec<Obstacle>>,
}

#[derive(StructOf, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Obstacle {
    pub collider: Collider,
    /// In relative coordinates.
    pub lights: Vec<Spotlight>,
    pub path: Option<Path>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Path {
    pub points: Vec<vec2<Coord>>,
    pub next_point: usize,
}

#[derive(StructOf, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Waypoint {
    pub collider: Collider,
}

#[derive(Serialize, Deserialize)]
struct LevelSerde {
    pub spawn_point: vec2<Coord>,
    #[serde(default)]
    pub global_light: GlobalLight,
    #[serde(default)]
    pub waypoints: Vec<Waypoint>,
    #[serde(default)]
    pub obstacles: Vec<Obstacle>,
}

impl From<Level> for LevelSerde {
    fn from(level: Level) -> Self {
        Self {
            spawn_point: level.spawn_point,
            global_light: level.global_light,
            waypoints: level
                .waypoints
                .inner
                .into_iter()
                .map(|(_, item)| item)
                .collect(),
            obstacles: level
                .obstacles
                .inner
                .into_iter()
                .map(|(_, item)| item)
                .collect(),
        }
    }
}

impl From<LevelSerde> for Level {
    fn from(level: LevelSerde) -> Self {
        let mut waypoints = StructOf::<Vec<Waypoint>>::new();
        for item in level.waypoints {
            waypoints.insert(item);
        }

        let mut obstacles = StructOf::<Vec<Obstacle>>::new();
        for item in level.obstacles {
            obstacles.insert(item);
        }

        Self {
            spawn_point: level.spawn_point,
            global_light: level.global_light,
            waypoints,
            obstacles,
        }
    }
}