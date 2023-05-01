use super::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(default)]
pub struct Collider {
    aabb: Aabb2<Coord>,
    pub rotation: Angle,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Collision {
    pub point: vec2<Coord>,
    pub normal: vec2<Coord>,
    pub penetration: Coord,
}

#[allow(dead_code)]
impl Collider {
    pub fn new(aabb: Aabb2<Coord>) -> Self {
        Self {
            aabb,
            rotation: Angle::ZERO,
        }
    }

    pub fn vertices(&self) -> [vec2<Coord>; 4] {
        let center = self.aabb.center();
        self.aabb
            .corners()
            .map(|p| (p - center).rotate(Coord::new(self.rotation.as_radians())) + center)
    }

    pub fn shape(&self) -> impl parry2d::shape::Shape {
        let points = self.vertices().map(|p| {
            let vec2(x, y) = p.map(Coord::as_f32);
            parry2d::math::Point::new(x, y)
        });
        parry2d::shape::ConvexPolygon::from_convex_hull(&points).unwrap()
    }

    pub fn raw(&self) -> Aabb2<Coord> {
        self.aabb
    }

    pub fn pos(&self) -> vec2<Coord> {
        self.aabb.center()
    }

    pub fn size(&self) -> vec2<Coord> {
        self.aabb.size()
    }

    pub fn teleport(&mut self, position: vec2<Coord>) {
        let delta = position - self.pos();
        self.translate(delta);
    }

    pub fn translate(&mut self, delta: vec2<Coord>) {
        self.aabb = self.aabb.translate(delta);
    }

    pub fn check(&self, other: &Self) -> bool {
        let iso = parry2d::math::Isometry::default();
        parry2d::query::intersection_test(&iso, &self.shape(), &iso, &other.shape()).unwrap()
    }

    pub fn collide(&self, other: &Self) -> Option<Collision> {
        let iso = parry2d::math::Isometry::default();
        parry2d::query::contact(&iso, &self.shape(), &iso, &other.shape(), 0.0)
            .unwrap()
            .map(|contact| {
                let normal = contact.normal1.into_inner();
                let point = contact.point1;
                Collision {
                    point: vec2(point.x, point.y).map(Coord::new),
                    normal: vec2(normal.x, normal.y).map(Coord::new),
                    penetration: Coord::new(-contact.dist),
                }
            })
    }
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            aabb: Aabb2::ZERO.extend_uniform(Coord::ONE),
            rotation: Angle::ZERO,
        }
    }
}
