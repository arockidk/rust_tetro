use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};
use wasm_bindgen::convert::IntoWasmAbi;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Debug, Eq, serde::Serialize, serde::Deserialize)]
pub struct Vec2(pub i32, pub i32);
#[wasm_bindgen]
impl Vec2 {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i32, y: i32) -> Vec2 {
        Vec2(x, y)
    }
}
impl Vec2 {
    pub fn to_usize(&self) -> (usize, usize) {
        (self.0.try_into().unwrap(), self.1.try_into().unwrap())
    }
}
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
impl Mul<i32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: i32) -> Self::Output {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}
impl AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        *self = Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        *self = Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}


