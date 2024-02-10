/**
 * Represents a keypress.
 * Bits 1 2 3 4 5 6 7 8
 * Bit 1: Left
 * Bit 2: Right
 * Bit 3: Soft Drop
 * Bit 4: Hard Drop
 * Bit 5: Counter Clockwise
 * Bit 6: Clockwise
 * Bit 7: Hold
 * Bit 8: 180 Rotation
 */
struct Action(u8);
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
