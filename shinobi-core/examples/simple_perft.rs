use shinobi_core::mov::Move;
use shinobi_core::mov::MoveList;
use shinobi_core::perft::perft;
use shinobi_core::Engine;
use shinobi_core::Position;
use shinobi_core::START_POS;
use shinobi_core::*;
use std::env;
use std::time::Instant;
const POS_2: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ";
fn main() {
    // Setup Position
    let mut shinobi = Engine::new();
    let pos = Position::from_fen(POS_2).unwrap();
    shinobi.position = pos;

    let start = Instant::now();
    let res = perft(&mut shinobi.position, &mut shinobi.move_gen, 3);
    let end = start.elapsed();

    println!("PERFT 3: {} TIME: {:?}", res, end);
    /*
    unsafe {
        println!("UNMAKE TIME: {} ns", UNMAKE_TIME);
    }
    */

    /*
    let mut move_gen_time = 0;
    let start = Instant::now();
    let _ = custom_perft(&mut shinobi.position, &mut shinobi.move_gen, 4);
    let elapsed = start.elapsed();
    */
    /*
    println!("PERFT: {} TIME: {} US", res, elapsed.as_micros());
    println!("NPS: {:.0} ", res as f64 / elapsed.as_secs_f64());
    println!("MAKE TIME: {} ns", make_time);
    println!("UNMAKE TIME: {} ns", unmake_time);
    println!("SIZE OF MOVE: {} BYTES", std::mem::size_of::<Move>());
    */

    /*
    unsafe {
        println!("PAWN_GEN_TIME: {} ns", PAWN_GEN_TIME);
        println!("KNIGHT_GEN_TIME: {} ns", KNIGHT_GEN_TIME);
        println!("BISHOP_GEN_TIME: {} ns", BISHOP_GEN_TIME);
        println!("ROOK_GEN_TIME: {} ns", ROOK_GEN_TIME);
        println!("QUEEN_GEN_TIME: {} ns", QUEEN_GEN_TIME);
        println!("KING_GEN_TIME: {} ns", KING_GEN_TIME);
    }
    println!("TIME: {:?}", elapsed);
    */
}
