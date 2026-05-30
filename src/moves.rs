use crate::pieces::*;
use wasm_bindgen::prelude::*;

pub struct Move {
    previous_square: Square,
    current_square: Square,
    color: Color,
    captured_piece: Option<PieceState>,
    promotion: Option<Piece>,
    castling: bool,
}

impl Move {
    pub fn validate(self, mv: Move) -> bool {
        if mv.current_square.row > 8 || mv.current_square.row < 1 || mv.current_square.column > 8 || mv.current_square.column < 1 {
            return false;
        }
        else if mv.previous_square.row > 8 || mv.previous_square.row < 1 || mv.previous_square.column > 8 || mv.previous_square.column < 1 {
            return false;
        }
        else if mv.previous_square == mv.current_square {
            return false;
        }
        else if mv.previous_square.piece != None {
            return false;
        }
        else if mv.current_square.piece == None {
            return false;
        }
        else if mv.previous_square.piece.color != mv.current_square.piece.color {
            return false;
        }
        else if mv.previous_square.piece.color != mv.color || mv.current_square.piece.color != mv.color {
            return false;
        }
        else if mv.captured_piece.row != mv.current_square.row || mv.captured_piece.column != mv.current_square.column {
            return false;
        }
        else if mv.captured_piece != None && mv.captured_piece.piece.color == mv.color {
            return false;
        }
        else if mv.promotion != None && mv.promotion.piece.color != mv.color {
            return false;
        }
        else if mv.promotion != None && mv.color == Color::White && mv.previous_square.piece != None && mv.previous_square.piece != Piece::Pawn {
            return false;
        }
        else if mv.promotion != None && mv.color == Color::Black && mv.previous_square.piece != None && mv.previous_square.piece != Piece::Pawn {
            return false;
        }
        else if mv.promotion != None && mv.castling {
            return false;
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
    let mut mv = Move {
        previous_square: prev_square,
        current_square: cur_square,
        color: color,
        captured_piece: captured_piece,
        promotion: promotion,
        castling: castling
    };

    if !mv.validate(mv) {
        eprintln!("Warning: Piece {}, id {} failed validation", state.piece, state.id);
        return None;
    }
    return Some(mv);
}

pub fn in_bounds(row: u32, col: u32) -> bool {
    if row <= 8 && row >= 1 && col <= 8 && col >= 1 {
        return true;
    } 

    return false;
}

fn add_pawn_move(
    moves: &mut Vec<Move>,
    state: PieceState,
    target_row: u32,
    target_col: u32,
    captured: Option<PieceState>,
) {
    if let Some(mv) = create(
        state,
        state.square,
        Square { row: target_row, column: target_col, piece_state: Some(state) },
        state.color,
        captured,
        None,
        false,
    ) {
        let mut final_move = mv;
        
        if target_row == 8 {
            let promotion_pieces = [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight];
            for &promotion in &promotion_pieces {
                final_move.promotion = Some(promotion);
                moves.push(final_move.clone());
            }
        } else {
            moves.push(final_move);
        }
    }
}

pub fn get_pawn_moves(state: PieceState, board: Board) -> Vec<Move> {
    if state.piece != Some(state.square).piece {
        println!("Warning: State pieces misaligned");
        return vec![];
    }

    match state.color {
        Color::White => get_white_pawn_moves(state, board),
        Color::Black => get_black_pawn_moves(state, board)
    }
}

pub fn get_white_pawn_moves(state: PieceState, board: Board) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let cur_row: u32 = state.square.row;
    let cur_col: u32 = state.square.column;

    if cur_row == 2 && board.board[cur_row + 2][cur_col].piece.is_none() {
        add_pawn_move(&mut moves, state, cur_row + 2, cur_col, None);
    }

    if cur_row + 1 <= 8 && board.board[cur_row + 1][cur_col].piece.is_none() {
        add_pawn_move(&mut moves, state, cur_row + 1, cur_col, None);
    }

    if cur_row < 8 && cur_col < 8 {
        if let Some(target_piece) = board.board[cur_row + 1][cur_col + 1].piece {
            add_pawn_move(&mut moves, state, cur_row + 1, cur_col + 1, target_piece);
        }
    }

    if cur_row < 8 && cur_col > 1 {
        if let Some(target_piece) = board.board[cur_row + 1][cur_col - 1].piece {
            add_pawn_move(&mut moves, state, cur_row + 1, cur_col - 1, target_piece);
        }
    }

    moves
}


pub fn get_black_pawn_moves(state: PieceState, board: Board) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let cur_row: u32 = state.square.row;
    let cur_col: u32 = state.square.column;

    if cur_row == 7 && board.board[cur_row - 2][cur_col].piece.is_none() {
        add_pawn_move(&mut moves, state, cur_row - 2, cur_col, None);
    }

    if cur_row - 1 >= 1 && board.board[cur_row - 1][cur_col].piece.is_none() {
        add_pawn_move(&mut moves, state, cur_row - 1, cur_col, None);
    }

    if cur_row > 1 && cur_col > 1 {
        if let Some(target_piece) = board.board[cur_row - 1][cur_col - 1].piece {
            add_pawn_move(&mut moves, state, cur_row - 1, cur_col - 1, target_piece);
        }
    }

    if cur_row > 1 && cur_col < 8 {
        if let Some(target_piece) = board.board[cur_row - 1][cur_col + 1].piece {
            add_pawn_move(&mut moves, state, cur_row - 1, cur_col + 1, target_piece);
        }
    }

    moves
}


pub fn get_knight_moves(state: PieceState, board: Board) -> Vec<Move> {
    if state.piece != Some(state.square).piece {
        println!("Warning: State pieces misaligned");
        return vec![];
    }

    let mut moves: Vec<Move> = vec![];
    let cur_row: i32 = state.square.row as i32;
    let cur_col: i32 = state.square.column as i32;

    let knight_deltas: [(i32, i32); 8] = [
        (2, 1), (2, -1), (1, 2), (-1, 2),
        (-2, 1), (-2, -1), (-1, 2), (-1, -2)
    ];

    for &(row, col) in &knight_deltas {
        let target_row: u32 = (cur_row + row - 1) as u32;
        let target_col: u32 = (cur_col + col - 1) as u32;

        if target_row <= 8 && target_row >= 1 && target_col <= 8 && target_col >= 1 {
            let potential_piece: Option<PieceState> = board.board[target_row][target_col].piece;

            match potential_piece {
                Some(piece_state) if potential_piece.color != state.color => {
                    if let Some(mut mv) = create(
                        state,
                        state.square, 
                        Square { row: target_row, column: target_col, piece_state: Some(state)}, 
                        state.color, 
                        Some(piece_state), 
                        None, 
                        false 
                    ) 
                    {
                        moves.push(mv);
                    }
                },
                None => {
                    if let Some(mut mv) = create(
                        state,
                        state.square, 
                        Square { row: target_row, column: target_col, piece_state: Some(state)}, 
                        state.color, 
                        None, 
                        None, 
                        false 
                    ) 
                    {
                        moves.push(mv);
                    }
                }
                _ => {}
            }
        }
    }
    
    moves
}

pub fn get_bishop_moves(state: PieceState, board: Board) -> Vec<Move> {
    if state.piece != Some(state.square).piece {
        println!("Warning: State pieces misaligned");
        return vec![];
    }

    let mut moves: Vec<Move> = vec![];
    let cur_row: i32 = state.square.row as i32;
    let cur_col: i32 = state.square.column as i32;

    let directions: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    for &(dr, dc) in &directions {
        for offset in 1..8 {
            let target_row = (cur_row + dr * offset) as u32;
            let target_col = (cur_col + dc * offset) as u32;

            if !in_bounds(target_row, target_col) {
                break; 
            }

            if let Some(piece_state) = board.board[target_row][target_col].piece {
                if piece_state.color != state.color {
                    if let Some(mv) = create(
                        state,
                        state.square,
                        Square { row: target_row, column: target_col, piece_state: Some(state) },
                        state.color,
                        Some(piece_state),
                        None,
                        false,
                    ) 
                    {
                        moves.push(mv);
                    }
                }
                break;
            } else {
                if let Some(mv) = create(
                    state,
                    state.square,
                    Square { row: target_row, column: target_col, piece_state: Some(state) },
                    state.color,
                    None,
                    None,
                    false,
                ) 
                {
                    moves.push(mv);
                }
            }
        }
    }

    moves
}


pub fn get_rook_moves(state: PieceState, board: Board) -> Vec<Move> {
    if state.piece != Some(state.square).piece {
        println!("Warning: State pieces misaligned");
        return vec![];
    }

    let mut moves: Vec<Move> = vec![];
    let cur_row: i32 = state.square.row as i32;
    let cur_col: i32 = state.square.column as i32;

    let directions: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    for &(dr, dc) in &directions {
        for offset in 1..8 {
            let target_row = (cur_row + dr * offset) as u32;
            let target_col = (cur_col + dc * offset) as u32;

            if !in_bounds(target_row, target_col) {
                break; 
            }

            if let Some(piece_state) = board.board[target_row][target_col].piece {
                if piece_state.color != state.color {
                    if let Some(mv) = create(
                        state,
                        state.square,
                        Square { row: target_row, column: target_col, piece_state: Some(state) },
                        state.color,
                        Some(piece_state),
                        None,
                        false,
                    ) 
                    {
                        moves.push(mv);
                    }
                }
                break;
            } else {
                if let Some(mv) = create(
                    state,
                    state.square,
                    Square { row: target_row, column: target_col, piece_state: Some(state) },
                    state.color,
                    None,
                    None,
                    false,
                )
                {
                    moves.push(mv);
                }
            }
        }
    }

    moves
}

pub fn get_king_moves(state: PieceState, board: Board) -> Vec<Move> {
    if state.piece != Some(state.square).piece {
        println!("Warning: State pieces misaligned");
        return vec![];
    }

    let mut moves: Vec<Move> = vec![];
    let cur_row: i32 = state.square.row as i32;
    let cur_col: i32 = state.square.column as i32;

    let directions: [(i32, i32); 8] = [
        (1, 1), (1, -1), (-1, 1), (-1, -1),
        (1, 0), (-1, 0), (0, 1), (0, -1)
    ];

    for &(row_offset, col_offset) in &directions {
        let target_row = (cur_row + row_offset) as u32;
        let target_col = (cur_col + col_offset) as u32;

        if !in_bounds(target_row, target_col) {
            break; 
        }

        if let Some(piece_state) = board.board[target_row][target_col].piece {
            if piece_state.color != state.color {
                if let Some(mv) = create(
                    state,
                    state.square,
                    Square { row: target_row, column: target_col, piece_state: Some(state) },
                    state.color,
                    Some(piece_state),
                    None,
                    false,
                ) 
                {
                    moves.push(mv);
                }
            }
            break;
        } else {
            if let Some(mv) = create(
                state,
                state.square,
                Square { row: target_row, column: target_col, piece_state: Some(state) },
                state.color,
                None,
                None,
                false,
            ) 
            {
                moves.push(mv);
            }
        }
    }

    if !state.has_moved {
        if is_kingside_castling_valid(state, board, cur_row as u32) {
            if let Some(mv) = create(
                state,
                state.square,
                Square { row: cur_row as u32, column: 7, piece_state: Some(state) },
                state.color,
                None,
                None,
                true,
            ) 
            {
                moves.push(mv);
            }
        }

        if is_queenside_castling_valid(state, board, cur_row as u32) {
            if let Some(mv) = create(
                state,
                state.square,
                Square { row: cur_row as u32, column: 3, piece_state: Some(state) },
                state.color,
                None,
                None,
                true,
            ) 
            {
                moves.push(mv);
            }
        }
    }

    moves
}

fn is_kingside_castling_valid(king: PieceState, board: Board, row: u32) -> bool {
    if let Some(rook) = board.board[row][8].piece {
        if rook.piece == Piece::Rook && rook.color == king.color && !rook.has_moved {
            return board.board[row][6].piece.is_none() 
                && board.board[row][7].piece.is_none();
        }
    }
    false
}

fn is_queenside_castling_valid(king: PieceState, board: Board, row: u32) -> bool {
    if let Some(rook) = board.board[row][1].piece {
        if rook.piece == Piece::Rook && rook.color == king.color && !rook.has_moved {
            return board.board[row][2].piece.is_none() 
                && board.board[row][3].piece.is_none() 
                && board.board[row][4].piece.is_none();
        }
    }
    false
}

pub fn get_queen_moves(state: PieceState, board: Board) -> Vec<Move> {
    if state.piece != Some(state.square).piece {
        println!("Warning: State pieces misaligned");
        return vec![];
    }

    let mut moves: Vec<Move> = vec![];
    let cur_row: i32 = state.square.row as i32;
    let cur_col: i32 = state.square.column as i32;

    let directions: [(i32, i32); 8] = [
        (1, 1), (1, -1), (-1, 1), (-1, -1),
        (1, 0), (-1, 0), (0, 1), (0, -1)
    ];

    for &(dr, dc) in &directions {
        for offset in 1..8 {
            let target_row = (cur_row + dr * offset) as u32;
            let target_col = (cur_col + dc * offset) as u32;

            if !in_bounds(target_row, target_col) {
                break; 
            }

            if let Some(piece_state) = board.board[target_row][target_col].piece {
                if piece_state.color != state.color {
                    if let Some(mv) = create(
                        state,
                        state.square,
                        Square { row: target_row, column: target_col, piece_state: Some(state) },
                        state.color,
                        Some(piece_state),
                        None,
                        false,
                    ) 
                    {
                        moves.push(mv);
                    }
                }
                break;
            } else {
                if let Some(mv) = create(
                    state,
                    state.square,
                    Square { row: target_row, column: target_col, piece_state: Some(state) },
                    state.color,
                    None,
                    None,
                    false,
                ) 
                {
                    moves.push(mv);
                }
            }
        }
    }

    moves
}