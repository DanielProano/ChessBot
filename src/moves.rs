use crate::pieces::*;
use wasm_bindgen::prelude::*;

pub struct Move {
    previous_square: Square,
    current_square: Square,
    color: Color,
    captured_piece: Option<Piece>,
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
        else if mv.promotion != None && mv.color == Color::White && mv.previous_square.piece != None && mv.previous_square.piece != Piece::WhitePawn {
            return false;
        }
        else if mv.promotion != None && mv.color == Color::Black && mv.previous_square.piece != None && mv.previous_square.piece != Piece::BlackPawn {
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
    captured_piece: Option<Piece>, 
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


pub fn get_pawn_moves(state: PieceState, board: Board) -> Vec<Move> {
    if state.piece != Some(state.square).piece {
        println!("Warning: State pieces misaligned");
        return vec![];
    }

    match state.color {
        Color::White => get_white_pawn_moves(state, board),
        Color::Black => get_black_pawn_moves(state, board)
    };
}

pub fn get_white_pawn_moves(state: PieceState, board: Board) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let cur_row: u32 = state.square.row;
    let cur_col: u32 = state.square.column;

    if cur_row == 2 && cur_row + 2 <= 8 && board.board[cur_row + 2][cur_col].piece.is_none() {
        if let Some(mv) = create(
            state,
            state.square, 
            Square { row: cur_row + 2, column: cur_col, piece: Some(Piece::WhitePawn)},
            Color::White, 
            None, 
            None, 
            false 
        ) {
            moves.push(mv);
        }
    }

    if cur_row + 1 > 8 {
        return moves;
    }

    if board.board[cur_row + 1][cur_col].piece == None {
        if let Some(mut mv) = create(
            state,
            state.square, 
            Square { row: cur_row + 1, column: cur_col, piece: Some(Piece::WhitePawn)},
            Color::White, 
            None, 
            None, 
            false 
        ) {
            if cur_row + 1 == 8 {
                for &promotion_piece in &[Piece::WhiteBishop, Piece::WhiteKnight, Piece::WhiteQueen, Piece::WhiteRook] {
                    mv.promotion = Some(promotion_piece);
                    moves.push(mv);
                }
            } else {
                moves.push(mv);
            }
        }
    }

    if cur_col + 1 <= 8 && board.board[cur_row + 1][cur_col + 1].piece {
        if let Some(mut mv)= create(
            state,
            state.square, 
            Square { row: cur_row + 1, column: cur_col + 1, piece: Some(Piece::WhitePawn)}, 
            Color::White, 
            Some(board.board[cur_row + 1][cur_col + 1].piece), 
            None, 
            false 
        ) {
            if cur_row + 1 == 8 {
                for &promotion_piece in &[Piece::WhiteBishop, Piece::WhiteKnight, Piece::WhiteQueen, Piece::WhiteRook] {
                    mv.promotion = Some(promotion_piece);
                    moves.push(mv);
                }
            } else {
                moves.push(mv);
            }
        }
    }

    if cur_col - 1 >= 1 && board.board[cur_row + 1][cur_col - 1].piece {
        if let Some(mut mv) = create(
            state,
            state.square, 
            Square { row: cur_row + 1, column: cur_col - 1, piece: Some(Piece::WhitePawn)}, 
            Color::White, 
            Some(board.board[cur_row + 1][cur_col - 1].piece), 
            None, 
            false 
        ) {
            if cur_row + 1 == 8 {
                for &promotion_piece in &[Piece::WhiteBishop, Piece::WhiteKnight, Piece::WhiteQueen, Piece::WhiteRook] {
                    mv.promotion = Some(promotion_piece);
                    moves.push(mv);
                }
            } else {
                moves.push(mv);
            }
        }
    }
    
    moves
}


pub fn get_knight_moves(state: PieceState, board: Board) -> Vec<Move> {
    if state.piece != Some(state.square).piece {
        println!("Warning: State pieces misaligned");
        return vec![];
    }

    match state.color {
        Color::White => get_white_knight_moves(state, board),
        Color::Black => get_black_knight_moves(state, board)
    };
}

pub fn get_white_knight_moves(state: PieceState, board: Board) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let cur_row: u32 = state.square.row;
    let cur_col: u32 = state.square.column;

    if cur_row + 2 <= 8 && cur_col + 1 <= 8 {
        let mut potential_piece: Piece = board.board[cur_row + 2][cur_col + 1].piece;
        if potential_piece.to_color() != state.color {
            if !potential_piece.is_none() {
                if let Some(mut mv) = create(
                    state,
                    state.square, 
                    Square { row: cur_row + 2, column: cur_col + 1, piece: Some(Piece::WhiteKnight)}, 
                    Color::White, 
                    Some(potential_piece), 
                    None, 
                    false 
                ) {
                    moves.push(mv);
                }
            } else {
                if let Some(mut mv) = create(
                    state,
                    state.square, 
                    Square { row: cur_row + 2, column: cur_col + 1, piece: Some(Piece::WhiteKnight)}, 
                    Color::White, 
                    None, 
                    None, 
                    false 
                ) {
                    moves.push(mv);
                }
            }
        }
    }

    let mut potential_piece: Piece = board.board[cur_row + 2][cur_col + 1].piece;
    if cur_row + 2 <= 8 && cur_col + 1 <= 8 && potential_piece.to_color() != state.color {
        if !potential_piece.is_none() {
            if let Some(mut mv) = create(
                state,
                state.square, 
                Square { row: cur_row + 2, column: cur_col + 1, piece: Some(Piece::WhiteKnight)}, 
                Color::White, 
                Some(potential_piece), 
                None, 
                false 
            ) {
                moves.push(mv);
            }
        } else {
            if let Some(mut mv) = create(
                state,
                state.square, 
                Square { row: cur_row + 2, column: cur_col + 1, piece: Some(Piece::WhiteKnight)}, 
                Color::White, 
                None, 
                None, 
                false 
            ) {
                moves.push(mv);
            }
        }
    }
    
    moves
}

pub fn get_white_knight_moves(state: PieceState, board: Board) -> Vec<Move> {
    let mut moves = vec![];
    
}

pub fn get_bishop_moves(state: PieceState, board: Board) -> Vec<Move> {
    if state.piece != Some(state.square).piece {
        println!("Warning: State pieces misaligned");
        return vec![];
    }

    match state.color {
        Color::White => get_white_bishop_moves(state, board),
        Color::Black => get_black_bishop_moves(state, board)
    };
}

pub fn get_knight_moves(state: PieceState, board: Board) -> Vec<Move> {
    if state.piece != Some(state.square).piece {
        println!("Warning: State pieces misaligned");
        return vec![];
    }

    match state.color {
        Color::White => get_white_knight_moves(state, board),
        Color::Black => get_black_knight_moves(state, board)
    };
}

pub fn get_king_moves(state: PieceState, board: Board) -> Vec<Move> {
    if state.piece != Some(state.square).piece {
        println!("Warning: State pieces misaligned");
        return vec![];
    }

    match state.color {
        Color::White => get_white_king_moves(state, board),
        Color::Black => get_black_king_moves(state, board)
    };
}

pub fn get_queen_moves(state: PieceState, board: Board) -> Vec<Move> {
    if state.piece != Some(state.square).piece {
        println!("Warning: State pieces misaligned");
        return vec![];
    }

    match state.color {
        Color::White => get_white_queen_moves(state, board),
        Color::Black => get_black_queen_moves(state, board)
    };
}