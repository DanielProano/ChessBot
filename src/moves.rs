use crate::pieces::*;

const KNIGHT_DELTAS: [(i32, i32); 8] = [
    (2, 1),
    (2, -1),
    (1, 2),
    (-1, 2),
    (-2, 1),
    (-2, -1),
    (-1, 2),
    (-1, -2)
];

const BISHOP_DELTAS: [(i32, i32); 4] = [
    (1, 1), 
    (1, -1), 
    (-1, 1), 
    (-1, -1)
];

const ROOK_DELTAS: [(i32, i32); 4] = [
    (1, 0), 
    (-1, 0), 
    (0, 1), 
    (0, -1)
];

const QUEEN_DELTAS: [(i32, i32); 8] = [
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1)
];

const KING_DELTAS: [(i32, i32); 8] = [
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1)
];

pub const EMPTY_MOVE: Move = Move {
    previous_square: Square { row: 0, column: 0, piece_state: None },
    current_square: Square { row: 0, column: 0, piece_state: None },
    color: Color::White,
    captured_piece: None,
    promotion: None,
    castling: false
};

// Previous square holds the previous state
// otherwise every previous square would be empty
// and thats not helpful
#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub previous_square: Square,
    pub current_square: Square,
    pub color: Color,
    pub captured_piece: Option<PieceState>,
    pub promotion: Option<Piece>,
    pub castling: bool
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
        else if self.previous_square.row > 8
            || self.previous_square.row < 1
            || self.previous_square.column > 8
            || self.previous_square.column < 1
        {
            return false;
        } 
        else if self.previous_square == self.current_square {
            return false;
        } 
        else if self.previous_square.piece_state == None {
            return false;
        } 
        else if self.current_square.piece_state == None {
            return false;
        } 
        else if let (Some(prev), Some(cur)) = (self.previous_square.piece_state, self.current_square.piece_state) {
            if prev.color != cur.color || prev.color != self.color || cur.color != self.color {
                return false;
            }
        } 
        else if let Some(captured) = self.captured_piece {
            if captured.location.0 != self.current_square.row || captured.location.1 != self.current_square.column {
                return false;
            }
        } 
        else if let Some(captured) = self.captured_piece { 
            if captured.color == self.color {
                return false;
            }
        } 
        else if self.promotion.is_some()
            && self.castling {
            return false;
        } 
        else if let (Some(cur_state), Some(prev_state)) = (self.current_square.piece_state, self.previous_square.piece_state) {
            if self.castling && (cur_state.piece != Piece::King || prev_state.piece != Piece::King) {
                return false;
            }
        }

        true
    }
}

pub fn create(
    state: PieceState,
    prev_square: Square,
    cur_square: Square,
    color: Color,
    captured_piece: Option<PieceState>,
    promotion: Option<Piece>,
    castling: bool
) -> Option<Move> {
    let mv = Move {
        previous_square: prev_square,
        current_square: cur_square,
        color: color,
        captured_piece: captured_piece,
        promotion: promotion,
        castling: castling
    };

    if !mv.validate() {
        eprintln!(
            "Warning: PieceState {:?} failed validation",
            state
        );
        return None;
    }
    return Some(mv);
}

pub fn access_board(board: &Board, row: usize, column: usize) -> Option<Square> {
    if !in_bounds(row, column) {
        return None;
    }

    let normalized_row = row - 1;
    let normalized_col = column - 1;

    Some(board.board[normalized_row][normalized_col])
}

pub fn in_bounds(row: usize, col: usize) -> bool {
    if row <= 8 && row >= 1 && col <= 8 && col >= 1 {
        return true;
    }

    return false;
}

pub fn square_is_attacked(square: Square, active_color: Color, board: &Board) -> bool {
    let cur_row = square.row as i32;
    let cur_col = square.column as i32;

    match active_color {
        Color::White => {
            for &(row, col) in &KNIGHT_DELTAS {
                let target_row = (cur_row + row) as usize;
                let target_col = (cur_col + col) as usize;

                if in_bounds(target_row, target_col) {
                    if let Some(square) = access_board(&board, target_row, target_col) {
                        if let Some(state) = square.piece_state {
                            if state.piece == Piece::Knight && state.color != active_color {
                                return true;
                            }
                        }
                    }
                }
            }

            if row_is_attacked(square, active_color, board) {
                return true;
            }

            if column_is_attacked(square, active_color, board) {
                return true;
            }

            if diagonal_is_attacked(square, active_color, board) {
                return true;
            }

            let pawn_deltas: [(i32, i32); 2] = [
                (1, 1),
                (1, -1)
            ];

            for &(row, col) in &pawn_deltas {
                let target_row = (cur_row + row) as usize;
                let target_col = (cur_col + col) as usize;

                if in_bounds(target_row, target_col) {
                    if let Some(square) = access_board(board, target_row, target_col) {
                        if let Some(state) = square.piece_state {
                            if state.piece == Piece::Pawn && state.color != active_color {
                                return true;
                            }
                        }
                    }
                }
            }

            return false;
        },
        Color::Black => {
            for &(row, col) in &KNIGHT_DELTAS {
                let target_row = (cur_row + row) as usize;
                let target_col = (cur_col + col) as usize;

                if in_bounds(target_row, target_col) {
                    if let Some(square) = access_board(board, target_row, target_col) {
                        if let Some(state) = square.piece_state {
                            if state.piece == Piece::Knight && state.color != active_color {
                                return true;
                            }
                        }
                    }
                }
            }

            if row_is_attacked(square, active_color, board) {
                return true;
            }

            if column_is_attacked(square, active_color, board) {
                return true;
            }

            if diagonal_is_attacked(square, active_color, board) {
                return true;
            }

            let pawn_deltas: [(i32, i32); 2] = [
                (-1, 1),
                (-1, -1)
            ];

            for &(row, col) in &pawn_deltas {
                let target_row = (cur_row + row) as usize;
                let target_column  = (cur_col + col) as usize;

                if in_bounds(target_row, target_column) {
                    if let Some(square) = access_board(board, target_row, target_column) {
                        if let Some(state) = square.piece_state {
                            if state.piece == Piece::Pawn && state.color != active_color {
                                return true;
                            }
                        }
                    }
                }
            }

            return false;
        }
    }
}

pub fn column_is_attacked(square: Square, active_color: Color, board: &Board) -> bool {
    let initial_row = square.row;
    let cur_col = square.column;

    for row in (initial_row + 1)..8 {
        if in_bounds(row, cur_col) {
            if let Some(square) = access_board(board, row, cur_col) {
                if let Some(state) = square.piece_state {
                    if (state.piece == Piece::Rook || state.piece == Piece::Queen) && state.color != active_color {
                        return true;
                    }
                    break;
                }
            }
        }
    }

    for row in (0..initial_row).rev() {
        if in_bounds(row, cur_col) {
            if let Some(square) = access_board(board, row, cur_col) {
                if let Some(state) = square.piece_state {
                    if (state.piece == Piece::Rook || state.piece == Piece::Queen) && state.color != active_color {
                        return true;
                    }
                    break;
                }
            }
        }
    }

    return false;
}

pub fn row_is_attacked(square: Square, active_color: Color, board: &Board) -> bool {
    let cur_row = square.row;
    let initial_col = square.column;

    for col in (initial_col + 1)..8 {
        if in_bounds(cur_row, col) {
            if let Some(square) = access_board(board, cur_row, col) {
                if let Some(state) = square.piece_state {
                    if (state.piece == Piece::Rook || state.piece == Piece::Queen) && state.color != active_color {
                        return true;
                    }
                    break;
                }
            }
        }
    }

    for col in (0..initial_col).rev() {
        if in_bounds(cur_row, col) {
            if let Some(square) = access_board(board, cur_row, col) {
                if let Some(state) = square.piece_state {
                    if (state.piece == Piece::Rook || state.piece == Piece::Queen) && state.color != active_color {
                        return true;
                    }
                    break;
                }
            }
        }
    }
    
    return false;
}

pub fn diagonal_is_attacked(square: Square, active_color: Color, board: &Board) -> bool {
    let initial_row: i32 = square.row as i32;
    let initial_col: i32 = square.column as i32;

    for &(r_idx, c_idx) in &BISHOP_DELTAS {
        for multiplier in 1..8 {
            let target_row = (initial_row + r_idx * multiplier) as usize;
            let target_col = (initial_col + c_idx * multiplier) as usize;

            if !in_bounds(target_row, target_col) {
                break;
            }

            if let Some(square) = access_board(board, target_row, target_col) {
                if let Some(state) = square.piece_state {
                    if (state.piece == Piece::Queen || state.piece == Piece::Bishop) && state.color != active_color {
                        return true;
                    }
                    break;
                }
            }
        }
    }

    return false;
}

pub fn get_pawn_moves(state: PieceState, board: &Board, board_state: BoardState) -> Vec<Move> {
    if state.piece != Piece::Pawn {
        println!("Warning: State is not representing a pawn");
        return vec![];
    }

    match state.color {
        Color::White => get_white_pawn_moves(state, board, board_state),
        Color::Black => get_black_pawn_moves(state, board, board_state),
    }
}

pub fn get_white_pawn_moves(state: PieceState, board: &Board, mut board_state: BoardState) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let cur_row: usize = state.location.0;
    let cur_col: usize = state.location.1;

    if cur_row == 2 {
        if let (Some(square), Some(inbetween_square), Some(target_square)) = (
            access_board(board, cur_row, cur_col), 
            access_board(board, cur_row + 1, cur_col), 
            access_board(board, cur_row + 2, cur_col)
        ) {
            if inbetween_square.piece_state.is_none() && target_square.piece_state.is_none() {
                add_pawn_move(&mut moves, state, square, cur_row + 2, cur_col, None);
                board_state.white_state.en_passant = Some(target_square);
            }
        }
    }

    if cur_row + 1 <= 8 {
        if let (Some(square), Some(target_square)) = (access_board(board, cur_row, cur_col), access_board(board, cur_row + 1, cur_col)) {
            if target_square.piece_state.is_none() {
                add_pawn_move(&mut moves, state, square, cur_row + 1, cur_col, None);
                board_state.white_state.en_passant = None;
            }
        }
    }

    if cur_row < 8 && cur_col > 1 {
        if let (Some(square), Some(target_square)) = (access_board(board, cur_row, cur_col), access_board(board, cur_row + 1, cur_col - 1)) {
            if let Some(target_piece) = target_square.piece_state {
                //Normal capture

                add_pawn_move(&mut moves, state, square, cur_row + 1, cur_col - 1, Some(target_piece));
                board_state.white_state.en_passant = None;
            } else if target_square.piece_state.is_none() {
                //Special en passant
                
                if let (Some(enemy_square), Some(special_square)) = (board_state.black_state.en_passant, access_board(board, cur_row, cur_col - 1)) {
                    if let Some(target_piece) = special_square.piece_state {
                        if enemy_square == special_square {
                            add_pawn_move(&mut moves, state, square, cur_row + 1, cur_col - 1, Some(target_piece));
                            board_state.black_state.en_passant = None;
                        }
                    }
                }
            }
        } 
    }

    if cur_row < 8 && cur_col < 8 {
        if let (Some(square), Some(target_square)) = (access_board(board, cur_row, cur_col), access_board(board, cur_row + 1, cur_col + 1)) {
            if let Some(target_piece) = target_square.piece_state {
                add_pawn_move(&mut moves, state, square, cur_row + 1, cur_col + 1, Some(target_piece));
                board_state.white_state.en_passant = None;
            } else if target_square.piece_state.is_none() {
                //Special en passant

                if let (Some(enemy_square), Some(special_square)) = (board_state.white_state.en_passant, access_board(board, cur_row, cur_col + 1)) {
                    if let Some(target_piece) = special_square.piece_state {
                        if enemy_square == special_square {
                            add_pawn_move(&mut moves, state, square, cur_row + 1, cur_col + 1, Some(target_piece));
                            board_state.black_state.en_passant = None;
                        }
                    }
                }
            }
        }
    }

    moves
}

pub fn get_black_pawn_moves(state: PieceState, board: &Board, mut board_state: BoardState) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let cur_row: usize = state.location.0;
    let cur_col: usize = state.location.1;

    if cur_row == 7 {
        if let (Some(square), Some(inbetween_square), Some(target_square)) = (
            access_board(board, cur_row, cur_col), 
            access_board(board, cur_row - 1, cur_col), 
            access_board(board, cur_row - 2, cur_col)
        ) {
            if inbetween_square.piece_state.is_none() && target_square.piece_state.is_none() {
                add_pawn_move(&mut moves, state, square, cur_row - 2, cur_col, None);
                board_state.black_state.en_passant = Some(target_square);
            }
        }
    }

    if cur_row - 1 >= 1 {
        if let (Some(square), Some(target_square)) = (access_board(board, cur_row, cur_col), access_board(board, cur_row - 1, cur_col)) {
            if target_square.piece_state.is_none() {
                add_pawn_move(&mut moves, state, square, cur_row - 1, cur_col, None);
                board_state.black_state.en_passant = None;
            }
        }
    }

    if cur_row > 1 && cur_col > 1 {
        if let (Some(square), Some(target_square)) = (access_board(board, cur_row, cur_col), access_board(board, cur_row - 1, cur_col - 1)) {
            if let Some(target_piece) = target_square.piece_state {
                //Normal pawn capture

                add_pawn_move(&mut moves, state, square, cur_row - 1, cur_col - 1, Some(target_piece));
                board_state.black_state.en_passant = None;
            } else if target_square.piece_state.is_none() {
                //Special en passant

                if let (Some(enemy_square), Some(special_square)) = (board_state.white_state.en_passant, access_board(board, cur_row, cur_col - 1)) {
                    if let Some(target_piece) = special_square.piece_state {
                        if enemy_square == special_square {
                            add_pawn_move(&mut moves, state, square, cur_row - 1, cur_col - 1, Some(target_piece));
                            board_state.black_state.en_passant = None;
                        }
                    }
                }
            }
        }
    }

    if cur_row > 1 && cur_col < 8 {
        if let (Some(square), Some(target_square)) = (access_board(board, cur_row, cur_col), access_board(board, cur_row - 1, cur_col + 1)) {
            if let Some(target_piece) = target_square.piece_state {
                //Normal pawn capture

                add_pawn_move(&mut moves, state, square, cur_row - 1, cur_col + 1, Some(target_piece));
                board_state.black_state.en_passant = None;
            } else if target_square.piece_state.is_none() {
                //Special en passant
                
                if let (Some(enemy_square), Some(special_square)) = (board_state.white_state.en_passant, access_board(board, cur_row, cur_col + 1)) {
                    if let Some(target_piece) = special_square.piece_state {
                        if enemy_square == special_square {
                            add_pawn_move(&mut moves, state, square, cur_row - 1, cur_col + 1, Some(target_piece));
                            board_state.black_state.en_passant = None;
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
        false
    ) {
        let mut final_move = mv;

        match state.color {
            Color::White => {
                if target_row == 8 {
                    let promotion_pieces = [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight];
                    for &promotion in &promotion_pieces {
                        final_move.promotion = Some(promotion);
                        moves.push(final_move.clone());
                    }
                } 
                else {
                    moves.push(final_move.clone());
                }
            },
            Color::Black => {
                if target_row == 1 {
                    let promotion_pieces = [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight];
                    for &promotion in &promotion_pieces {
                        final_move.promotion = Some(promotion);
                        moves.push(final_move.clone());
                    }
                } 
                else {
                    moves.push(final_move.clone());
                }
            }
        }   
    }
}

pub fn get_knight_moves(state: PieceState, board: &Board) -> Vec<Move> {
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

        if in_bounds(target_row, target_col) {
            if let (Some(previous_square), Some(target_square)) = (access_board(board, cur_row, cur_col), access_board(board, target_row, target_col)) {
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
                            false
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
                            false
                        ) {
                            moves.push(mv);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    moves
}

pub fn get_bishop_moves(state: PieceState, board: &Board) -> Vec<Move> {
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

            if let (Some(previous_square), Some(target_square)) = (access_board(board, cur_row, cur_col), access_board(board, target_row, target_col)) {
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
                            false
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
                        false
                    ) {
                        moves.push(mv);
                    }
                }
            }
        }
    }

    moves
}

pub fn get_rook_moves(state: PieceState, board: &Board) -> Vec<Move> {
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

            if let (Some(previous_square), Some(target_square)) = (access_board(board, cur_row, cur_col), access_board(board, target_row, target_col)) {
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
                            false
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


pub fn get_queen_moves(state: PieceState, board: &Board) -> Vec<Move> {
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

            if let (Some(previous_square), Some(target_square)) = (access_board(board, cur_row, cur_col), access_board(board, target_row, target_col)) {
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
                            false
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

pub fn get_king_moves(state: PieceState, board: &Board) -> Vec<Move> {
    if state.piece != Piece::King {
        println!("Warning: State pieces misaligned");
        return vec![];
    }

    let mut moves: Vec<Move> = vec![];
    let cur_row = state.location.0;
    let cur_col = state.location.1;

    for &(row_offset, col_offset) in &KING_DELTAS {
        let target_row = (cur_row as i32 + row_offset) as usize;
        let target_col = (cur_col as i32 + col_offset) as usize;

        if !in_bounds(target_row, target_col) {
            continue;
        }

        if let (Some(previous_square), Some(target_square)) = (access_board(board, cur_row, cur_col), access_board(board, target_row, target_col)) {
            if square_is_attacked(target_square, state.color, board) {
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
                        false
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
                    false
                ) {
                    moves.push(mv);
                }
            }
        }
    }

    if !state.has_moved {
        if let Some(previous_square) = access_board(board, cur_row, cur_col) {
            if is_kingside_castling_valid(state, board, cur_row) {
                if let Some(mv) = create(
                    state,
                    previous_square,
                    Square {
                        row: cur_row as usize,
                        column: 7,
                        piece_state: Some(state),
                    },
                    state.color,
                    None,
                    None,
                    true
                ) {
                    moves.push(mv);
                }
            }

            if is_queenside_castling_valid(state, board, cur_row as usize) {
                if let Some(mv) = create(
                    state,
                    previous_square,
                    Square {
                        row: cur_row as usize,
                        column: 3,
                        piece_state: Some(state),
                    },
                    state.color,
                    None,
                    None,
                    true
                ) {
                    moves.push(mv);
                }
            }
        }
    }

    moves
}

fn is_kingside_castling_valid(king: PieceState, board: &Board, row: usize) -> bool {
    if let Some(rook) = board.board[row - 1][7].piece_state {
        if rook.piece == Piece::Rook && rook.color == king.color && !rook.has_moved && !king.has_moved {
            return board.board[row - 1][6].piece_state.is_none() 
                && board.board[row - 1][7].piece_state.is_none();
        }
    }
    false
}

fn is_queenside_castling_valid(king: PieceState, board: &Board, row: usize) -> bool {
    if let Some(rook) = board.board[row - 1][0].piece_state {
        if rook.piece == Piece::Rook && rook.color == king.color && !rook.has_moved && !king.has_moved {
            return board.board[row - 1][2].piece_state.is_none()
                && board.board[row - 1][3].piece_state.is_none()
                && board.board[row - 1][4].piece_state.is_none();
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

    // ---- validate ----

    #[test]
    fn test_validate_rejects_out_of_bounds() {
        let mv = Move {
            previous_square: Square { row: 0, column: 1, piece_state: Some(PieceState {
                color: Color::White, piece: Piece::Pawn, location: (0, 1), has_moved: false, dead: false
            })},
            current_square: Square { row: 1, column: 1, piece_state: Some(PieceState {
                color: Color::White, piece: Piece::Pawn, location: (1, 1), has_moved: false, dead: false
            })},
            color: Color::White,
            captured_piece: None,
            promotion: None,
            castling: false,
        };
        assert!(!mv.validate());
    }

    #[test]
    fn test_validate_rejects_same_square() {
        let square = Square { row: 4, column: 4, piece_state: Some(PieceState {
            color: Color::White, piece: Piece::Pawn, location: (4, 4), has_moved: false, dead: false
        })};
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
            previous_square: Square { row: 2, column: 1, piece_state: None },
            current_square: Square { row: 3, column: 1, piece_state: Some(PieceState {
                color: Color::White, piece: Piece::Pawn, location: (3, 1), has_moved: true, dead: false
            })},
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
        let moves = get_white_pawn_moves(state, &board, empty_board_state());
        assert!(moves.iter().any(|m| m.current_square.row == 3 && m.current_square.column == 4));
    }

    #[test]
    fn test_white_pawn_double_push_from_rank_2() {
        let mut board = empty_board();
        place_piece(&mut board, 2, 4, Piece::Pawn, Color::White);
        let state = board.board[1][3].piece_state.unwrap();
        let moves = get_white_pawn_moves(state, &board, empty_board_state());
        assert!(moves.iter().any(|m| m.current_square.row == 4 && m.current_square.column == 4));
    }

    #[test]
    fn test_white_pawn_blocked() {
        let mut board = empty_board();
        place_piece(&mut board, 2, 4, Piece::Pawn, Color::White);
        place_piece(&mut board, 3, 4, Piece::Pawn, Color::Black);
        let state = board.board[1][3].piece_state.unwrap();
        let moves = get_white_pawn_moves(state, &board, empty_board_state());
        assert!(moves.is_empty());
    }

    #[test]
    fn test_white_pawn_capture() {
        let mut board = empty_board();
        place_piece(&mut board, 2, 4, Piece::Pawn, Color::White);
        place_piece(&mut board, 3, 5, Piece::Pawn, Color::Black);
        let state = board.board[1][3].piece_state.unwrap();
        let moves = get_white_pawn_moves(state, &board, empty_board_state());
        assert!(moves.iter().any(|m| m.current_square.row == 3 && m.current_square.column == 5 && m.captured_piece.is_some()));
    }

    #[test]
    fn test_white_pawn_promotion() {
        let mut board = empty_board();
        place_piece(&mut board, 7, 4, Piece::Pawn, Color::White);
        let state = board.board[6][3].piece_state.unwrap();
        let moves = get_white_pawn_moves(state, &board, empty_board_state());
        assert_eq!(moves.iter().filter(|m| m.promotion.is_some()).count(), 4);
    }

    #[test]
    fn test_black_pawn_single_push() {
        let mut board = empty_board();
        place_piece(&mut board, 7, 4, Piece::Pawn, Color::Black);
        let state = board.board[6][3].piece_state.unwrap();
        let moves = get_black_pawn_moves(state, &board, empty_board_state());
        assert!(moves.iter().any(|m| m.current_square.row == 6 && m.current_square.column == 4));
    }

    #[test]
    fn test_black_pawn_double_push_from_rank_7() {
        let mut board = empty_board();
        place_piece(&mut board, 7, 4, Piece::Pawn, Color::Black);
        let state = board.board[6][3].piece_state.unwrap();
        let moves = get_black_pawn_moves(state, &board, empty_board_state());
        assert!(moves.iter().any(|m| m.current_square.row == 5 && m.current_square.column == 4));
    }

    #[test]
    fn test_black_pawn_capture() {
        let mut board = empty_board();
        place_piece(&mut board, 7, 4, Piece::Pawn, Color::Black);
        place_piece(&mut board, 6, 5, Piece::Pawn, Color::White);
        let state = board.board[6][3].piece_state.unwrap();
        let moves = get_black_pawn_moves(state, &board, empty_board_state());
        assert!(moves.iter().any(|m| m.current_square.row == 6 && m.current_square.column == 5 && m.captured_piece.is_some()));
    }

    //knight moves

    #[test]
    fn test_knight_moves_from_center() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Knight, Color::White);
        let state = board.board[3][3].piece_state.unwrap();
        let moves = get_knight_moves(state, &board);
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn test_knight_moves_from_corner() {
        let mut board = empty_board();
        place_piece(&mut board, 1, 1, Piece::Knight, Color::White);
        let state = board.board[0][0].piece_state.unwrap();
        let moves = get_knight_moves(state, &board);
        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn test_knight_cannot_capture_own_piece() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Knight, Color::White);
        place_piece(&mut board, 6, 5, Piece::Pawn, Color::White);
        let state = board.board[3][3].piece_state.unwrap();
        let moves = get_knight_moves(state, &board);
        assert!(moves.iter().all(|m| !(m.current_square.row == 6 && m.current_square.column == 5)));
    }

    #[test]
    fn test_knight_can_capture_enemy() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Knight, Color::White);
        place_piece(&mut board, 6, 5, Piece::Pawn, Color::Black);
        let state = board.board[3][3].piece_state.unwrap();
        let moves = get_knight_moves(state, &board);
        assert!(moves.iter().any(|m| m.current_square.row == 6 && m.current_square.column == 5 && m.captured_piece.is_some()));
    }

    // ---- bishop moves ----

    #[test]
    fn test_bishop_moves_open_board() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Bishop, Color::White);
        let state = board.board[3][3].piece_state.unwrap();
        let moves = get_bishop_moves(state, &board);
        assert_eq!(moves.len(), 13);
    }

    #[test]
    fn test_bishop_blocked_by_own_piece() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Bishop, Color::White);
        place_piece(&mut board, 5, 5, Piece::Pawn, Color::White);
        let state = board.board[3][3].piece_state.unwrap();
        let moves = get_bishop_moves(state, &board);
        assert!(moves.iter().all(|m| !(m.current_square.row == 5 && m.current_square.column == 5)));
    }

    #[test]
    fn test_bishop_captures_enemy_and_stops() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Bishop, Color::White);
        place_piece(&mut board, 6, 6, Piece::Pawn, Color::Black);
        let state = board.board[3][3].piece_state.unwrap();
        let moves = get_bishop_moves(state, &board);
        assert!(moves.iter().any(|m| m.current_square.row == 6 && m.current_square.column == 6 && m.captured_piece.is_some()));
        assert!(moves.iter().all(|m| !(m.current_square.row == 7 && m.current_square.column == 7)));
    }

    // ---- rook moves ----

    #[test]
    fn test_rook_moves_open_board() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Rook, Color::White);
        let state = board.board[3][3].piece_state.unwrap();
        let moves = get_rook_moves(state, &board);
        assert_eq!(moves.len(), 14);
    }

    #[test]
    fn test_rook_blocked_by_own_piece() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Rook, Color::White);
        place_piece(&mut board, 4, 6, Piece::Pawn, Color::White);
        let state = board.board[3][3].piece_state.unwrap();
        let moves = get_rook_moves(state, &board);
        assert!(moves.iter().all(|m| m.current_square.column < 6 || m.current_square.row != 4));
    }

    #[test]
    fn test_rook_captures_enemy_and_stops() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Rook, Color::White);
        place_piece(&mut board, 4, 6, Piece::Pawn, Color::Black);
        let state = board.board[3][3].piece_state.unwrap();
        let moves = get_rook_moves(state, &board);
        assert!(moves.iter().any(|m| m.current_square.row == 4 && m.current_square.column == 6 && m.captured_piece.is_some()));
        assert!(moves.iter().all(|m| !(m.current_square.row == 4 && m.current_square.column == 7)));
    }

    // ---- queen moves ----

    #[test]
    fn test_queen_moves_open_board() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Queen, Color::White);
        let state = board.board[3][3].piece_state.unwrap();
        let moves = get_queen_moves(state, &board);
        assert_eq!(moves.len(), 27);
    }

    #[test]
    fn test_queen_combines_rook_and_bishop() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Queen, Color::White);
        let state = board.board[3][3].piece_state.unwrap();
        let moves = get_queen_moves(state, &board);
        // can reach same squares as rook + bishop from center
        assert!(moves.iter().any(|m| m.current_square.row == 4 && m.current_square.column == 8)); // rook direction
        assert!(moves.iter().any(|m| m.current_square.row == 7 && m.current_square.column == 7)); // bishop direction
    }

    // ---- king moves ----

    #[test]
    fn test_king_moves_from_center() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::King, Color::White);
        let state = board.board[3][3].piece_state.unwrap();
        let moves = get_king_moves(state, &board);
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn test_king_cannot_capture_own_piece() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::King, Color::White);
        place_piece(&mut board, 5, 5, Piece::Pawn, Color::White);
        let state = board.board[3][3].piece_state.unwrap();
        let moves = get_king_moves(state, &board);
        assert!(moves.iter().all(|m| !(m.current_square.row == 5 && m.current_square.column == 5)));
    }

    #[test]
    fn test_king_moves_from_corner() {
        let mut board = empty_board();
        place_piece(&mut board, 1, 1, Piece::King, Color::White);
        let state = board.board[0][0].piece_state.unwrap();
        let moves = get_king_moves(state, &board);
        assert_eq!(moves.len(), 3);
    }

    // ---- square_is_attacked ----

    #[test]
    fn test_square_attacked_by_knight() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Knight, Color::Black);
        let square = Square { row: 2, column: 3, piece_state: None };
        assert!(square_is_attacked(square, Color::White, &board));
    }

    #[test]
    fn test_square_not_attacked() {
        let mut board = empty_board();
        place_piece(&mut board, 1, 1, Piece::Rook, Color::Black);
        let square = Square { row: 4, column: 4, piece_state: None };
        assert!(!square_is_attacked(square, Color::White, &board));
    }

    #[test]
    fn test_square_attacked_by_rook() {
        let mut board = empty_board();
        place_piece(&mut board, 4, 1, Piece::Rook, Color::Black);
        let square = Square { row: 4, column: 5, piece_state: None };
        assert!(square_is_attacked(square, Color::White, &board));
    }

    #[test]
    fn test_square_attacked_by_bishop() {
        let mut board = empty_board();
        place_piece(&mut board, 1, 1, Piece::Bishop, Color::Black);
        let square = Square { row: 4, column: 4, piece_state: None };
        assert!(square_is_attacked(square, Color::White, &board));
    }

    #[test]
    fn test_white_pawn_en_passant_left() {
        let mut board = empty_board();
        place_piece(&mut board, 5, 4, Piece::Pawn, Color::White);
        place_piece(&mut board, 5, 3, Piece::Pawn, Color::Black);

        let mut board_state = empty_board_state();
        // black just double pushed to col 3, row 5 — en passant target is row 6, col 3
        board_state.black_state.en_passant = Some(Square {
            row: 5, column: 3, piece_state: board.board[4][2].piece_state
        });

        let state = board.board[4][3].piece_state.unwrap();
        let moves = get_white_pawn_moves(state, &board, board_state);
        assert!(
            moves.iter().any(|m| m.current_square.row == 6 && m.current_square.column == 3 && m.captured_piece.is_some()),
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
            row: 4, column: 5, piece_state: board.board[3][4].piece_state
        });

        let state = board.board[3][3].piece_state.unwrap();
        let moves = get_black_pawn_moves(state, &board, board_state);
        assert!(
            moves.iter().any(|m| m.current_square.row == 3 && m.current_square.column == 5 && m.captured_piece.is_some()),
            "black should be able to capture en passant to the right"
        );
    }
}