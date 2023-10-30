use shinobi::{load_fen, perft::perft, Engine, Position};

const POS_2: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ";
const POS_3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ";
const POS_4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
const POS_5: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
const POS_6: &str = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";

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

#[test]
fn perft_pos_2_depth_1() {
    let position = Position::from_fen(POS_2);
    let mut shinobi = Engine::new(position);
    let depth = 1;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 48);
}

#[test]
fn perft_pos_2_depth_2() {
    let position = Position::from_fen(POS_2);
    let mut shinobi = Engine::new(position);
    let depth = 2;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 2039);
}

#[test]
fn perft_pos_2_depth_3() {
    let position = Position::from_fen(POS_2);
    let mut shinobi = Engine::new(position);
    let depth = 3;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 97862);
}

#[test]
fn perft_pos_2_depth_4() {
    let position = Position::from_fen(POS_2);
    let mut shinobi = Engine::new(position);
    let depth = 4;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 4_085_603);
}

#[test]
fn perft_pos_2_depth_5() {
    let position = Position::from_fen(POS_2);
    let mut shinobi = Engine::new(position);
    let depth = 5;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 193_690_690);
}

#[test]
fn perft_pos_3_depth_1() {
    let position = Position::from_fen(POS_3);
    let mut shinobi = Engine::new(position);
    let depth = 1;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 14);
}

#[test]
fn perft_pos_3_depth_2() {
    let position = Position::from_fen(POS_3);
    let mut shinobi = Engine::new(position);
    let depth = 2;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 191);
}

#[test]
fn perft_pos_3_depth_3() {
    let position = Position::from_fen(POS_3);
    let mut shinobi = Engine::new(position);
    let depth = 3;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 2812);
}

#[test]
fn perft_pos_3_depth_4() {
    let position = Position::from_fen(POS_3);
    let mut shinobi = Engine::new(position);
    let depth = 4;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 43238);
}

#[test]
fn perft_pos_3_depth_5() {
    let position = Position::from_fen(POS_3);
    let mut shinobi = Engine::new(position);
    let depth = 5;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 674_624);
}

#[test]
fn perft_pos_3_depth_6() {
    let position = Position::from_fen(POS_3);
    let mut shinobi = Engine::new(position);
    let depth = 6;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 11_030_083);
}

#[test]
fn perft_pos_4_depth_1() {
    let position = Position::from_fen(POS_4);
    let mut shinobi = Engine::new(position);
    let depth = 1;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 6);
}

#[test]
fn perft_pos_4_depth_2() {
    let position = Position::from_fen(POS_4);
    let mut shinobi = Engine::new(position);
    let depth = 2;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 264);
}

#[test]
fn perft_pos_4_depth_3() {
    let position = Position::from_fen(POS_4);
    let mut shinobi = Engine::new(position);
    let depth = 3;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 9467);
}

#[test]
fn perft_pos_4_depth_4() {
    let position = Position::from_fen(POS_4);
    let mut shinobi = Engine::new(position);
    let depth = 4;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 422_333);
}

#[test]
fn perft_pos_4_depth_5() {
    let position = Position::from_fen(POS_4);
    let mut shinobi = Engine::new(position);
    let depth = 5;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 15_833_292);
}

#[test]
fn perft_pos_5_depth_1() {
    let position = Position::from_fen(POS_5);
    let mut shinobi = Engine::new(position);
    let depth = 1;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 44);
}

#[test]
fn perft_pos_5_depth_2() {
    let position = Position::from_fen(POS_5);
    let mut shinobi = Engine::new(position);
    let depth = 2;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 1486);
}

#[test]
fn perft_pos_5_depth_3() {
    let position = Position::from_fen(POS_5);
    let mut shinobi = Engine::new(position);
    let depth = 3;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 62379);
}

#[test]
fn perft_pos_5_depth_4() {
    let position = Position::from_fen(POS_5);
    let mut shinobi = Engine::new(position);
    let depth = 4;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 2_103_487);
}

#[test]
fn perft_pos_5_depth_5() {
    let position = Position::from_fen(POS_5);
    let mut shinobi = Engine::new(position);
    let depth = 5;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 89_941_194);
}

#[test]
fn perft_pos_6_depth_1() {
    let position = Position::from_fen(POS_6);
    let mut shinobi = Engine::new(position);
    let depth = 1;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 46);
}

#[test]
fn perft_pos_6_depth_2() {
    let position = Position::from_fen(POS_6);
    let mut shinobi = Engine::new(position);
    let depth = 2;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 2079);
}

#[test]
fn perft_pos_6_depth_3() {
    let position = Position::from_fen(POS_6);
    let mut shinobi = Engine::new(position);
    let depth = 3;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 89890);
}

#[test]
fn perft_pos_6_depth_4() {
    let position = Position::from_fen(POS_6);
    let mut shinobi = Engine::new(position);
    let depth = 4;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 3_894_594);
}

#[test]
fn perft_pos_6_depth_5() {
    let position = Position::from_fen(POS_6);
    let mut shinobi = Engine::new(position);
    let depth = 5;
    let nodes = perft(&mut shinobi.position, &mut shinobi.move_gen, depth);

    assert_eq!(nodes, 164_075_551);
}
