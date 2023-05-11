use crate::{MoveGenerator, Position};

pub fn perft(position: &mut Position, move_generator: &mut MoveGenerator, depth: u32) -> u64 {
    let mut num_positions: u64 = 0;
    let moves = move_generator.generate_legal_moves(position, position.state.turn);

    if depth == 1 {
        return moves.len() as u64;
    }

    for mv in moves {
        position.make_move(mv);
        num_positions += perft(position, move_generator, depth - 1);
        position.unmake();
    }

    return num_positions;
}

static mut NODES: u64 = 0;

pub fn perft_driver(position: &mut Position, move_generator: &mut MoveGenerator, depth: u32) {
    let moves = move_generator.generate_legal_moves(position, position.state.turn);
    if depth == 1 {
        unsafe {
            NODES += moves.len() as u64;
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
    //println!(" PERFORMANCE TEST");

    let moves = move_generator.generate_legal_moves(position, position.state.turn);

    for mv in moves {
        position.make_move(mv);

        unsafe {
            let cummulative_nodes: u64 = NODES;
            perft_driver(position, move_generator, depth - 1);

            let old_nodes: u64 = NODES - cummulative_nodes;

            position.unmake();

            println!("{}: {}", mv, old_nodes);
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
