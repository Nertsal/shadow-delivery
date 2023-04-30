#![allow(dead_code)]

use geng::prelude::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Angle(f32);

impl Angle {
    pub const ZERO: Self = Self(0.0);

    pub fn new_radians(radians: f32) -> Self {
        Self(normalize_radians(radians))
    }

    pub fn new_degrees(degrees: f32) -> Self {
        Self::new_radians(degrees_to_radians(degrees))
    }

    pub fn as_radians(self) -> f32 {
        self.0
    }

    pub fn as_degrees(self) -> f32 {
        radians_to_degrees(self.0)
    }

    pub fn unit_direction(self) -> vec2<f32> {
        vec2::UNIT_X.rotate(self.as_radians())
    }
}

/// Normalizes the angle in radians to the range -π..π.
pub fn normalize_radians(radians: f32) -> f32 {
    let tau = 2.0 * f32::PI;
    let mut angle = (radians / tau).fract();
    if angle > 0.5 {
        angle -= 1.0;
    }
    angle * tau
}

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees / 180.0 * f32::PI
}

pub fn radians_to_degrees(radians: f32) -> f32 {
    radians / f32::PI * 180.0
}

impl Neg for Angle {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Add<Self> for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new_radians(self.0 + rhs.0)
    }
}

impl AddAssign<Self> for Angle {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::new_radians(self.0 + rhs.0)
    }
}

impl Sub<Self> for Angle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new_radians(self.0 - rhs.0)
    }
}
