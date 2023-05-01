use super::*;

#[derive(Clone, Serialize, Deserialize)]
#[serde(into = "LevelSerde", from = "LevelSerde")]
pub struct Level {
    pub spawn_point: vec2<Coord>,
    pub global_light: GlobalLight,
    pub waypoints: StructOf<Vec<Waypoint>>,
    pub obstacles: StructOf<Vec<Obstacle>>,
    pub lamps: StructOf<Vec<Lamp>>,
    pub props: StructOf<Vec<Prop>>,
}

pub type PropType = String;

#[derive(StructOf, Serialize, Deserialize, Default)]
pub struct Prop {
    pub collider: Collider,
    pub prop: PropType,
}

#[derive(StructOf, Serialize, Deserialize)]
#[serde(default)]
pub struct Lamp {
    pub collider: Collider,
    /// In relative coordinates.
    pub light: Spotlight,
    pub state: LampState,
    pub up_time: Time,
    pub down_time: Time,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LampState {
    Up(Time),
    Down(Time),
}

impl Default for Lamp {
    fn default() -> Self {
        Self {
            collider: default(),
            light: default(),
            state: default(),
            up_time: Time::ONE,
            down_time: Time::ZERO,
        }
    }
}

impl Default for LampState {
    fn default() -> Self {
        Self::Down(Time::ZERO)
    }
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
    #[serde(default)]
    pub lamps: Vec<Lamp>,
    #[serde(default)]
    pub props: Vec<Prop>,
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
            lamps: level
                .lamps
                .inner
                .into_iter()
                .map(|(_, item)| item)
                .collect(),
            props: level
                .props
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

        let mut lamps = StructOf::<Vec<Lamp>>::new();
        for item in level.lamps {
            lamps.insert(item);
        }

        let mut props = StructOf::<Vec<Prop>>::new();
        for item in level.props {
            props.insert(item);
        }

        Self {
            spawn_point: level.spawn_point,
            global_light: level.global_light,
            waypoints,
            obstacles,
            lamps,
            props,
        }
    }
}
