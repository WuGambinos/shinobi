use crate::{
    get_file, get_rank, init_slider_attacks, BitBoard, Move, Piece, Position, SMagic, Side,
    SquareLabel, A_FILE, B_FILE, EMPTY_BITBOARD, F_FILE, G_FILE, H_FILE,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug)]
pub struct PinInfo {
    piece_pinning: Piece,
    pinned_piece: Piece,
    moves: BitBoard,
}

impl PinInfo {
    pub fn new(moves: BitBoard, pinned_piece: Piece, piece_pinning: Piece) -> PinInfo {
        PinInfo {
            moves,
            pinned_piece,
            piece_pinning,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MoveGenerator {
    pub knight_moves: [BitBoard; 64],
    pub pawn_pushes: [[BitBoard; 64]; 2],
    pub pawn_attacks: [[BitBoard; 64]; 2],
    pub king_moves: [BitBoard; 64],
    pub rook_moves: [BitBoard; 102400],
    pub bishop_moves: [BitBoard; 5248],
    pub bishop_tbl: [SMagic; 64],
    pub rook_tbl: [SMagic; 64],
}

impl MoveGenerator {
    pub fn new() -> MoveGenerator {
        let mut move_gen = MoveGenerator {
            rook_moves: [EMPTY_BITBOARD; 102400],
            bishop_moves: [EMPTY_BITBOARD; 5248],
            knight_moves: [EMPTY_BITBOARD; 64],
            pawn_pushes: [[EMPTY_BITBOARD; 64]; 2],
            pawn_attacks: [[EMPTY_BITBOARD; 64]; 2],
            king_moves: [EMPTY_BITBOARD; 64],
            bishop_tbl: [SMagic::new(0, 0, 0, 0); 64],
            rook_tbl: [SMagic::new(0, 0, 0, 0); 64],
        };

        move_gen.fill_knight_moves();
        move_gen.fill_king_moves();
        move_gen.fill_pawn_attacks(Side::White);
        move_gen.fill_pawn_attacks(Side::Black);
        init_slider_attacks(&mut move_gen, true);
        init_slider_attacks(&mut move_gen, false);

        move_gen
    }

    pub fn north_one(&self, bitboard: BitBoard) -> BitBoard {
        return bitboard << 8;
    }

    pub fn south_one(&self, bitboard: BitBoard) -> BitBoard {
        return bitboard >> 8;
    }

    pub fn east_one(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard << 1) & BitBoard(!A_FILE);
    }

    pub fn west_one(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard >> 1) & BitBoard(!H_FILE);
    }

    pub fn north_east_one(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard << 9) & BitBoard(!A_FILE);
    }

    pub fn north_west_one(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard << 7) & BitBoard(!H_FILE);
    }

    pub fn south_east_one(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard >> 7) & BitBoard(!(A_FILE));
    }

    pub fn south_west_one(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard >> 9) & BitBoard(!(H_FILE));
    }

    pub fn north_north_east(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard << 17) & BitBoard(!(A_FILE));
    }

    pub fn north_east_east(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard << 10) & BitBoard(!(A_FILE | B_FILE));
    }

    pub fn south_east_east(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard >> 6) & BitBoard(!(A_FILE | B_FILE));
    }

    pub fn south_south_east(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard >> 15) & BitBoard(!(A_FILE));
    }

    pub fn north_north_west(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard << 15) & BitBoard(!(H_FILE));
    }

    pub fn north_west_west(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard << 6) & BitBoard(!(G_FILE | H_FILE));
    }

    pub fn south_west_west(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard >> 10) & BitBoard(!(G_FILE | H_FILE));
    }

    pub fn south_south_west(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard >> 17) & BitBoard(!(H_FILE));
    }

    pub fn white_single_push_target(&self, position: &Position, bitboard: BitBoard) -> BitBoard {
        return self.north_one(bitboard) & position.empty_bitboard;
    }

    pub fn white_double_push_target(&self, position: &Position, bitboard: BitBoard) -> BitBoard {
        const RANK4: BitBoard = BitBoard(0x0000_0000_FF00_0000);
        let single_pushes = self.white_single_push_target(position, bitboard);
        return self.north_one(single_pushes) & position.empty_bitboard & RANK4;
    }

    pub fn white_pawns_able_push(&self, position: &Position, empty: BitBoard) -> BitBoard {
        return self.south_one(empty)
            & position.piece_bitboards[Side::White as usize][Piece::Pawn as usize];
    }

    pub fn white_pawns_able_double_push(&self, position: &Position) -> BitBoard {
        const RANK4: BitBoard = BitBoard(0x0000_0000_FF00_0000);
        let empty_rank_3 =
            self.south_one(position.empty_bitboard & RANK4) & position.empty_bitboard;
        return self.white_pawns_able_push(position, empty_rank_3);
    }

    pub fn black_single_push_target(&self, position: &Position, bitboard: BitBoard) -> BitBoard {
        return self.south_one(bitboard) & position.empty_bitboard;
    }

    pub fn black_double_push_target(&self, position: &Position, bitboard: BitBoard) -> BitBoard {
        const RANK5: BitBoard = BitBoard(0x0000_00FF_0000_0000);
        let single_pushes = self.black_single_push_target(position, bitboard);
        return self.south_one(single_pushes) & position.empty_bitboard & RANK5;
    }

    pub fn fill_pawn_pushes(&mut self, position: &Position, side: Side) {
        for square in SquareLabel::iter() {
            let mut pushes: BitBoard = EMPTY_BITBOARD;

            match side {
                Side::White => {
                    let mut white_pawns: BitBoard = EMPTY_BITBOARD;
                    white_pawns.set_bit(square);
                    let single_push = self.white_single_push_target(position, white_pawns);
                    let double_push = self.white_double_push_target(position, white_pawns);
                    pushes |= single_push;
                    if single_push != EMPTY_BITBOARD {
                        pushes |= double_push;
                    }
                    self.pawn_pushes[side as usize][square as usize] = pushes;
                }
                Side::Black => {
                    let mut black_pawns: BitBoard = EMPTY_BITBOARD;
                    black_pawns.set_bit(square);
                    pushes |= self.black_single_push_target(position, black_pawns);
                    pushes |= self.black_double_push_target(position, black_pawns);
                    self.pawn_pushes[side as usize][square as usize] = pushes;
                }
            };
        }
    }

    pub fn white_pawn_east_attacks(&self, bitboard: BitBoard) -> BitBoard {
        return self.north_east_one(bitboard);
    }

    pub fn white_pawn_west_attacks(&self, bitboard: BitBoard) -> BitBoard {
        return self.north_west_one(bitboard);
    }

    pub fn black_pawn_east_attacks(&self, bitboard: BitBoard) -> BitBoard {
        return self.south_east_one(bitboard);
    }

    pub fn black_pawn_west_attacks(&self, bitboard: BitBoard) -> BitBoard {
        return self.south_west_one(bitboard);
    }

    pub fn fill_pawn_attacks(&mut self, side: Side) {
        for square in SquareLabel::iter() {
            let mut moves: BitBoard = EMPTY_BITBOARD;
            let mut bitboard: BitBoard = EMPTY_BITBOARD;

            bitboard.set_bit(square);

            match side {
                Side::White => {
                    moves |= self.white_pawn_east_attacks(bitboard);
                    moves |= self.white_pawn_west_attacks(bitboard);
                    self.pawn_attacks[side as usize][square as usize] = moves;
                }
                Side::Black => {
                    moves |= self.black_pawn_east_attacks(bitboard);
                    moves |= self.black_pawn_west_attacks(bitboard);
                    self.pawn_attacks[side as usize][square as usize] = moves;
                }
            }
        }
    }

    pub fn fill_pawn_moves(&mut self, position: &Position, side: Side) {
        self.fill_pawn_pushes(position, side);
        self.fill_pawn_attacks(side);
    }

    pub fn attacks_to_king(&self, position: &Position, side: Side) -> BitBoard {
        let king_square = match side {
            Side::White => position.white_king_square,
            Side::Black => position.black_king_square,
        };

        let enemy = match side {
            Side::White => Side::Black,
            Side::Black => Side::White,
        };

        let opponent_pawns = position.get_piece_bitboard(Piece::Pawn, enemy);
        let opponent_knights = position.get_piece_bitboard(Piece::Knight, enemy);
        let opponent_rooks = position.get_piece_bitboard(Piece::Rook, enemy);
        let opponent_bishop = position.get_piece_bitboard(Piece::Bishop, enemy);
        let opponent_queen = position.get_piece_bitboard(Piece::Queen, enemy);

        return (self.get_bishop_moves(king_square as u64, position.main_bitboard)
            & opponent_bishop)
            | (self.get_rook_moves(king_square as u64, position.main_bitboard) & opponent_rooks)
            | (self.knight_moves[king_square as usize] & opponent_knights)
            | (self.pawn_attacks[side as usize][king_square as usize] & opponent_pawns)
            | (self.get_queen_moves(king_square as u64, position.main_bitboard) & opponent_queen);
    }

    pub fn fill_king_moves(&mut self) {
        for square in SquareLabel::iter() {
            let mut moves: BitBoard = EMPTY_BITBOARD;
            let mut bitboard: BitBoard = EMPTY_BITBOARD;

            bitboard.set_bit(square);

            moves |= self.east_one(bitboard) | self.west_one(bitboard);
            bitboard |= moves;
            moves |= self.north_one(bitboard) | self.south_one(bitboard);

            self.king_moves[square as usize] = moves;
        }
    }

    pub fn fill_knight_moves(&mut self) {
        for square in SquareLabel::iter() {
            let mut moves: BitBoard = EMPTY_BITBOARD;
            let mut bitboard: BitBoard = EMPTY_BITBOARD;

            bitboard.set_bit(square);

            moves |= self.north_north_east(bitboard);
            moves |= self.north_east_east(bitboard);
            moves |= self.south_east_east(bitboard);
            moves |= self.south_south_east(bitboard);
            moves |= self.north_north_west(bitboard);
            moves |= self.north_west_west(bitboard);
            moves |= self.south_west_west(bitboard);
            moves |= self.south_south_west(bitboard);

            self.knight_moves[square as usize] = moves;
        }
    }
    pub fn get_bishop_moves(&self, square: u64, occupancy: BitBoard) -> BitBoard {
        let index = self.bishop_tbl[square as usize].get_index(occupancy);
        return BitBoard(self.bishop_moves[index].0);
    }

    pub fn get_rook_moves(&self, square: u64, occupancy: BitBoard) -> BitBoard {
        let index = self.rook_tbl[square as usize].get_index(occupancy);
        return BitBoard(self.rook_moves[index].0);
    }

    pub fn get_queen_moves(&self, square: u64, occupancy: BitBoard) -> BitBoard {
        return self.get_rook_moves(square, occupancy) | self.get_bishop_moves(square, occupancy);
    }

    pub fn create_moves(
        &mut self,
        position: &Position,
        piece: Piece,
        side: Side,
        piece_moves: BitBoard,
        square: SquareLabel,
        moves: &mut Vec<Move>,
    ) {
        let mut n: u64 = (piece_moves & (!position.main_bitboard)).0;
        let mut i = 0;

        // Quiet Moves
        while n > 0 {
            let bit = n & 1;
            if bit == 1 {
                moves.push(Move::new(piece, square, SquareLabel::from(i)));
            }

            n = n >> 1;
            i += 1;
        }

        let mut j = 0;

        let mut n2: u64 = if piece == Piece::Pawn {
            (self.pawn_attacks[side as usize][square as usize] & (position.enemy_bitboard())).0
        } else {
            (piece_moves & (position.enemy_bitboard())).0
        };

        // Captures
        while n2 > 0 {
            let bit = n2 & 1;
            if bit == 1 {
                let mv = Move::new(piece, square, SquareLabel::from(j));
                moves.push(mv);
            }

            n2 = n2 >> 1;
            j += 1;
        }
    }

    pub fn generate_legal_moves(&mut self, position: &mut Position, side: Side) -> Vec<Move> {
        let mut moves: Vec<Move> = self.generate_moves(position, side);

        for i in (0..moves.len()).rev() {
            let mv: Move = moves[i];
            position.make_move(mv);

            if self.attacks_to_king(position, side) != EMPTY_BITBOARD {
                moves.remove(i);
            }

            position.unmake();
        }

        moves
    }

    pub fn get_pinned_piece(
        &self,
        position: &Position,
        pin_bitboard: BitBoard,
    ) -> Vec<(Option<Piece>, Option<SquareLabel>)> {
        let mut pinned_pieces: Vec<(Option<Piece>, Option<SquareLabel>)> = Vec::new();
        pinned_pieces.push((None, None));
        let mut n = pin_bitboard.0;
        let mut i = 0;
        while n > 0 {
            let bit = n & 1;
            if bit == 1 {
                let square = SquareLabel::from(i);
                let piece = position.get_piece_on_square(square, position.state.turn);
                println!("SQUARE: {:?} PIECE: {:?}", square, piece);
                pinned_pieces.push((piece, Some(square)));
            }
            n = n >> 1;
            i += 1;
        }

        pinned_pieces
    }

    pub fn find_pinned_moves(
        &self,
        position: &Position,
        king_square: SquareLabel,
    ) -> (bool, Vec<PinInfo>, Vec<PinInfo>, Vec<PinInfo>, BitBoard) {
        // Opponent's sliding pieces
        let rooks: BitBoard = position.get_piece_bitboard(Piece::Rook, position.state.enemy());
        let bishops: BitBoard = position.get_piece_bitboard(Piece::Bishop, position.state.enemy());
        let queens: BitBoard = position.get_piece_bitboard(Piece::Queen, position.state.enemy());

        // Moves from opponent's sliding pieces
        let mut rook_moves: BitBoard = BitBoard(0);
        let mut bishop_moves: BitBoard = BitBoard(0);
        let mut queen_moves: BitBoard = BitBoard(0);

        let mut queen_square = SquareLabel::from(0);
        let mut bishop_square = SquareLabel::from(0);
        let mut rook_square = SquareLabel::from(0);

        // Sliding moves from current player's king square
        let rook_moves_from_king: BitBoard = self.get_rook_moves(
            king_square as u64,
            position.side_bitboards[position.state.enemy() as usize],
        );
        let bishop_moves_from_king: BitBoard = self.get_bishop_moves(
            king_square as u64,
            position.side_bitboards[position.state.enemy() as usize],
        );

        let queen_moves_from_king: BitBoard = self.get_queen_moves(
            king_square as u64,
            position.side_bitboards[position.state.enemy() as usize],
        );

        // Find moves from opponents sliding pieces
        for piece in Piece::iter() {
            if piece == Piece::Rook {
                let mut n = rooks.0;
                let mut i = 0;
                while n > 0 {
                    let bit = n & 1;
                    if bit == 1 {
                        rook_square = SquareLabel::from(i);
                        rook_moves |= self.get_rook_moves(i, position.main_bitboard);
                    }
                    n = n >> 1;
                    i += 1;
                }
            } else if piece == Piece::Bishop {
                let mut n = bishops.0;
                let mut i = 0;
                while n > 0 {
                    let bit = n & 1;
                    if bit == 1 {
                        bishop_square = SquareLabel::from(i);
                        bishop_moves |= self.get_bishop_moves(i, position.main_bitboard);
                    }
                    n = n >> 1;
                    i += 1;
                }
            } else if piece == Piece::Queen {
                let mut n = queens.0;
                let mut i = 0;
                while n > 0 {
                    let bit = n & 1;
                    if bit == 1 {
                        queen_square = SquareLabel::from(i);
                        queen_moves |= self.get_queen_moves(i, position.main_bitboard);
                    }
                    n = n >> 1;
                    i += 1;
                }
                let possible_queen_pins =
                    (rook_moves_from_king | bishop_moves_from_king) & queen_moves;

                let possible_rook_pins = (rook_moves_from_king) & rook_moves;

                let possible_bishop_pins = (bishop_moves_from_king) & bishop_moves;

                let bishop_pins_bitboard: BitBoard =
                    possible_bishop_pins & position.side_bitboards[position.state.turn as usize];

                let rook_pins_bitboard: BitBoard =
                    possible_rook_pins & position.side_bitboards[position.state.turn as usize];

                let queen_pins_bitboard: BitBoard =
                    possible_queen_pins & position.side_bitboards[position.state.turn as usize];

                let queen_pinned_pieces_and_squares: Vec<(Option<Piece>, Option<SquareLabel>)> =
                    self.get_pinned_piece(position, queen_pins_bitboard);

                let rook_pinned_pieces_and_squares: Vec<(Option<Piece>, Option<SquareLabel>)> =
                    self.get_pinned_piece(position, rook_pins_bitboard);

                let bishop_pinned_pieces_and_squares: Vec<(Option<Piece>, Option<SquareLabel>)> =
                    self.get_pinned_piece(position, bishop_pins_bitboard);

                let current_king_bitboard =
                    position.get_piece_bitboard(Piece::King, position.state.turn);

                let mut queen_pins: Vec<PinInfo> = Vec::new();
                let mut rook_pins: Vec<PinInfo> = Vec::new();
                let mut bishop_pins: Vec<PinInfo> = Vec::new();

                for (bishop_pinned_piece, square) in bishop_pinned_pieces_and_squares {
                    let pinning_bishop_to_king_moves =
                        self.get_bishop_moves(bishop_square as u64, current_king_bitboard);
                    let mut pinned_bishop_moves = EMPTY_BITBOARD;
                    if let Some(b_pinned_piece) = bishop_pinned_piece {
                        pinned_bishop_moves = if b_pinned_piece.is_bishop()
                            || b_pinned_piece.is_queen()
                            || b_pinned_piece.is_pawn()
                        {
                            pinning_bishop_to_king_moves & bishop_moves_from_king
                        } else {
                            EMPTY_BITBOARD
                        };
                    }
                    if let Some(b_pinned_piece) = bishop_pinned_piece {
                        bishop_pins.push(PinInfo::new(
                            pinned_bishop_moves,
                            b_pinned_piece,
                            Piece::Bishop,
                        ));
                    }
                }

                for (rook_pinned_piece, square) in rook_pinned_pieces_and_squares {
                    let pinning_rook_to_king_moves =
                        self.get_rook_moves(rook_square as u64, current_king_bitboard);
                    let mut pinned_rook_moves = EMPTY_BITBOARD;
                    if let Some(r_pinned_piece) = rook_pinned_piece {
                        pinned_rook_moves = if r_pinned_piece.is_rook() || r_pinned_piece.is_queen()
                        {
                            pinning_rook_to_king_moves & rook_moves_from_king
                        } else {
                            EMPTY_BITBOARD
                        };
                    }
                    if let Some(r_pinned_piece) = rook_pinned_piece {
                        rook_pins.push(PinInfo::new(
                            pinned_rook_moves,
                            r_pinned_piece,
                            Piece::Rook,
                        ));
                    }
                }

                for (queen_pinned_piece, square) in queen_pinned_pieces_and_squares {
                    let pinning_queen_to_king_moves =
                        self.get_queen_moves(queen_square as u64, current_king_bitboard);
                    let pinned_queen_moves = pinning_queen_to_king_moves & queen_moves_from_king;
                    if let Some(q_pinned_piece) = queen_pinned_piece {
                        queen_pins.push(PinInfo::new(
                            pinned_queen_moves,
                            q_pinned_piece,
                            Piece::Queen,
                        ));
                    }
                }

                if queen_pins_bitboard != EMPTY_BITBOARD
                    || bishop_pins_bitboard != EMPTY_BITBOARD
                    || rook_pins_bitboard != EMPTY_BITBOARD
                {
                    return (
                        true,
                        queen_pins,
                        rook_pins,
                        bishop_pins,
                        bishop_pins_bitboard | rook_pins_bitboard | queen_pins_bitboard,
                    );
                }
            }
        }

        return (false, Vec::new(), Vec::new(), Vec::new(), EMPTY_BITBOARD);
    }

    fn moves_to_pins_moves(
        &mut self,
        position: &Position,
        piece: Piece,
        moves: &mut BitBoard,
        pins: &[Vec<PinInfo>; 3],
        pin_bitboard: &BitBoard,
    ) {
        let queen_pins = &pins[0];
        let rook_pins = &pins[1];
        let bishop_pins = &pins[2];

        for pin in queen_pins {
            if pin.pinned_piece == piece {
                let test_pin =
                    *pin_bitboard & position.get_piece_bitboard(piece, position.state.turn);
                if test_pin != EMPTY_BITBOARD {
                    *moves = EMPTY_BITBOARD | pin.moves;
                }
            }
        }

        for pin in rook_pins {
            if pin.pinned_piece == piece {
                let test_pin =
                    *pin_bitboard & position.get_piece_bitboard(piece, position.state.turn);
                if test_pin != EMPTY_BITBOARD {
                    *moves = EMPTY_BITBOARD | pin.moves;
                }
            }
        }

        for pin in bishop_pins {
            if pin.pinned_piece == piece {
                let test_pin =
                    *pin_bitboard & position.get_piece_bitboard(piece, position.state.turn);
                if test_pin != EMPTY_BITBOARD {
                    *moves = EMPTY_BITBOARD | pin.moves;
                }
            }
        }
    }

    pub fn generate_moves(&mut self, position: &Position, side: Side) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let king_square = match side {
            Side::White => position.white_king_square,
            Side::Black => position.black_king_square,
        };

        let opponent_side = match side {
            Side::White => Side::Black,
            Side::Black => Side::White,
        };

        self.fill_pawn_moves(position, side);
        let checkers = self.attacks_to_king(position, side);
        /*
        if checks != EMPTY_BITBOARD {
            println!("CHECK");
        }
        */

        let (is_pinned, queen_pins, rook_pins, bishop_pins, pin_bitboard) =
            self.find_pinned_moves(position, king_square);

        let pins: [Vec<PinInfo>; 3] = [queen_pins, rook_pins, bishop_pins];

        for square in SquareLabel::iter() {
            let piece: Option<Piece> = position.get_piece_on_square(square, side);

            let mut capture_mask = BitBoard(0xFFFFFFFFFFFFFFFF);
            let mut push_mask = BitBoard(0xFFFFFFFFFFFFFFFF);

            // Single check
            if checkers.0.count_ones() == 1 {
                capture_mask = checkers;

                let checker_square = checkers.bitscan_forward();
                let checker_piece = position
                    .get_piece_on_square(checker_square, opponent_side)
                    .unwrap();

                let piece_file = get_file(checker_square);
                let king_file = get_file(king_square);
                push_mask = match checker_piece {
                    Piece::Rook => {
                        if piece_file == king_file {
                            self.get_rook_moves(king_square as u64, position.main_bitboard)
                                & BitBoard(get_file(king_square))
                        } else {
                            self.get_rook_moves(king_square as u64, position.main_bitboard)
                                & BitBoard(get_rank(king_square))
                        }
                    }
                    Piece::Bishop => {
                        self.get_bishop_moves(king_square as u64, position.main_bitboard)
                            & self.get_bishop_moves(checker_square as u64, position.main_bitboard)
                    }
                    Piece::Queen => {
                        if piece_file == king_file {
                            self.get_queen_moves(king_square as u64, position.main_bitboard)
                                & BitBoard(get_file(king_square))
                        } else {
                            self.get_queen_moves(king_square as u64, position.main_bitboard)
                                & self
                                    .get_queen_moves(checker_square as u64, position.main_bitboard)
                        }
                    }
                    _ => BitBoard(0),
                };
                if let Some(p) = piece {
                    match p {
                        Piece::Pawn => {
                            let mut pawn_pushes = self.pawn_pushes[side as usize][square as usize];
                            pawn_pushes = pawn_pushes & push_mask;
                            self.create_moves(position, p, side, pawn_pushes, square, &mut moves);
                        }
                        Piece::Knight => {
                            let mut knight_moves = self.knight_moves[square as usize];
                            knight_moves =
                                (knight_moves & capture_mask) | (knight_moves & push_mask);

                            self.create_moves(position, p, side, knight_moves, square, &mut moves);
                        }

                        Piece::Queen => {
                            let mut queen_moves =
                                self.get_queen_moves(square as u64, position.main_bitboard);
                            queen_moves = (queen_moves & capture_mask) | (queen_moves & push_mask);

                            self.create_moves(position, p, side, queen_moves, square, &mut moves);
                        }

                        Piece::Rook => {
                            let mut rook_moves =
                                self.get_rook_moves(square as u64, position.main_bitboard);
                            rook_moves = (rook_moves & capture_mask) | (rook_moves & push_mask);

                            self.create_moves(position, p, side, rook_moves, square, &mut moves);
                        }
                        Piece::Bishop => {
                            let mut bishop_moves =
                                self.get_bishop_moves(square as u64, position.main_bitboard);
                            bishop_moves =
                                (bishop_moves & capture_mask) | (bishop_moves & push_mask);
                            self.create_moves(position, p, side, bishop_moves, square, &mut moves);
                        }

                        Piece::King => {
                            let mut king_moves = self.king_moves[square as usize];
                            king_moves = king_moves & capture_mask;
                            self.create_moves(position, p, side, king_moves, square, &mut moves);
                        }
                    }
                }
            } else {
                if let Some(p) = piece {
                    match p {
                        Piece::Pawn => {
                            let mut pawn_pushes = self.pawn_pushes[side as usize][square as usize];
                            let bitboard = pin_bitboard;

                            if is_pinned && bitboard.get_bit(square as u64) == 1 {
                                self.moves_to_pins_moves(
                                    position,
                                    p,
                                    &mut pawn_pushes,
                                    &pins,
                                    &pin_bitboard,
                                );
                                pawn_pushes = EMPTY_BITBOARD;
                            }
                            self.create_moves(position, p, side, pawn_pushes, square, &mut moves);
                        }
                        Piece::Knight => {
                            let mut knight_moves = self.knight_moves[square as usize];
                            if is_pinned {
                                self.moves_to_pins_moves(
                                    position,
                                    p,
                                    &mut knight_moves,
                                    &pins,
                                    &pin_bitboard,
                                );
                            }
                            self.create_moves(position, p, side, knight_moves, square, &mut moves);
                        }

                        Piece::Queen => {
                            let mut queen_moves =
                                self.get_queen_moves(square as u64, position.main_bitboard);
                            if is_pinned {
                                self.moves_to_pins_moves(
                                    position,
                                    p,
                                    &mut queen_moves,
                                    &pins,
                                    &pin_bitboard,
                                );
                            }
                            self.create_moves(position, p, side, queen_moves, square, &mut moves);
                        }

                        Piece::Rook => {
                            let mut rook_moves =
                                self.get_rook_moves(square as u64, position.main_bitboard);
                            if is_pinned {
                                self.moves_to_pins_moves(
                                    position,
                                    p,
                                    &mut rook_moves,
                                    &pins,
                                    &pin_bitboard,
                                );
                            }
                            self.create_moves(position, p, side, rook_moves, square, &mut moves);
                        }
                        Piece::Bishop => {
                            let mut bishop_moves =
                                self.get_bishop_moves(square as u64, position.main_bitboard);
                            if is_pinned {
                                self.moves_to_pins_moves(
                                    position,
                                    p,
                                    &mut bishop_moves,
                                    &pins,
                                    &pin_bitboard,
                                );
                            }
                            self.create_moves(position, p, side, bishop_moves, square, &mut moves);
                        }

                        Piece::King => {
                            let king_moves = self.king_moves[square as usize];
                            self.create_moves(position, p, side, king_moves, square, &mut moves);
                        }
                    }
                }
            }
        }
        moves
    }
}
