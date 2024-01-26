use crate::field::Field;
use wasm_bindgen::prelude::*;
pub struct Page {
    comment: String,
    
}
fn b64_encode(v: i64) -> String {
    static CHARSET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let char_vec: Vec<char> = CHARSET.chars().collect();
    let mut cur: i64 = v;
    let mut str: String = String::new();
    while cur > 0 {
        str.push(char_vec[(cur % 64) as usize]);
        cur = (cur - cur % 64) / 64;
    }
    
    return str;
    
}
#[wasm_bindgen]
pub fn encode_fumen(field: &Field) -> String {
    let mut base: String = String::from("v115@");
    let mut count: i64 = 0;
    let mut previous_tile: i8 = field.get_tile(0,0);
    // println!("{:?}",field.get_tile_array());
    for tile in field.get_tile_array() {
        if previous_tile == tile {
            count += 1;
        } else {
            let diff: i64 = (previous_tile + 8) as i64;
            let poll2 = diff * 240 + count - 1;
            println!("{} {} {}", poll2, diff, count);
            if count == 240 {
                base += "vhA";
            } else {
                let b64str = b64_encode(poll2);
                base += &b64str;
            }
            

            previous_tile = tile;
            count = 1;
        }

    }

    let diff: i64 = (previous_tile + 8) as i64;
    let poll2 = diff * 240 + count - 1;

    println!("{} {} {}", poll2, diff, count);
    let b64str = b64_encode(poll2);
    base += &b64str;

    
    return base;
    
}
