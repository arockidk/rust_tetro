use std::ops::{Add, Mul, Sub};
#[derive(Clone, Copy)]
pub struct Vec2(pub i64, pub i64);
impl Add<Vec2> for Vec2 { 
    type Output = Vec2; 
    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl Mul<i64> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: i64) -> Self::Output {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}
