use shinobi::load_fen;
use shinobi::perft::perft;
use shinobi::perft::perft_test;
use shinobi::MoveGenerator;
use shinobi::Position;
use std::env;
use std::time::Instant;
fn main() {
    let args: Vec<String> = env::args().collect();

    let mut position: Position = Position::empty();
    let mut depth = "";
    let mut fen = "";
    let mut res = String::new();

    // Parse Input
    for (i, arg) in args.iter().enumerate() {
        if i == 1 {
            depth = arg;
        } else if i > 1 {
            let new = String::from(" ".to_owned() + arg);
            res.push_str(new.as_str());
        }
    }
    fen = res.as_str();
    fen = fen.trim();

    let d = match depth.parse::<u32>() {
        Ok(dpth) => dpth,
        Err(_) => 0,
    };

    // Setup Position
    let grid = load_fen(fen, &mut position.state);
    position.from_grid(grid);
    let mut move_gen = MoveGenerator::new();

    let start = Instant::now();
    let res = perft(&mut position, &mut move_gen, d);
    let elapsed = start.elapsed();
    println!("PERFT: {} TIME: {} US", res, elapsed.as_micros());
    println!("NPS: {:.0} ", res as f64 / elapsed.as_secs_f64());

    //perft_test(&mut position, &mut move_gen, d);
}
