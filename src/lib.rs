#![allow(dead_code)]

mod field;
mod piece;
mod fumen;
mod queue;
mod math;
mod kicks;
mod board;
mod vec2;
#[cfg(test)]
mod tests {
    use crate::field;
    use crate::fumen;
    use crate::piece::get_pieces;
    use crate::queue::Queue;
    use crate::queue::choose;
    

  

    #[test]
    fn fumen_test() {
        let demo = field::Field::new();
        // fumen::encode_fumen(&demo);
        assert_eq!(fumen::encode_fumen(&demo), "v115@vh");
        let mut two_grey = field::Field::new();
        two_grey.set_tile(0, 0, 8);
        two_grey.set_tile(1, 0, 8);
        assert_eq!(fumen::encode_fumen(&two_grey),"v115@B8th");
        let grey_6p_pco = field::Field::from_4h_array([
            8,8,0,0,0,0,0,8,8,8,
            8,8,8,0,0,0,0,8,8,8,
            8,8,8,8,0,0,0,8,8,8,
            8,8,8,0,0,0,0,8,8,8
        ]);
        assert_eq!(fumen::encode_fumen(&grey_6p_pco),"v115@9gB8EeF8DeG8CeF8DeC8Je")
    }
    #[test]
    fn queue_test () {
        let allp4 = choose(Vec::from(get_pieces()), 4);
        for queue in allp4 {
            println!("{}", queue)
        }
    }
}
