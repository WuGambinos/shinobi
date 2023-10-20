use shinobi::{load_fen, perft::perft, Engine, Position};

#[cfg(test)]
#[test]
fn test_add() {
    assert_eq!(1 + 1, 1 + 1);
}

#[test]
fn perft_starting_pos_1() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let position = Position::from_fen(fen);
    let mut shinobi = Engine::new(position);

    let depth = 1;

    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 20);
}

#[test]
fn perft_starting_pos_2() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let position = Position::from_fen(fen);
    let mut shinobi = Engine::new(position);

    let depth = 2;

    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);
    assert_eq!(nodes, 400);
}

#[test]
fn perft_starting_pos_3() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let position = Position::from_fen(fen);
    let mut shinobi = Engine::new(position);

    let depth = 3;

    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);
    assert_eq!(nodes, 8902);
}

#[test]
fn perft_starting_pos_4() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let position = Position::from_fen(fen);
    let mut shinobi = Engine::new(position);

    let depth = 4;

    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);
    assert_eq!(nodes, 197_281);
}

#[test]
fn perft_starting_pos_5() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let position = Position::from_fen(fen);
    let mut shinobi = Engine::new(position);

    let depth = 5;

    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);
    assert_eq!(nodes, 4_865_609);
}

#[test]
fn perft_starting_pos_6() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let position = Position::from_fen(fen);
    let mut shinobi = Engine::new(position);

    let depth = 6;

    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);
    assert_eq!(nodes, 119_060_324);
}
