use criterion::{black_box, criterion_group, criterion_main, Criterion};
use modular_bitfield::prelude::*;
use shinobi_core::mov::Move;
use shinobi_core::mov::MoveList;
use shinobi_core::MoveGenerator;
use shinobi_core::{perft::*, Engine, Position, Side, START_POS};

fn perft_starting_pos_depth_1(c: &mut Criterion) {
    let depth = 1;
    let mut engine = Engine::new();

    c.bench_function("Perft 1 Starting POS", |b| {
        b.iter(|| perft(&mut engine.position, &mut engine.move_gen, black_box(depth)))
    });
}

fn perft_starting_pos_depth_2(c: &mut Criterion) {
    let depth = 2;
    let mut engine = Engine::new();

    c.bench_function("Perft 2 Starting POS", |b| {
        b.iter(|| perft(&mut engine.position, &mut engine.move_gen, black_box(depth)))
    });
}

fn perft_starting_pos_depth_3(c: &mut Criterion) {
    let depth = 3;
    let mut engine = Engine::new();

    for _ in 0..5 {
        c.bench_function("Perft 3 Starting POS", |b| {
            b.iter(|| perft(&mut engine.position, &mut engine.move_gen, black_box(depth)))
        });
    }
}

fn perft_starting_pos_depth_4(c: &mut Criterion) {
    let depth = 4;
    let mut engine = Engine::new();
    for _ in 0..5 {
        c.bench_function("Perft 4 Starting POS", |b| {
            b.iter(|| perft(&mut engine.position, &mut engine.move_gen, black_box(depth)))
        });
    }
}

fn perft_starting_pos_depth_5(c: &mut Criterion) {
    let depth = 5;
    let mut engine = Engine::new();

    for _ in 0..5 {
        c.bench_function("Perft 5 Starting POS", |b| {
            b.iter(|| perft(&mut engine.position, &mut engine.move_gen, black_box(depth)))
        });
    }
}

fn board_clone_100(c: &mut Criterion) {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let position = Position::from_fen(fen).unwrap();
    c.bench_function("Board Clone 100", |b| {
        b.iter(|| {
            for _ in 0..100 {
                black_box(position.clone());
            }
        })
    });
}

fn knight_gen_bench(c: &mut Criterion) {
    let mut engine = Engine::new();
    let mut moves = MoveList::new();

    for _ in 0..5 {
        c.bench_function("Generating Knight Moves", |b| {
            b.iter(|| {
                black_box(engine.move_gen.gen_knight_moves(
                    &mut engine.position,
                    Side::White,
                    &mut moves,
                ));
                moves.clear();
            });
        });
    }
}

fn pawn_gen_bench(c: &mut Criterion) {
    let mut engine = Engine::new();
    let mut moves = MoveList::new();

    for _ in 0..5 {
        c.bench_function("Generating Pawn Moves", |b| {
            b.iter(|| {
                black_box(
                    engine
                        .move_gen
                        .gen_pawn_moves(&mut engine.position, &mut moves),
                );
                moves.clear();
            });
        });
    }
}

fn move_gen_bench(c: &mut Criterion) {
    let mut engine = Engine::new();

    for _ in 0..5 {
        c.bench_function("Generating Moves", |b| {
            b.iter(|| black_box(engine.move_gen.generate_moves(&mut engine.position, Side::White)))
        });
    }
}

fn move_list_creation_bench(c: &mut Criterion) {
    for _ in 0..5 {
        c.bench_function("New Move List Creation", |b| {
            b.iter(|| {
                black_box(MoveList::new());
            });
        });
    }
}

criterion_group!(
    benches,
    move_gen_bench,
    /*
    move_list_creation_bench,
    pawn_gen_bench,
    knight_gen_bench
    perft_starting_pos_depth_1,
                     perft_starting_pos_depth_2,
                     move_list_creation_bench,
                     perft_starting_pos_depth_3,
                     perft_starting_pos_depth_4,
                     perft_starting_pos_depth_5,
                     board_clone_100,
                     */
);
criterion_main!(benches);
