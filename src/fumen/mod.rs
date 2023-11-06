use crate::field::Field;
fn b64_encode(v: i64) {
    static CHARSET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let cur: i64;
    let buff: Vec<u8> = Vec::new();
}
pub fn encode_fumen(field: &Field) {
    let mut base: String = String::from("v115@");
    let mut count: i64 = 0;
    let mut previous_tile: i8 = 0;
    for tile in field.get_tile_array() {
        if previous_tile == tile {
            count += 1;
        } else {
            let diff: i64 = (previous_tile + 8) as i64;
            let poll2 = diff * 240 + count;
          
            let 
            previous_tile = tile;
        }

    }
    
}