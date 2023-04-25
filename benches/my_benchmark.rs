use criterion::{black_box, criterion_group, criterion_main, Criterion};
use shinobi::{load_fen, perft::*, MoveGenerator, Position};

fn perft_starting_pos_depth_1(c: &mut Criterion) {
    let mut position = Position::new();
    let mut move_gen = MoveGenerator::new();
    let depth = 1;
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let grid = load_fen(fen, &mut position.state);

    position.from_grid(grid);
    c.bench_function("Perft 1 Starting POS", |b| {
        b.iter(|| perft(&mut position, &mut move_gen, black_box(depth)))
    });
}

fn perft_starting_pos_depth_3(c: &mut Criterion) {
    let mut position = Position::new();
    let mut move_gen = MoveGenerator::new();
    let depth = 3;
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let grid = load_fen(fen, &mut position.state);

    position.from_grid(grid);
    for _ in 0..5 {
        c.bench_function("Perft 3 Starting POS", |b| {
            b.iter(|| perft(&mut position, &mut move_gen, black_box(depth)))
        });
    }
}

fn perft_starting_pos_depth_4(c: &mut Criterion) {
    let mut position = Position::new();
    let mut move_gen = MoveGenerator::new();
    let depth = 4;
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let grid = load_fen(fen, &mut position.state);

    position.from_grid(grid);
    for _ in 0..5 {
        c.bench_function("Perft 4 Starting POS", |b| {
            b.iter(|| perft(&mut position, &mut move_gen, black_box(depth)))
        });
    }
}

fn perft_starting_pos_depth_5(c: &mut Criterion) {
    let mut position = Position::new();
    let mut move_gen = MoveGenerator::new();
    let depth = 5;
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let grid = load_fen(fen, &mut position.state);

    position.from_grid(grid);
    for _ in 0..5 {
        c.bench_function("Perft 5 Starting POS", |b| {
            b.iter(|| perft(&mut position, &mut move_gen, black_box(depth)))
        });
    }
}

criterion_group!(benches, perft_starting_pos_depth_5);
criterion_main!(benches);
