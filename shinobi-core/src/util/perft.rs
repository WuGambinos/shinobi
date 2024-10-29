use crate::mov::MoveType;
use crate::EMPTY_BITBOARD;
use crate::{MoveGenerator, Position};
use std::time::Instant;

pub fn perft(position: &mut Position, move_generator: &MoveGenerator, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut num_positions: u64 = 0;
    let moves =
        move_generator.generate_moves(position, position.state.current_turn(), MoveType::All);
    let side = position.state.current_turn();

    for i in 0..moves.len() {
        let mv = moves.get(i);
        position.make_move(mv);
        let in_check = move_generator.attacks_to_king(position, side) != EMPTY_BITBOARD;
        if !in_check {
            num_positions += perft(position, move_generator, depth - 1);
        }
        position.unmake();
    }

    num_positions
}

static mut NODES: u64 = 0;

pub fn perft_test(position: &mut Position, move_generator: &MoveGenerator, depth: u32) {
    //println!(" PERFORMANCE TEST");

    let moves =
        move_generator.generate_legal_moves(position, position.state.current_turn(), MoveType::All);

    for i in 0..moves.len() {
        let mv = moves.get(i);
        position.make_move(mv);

        unsafe {
            let cummulative_nodes: u64 = NODES;
            perft_driver(position, move_generator, depth - 1);

            let old_nodes: u64 = NODES - cummulative_nodes;

            position.unmake();

            println!("{} {}", mv, old_nodes);
            // used for perftree
            //println!("{}: {}", mv, old_nodes);
        }
    }

    //println!("DEPTH: {}", depth);
    unsafe {
        println!();
        println!("{}", NODES);
        //println!("NODES: {}", NODES);
    }
}

fn perft_driver(position: &mut Position, move_generator: &MoveGenerator, depth: u32) {
    let moves =
        move_generator.generate_legal_moves(position, position.state.current_turn(), MoveType::All);
    if depth == 1 {
        unsafe {
            NODES += moves.len() as u64;
        }
        return;
    }

    for i in 0..moves.len() {
        let mv = moves.get(i);
        position.make_move(mv);

        perft_driver(position, move_generator, depth - 1);

        position.unmake();
    }
}
