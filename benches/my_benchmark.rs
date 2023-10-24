use criterion::{black_box, criterion_group, criterion_main, Criterion};
use shinobi::{
    load_fen, perft::*, BitBoard, Engine, IntoEnumIterator, MoveGenerator, Piece, Position, Rng,
    Side, SquareLabel, SquareLabelIter,
};

fn perft_starting_pos_depth_1(c: &mut Criterion) {
    let depth = 1;
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let position = Position::from_fen(fen);
    let mut engine = Engine::new(position);

    c.bench_function("Perft 1 Starting POS", |b| {
        b.iter(|| perft(&mut engine.position, &mut engine.move_gen, black_box(depth)))
    });
}

fn perft_starting_pos_depth_2(c: &mut Criterion) {
    let depth = 1;
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let position = Position::from_fen(fen);
    let mut engine = Engine::new(position);

    c.bench_function("Perft 1 Starting POS", |b| {
        b.iter(|| perft(&mut engine.position, &mut engine.move_gen, black_box(depth)))
    });
}

fn perft_starting_pos_depth_3(c: &mut Criterion) {
    let depth = 3;
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let position = Position::from_fen(fen);
    let mut engine = Engine::new(position);

    for _ in 0..5 {
        c.bench_function("Perft 3 Starting POS", |b| {
            b.iter(|| perft(&mut engine.position, &mut engine.move_gen, black_box(depth)))
        });
    }
}

fn perft_starting_pos_depth_4(c: &mut Criterion) {
    let depth = 4;
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let position = Position::from_fen(fen);
    let mut engine = Engine::new(position);
    for _ in 0..5 {
        c.bench_function("Perft 4 Starting POS", |b| {
            b.iter(|| perft(&mut engine.position, &mut engine.move_gen, black_box(depth)))
        });
    }
}

fn perft_starting_pos_depth_5(c: &mut Criterion) {
    let depth = 5;
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let position = Position::from_fen(fen);
    let mut engine = Engine::new(position);

    for _ in 0..5 {
        c.bench_function("Perft 5 Starting POS", |b| {
            b.iter(|| perft(&mut engine.position, &mut engine.move_gen, black_box(depth)))
        });
    }
}

fn board_clone_100(c: &mut Criterion) {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let position = Position::from_fen(fen);
    c.bench_function("Board Clone 100", |b| {
        b.iter(|| {
            for _ in 0..100 {
                black_box(position.clone());
            }
        })
    });
}

fn move_gen_bench(c: &mut Criterion) {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let position = Position::from_fen(fen);
    let mut engine = Engine::new(position);

    for _ in 0..5 {
        c.bench_function("Generating Moves", |b| {
            b.iter(|| {
                black_box(
                    engine
                        .move_gen
                        .generate_moves(&mut engine.position, shinobi::Side::White),
                )
            })
        });
    }
}

criterion_group!(
    benches,
    perft_starting_pos_depth_1,
    perft_starting_pos_depth_2,
    perft_starting_pos_depth_3,
    perft_starting_pos_depth_4,
    perft_starting_pos_depth_5,
    board_clone_100,
    move_gen_bench,
);
criterion_main!(benches);
