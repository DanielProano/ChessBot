use crate::pieces::*;
use crate::utils::*;

pub const KNIGHT_DELTAS: [(i32, i32); 8] = [
    (2, 1),
    (2, -1),
    (1, 2),
    (1, -2),
    (-2, 1),
    (-2, -1),
    (-1, 2),
    (-1, -2),
];

pub const BISHOP_DELTAS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
pub const ROOK_DELTAS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub const QUEEN_DELTAS: [(i32, i32); 8] = [
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
];

pub const KING_DELTAS: [(i32, i32); 8] = [
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
];

pub const EMPTY_MOVE: Move = Move {
    previous_square: Square {
        row: 0,
        column: 0,
        piece_state: None,
    },
    current_square: Square {
        row: 0,
        column: 0,
        piece_state: None,
    },
    color: Color::White,
    captured_piece: None,
    promotion: None,
    castling: false,
};

// Previous square holds the previous state
// otherwise every previous square would be empty
// and thats not helpful
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    pub previous_square: Square,
    pub current_square: Square,
    pub color: Color,
    pub captured_piece: Option<PieceState>,
    pub promotion: Option<Piece>,
    pub castling: bool,
}

impl Move {
    pub fn validate(&self) -> bool {
        if self.current_square.row > 8
            || self.current_square.row < 1
            || self.current_square.column > 8
            || self.current_square.column < 1
        {
            return false;
        }

        if self.previous_square.row > 8
            || self.previous_square.row < 1
            || self.previous_square.column > 8
            || self.previous_square.column < 1
        {
            return false;
        }

        if self.previous_square == self.current_square {
            return false;
        }

        if self.previous_square.piece_state == None {
            return false;
        }

        if self.current_square.piece_state == None {
            return false;
        }

        if let (Some(prev), Some(cur)) = (
            self.previous_square.piece_state,
            self.current_square.piece_state,
        ) {
            if prev.color != cur.color || prev.color != self.color || cur.color != self.color {
                return false;
            }
        }

        if let Some(captured) = self.captured_piece {
            if captured.color == self.color {
                return false;
            }
        }

        if self.promotion.is_some() && self.castling {
            return false;
        }

        if let (Some(cur_state), Some(prev_state)) = (
            self.current_square.piece_state,
            self.previous_square.piece_state,
        ) {
            if self.castling && (cur_state.piece != Piece::King || prev_state.piece != Piece::King)
            {
                return false;
            }
        }

        true
    }
}

pub fn create(
    mut state: PieceState,
    prev_square: Square,
    cur_square: Square,
    color: Color,
    captured_piece: Option<PieceState>,
    promotion: Option<Piece>,
    castling: bool,
) -> Option<Move> {
    state.has_moved = true;

    let mv = Move {
        previous_square: prev_square,
        current_square: cur_square,
        color: color,
        captured_piece: captured_piece,
        promotion: promotion,
        castling: castling,
    };

    if !mv.validate() {
        return None;
    }

    return Some(mv);
}

pub fn get_pawn_moves(
    state: PieceState,
    board: &Board,
    board_state: BoardState,
    check_mask: &CheckMask,
) -> Vec<Move> {
    if state.piece != Piece::Pawn {
        println!("Warning: State is not representing a pawn");
        return vec![];
    }

    match state.color {
        Color::White => get_white_pawn_moves(state, board, board_state, check_mask),
        Color::Black => get_black_pawn_moves(state, board, board_state, check_mask),
    }
}

pub fn get_white_pawn_moves(
    state: PieceState,
    board: &Board,
    mut board_state: BoardState,
    check_mask: &CheckMask,
) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let cur_row: usize = state.location.0;
    let cur_col: usize = state.location.1;

    if cur_row == 2 {
        if let (Some(square), Some(inbetween_square), Some(target_square)) = (
            access_board(board, cur_row, cur_col),
            access_board(board, cur_row + 1, cur_col),
            access_board(board, cur_row + 2, cur_col),
        ) {
            if check_mask.check_mask[target_square.row - 1][target_square.column - 1] {
                if inbetween_square.piece_state.is_none() && target_square.piece_state.is_none() {
                    add_pawn_move(&mut moves, state, square, cur_row + 2, cur_col, None);
                    board_state.white_state.en_passant = Some(target_square);
                }
            }
        }
    }

    if cur_row + 1 <= 8 {
        if let (Some(square), Some(target_square)) = (
            access_board(board, cur_row, cur_col),
            access_board(board, cur_row + 1, cur_col),
        ) {
            if check_mask.check_mask[target_square.row - 1][target_square.column - 1] {
                if target_square.piece_state.is_none() {
                    add_pawn_move(&mut moves, state, square, cur_row + 1, cur_col, None);
                    board_state.white_state.en_passant = None;
                }
            }
        }
    }

    if cur_row < 8 && cur_col > 1 {
        if let (Some(square), Some(target_square)) = (
            access_board(board, cur_row, cur_col),
            access_board(board, cur_row + 1, cur_col - 1),
        ) {
            if check_mask.check_mask[target_square.row - 1][target_square.column - 1] {
                if let Some(target_piece) = target_square.piece_state {
                    //Normal capture

                    add_pawn_move(
                        &mut moves,
                        state,
                        square,
                        cur_row + 1,
                        cur_col - 1,
                        Some(target_piece),
                    );
                    board_state.white_state.en_passant = None;
                } else if target_square.piece_state.is_none() {
                    //Special en passant

                    if let (Some(enemy_square), Some(special_square)) = (
                        board_state.black_state.en_passant,
                        access_board(board, cur_row, cur_col - 1),
                    ) {
                        if let Some(target_piece) = special_square.piece_state {
                            if enemy_square.row == special_square.row && enemy_square.column == special_square.column {
                                add_pawn_move(
                                    &mut moves,
                                    state,
                                    square,
                                    cur_row + 1,
                                    cur_col - 1,
                                    Some(target_piece),
                                );
                                board_state.black_state.en_passant = None;
                            }
                        }
                    }
                }
            }
        }
    }

    if cur_row < 8 && cur_col < 8 {
        if let (Some(square), Some(target_square)) = (
            access_board(board, cur_row, cur_col),
            access_board(board, cur_row + 1, cur_col + 1),
        ) {
            if check_mask.check_mask[target_square.row - 1][target_square.column - 1] {
                if let Some(target_piece) = target_square.piece_state {
                    add_pawn_move(
                        &mut moves,
                        state,
                        square,
                        cur_row + 1,
                        cur_col + 1,
                        Some(target_piece),
                    );
                    board_state.white_state.en_passant = None;
                } else if target_square.piece_state.is_none() {
                    //Special en passant

                    if let (Some(enemy_square), Some(special_square)) = (
                        board_state.black_state.en_passant,
                        access_board(board, cur_row, cur_col + 1),
                    ) {
                        if let Some(target_piece) = special_square.piece_state {
                            if enemy_square.row == special_square.row && enemy_square.column == special_square.column {
                                add_pawn_move(
                                    &mut moves,
                                    state,
                                    square,
                                    cur_row + 1,
                                    cur_col + 1,
                                    Some(target_piece),
                                );
                                board_state.black_state.en_passant = None;
                            }
                        }
                    }
                }
            }
        }
    }

    moves
}

pub fn get_black_pawn_moves(
    state: PieceState,
    board: &Board,
    mut board_state: BoardState,
    check_mask: &CheckMask,
) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let cur_row: usize = state.location.0;
    let cur_col: usize = state.location.1;

    if cur_row == 7 {
        if let (Some(square), Some(inbetween_square), Some(target_square)) = (
            access_board(board, cur_row, cur_col),
            access_board(board, cur_row - 1, cur_col),
            access_board(board, cur_row - 2, cur_col),
        ) {
            if check_mask.check_mask[target_square.row - 1][target_square.column - 1] {
                if inbetween_square.piece_state.is_none() && target_square.piece_state.is_none() {
                    add_pawn_move(&mut moves, state, square, cur_row - 2, cur_col, None);
                    board_state.black_state.en_passant = Some(target_square);
                }
            }
        }
    }

    if cur_row - 1 >= 1 {
        if let (Some(square), Some(target_square)) = (
            access_board(board, cur_row, cur_col),
            access_board(board, cur_row - 1, cur_col),
        ) {
            if check_mask.check_mask[target_square.row - 1][target_square.column - 1] {
                if target_square.piece_state.is_none() {
                    add_pawn_move(&mut moves, state, square, cur_row - 1, cur_col, None);
                    board_state.black_state.en_passant = None;
                }
            }
        }
    }

    if cur_row > 1 && cur_col > 1 {
        if let (Some(square), Some(target_square)) = (
            access_board(board, cur_row, cur_col),
            access_board(board, cur_row - 1, cur_col - 1),
        ) {
            if check_mask.check_mask[target_square.row - 1][target_square.column - 1] {
                if let Some(target_piece) = target_square.piece_state {
                    //Normal pawn capture

                    add_pawn_move(
                        &mut moves,
                        state,
                        square,
                        cur_row - 1,
                        cur_col - 1,
                        Some(target_piece),
                    );
                    board_state.black_state.en_passant = None;
                } else if target_square.piece_state.is_none() {
                    //Special en passant

                    if let (Some(enemy_square), Some(special_square)) = (
                        board_state.white_state.en_passant,
                        access_board(board, cur_row, cur_col - 1),
                    ) {
                        if let Some(target_piece) = special_square.piece_state {
                            if enemy_square.row == special_square.row && enemy_square.column == special_square.column {
                                add_pawn_move(
                                    &mut moves,
                                    state,
                                    square,
                                    cur_row - 1,
                                    cur_col - 1,
                                    Some(target_piece),
                                );
                                board_state.black_state.en_passant = None;
                            }
                        }
                    }
                }
            }
        }
    }

    if cur_row > 1 && cur_col < 8 {
        if let (Some(square), Some(target_square)) = (
            access_board(board, cur_row, cur_col),
            access_board(board, cur_row - 1, cur_col + 1),
        ) {
            if check_mask.check_mask[target_square.row - 1][target_square.column - 1] {
                if let Some(target_piece) = target_square.piece_state {
                    //Normal pawn capture

                    add_pawn_move(
                        &mut moves,
                        state,
                        square,
                        cur_row - 1,
                        cur_col + 1,
                        Some(target_piece),
                    );
                    board_state.black_state.en_passant = None;
                } else if target_square.piece_state.is_none() {
                    //Special en passant

                    if let (Some(enemy_square), Some(special_square)) = (
                        board_state.white_state.en_passant,
                        access_board(board, cur_row, cur_col + 1),
                    ) {
                        if let Some(target_piece) = special_square.piece_state {
                            if enemy_square.row == special_square.row && enemy_square.column == special_square.column {
                                debug_assert_eq!(
                                    target_piece.location,
                                    (special_square.row, special_square.column),
                                    "piece location mismatch on board"
                                );
                                add_pawn_move(
                                    &mut moves,
                                    state,
                                    square,
                                    cur_row - 1,
                                    cur_col + 1,
                                    Some(target_piece),
                                );
                                board_state.black_state.en_passant = None;
                            }
                        }
                    }
                }
            }
        }
    }

    moves
}

fn add_pawn_move(
    moves: &mut Vec<Move>,
    mut state: PieceState,
    prev_square: Square,
    target_row: usize,
    target_col: usize,
    mut captured: Option<PieceState>,
) {
    state.has_moved = true;
    state.location = (target_row, target_col);
    if let Some(ref mut cap) = captured {
        cap.dead = true;
    }

    if let Some(mv) = create(
        state,
        prev_square,
        Square {
            row: target_row,
            column: target_col,
            piece_state: Some(state),
        },
        state.color,
        captured,
        None,
        false,
    ) {
        let mut final_move = mv;

        match state.color {
            Color::White => {
                if target_row == 8 {
                    let promotion_pieces =
                        [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight];
                    for &promotion in &promotion_pieces {
                        final_move.promotion = Some(promotion);
                        moves.push(final_move.clone());
                    }
                } else {
                    moves.push(final_move.clone());
                }
            }
            Color::Black => {
                if target_row == 1 {
                    let promotion_pieces =
                        [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight];
                    for &promotion in &promotion_pieces {
                        final_move.promotion = Some(promotion);
                        moves.push(final_move.clone());
                    }
                } else {
                    moves.push(final_move.clone());
                }
            }
        }
    }
}

pub fn get_knight_moves(state: PieceState, board: &Board, check_mask: &CheckMask) -> Vec<Move> {
    if state.piece != Piece::Knight {
        println!("Warning: State is not representing a Knight");
        return vec![];
    }

    let mut moves: Vec<Move> = vec![];
    let cur_row = state.location.0;
    let cur_col = state.location.1;

    for &(row, col) in &KNIGHT_DELTAS {
        let target_row: usize = (cur_row as i32 + row) as usize;
        let target_col: usize = (cur_col as i32 + col) as usize;

        if !in_bounds(target_row, target_col) {
            continue;
        }

        if !check_mask.check_mask[target_row - 1][target_col - 1] {
            continue;
        }

        if let (Some(previous_square), Some(target_square)) = (
            access_board(board, cur_row, cur_col),
            access_board(board, target_row, target_col),
        ) {
            let potential_piece: Option<PieceState> = target_square.piece_state;

            match potential_piece {
                Some(mut captured) if captured.color != state.color => {
                    captured.dead = true;
                    if let Some(mv) = create(
                        state,
                        previous_square,
                        Square {
                            row: target_row,
                            column: target_col,
                            piece_state: Some(state),
                        },
                        state.color,
                        Some(captured),
                        None,
                        false,
                    ) {
                        moves.push(mv);
                    }
                }
                None => {
                    if let Some(mv) = create(
                        state,
                        previous_square,
                        Square {
                            row: target_row,
                            column: target_col,
                            piece_state: Some(state),
                        },
                        state.color,
                        None,
                        None,
                        false,
                    ) {
                        moves.push(mv);
                    }
                }
                _ => {}
            }
        }
    }

    moves
}

pub fn get_bishop_moves(state: PieceState, board: &Board, check_mask: &CheckMask) -> Vec<Move> {
    if state.piece != Piece::Bishop {
        println!("Warning: State does not represent a Bishop");
        return vec![];
    }

    let mut moves: Vec<Move> = vec![];
    let cur_row = state.location.0;
    let cur_col = state.location.1;

    for &(dr, dc) in &BISHOP_DELTAS {
        for offset in 1..8 {
            let target_row = (cur_row as i32 + dr * offset) as usize;
            let target_col = (cur_col as i32 + dc * offset) as usize;

            if !in_bounds(target_row, target_col) {
                break;
            }

            if !check_mask.check_mask[target_row - 1][target_col - 1] {
                continue;
            }

            if let (Some(previous_square), Some(target_square)) = (
                access_board(board, cur_row, cur_col),
                access_board(board, target_row, target_col),
            ) {
                if let Some(captured) = target_square.piece_state {
                    if captured.color != state.color {
                        if let Some(mv) = create(
                            state,
                            previous_square,
                            Square {
                                row: target_row,
                                column: target_col,
                                piece_state: Some(state),
                            },
                            state.color,
                            Some(captured),
                            None,
                            false,
                        ) {
                            moves.push(mv);
                        }
                    }
                    break;
                } else {
                    if let Some(mv) = create(
                        state,
                        previous_square,
                        Square {
                            row: target_row,
                            column: target_col,
                            piece_state: Some(state),
                        },
                        state.color,
                        None,
                        None,
                        false,
                    ) {
                        moves.push(mv);
                    }
                }
            }
        }
    }

    moves
}

pub fn get_rook_moves(state: PieceState, board: &Board, check_mask: &CheckMask) -> Vec<Move> {
    if state.piece != Piece::Rook {
        println!("Warning: State does not represent a Rook");
        return vec![];
    }

    let mut moves: Vec<Move> = vec![];
    let cur_row = state.location.0;
    let cur_col = state.location.1;

    for &(dr, dc) in &ROOK_DELTAS {
        for offset in 1..8 {
            let target_row = (cur_row as i32 + dr * offset) as usize;
            let target_col = (cur_col as i32 + dc * offset) as usize;

            if !in_bounds(target_row, target_col) {
                break;
            }

            if !check_mask.check_mask[target_row - 1][target_col - 1] {
                continue;
            }

            if let (Some(previous_square), Some(target_square)) = (
                access_board(board, cur_row, cur_col),
                access_board(board, target_row, target_col),
            ) {
                if let Some(captured) = target_square.piece_state {
                    if captured.color != state.color {
                        if let Some(mv) = create(
                            state,
                            previous_square,
                            Square {
                                row: target_row,
                                column: target_col,
                                piece_state: Some(state),
                            },
                            state.color,
                            Some(captured),
                            None,
                            false,
                        ) {
                            moves.push(mv);
                        }
                    }
                    break;
                } else {
                    if let Some(mv) = create(
                        state,
                        previous_square,
                        Square {
                            row: target_row,
                            column: target_col,
                            piece_state: Some(state),
                        },
                        state.color,
                        None,
                        None,
                        false,
                    ) {
                        moves.push(mv);
                    }
                }
            }
        }
    }

    moves
}

pub fn get_queen_moves(state: PieceState, board: &Board, check_mask: &CheckMask) -> Vec<Move> {
    if state.piece != Piece::Queen {
        println!("Warning: State pieces misaligned");
        return vec![];
    }

    let mut moves: Vec<Move> = vec![];
    let cur_row = state.location.0;
    let cur_col = state.location.1;

    for &(dr, dc) in &QUEEN_DELTAS {
        for offset in 1..8 {
            let target_row = (cur_row as i32 + dr * offset) as usize;
            let target_col = (cur_col as i32 + dc * offset) as usize;

            if !in_bounds(target_row, target_col) {
                break;
            }

            if !check_mask.check_mask[target_row - 1][target_col - 1] {
                continue;
            }

            if let (Some(previous_square), Some(target_square)) = (
                access_board(board, cur_row, cur_col),
                access_board(board, target_row, target_col),
            ) {
                if let Some(piece_state) = target_square.piece_state {
                    if piece_state.color != state.color {
                        if let Some(mv) = create(
                            state,
                            previous_square,
                            Square {
                                row: target_row,
                                column: target_col,
                                piece_state: Some(state),
                            },
                            state.color,
                            Some(piece_state),
                            None,
                            false,
                        ) {
                            moves.push(mv);
                        }
                    }
                    break;
                } else {
                    if let Some(mv) = create(
                        state,
                        previous_square,
                        Square {
                            row: target_row,
                            column: target_col,
                            piece_state: Some(state),
                        },
                        state.color,
                        None,
                        None,
                        false,
                    ) {
                        moves.push(mv);
                    }
                }
            }
        }
    }

    moves
}

pub fn get_king_moves(state: PieceState, board: &Board, check_mask: &CheckMask) -> Vec<Move> {
    if state.piece != Piece::King {
        println!("Warning: State pieces misaligned");
        return vec![];
    }

    let mut moves: Vec<Move> = vec![];
    let cur_row = state.location.0;
    let cur_col = state.location.1;

    //Normal King moves
    for &(row_offset, col_offset) in &KING_DELTAS {
        let target_row = (cur_row as i32 + row_offset) as usize;
        let target_col = (cur_col as i32 + col_offset) as usize;

        if !in_bounds(target_row, target_col) {
            continue;
        }

        if !check_mask.check_mask[target_row - 1][target_col - 1] {
            continue;
        }

        if let (Some(previous_square), Some(target_square)) = (
            access_board(board, cur_row, cur_col),
            access_board(board, target_row, target_col),
        ) {
            if square_is_attacked(target_square, state.color, board, None) {
                continue;
            }

            if let Some(captured) = target_square.piece_state {
                if captured.color != state.color {
                    if let Some(mv) = create(
                        state,
                        previous_square,
                        Square {
                            row: target_row,
                            column: target_col,
                            piece_state: Some(state),
                        },
                        state.color,
                        Some(captured),
                        None,
                        false,
                    ) {
                        moves.push(mv);
                    }
                }
                continue;
            } else {
                if let Some(mv) = create(
                    state,
                    previous_square,
                    Square {
                        row: target_row,
                        column: target_col,
                        piece_state: Some(state),
                    },
                    state.color,
                    None,
                    None,
                    false,
                ) {
                    moves.push(mv);
                }
            }
        }
    }

    // Castling moves
    if !state.has_moved {
        if let Some(previous_square) = access_board(board, cur_row, cur_col) {
            let mut target_king_square = Square {
                row: cur_row as usize,
                column: 7,
                piece_state: Some(state),
            };
            let king_square = Square {
                row: cur_row as usize,
                column: 5,
                piece_state: Some(state),
            };

            let king_side_transit_square = Square {
                row: cur_row as usize,
                column: 6,
                piece_state: Some(state),
            };
            if !square_is_attacked(target_king_square, state.color, board, None)
                && !square_is_attacked(king_side_transit_square, state.color, board, None)
                && !square_is_attacked(king_square, state.color, board, None)
            {
                if is_kingside_castling_valid(state, board, cur_row) {
                    if let Some(mv) = create(
                        state,
                        previous_square,
                        target_king_square,
                        state.color,
                        None,
                        None,
                        true,
                    ) {
                        moves.push(mv);
                    }
                }
            }

            target_king_square = Square {
                row: cur_row as usize,
                column: 3,
                piece_state: Some(state),
            };

            let queen_side_transit_square = Square {
                row: cur_row as usize,
                column: 4,
                piece_state: Some(state),
            };
            if !square_is_attacked(target_king_square, state.color, board, None)
                && !square_is_attacked(queen_side_transit_square, state.color, board, None)
                && !square_is_attacked(king_square, state.color, board, None)
            {
                if is_queenside_castling_valid(state, board, cur_row as usize) {
                    if let Some(mv) = create(
                        state,
                        previous_square,
                        target_king_square,
                        state.color,
                        None,
                        None,
                        true,
                    ) {
                        moves.push(mv);
                    }
                }
            }
        }
    }

    moves
}

fn is_kingside_castling_valid(king: PieceState, board: &Board, row: usize) -> bool {
    if let Some(rook) = board.board[row - 1][7].piece_state {
        if rook.piece == Piece::Rook
            && rook.color == king.color
            && !rook.has_moved
            && !king.has_moved
        {
            return board.board[row - 1][5].piece_state.is_none()
                && board.board[row - 1][6].piece_state.is_none();
        }
    }

    false
}

fn is_queenside_castling_valid(king: PieceState, board: &Board, row: usize) -> bool {
    if let Some(rook) = board.board[row - 1][0].piece_state {
        if rook.piece == Piece::Rook
            && rook.color == king.color
            && !rook.has_moved
            && !king.has_moved
        {
            return board.board[row - 1][1].piece_state.is_none()
                && board.board[row - 1][2].piece_state.is_none()
                && board.board[row - 1][3].piece_state.is_none();
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pieces::*;

    fn empty_board() -> Board {
        EMPTY_BOARD
    }

    fn place_piece(board: &mut Board, row: usize, col: usize, piece: Piece, color: Color) {
        board.board[row - 1][col - 1] = Square {
            row,
            column: col,
            piece_state: Some(PieceState {
                color,
                piece,
                location: (row, col),
                has_moved: false,
                dead: false,
            }),
        };
    }

    fn empty_board_state() -> BoardState {
        BoardState {
            board: EMPTY_BOARD,
            active_color: Color::White,
            white_state: ColorState {
                color: Color::White,
                in_check: false,
                en_passant: None,
                castling: CastlingRights {
                    castle_kingside: false,
                    castle_queenside: false,
                },
            },
            black_state: ColorState {
                color: Color::Black,
                in_check: false,
                en_passant: None,
                castling: CastlingRights {
                    castle_kingside: false,
                    castle_queenside: false,
                },
            },
            draw: DrawConditions {
                draw: false,
                fifty_move_counter: 0,
                threefold_counter: 0,
            },
            time: None,
        }
    }

    fn full_check_mask() -> CheckMask {
        CheckMask {
            check_mask: [[true; 8]; 8],
        }
    }

    // ---- validate ----

    #[test]
    fn test_validate_rejects_out_of_bounds() {
        let mv = Move {
            previous_square: Square {
                row: 0,
                column: 1,
                piece_state: Some(PieceState {
                    color: Color::White,
                    piece: Piece::Pawn,
                    location: (0, 1),
                    has_moved: false,
                    dead: false,
                }),
            },
            current_square: Square {
                row: 1,
                column: 1,
                piece_state: Some(PieceState {
                    color: Color::White,
                    piece: Piece::Pawn,
                    location: (1, 1),
                    has_moved: false,
                    dead: false,
                }),
            },
            color: Color::White,
            captured_piece: None,
            promotion: None,
            castling: false,
        };
        assert!(!mv.validate());
    }

    #[test]
    fn test_validate_rejects_same_square() {
        let square = Square {
            row: 4,
            column: 4,
            piece_state: Some(PieceState {
                color: Color::White,
                piece: Piece::Pawn,
                location: (4, 4),
                has_moved: false,
                dead: false,
            }),
        };
        let mv = Move {
            previous_square: square,
            current_square: square,
            color: Color::White,
            captured_piece: None,
            promotion: None,
            castling: false,
        };
        assert!(!mv.validate());
    }

    #[test]
    fn test_validate_rejects_missing_piece_on_origin() {
        let mv = Move {
            previous_square: Square {
                row: 2,
                column: 1,
                piece_state: None,
            },
            current_square: Square {
                row: 3,
                column: 1,
                piece_state: Some(PieceState {
                    color: Color::White,
                    piece: Piece::Pawn,
                    location: (3, 1),
                    has_moved: true,
                    dead: false,
                }),
            },
            color: Color::White,
            captured_piece: None,
            promotion: None,
            castling: false,
        };
        assert!(!mv.validate());
    }

    // ---- white pawn moves ----

    #[test]
    fn test_white_pawn_single_push() {
        let mut board = empty_board();
        place_piece(&mut board, 2, 4, Piece::Pawn, Color::White);
        let state = board.board[1][3].piece_state.unwrap();
        let moves = get_white_pawn_moves(state, &board, empty_board_state(), &full_check_mask());
        assert!(
            moves
                .iter()
                .any(|m| m.current_square.row == 3 && m.current_square.column == 4)
        );
    }

    #[test]
    fn test_white_pawn_double_push_from_rank_2() {
        let mut board = empty_board();
        place_piece(&mut board, 2, 4, Piece::Pawn, Color::White);
        let state = board.board[1][3].piece_state.unwrap();
        let moves = get_white_pawn_moves(state, &board, empty_board_state(), &full_check_mask());
        assert!(
            moves
                .iter()
                .any(|m| m.current_square.row == 4 && m.current_square.column == 4)
        );
    }

    #[test]
    fn test_white_pawn_blocked() {
        let mut board = empty_board();
        place_piece(&mut board, 2, 4, Piece::Pawn, Color::White);
        place_piece(&mut board, 3, 4, Piece::Pawn, Color::Black);
        let state = board.board[1][3].piece_state.unwrap();
        let moves = get_white_pawn_moves(state, &board, empty_board_state(), &full_check_mask());
        assert!(moves.is_empty());
    }

    #[test]
    fn test_white_pawn_capture() {
        let mut board = empty_board();
        place_piece(&mut board, 2, 4, Piece::Pawn, Color::White);
        place_piece(&mut board, 3, 5, Piece::Pawn, Color::Black);
        let state = board.board[1][3].piece_state.unwrap();
        let moves = get_white_pawn_moves(state, &board, empty_board_state(), &full_check_mask());
        assert!(moves.iter().any(|m| m.current_square.row == 3
            && m.current_square.column == 5
            && m.captured_piece.is_some()));
    }

    #[test]
    fn test_white_pawn_promotion() {
        let mut board = empty_board();
        place_piece(&mut board, 7, 4, Piece::Pawn, Color::White);
        let state = board.board[6][3].piece_state.unwrap();
        let moves = get_white_pawn_moves(state, &board, empty_board_state(), &full_check_mask());
        assert_eq!(moves.iter().filter(|m| m.promotion.is_some()).count(), 4);
    }

    #[test]
    fn test_black_pawn_single_push() {
        let mut board = empty_board();
        place_piece(&mut board, 7, 4, Piece::Pawn, Color::Black);
        let state = board.board[6][3].piece_state.unwrap();
        let moves = get_black_pawn_moves(state, &board, empty_board_state(), &full_check_mask());
        assert!(
            moves
                .iter()
                .any(|m| m.current_square.row == 6 && m.current_square.column == 4)
        );
    }

    #[test]
    fn test_black_pawn_double_push_from_rank_7() {
        let mut board = empty_board();
        place_piece(&mut board, 7, 4, Piece::Pawn, Color::Black);
        let state = board.board[6][3].piece_state.unwrap();
        let moves = get_black_pawn_moves(state, &board, empty_board_state(), &full_check_mask());
        assert!(
            moves
                .iter()
                .any(|m| m.current_square.row == 5 && m.current_square.column == 4)
        );
    }

    #[test]
    fn test_black_pawn_capture() {
        let mut board = empty_board();
        place_piece(&mut board, 7, 4, Piece::Pawn, Color::Black);
        place_piece(&mut board, 6, 5, Piece::Pawn, Color::White);
        let state = board.board[6][3].piece_state.unwrap();
        let moves = get_black_pawn_moves(state, &board, empty_board_state(), &full_check_mask());
        assert!(moves.iter().any(|m| m.current_square.row == 6
            && m.current_square.column == 5
            && m.captured_piece.is_some()));
    }

    //knight moves

    #[test]
    fn test_knight_moves_from_center() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Knight, Color::White);
        let state = board.board[3][3].piece_state.unwrap();
        let mask = full_check_mask();
        let moves = get_knight_moves(state, &board, &mask);
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn test_knight_moves_from_corner() {
        let mut board = empty_board();
        place_piece(&mut board, 1, 1, Piece::Knight, Color::White);
        let state = board.board[0][0].piece_state.unwrap();
        let mask = full_check_mask();
        let moves = get_knight_moves(state, &board, &mask);
        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn test_knight_cannot_capture_own_piece() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Knight, Color::White);
        place_piece(&mut board, 6, 5, Piece::Pawn, Color::White);
        let state = board.board[3][3].piece_state.unwrap();
        let mask = full_check_mask();
        let moves = get_knight_moves(state, &board, &mask);
        assert!(
            moves
                .iter()
                .all(|m| !(m.current_square.row == 6 && m.current_square.column == 5))
        );
    }

    #[test]
    fn test_knight_can_capture_enemy() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Knight, Color::White);
        place_piece(&mut board, 6, 5, Piece::Pawn, Color::Black);
        let state = board.board[3][3].piece_state.unwrap();
        let mask = full_check_mask();
        let moves = get_knight_moves(state, &board, &mask);
        assert!(moves.iter().any(|m| m.current_square.row == 6
            && m.current_square.column == 5
            && m.captured_piece.is_some()));
    }

    // ---- bishop moves ----

    #[test]
    fn test_bishop_moves_open_board() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Bishop, Color::White);
        let state = board.board[3][3].piece_state.unwrap();
        let mask = full_check_mask();
        let moves = get_bishop_moves(state, &board, &mask);
        assert_eq!(moves.len(), 13);
    }

    #[test]
    fn test_bishop_blocked_by_own_piece() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Bishop, Color::White);
        place_piece(&mut board, 5, 5, Piece::Pawn, Color::White);
        let state = board.board[3][3].piece_state.unwrap();
        let mask = full_check_mask();
        let moves = get_bishop_moves(state, &board, &mask);
        assert!(
            moves
                .iter()
                .all(|m| !(m.current_square.row == 5 && m.current_square.column == 5))
        );
    }

    #[test]
    fn test_bishop_captures_enemy_and_stops() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Bishop, Color::White);
        place_piece(&mut board, 6, 6, Piece::Pawn, Color::Black);
        let state = board.board[3][3].piece_state.unwrap();
        let mask = full_check_mask();
        let moves = get_bishop_moves(state, &board, &mask);
        assert!(moves.iter().any(|m| m.current_square.row == 6
            && m.current_square.column == 6
            && m.captured_piece.is_some()));
        assert!(
            moves
                .iter()
                .all(|m| !(m.current_square.row == 7 && m.current_square.column == 7))
        );
    }

    // ---- rook moves ----

    #[test]
    fn test_rook_moves_open_board() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Rook, Color::White);
        let state = board.board[3][3].piece_state.unwrap();
        let mask = full_check_mask();
        let moves = get_rook_moves(state, &board, &mask);
        assert_eq!(moves.len(), 14);
    }

    #[test]
    fn test_rook_blocked_by_own_piece() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Rook, Color::White);
        place_piece(&mut board, 4, 6, Piece::Pawn, Color::White);
        let state = board.board[3][3].piece_state.unwrap();
        let mask = full_check_mask();
        let moves = get_rook_moves(state, &board, &mask);
        assert!(
            moves
                .iter()
                .all(|m| m.current_square.column < 6 || m.current_square.row != 4)
        );
    }

    #[test]
    fn test_rook_captures_enemy_and_stops() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Rook, Color::White);
        place_piece(&mut board, 4, 6, Piece::Pawn, Color::Black);
        let state = board.board[3][3].piece_state.unwrap();
        let mask = full_check_mask();
        let moves = get_rook_moves(state, &board, &mask);
        assert!(moves.iter().any(|m| m.current_square.row == 4
            && m.current_square.column == 6
            && m.captured_piece.is_some()));
        assert!(
            moves
                .iter()
                .all(|m| !(m.current_square.row == 4 && m.current_square.column == 7))
        );
    }

    // ---- queen moves ----

    #[test]
    fn test_queen_moves_open_board() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Queen, Color::White);
        let state = board.board[3][3].piece_state.unwrap();
        let mask = full_check_mask();
        let moves = get_queen_moves(state, &board, &mask);
        assert_eq!(moves.len(), 27);
    }

    #[test]
    fn test_queen_combines_rook_and_bishop() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Queen, Color::White);
        let state = board.board[3][3].piece_state.unwrap();
        let mask = full_check_mask();
        let moves = get_queen_moves(state, &board, &mask);
        // can reach same squares as rook + bishop from center
        assert!(
            moves
                .iter()
                .any(|m| m.current_square.row == 4 && m.current_square.column == 8)
        ); // rook direction
        assert!(
            moves
                .iter()
                .any(|m| m.current_square.row == 7 && m.current_square.column == 7)
        ); // bishop direction
    }

    // ---- king moves ----

    #[test]
    fn test_king_moves_from_center() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::King, Color::White);
        let state = board.board[3][3].piece_state.unwrap();
        let mask = full_check_mask();
        let moves = get_king_moves(state, &board, &mask);
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn test_king_cannot_capture_own_piece() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::King, Color::White);
        place_piece(&mut board, 5, 5, Piece::Pawn, Color::White);
        let state = board.board[3][3].piece_state.unwrap();
        let mask = full_check_mask();
        let moves = get_king_moves(state, &board, &mask);
        assert!(
            moves
                .iter()
                .all(|m| !(m.current_square.row == 5 && m.current_square.column == 5))
        );
    }

    #[test]
    fn test_king_moves_from_corner() {
        let mut board = empty_board();
        place_piece(&mut board, 1, 1, Piece::King, Color::White);
        let state = board.board[0][0].piece_state.unwrap();
        let mask = full_check_mask();
        let moves = get_king_moves(state, &board, &mask);
        assert_eq!(moves.len(), 3);
    }

    // ---- square_is_attacked ----

    #[test]
    fn test_square_attacked_by_knight() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Knight, Color::Black);
        let square = Square {
            row: 2,
            column: 3,
            piece_state: None,
        };
        let mut mask = CheckMask {
            check_mask: [[false; 8]; 8],
        };
        assert!(square_is_attacked(
            square,
            Color::White,
            &board,
            Some(&mut mask)
        ));
    }

    #[test]
    fn test_square_not_attacked() {
        let mut board = empty_board();
        place_piece(&mut board, 1, 1, Piece::Rook, Color::Black);
        let square = Square {
            row: 4,
            column: 4,
            piece_state: None,
        };
        let mut mask = CheckMask {
            check_mask: [[false; 8]; 8],
        };
        assert!(!square_is_attacked(
            square,
            Color::White,
            &board,
            Some(&mut mask)
        ));
    }

    #[test]
    fn test_square_attacked_by_rook() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 1, Piece::Rook, Color::Black);
        let square = Square {
            row: 4,
            column: 5,
            piece_state: None,
        };
        let mut mask = CheckMask {
            check_mask: [[false; 8]; 8],
        };
        assert!(square_is_attacked(
            square,
            Color::White,
            &board,
            Some(&mut mask)
        ));
    }

    #[test]
    fn test_square_attacked_by_bishop() {
        let mut board = empty_board();
        place_piece(&mut board, 1, 1, Piece::Bishop, Color::Black);
        let square = Square {
            row: 4,
            column: 4,
            piece_state: None,
        };
        let mut mask = CheckMask {
            check_mask: [[false; 8]; 8],
        };
        assert!(square_is_attacked(
            square,
            Color::White,
            &board,
            Some(&mut mask)
        ));
    }

    #[test]
    fn test_white_pawn_en_passant_left() {
        let mut board = empty_board();
        place_piece(&mut board, 5, 4, Piece::Pawn, Color::White);
        place_piece(&mut board, 5, 3, Piece::Pawn, Color::Black);

        let mut board_state = empty_board_state();
        // black just double pushed to col 3, row 5 — en passant target is row 6, col 3
        board_state.black_state.en_passant = Some(Square {
            row: 5,
            column: 3,
            piece_state: board.board[4][2].piece_state,
        });

        let state = board.board[4][3].piece_state.unwrap();
        let mask = full_check_mask();
        let moves = get_white_pawn_moves(state, &board, board_state, &mask);
        assert!(
            moves.iter().any(|m| m.current_square.row == 6
                && m.current_square.column == 3
                && m.captured_piece.is_some()),
            "white should be able to capture en passant to the left"
        );
    }

    #[test]
    fn test_black_pawn_en_passant_right() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Pawn, Color::Black);
        place_piece(&mut board, 4, 5, Piece::Pawn, Color::White);

        let mut board_state = empty_board_state();
        board_state.white_state.en_passant = Some(Square {
            row: 4,
            column: 5,
            piece_state: board.board[3][4].piece_state,
        });

        let state = board.board[3][3].piece_state.unwrap();
        let mask = full_check_mask();
        let moves = get_black_pawn_moves(state, &board, board_state, &mask);
        assert!(
            moves.iter().any(|m| m.current_square.row == 3
                && m.current_square.column == 5
                && m.captured_piece.is_some()),
            "black should be able to capture en passant to the right"
        );
    }

    #[test]
    fn test_queen_cannot_move_when_blocked_by_own_pawns() {
        // Starting position - queen is completely hemmed in by own pieces
        // Queen at d1 should have 0 moves
        let board = START_BOARD.clone();
        let board_state = empty_board_state();
        let mask = create_check_mask(&board, Color::White);
        let queen_state = board.board[0][3].piece_state.unwrap();
        let queen_moves = get_queen_moves(queen_state, &board, &mask);
        assert_eq!(queen_moves.len(), 0, "queen should have 0 moves in starting position, got {}", queen_moves.len());
    }

    #[test]
    fn test_bishop_cannot_move_when_blocked_by_own_pawns() {
        // Starting position - both bishops are completely hemmed in
        let board = START_BOARD.clone();
        let board_state = empty_board_state();
        let mask = create_check_mask(&board, Color::White);
        let bishop_state = board.board[0][5].piece_state.unwrap(); // f1 bishop
        let bishop_moves = get_bishop_moves(bishop_state, &board, &mask);
        assert_eq!(bishop_moves.len(), 0, "f1 bishop should have 0 moves in starting position, got {}", bishop_moves.len());
    }
    #[test]
    fn test_queen_moves_after_e4_only_diagonal() {
        let mut board = START_BOARD.clone();
        board.board[1][4].piece_state = None;
        board.board[3][4] = Square { row: 4, column: 5, piece_state: Some(PieceState {
            color: Color::White, piece: Piece::Pawn,
            location: (4, 5), has_moved: true, dead: false
        })};
        let board_state = empty_board_state();
        let mask = create_check_mask(&board, Color::White);
        let queen_state = board.board[0][3].piece_state.unwrap();
        let queen_moves = get_queen_moves(queen_state, &board, &mask);
        // Queen on d1 can slide diagonally through e2 to f3, g4, h5 (f2 pawn does NOT block this diagonal)
        // d1->e2->f3->g4->h5, stops at h5 (edge). 4 moves.
        assert_eq!(queen_moves.len(), 4, "queen should reach e2/f3/g4/h5, got {} moves: {:?}",
            queen_moves.len(),
            queen_moves.iter().map(|m| (m.current_square.row, m.current_square.column)).collect::<Vec<_>>());
    }

    #[test]
    fn test_bishop_f1_moves_after_e4_only_one_square() {
        let mut board = START_BOARD.clone();
        board.board[1][4].piece_state = None;
        board.board[3][4] = Square { row: 4, column: 5, piece_state: Some(PieceState {
            color: Color::White, piece: Piece::Pawn,
            location: (4, 5), has_moved: true, dead: false
        })};
        let board_state = empty_board_state();
        let mask = create_check_mask(&board, Color::White);
        let bishop_state = board.board[0][5].piece_state.unwrap();
        let bishop_moves = get_bishop_moves(bishop_state, &board, &mask);
        // f1 bishop slides e2->d3->c4->b5->a6, all empty. 5 moves.
        assert_eq!(bishop_moves.len(), 5, "f1 bishop should reach e2/d3/c4/b5/a6, got {}: {:?}",
            bishop_moves.len(),
            bishop_moves.iter().map(|m| (m.current_square.row, m.current_square.column)).collect::<Vec<_>>());
    }

    #[test]
    fn test_king_cannot_move_to_e2_after_e4_if_attacked() {
        let mut board = START_BOARD.clone();
        board.board[1][4].piece_state = None;
        board.board[3][4] = Square { row: 4, column: 5, piece_state: Some(PieceState {
            color: Color::White, piece: Piece::Pawn,
            location: (4, 5), has_moved: true, dead: false
        })};
        board.board[6][3].piece_state = None;
        board.board[4][3] = Square { row: 5, column: 4, piece_state: Some(PieceState {
            color: Color::Black, piece: Piece::Pawn,
            location: (5, 4), has_moved: true, dead: false
        })};
        let board_state = empty_board_state();
        let mask = create_check_mask(&board, Color::White);
        let king_state = board.board[0][4].piece_state.unwrap();
        let king_moves = get_king_moves(king_state, &board, &mask);
        // e2 is empty so king can move there (it's not attacked by anything)
        assert_eq!(king_moves.len(), 1, "king should have 1 move (e2), got {}: {:?}",
            king_moves.len(),
            king_moves.iter().map(|m| (m.current_square.row, m.current_square.column)).collect::<Vec<_>>());
    }

    #[test]
fn test_black_pawn_double_push_after_qh5() {
    let mut board = START_BOARD.clone();
    board.board[1][4].piece_state = None;
    board.board[3][4] = Square { row: 4, column: 5, piece_state: Some(PieceState {
        color: Color::White, piece: Piece::Pawn,
        location: (4, 5), has_moved: true, dead: false
    })};
    board.board[6][4].piece_state = None;
    board.board[5][4] = Square { row: 6, column: 5, piece_state: Some(PieceState {
        color: Color::Black, piece: Piece::Pawn,
        location: (6, 5), has_moved: true, dead: false
    })};
    board.board[0][3].piece_state = None;
    board.board[4][7] = Square { row: 5, column: 8, piece_state: Some(PieceState {
        color: Color::White, piece: Piece::Queen,
        location: (5, 8), has_moved: true, dead: false
    })};

    let board_state = empty_board_state();
    let mask = create_check_mask(&board, Color::Black);

    // g7 pawn should have 2 moves: g6 and g5
    let g7_pawn = board.board[6][6].piece_state.unwrap();
    let g7_moves = get_black_pawn_moves(g7_pawn, &board, board_state, &mask);
    assert_eq!(g7_moves.len(), 2, "g7 pawn should have 2 moves, got {}: {:?}",
        g7_moves.len(),
        g7_moves.iter().map(|m| (m.current_square.row, m.current_square.column)).collect::<Vec<_>>());

    // h7 pawn should have 2 moves: h6 and h5
    let h7_pawn = board.board[6][7].piece_state.unwrap();
    let h7_moves = get_black_pawn_moves(h7_pawn, &board, board_state, &mask);
    assert_eq!(h7_moves.len(), 2, "h7 pawn should have 2 moves, got {}: {:?}",
        h7_moves.len(),
        h7_moves.iter().map(|m| (m.current_square.row, m.current_square.column)).collect::<Vec<_>>());

    // specifically verify h5 double push is legal (queen is on h5 but that's white's piece,
    // black pawn moving to h5 would capture it — that should be illegal since h5 has a white piece
    // so h7 pawn should only have h6 as a single push, NOT h5)
    // Wait — h7->h5 is blocked by the queen on h5. So h7 pawn has only 1 move: h6
    assert_eq!(h7_moves.len(), 1, "h7 pawn should have only h6 (h5 blocked by queen), got {}: {:?}",
        h7_moves.len(),
        h7_moves.iter().map(|m| (m.current_square.row, m.current_square.column)).collect::<Vec<_>>());
}

#[test]
fn test_g7_pawn_moves_after_qh5() {
    let mut board = START_BOARD.clone();
    board.board[1][4].piece_state = None;
    board.board[3][4] = Square { row: 4, column: 5, piece_state: Some(PieceState {
        color: Color::White, piece: Piece::Pawn,
        location: (4, 5), has_moved: true, dead: false
    })};
    board.board[6][4].piece_state = None;
    board.board[5][4] = Square { row: 6, column: 5, piece_state: Some(PieceState {
        color: Color::Black, piece: Piece::Pawn,
        location: (6, 5), has_moved: true, dead: false
    })};
    board.board[0][3].piece_state = None;
    board.board[4][7] = Square { row: 5, column: 8, piece_state: Some(PieceState {
        color: Color::White, piece: Piece::Queen,
        location: (5, 8), has_moved: true, dead: false
    })};

    let board_state = empty_board_state();
    let mask = create_check_mask(&board, Color::Black);

    // g7 pawn: g6 and g5 should both be legal
    // queen on h5 attacks g6 but that doesn't affect g7 pawn's moves
    // since the black king on e8 is not exposed by g7 moving
    let g7_pawn = board.board[6][6].piece_state.unwrap();
    let g7_moves = get_black_pawn_moves(g7_pawn, &board, board_state, &mask);
    println!("g7 pawn moves: {:?}", 
        g7_moves.iter().map(|m| (m.current_square.row, m.current_square.column)).collect::<Vec<_>>());
    assert_eq!(g7_moves.len(), 2, "g7 pawn should have g6 and g5");

    // now check is_legal_move filters correctly
    for mv in &g7_moves {
        let legal = is_legal_move(mv, &board, Color::Black);
        println!("Move to ({},{}) legal: {}", mv.current_square.row, mv.current_square.column, legal);
        assert!(legal, "g7 pawn move to ({},{}) should be legal", 
            mv.current_square.row, mv.current_square.column);
    }
}

#[test]
fn test_black_move_breakdown_after_qh5() {
    let mut board = START_BOARD.clone();
    board.board[1][4].piece_state = None;
    board.board[3][4] = Square { row: 4, column: 5, piece_state: Some(PieceState {
        color: Color::White, piece: Piece::Pawn,
        location: (4, 5), has_moved: true, dead: false
    })};
    board.board[6][4].piece_state = None;
    board.board[5][4] = Square { row: 6, column: 5, piece_state: Some(PieceState {
        color: Color::Black, piece: Piece::Pawn,
        location: (6, 5), has_moved: true, dead: false
    })};
    board.board[0][3].piece_state = None;
    board.board[4][7] = Square { row: 5, column: 8, piece_state: Some(PieceState {
        color: Color::White, piece: Piece::Queen,
        location: (5, 8), has_moved: true, dead: false
    })};

    let board_state = empty_board_state();
    let mask = create_check_mask(&board, Color::Black);

    let mut total = 0;

    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece_state) = board.board[row][col].piece_state {
                if piece_state.color == Color::Black {
                    let mut moves: Vec<Move> = match piece_state.piece {
                        Piece::Pawn => get_black_pawn_moves(piece_state, &board, board_state, &mask),
                        Piece::Knight => get_knight_moves(piece_state, &board, &mask),
                        Piece::Bishop => get_bishop_moves(piece_state, &board, &mask),
                        Piece::Rook => get_rook_moves(piece_state, &board, &mask),
                        Piece::Queen => get_queen_moves(piece_state, &board, &mask),
                        Piece::King => get_king_moves(piece_state, &board, &mask),
                    };
                    moves.retain(|mv| is_legal_move(mv, &board, Color::Black));
                    println!("{:?} at ({},{}) has {} moves: {:?}",
                        piece_state.piece,
                        row + 1, col + 1,
                        moves.len(),
                        moves.iter().map(|m| (m.current_square.row, m.current_square.column)).collect::<Vec<_>>());
                    total += moves.len();
                }
            }
        }
    }
    println!("Total: {}", total);
    assert_eq!(total, 27, "black should have 29 moves after 1.e4 e6 2.Qh5");
}

#[test]
fn test_black_move_breakdown_after_qh5_verbose() {
    let mut board = START_BOARD.clone();
    board.board[1][4].piece_state = None;
    board.board[3][4] = Square { row: 4, column: 5, piece_state: Some(PieceState {
        color: Color::White, piece: Piece::Pawn,
        location: (4, 5), has_moved: true, dead: false
    })};
    board.board[6][4].piece_state = None;
    board.board[5][4] = Square { row: 6, column: 5, piece_state: Some(PieceState {
        color: Color::Black, piece: Piece::Pawn,
        location: (6, 5), has_moved: true, dead: false
    })};
    board.board[0][3].piece_state = None;
    board.board[4][7] = Square { row: 5, column: 8, piece_state: Some(PieceState {
        color: Color::White, piece: Piece::Queen,
        location: (5, 8), has_moved: true, dead: false
    })};

    let board_state = empty_board_state();
    let mask = create_check_mask(&board, Color::Black);

    println!("\n=== BOARD POSITION ===");
    for row in (0..8).rev() {
        print!("{} ", row + 1);
        for col in 0..8 {
            let ch = match board.board[row][col].piece_state {
                Some(s) => s.piece.to_char(s.color),
                None => ".".to_string(),
            };
            print!("{} ", ch);
        }
        println!();
    }
    println!("  a b c d e f g h");

    println!("\n=== BLACK MOVES ===");
    let mut total = 0;
    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece_state) = board.board[row][col].piece_state {
                if piece_state.color == Color::Black {
                    let mut moves: Vec<Move> = match piece_state.piece {
                        Piece::Pawn => get_black_pawn_moves(piece_state, &board, board_state, &mask),
                        Piece::Knight => get_knight_moves(piece_state, &board, &mask),
                        Piece::Bishop => get_bishop_moves(piece_state, &board, &mask),
                        Piece::Rook => get_rook_moves(piece_state, &board, &mask),
                        Piece::Queen => get_queen_moves(piece_state, &board, &mask),
                        Piece::King => get_king_moves(piece_state, &board, &mask),
                    };
                    moves.retain(|mv| is_legal_move(mv, &board, Color::Black));

                    let col_char = (b'a' + col as u8) as char;
                    for mv in &moves {
                        let to_col = (b'a' + mv.current_square.column as u8 - 1) as char;
                        println!("  {:?} {}{}  ->  {}{}{}",
                            piece_state.piece,
                            col_char, row + 1,
                            to_col, mv.current_square.row,
                            if mv.captured_piece.is_some() { " (capture)" } else { "" }
                        );
                    }
                    total += moves.len();
                }
            }
        }
    }
    println!("\nTotal: {}", total);
    assert_eq!(total, 27, "black should have 27 moves after 1.e4 e6 2.Qh5");
}
}

