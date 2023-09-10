use std::ops::{Add, AddAssign, Sub, SubAssign};

use nalgebra::Vector2;

pub type Vector = Vector2<f32>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    vec: Vector2<f32>,
}

impl Location {
    pub const ORIGIN: Self = Self::new(0., 0.);

    pub const fn new(x: f32, y: f32) -> Self {
        Self {
            vec: Vector2::new(x, y),
        }
    }

    pub fn x(&self) -> f32 {
        self.vec.x
    }

    pub fn y(&self) -> f32 {
        self.vec.y
    }

    pub fn clamp(&self, min: Location, max: Location) -> Self {
        debug_assert!(min.x() < max.x(), "min.x: {}, max.x: {}", min.x(), max.x());
        debug_assert!(min.y() < max.y(), "min.y: {}, max.y: {}", min.y(), max.y());

        Self::new(
            self.x().clamp(min.x(), max.x()),
            self.y().clamp(min.y(), max.y()),
        )
    }
    /*
    /// Returns the reflected location and the normal of the surface it hit if it hit a surface.
    pub fn move_reflect(&self, delta: Vector, min: Location, max: Location) -> (Self, Option<Vector>) {
        debug_assert!(min.x() < max.x(), "min.x: {}, max.x: {}", min.x(), max.x());
        debug_assert!(min.y() < max.y(), "min.y: {}, max.y: {}", min.y(), max.y());

        let diff = max - min;
        let loc = *self - min;

        let (new_loc, normal) = if loc.x() < 0. {
            if loc.y() < 0. {

            }
        }

        let x = diff.x - loc.x;
        let x = if x < 0. { x + diff.x } else { x };

        let y = diff.y - loc.y;
        let y = if y < 0. { y + diff.y } else { y };

        min + (Self::new(x, y) - Location::ORIGIN)
    }*/

    pub fn wrap(&self, min: Location, max: Location) -> Self {
        debug_assert!(min.x() < max.x(), "min.x: {}, max.x: {}", min.x(), max.x());
        debug_assert!(min.y() < max.y(), "min.y: {}, max.y: {}", min.y(), max.y());

        let diff = max - min;
        let loc = *self - min;

        let x = (loc.x) % (diff.x);
        let x = if x < 0. { x + diff.x } else { x };

        let y = (loc.y) % (diff.y);
        let y = if y < 0. { y + diff.y } else { y };

        min + (Self::new(x, y) - Location::ORIGIN)
    }
}

impl Default for Location {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl Add<Vector2<f32>> for Location {
    type Output = Self;

    fn add(self, rhs: Vector2<f32>) -> Self::Output {
        Location {
            vec: self.vec + rhs,
        }
    }
}

impl AddAssign<Vector2<f32>> for Location {
    fn add_assign(&mut self, rhs: Vector2<f32>) {
        *self = *self + rhs;
    }
}

impl Sub<Vector2<f32>> for Location {
    type Output = Self;

    fn sub(self, rhs: Vector2<f32>) -> Self::Output {
        Location {
            vec: self.vec - rhs,
        }
    }
}

impl SubAssign<Vector2<f32>> for Location {
    fn sub_assign(&mut self, rhs: Vector2<f32>) {
        *self = *self - rhs;
    }
}

impl Sub<Location> for Location {
    type Output = Vector2<f32>;

    fn sub(self, rhs: Location) -> Self::Output {
        self.vec - rhs.vec
    }
}

impl From<Vector2<f32>> for Location {
    fn from(vec: Vector2<f32>) -> Self {
        Self { vec }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn wrap_0_to_max() {
        let min = Location::new(0., 0.);
        let max = Location::new(10., 10.);

        let loc = Location::new(0., 0.);
        let expected = Location::new(0., 0.);
        assert_eq!(expected, loc.wrap(min, max));

        let loc = Location::new(1., 2.);
        let expected = Location::new(1., 2.);
        assert_eq!(expected, loc.wrap(min, max));

        let loc = Location::new(13., 34.5);
        let expected = Location::new(3., 4.5);
        assert_eq!(expected, loc.wrap(min, max));

        let loc = Location::new(-3., -4.);
        let expected = Location::new(7., 6.);
        assert_eq!(expected, loc.wrap(min, max));

        let loc = Location::new(-123., -164.);
        let expected = Location::new(7., 6.);
        assert_eq!(expected, loc.wrap(min, max));
    }

    #[test]
    pub fn wrap_positive_to_other() {
        let min = Location::new(5., 7.);
        let max = Location::new(10., 10.);

        let loc = Location::new(0., 0.);
        let expected = Location::new(5., 9.);
        assert_eq!(expected, loc.wrap(min, max));

        let loc = Location::new(1., 2.);
        let expected = Location::new(6., 8.);
        assert_eq!(expected, loc.wrap(min, max));

        let loc = Location::new(13., 34.5);
        let expected = Location::new(8., 7.5);
        assert_eq!(expected, loc.wrap(min, max));

        let loc = Location::new(-3., -4.);
        let expected = Location::new(7., 8.);
        assert_eq!(expected, loc.wrap(min, max));

        let loc = Location::new(-123., -164.);
        let expected = Location::new(7., 7.);
        assert_eq!(expected, loc.wrap(min, max));
    }
}
