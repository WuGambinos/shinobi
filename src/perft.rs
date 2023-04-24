use crate::{MoveGenerator, Position, EMPTY_BITBOARD};

pub fn legal_perft(position: &mut Position, move_generator: &mut MoveGenerator, depth: u32) -> u32 {
    let mut num_positions: u32 = 0;
    let moves = move_generator.generate_moves(position, position.state.turn);
    if depth == 0 {
        return 1;
    }

    for mv in moves {
        position.make_move(mv);

        let checks = move_generator.attacks_to_king(position, position.state.turn);
        if checks == EMPTY_BITBOARD {
            num_positions += legal_perft(position, move_generator, depth - 1);
        }
        position.unmake();
    }

    return num_positions;
}

pub fn perft(position: &mut Position, move_generator: &mut MoveGenerator, depth: u32) -> u32 {
    let mut num_positions: u32 = 0;
    let moves = move_generator.generate_legal_moves(position, position.state.turn);

    for mv in &moves {
        println!("MOVE: {}", mv);
    }

    if depth == 1 {
        return moves.len() as u32;
    }

    for mv in moves {
        position.make_move(mv);
        num_positions += perft(position, move_generator, depth - 1);
        position.unmake();
    }

    return num_positions;
}

static mut NODES: u32 = 0;

pub fn perft_driver(position: &mut Position, move_generator: &mut MoveGenerator, depth: u32) {
    let moves = move_generator.generate_legal_moves(position, position.state.turn);
    if depth == 1 {
        unsafe {
            NODES += moves.len() as u32;
        }
        return;
    }

    for mv in moves {
        position.make_move(mv);

        perft_driver(position, move_generator, depth - 1);

        position.unmake();
    }
}

pub fn perft_test(position: &mut Position, move_generator: &mut MoveGenerator, depth: u32) {
    println!(" PERFORMANCE TEST");

    let moves = move_generator.generate_legal_moves(position, position.state.turn);

    for mv in moves {
        position.make_move(mv);

        unsafe {
            let cummulative_nodes: u32 = NODES;
            perft_driver(position, move_generator, depth - 1);

            let old_nodes: u32 = NODES - cummulative_nodes;

            position.unmake();

            println!("{}: {}", mv, old_nodes);
        }
    }

    println!("DEPTH: {}", depth);
    unsafe {
        println!("NODES: {}", NODES);
    }
}
