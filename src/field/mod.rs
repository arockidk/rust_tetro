


pub struct Field {
    tiles: [i8; 240]

}
impl Field {
    pub fn new() -> Field {
        
        let new_field = Field { 
           
            tiles: [0; 240]
        };

        return new_field;
    }

    pub fn tile_occupied(self: &Field, x: usize, y: usize) -> bool {
        return self.tiles[y * 10 + x] != 0;
    }
    pub fn get_tile(self: &Field, x: usize, y: usize) -> i8 {
        return self.tiles[y * 10 + x];
    }
    pub fn set_tile(self: &mut Field, x: usize, y: usize, new: i8) {
        self.tiles[y * 10 + x] = new;
    }
    pub fn get_tile_array(self: &Field) -> [i8; 240] {
        return self.tiles;
    }

}