use crate::field::Field;
pub fn encodeFumen(field: &Field) {
    let mut base: String = String::from("v115@");
    let mut count: i64 = 0;
    let mut previous_tile: i8 = 0;
    for tile in field.get_tile_array() {
        if previous_tile == tile {
            count += 1;
        } else {
            
            previous_tile = tile;
        }

    }
    
}