use super::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Collider(Aabb2<Coord>);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Collision {
    pub normal: vec2<Coord>,
    pub penetration: Coord,
    pub offset_dir: vec2<Coord>,
    pub offset: Coord,
}

impl Collider {
    pub fn new(aabb: Aabb2<Coord>) -> Self {
        Self(aabb)
    }

    pub fn raw(&self) -> Aabb2<Coord> {
        self.0
    }

    pub fn pos(&self) -> vec2<Coord> {
        self.0.center()
    }

    pub fn size(&self) -> vec2<Coord> {
        self.0.size()
    }

    pub fn teleport(&mut self, position: vec2<Coord>) {
        let delta = position - self.pos();
        self.translate(delta);
    }

    pub fn translate(&mut self, delta: vec2<Coord>) {
        self.0 = self.0.translate(delta);
    }

    pub fn check(&self, other: &Self) -> bool {
        self.0.intersects(&other.0)
    }

    pub fn collide(&self, other: &Self) -> Option<Collision> {
        if !self.check(other) {
            return None;
        }

        let dx_right = self.0.max.x - other.0.min.x;
        let dx_left = other.0.max.x - self.0.min.x;
        let dy_up = self.0.max.y - other.0.min.y;
        let dy_down = other.0.max.y - self.0.min.y;

        let (nx, px) = if dx_right < dx_left {
            (-Coord::ONE, dx_right)
        } else {
            (Coord::ONE, dx_left)
        };
        let (ny, py) = if dy_up < dy_down {
            (-Coord::ONE, dy_up)
        } else {
            (Coord::ONE, dy_down)
        };

        if px <= Coord::ZERO || py <= Coord::ZERO {
            None
        } else if px < py {
            Some(Collision {
                normal: vec2(nx, Coord::ZERO),
                penetration: px,
                offset_dir: vec2(Coord::ZERO, ny),
                offset: py,
            })
        } else {
            Some(Collision {
                normal: vec2(Coord::ZERO, ny),
                penetration: py,
                offset_dir: vec2(nx, Coord::ZERO),
                offset: px,
            })
        }
    }
}

impl Collision {
    pub fn rotate(self) -> Self {
        Self {
            normal: self.offset_dir,
            penetration: self.offset,
            offset_dir: self.normal,
            offset: self.penetration,
        }
    }
}
