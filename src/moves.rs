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

// Previous square holds the previous state
// otherwise every previous square would be empty
// and thats not helpful
#[derive(Debug, Clone, Copy)]
pub struct Move {
    previous_square: Square,
    current_square: Square,
    color: Color,
    captured_piece: Option<PieceState>,
    promotion: Option<Piece>,
    castling: bool
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
        else if self.previous_square.piece_state != None {
            return false;
        } 
        else if self.current_square.piece_state == None {
            return false;
        } 
        else if let (Some(prev), Some(cur)) = (self.previous_square.piece_state, self.current_square.piece_state) {
            if prev.color != cur.color {
                return false;
            }
        } 
        else if let (Some(prev), Some(cur)) = (self.previous_square.piece_state, self.current_square.piece_state) {
            if prev.color != self.color || cur.color != self.color {
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
            "Warning: Piece {:?}, id {} failed validation",
            state.piece, state.id
        );
        return None;
    }
    return Some(mv);
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
                    if let Some(state) = board.board[target_row - 1][target_col - 1].piece_state {
                        if state.piece == Piece::Knight && state.color != active_color {
                            return true;
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
                    if let Some(state) = board.board[target_row - 1][target_col - 1].piece_state {
                        if state.piece == Piece::Pawn && state.color != active_color {
                            return true;
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
                    if let Some(state) = board.board[target_row - 1][target_col - 1].piece_state {
                        if state.piece == Piece::Knight && state.color != active_color {
                            return true;
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
                let target_col  = (cur_col + col) as usize;

                if in_bounds(target_row, target_col) {
                    if let Some(state) = board.board[target_row - 1][target_col - 1].piece_state {
                        if state.piece == Piece::Pawn && state.color != active_color {
                            return true;
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
        if in_bounds(row as usize, cur_col as usize) {
            if let Some(state) = board.board[row - 1][cur_col - 1].piece_state {
                if (state.piece == Piece::Rook || state.piece == Piece::Queen) && state.color != active_color {
                    return true;
                }
                break;
            }
        }
    }

    for row in (0..initial_row).rev() {
        if in_bounds(row as usize, cur_col as usize) {
            if let Some(state) = board.board[(row - 1) as usize][(cur_col - 1) as usize].piece_state {
                if (state.piece == Piece::Rook || state.piece == Piece::Queen) && state.color != active_color {
                    return true;
                }
                break;
            }
        }
    }

    return false;
}

pub fn row_is_attacked(square: Square, active_color: Color, board: &Board) -> bool {
    let cur_row = square.row;
    let initial_col = square.column;

    for col in (initial_col + 1)..8 {
        if in_bounds(cur_row as usize, col as usize) {
            if let Some(state) = board.board[(cur_row - 1) as usize][(col - 1) as usize].piece_state {
                if (state.piece == Piece::Rook || state.piece == Piece::Queen) && state.color != active_color {
                    return true;
                }
                break;
            }
        }
    }

    for col in (0..initial_col).rev() {
        if in_bounds(cur_row as usize, col as usize) {
            if let Some(state) = board.board[(cur_row - 1) as usize][(col - 1) as usize].piece_state {
                if (state.piece == Piece::Rook || state.piece == Piece::Queen) && state.color != active_color {
                    return true;
                }
                break;
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

            let target_sqr = board.board[target_row -1 ][target_col - 1];
            if let Some(state) = target_sqr.piece_state {
                if (state.piece == Piece::Queen || state.piece == Piece::Bishop) && state.color != active_color {
                    return true;
                }
                break;
            }
        }
    }

    return false;
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

pub fn get_pawn_moves(state: PieceState, board: &Board) -> Vec<Move> {
    if state.piece != Piece::Pawn {
        println!("Warning: State is not representing a pawn");
        return vec![];
    }

    match state.color {
        Color::White => get_white_pawn_moves(state, board),
        Color::Black => get_black_pawn_moves(state, board),
    }
}

pub fn get_white_pawn_moves(state: PieceState, board: &Board) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let cur_row: usize = state.location.0;
    let cur_col: usize = state.location.1;

    if cur_row == 2 && board.board[cur_row + 2][cur_col].piece_state.is_none() {
        add_pawn_move(&mut moves, state, board.board[cur_row + 2][cur_col], cur_row + 2, cur_col, None);
    }

    if cur_row + 1 <= 8 && board.board[cur_row + 1][cur_col].piece_state.is_none() {
        add_pawn_move(&mut moves, state, board.board[cur_row + 1][cur_col], cur_row + 1, cur_col, None);
    }

    if cur_row < 8 && cur_col < 8 {
        if let Some(target_piece) = board.board[cur_row + 1][cur_col + 1].piece_state {
            add_pawn_move(&mut moves, state, board.board[cur_row + 1][cur_col + 1], cur_row + 1, cur_col + 1, Some(target_piece));
        }
    }

    if cur_row < 8 && cur_col > 1 {
        if let Some(target_piece) = board.board[cur_row + 1][cur_col - 1].piece_state {
            add_pawn_move(&mut moves, state, board.board[cur_row + 1][cur_col - 1], cur_row + 1, cur_col - 1, Some(target_piece));
        }
    }

    moves
}

pub fn get_black_pawn_moves(state: PieceState, board: &Board) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let cur_row: usize = state.location.0;
    let cur_col: usize = state.location.1;

    if cur_row == 7 && board.board[cur_row - 2][cur_col].piece_state.is_none() {
        add_pawn_move(&mut moves, state, board.board[cur_row - 2][cur_col], cur_row - 2, cur_col, None);
    }

    if cur_row - 1 >= 1 && board.board[cur_row - 1][cur_col].piece_state.is_none() {
        add_pawn_move(&mut moves, state, board.board[cur_row - 1][cur_col], cur_row - 1, cur_col, None);
    }

    if cur_row > 1 && cur_col > 1 {
        if let Some(target_piece) = board.board[cur_row - 1][cur_col - 1].piece_state {
            add_pawn_move(&mut moves, state, board.board[cur_row - 1][cur_col - 1], cur_row - 1, cur_col - 1, Some(target_piece));
        }
    }

    if cur_row > 1 && cur_col < 8 {
        if let Some(target_piece) = board.board[cur_row - 1][cur_col + 1].piece_state {
            add_pawn_move(&mut moves, state, board.board[cur_row - 1][cur_col + 1], cur_row - 1, cur_col + 1, Some(target_piece));
        }
    }

    moves
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
            let previous_square = board.board[cur_row - 1][cur_col - 1];
            let target_square = board.board[target_row - 1][target_col - 1];
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

            let previous_square = board.board[cur_row - 1][cur_col - 1];
            let target_square = board.board[target_row - 1][target_col - 1];
            
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

            let previous_square = board.board[cur_row - 1][cur_col - 1];
            let target_square = board.board[target_row - 1][target_col - 1];
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

            let previous_square = board.board[cur_row - 1][cur_col - 1];
            let target_square = board.board[target_row - 1][target_col - 1];

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
            break;
        }

        let previous_square = board.board[cur_row - 1][cur_col - 1];
        let target_square = board.board[target_row - 1][target_col - 1];

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

    if !state.has_moved {
        let previous_square = board.board[cur_row - 1][cur_col - 1];
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

    moves
}

fn is_kingside_castling_valid(king: PieceState, board: &Board, row: usize) -> bool {
    if let Some(rook) = board.board[row][8].piece_state {
        if rook.piece == Piece::Rook && rook.color == king.color && !rook.has_moved {
            return board.board[row][6].piece_state.is_none() && board.board[row][7].piece_state.is_none();
        }
    }
    false
}

fn is_queenside_castling_valid(king: PieceState, board: &Board, row: usize) -> bool {
    if let Some(rook) = board.board[row][1].piece_state {
        if rook.piece == Piece::Rook && rook.color == king.color && !rook.has_moved {
            return board.board[row][2].piece_state.is_none()
                && board.board[row][3].piece_state.is_none()
                && board.board[row][4].piece_state.is_none();
        }
    }
    false
}