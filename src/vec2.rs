use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};
use wasm_bindgen::convert::IntoWasmAbi;
use ts_rs::TS;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Debug, Eq, TS, serde::Serialize, serde::Deserialize)]
#[ts(export)]
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


