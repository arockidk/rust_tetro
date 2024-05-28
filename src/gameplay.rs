use wasm_bindgen::prelude::wasm_bindgen;

/**
 * Represents a keypress.
 * Bits 8 7 6 5 4 3 2 1
 * Bit 8: Left
 * Bit 7: Right
 * Bit 6: Soft Drop
 * Bit 5: Hard Drop
 * Bit 4: Counter Clockwise
 * Bit 3: Clockwise
 * Bit 2: Hold
 * Bit 1: 180 Rotation
 */
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[wasm_bindgen]
pub struct Action(pub u8);
#[wasm_bindgen]
impl Action {
    pub fn get_left(self) -> bool {
        return (self.0 & 0b10000000) != 0;
    }
    pub fn get_right(self) -> bool {
        return (self.0 & 0b01000000) != 0;
    }
    pub fn get_soft_drop(self) -> bool {
        return (self.0 & 0b00100000) != 0;
    }
    pub fn get_hard_drop(self) -> bool {
        return (self.0 & 0b00010000) != 0;
    }
    pub fn get_counter_clockwise(self) -> bool {
        return (self.0 & 0b00001000) != 0;
    }
    pub fn get_clockwise(self) -> bool {
        return (self.0 & 0b00000100) != 0;
    }
    pub fn get_hold(self) -> bool {
        return (self.0 & 0b00000010) != 0;
    }
    pub fn get_180_rotation(self) -> bool {
        return (self.0 & 0b00000001) != 0;
    }
    pub fn set_left(mut self, value: bool) {
        if value {
            self.0 |= 0b10000000;
        } else {
            self.0 &= 0b01111111;
        }
    }
    pub fn set_right(mut self, value: bool) {
        if value {
            self.0 |= 0b01000000;
        } else {
            self.0 &= 0b10111111;
        }
    }
    pub fn set_soft_drop(mut self, value: bool) {
        if value {
            self.0 |= 0b00100000;
        } else {
            self.0 &= 0b11011111;
        }
    }
    pub fn set_hard_drop(mut self, value: bool) {
        if value {
            self.0 |= 0b00010000;
        } else {
            self.0 &= 0b11101111;
        }
    }
    pub fn set_counter_clockwise(mut self, value: bool) {
        if value {
            self.0 |= 0b00001000;
        } else {
            self.0 &= 0b11110111;
        }
    }
    pub fn set_clockwise(mut self, value: bool) {
        if value {
            self.0 |= 0b00000100;
        } else {
            self.0 &= 0b11111011;
        }
    }
    pub fn set_hold(mut self, value: bool) {
        if value {
            self.0 |= 0b00000010;
        } else {
            self.0 &= 0b11111101;
        }
    }
    pub fn set_180_rotation(mut self, value: bool) {
        if value {
            self.0 |= 0b00000001;
        } else {
            self.0 &= 0b11111110;
        }
    }
}
