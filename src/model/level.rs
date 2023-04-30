use super::*;

#[derive(Clone, Serialize, Deserialize)]
#[serde(into = "LevelSerde", from = "LevelSerde")]
pub struct Level {
    pub waypoints: StructOf<Vec<Waypoint>>,
    pub obstacles: StructOf<Vec<Obstacle>>,
    pub global_light: GlobalLight,
}

#[derive(StructOf, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Obstacle {
    pub collider: Collider,
    /// In relative coordinates.
    pub lights: Vec<Spotlight>,
}

#[derive(StructOf, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Waypoint {
    pub collider: Collider,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct LevelSerde {
    pub waypoints: Vec<Waypoint>,
    pub obstacles: Vec<Obstacle>,
    pub global_light: GlobalLight,
}

impl From<Level> for LevelSerde {
    fn from(level: Level) -> Self {
        Self {
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
            global_light: level.global_light,
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
            waypoints,
            obstacles,
            global_light: level.global_light,
        }
    }
}
