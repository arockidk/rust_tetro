use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use rust_tetro::{board::TetBoard, pc_utils::{bitboard::Placements, bitpiece::BitPiece, BitBoard}, piece::{Direction, PieceColor}};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("mod_test", |b| b.iter(
        || {
            // for i in black_box(0..1) {
                let bp = BitPiece::new(
                    black_box(20),
                    black_box(Direction::West),
                    black_box(PieceColor::L)
                );
                black_box(bp.wall_overlap());

            // }

            

        }
    ));
}
fn bit_repr_benchmark(c: &mut Criterion) {
    c.bench_function("bit_repr", |b| b.iter(
        || {
            // for _ in black_box(0..1) {
                black_box(BitPiece::new(1, Direction::West, PieceColor::L).bit_repr());
            // }
        }
    ));
}
fn clear_shift_benchmark(c: &mut Criterion) {
    c.bench_function("clear_shift", |b| b.iter(
        || {
            // for _ in black_box(0..1) {
                black_box(black_box(BitBoard(0b1110000001_1100000001_1111111111_1110001111)).get_shift_cleared());
            // }
        }
    ));
}
fn bp_create(c: &mut Criterion) {
    c.bench_function("bp_create", |b| b.iter(
        || {
            // for _ in black_box(0..1) {
                black_box(BitPiece::new(
                    black_box(1), 
                    black_box(Direction::West),
                    black_box(PieceColor::L)
                ));
            // }
        }
    ));
}
fn bit_placements(c: &mut Criterion) {

    c.bench_function("bit_placements", |b| b.iter(
        || {
            for i in 0..1200 {
                let s_kick = TetBoard::load_fumen("v115@RhE8BeG8BeD8JeAgH");
                let mut s_bb = BitBoard::from(s_kick);
                s_bb.0 |= 1 << ((i*154256)%23);
                let s_cmaps = s_bb.gen_collision_maps(PieceColor::S as usize);
                let mut s_placements = Placements::new(s_cmaps);
                black_box(s_placements).gen_all();
                black_box(s_placements.filter_congruents(PieceColor::S as usize));
            }

        }
    ));
    
    
}
fn clear_benchmark(c: &mut Criterion) {
    c.bench_function("clear", |b| b.iter(
        || {
            let (mut cleared, lines) = black_box(
                black_box(
                    BitBoard(0b1110000001_1100000001_1111111111_1110001111)
                ).cleared()
            );
            let bp = black_box(
                BitPiece::new(
                    black_box(20),
                    black_box(Direction::West),
                    black_box(PieceColor::L)
                )
            );
            let mut a = 0;
            cleared |= black_box(bp.bit_repr());
            a += cleared;
            cleared = a;

        }
    ));
}
criterion_group!(benches, clear_benchmark);
criterion_main!(benches);