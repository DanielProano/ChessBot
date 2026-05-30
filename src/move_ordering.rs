use crate::moves::*;
use crate::pieces::*;

pub fn generate_moves(state: &BoardState,board: &Board) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    for row in 0..=7 {
        for col in 0..=7 {
            if let Some(piece_state) = board.board[row][col].piece_state {
                match piece_state.piece {
                    Piece::Pawn => {
                        match piece_state.color {
                            Color::White => moves.extend(get_white_pawn_moves(piece_state, board)),
                            Color::Black => moves.extend(get_black_pawn_moves(piece_state, board))
                        }
                    },
                    Piece::Bishop => {
                        moves.extend(get_bishop_moves(piece_state, board))
                    },
                    Piece::Knight => {
                        moves.extend(get_knight_moves(piece_state, board))
                    },
                    Piece::Rook => {
                        moves.extend(get_rook_moves(piece_state, board))
                    },
                    Piece::Queen => {
                        moves.extend(get_queen_moves(piece_state, board));
                    },
                    Piece::King => {
                        moves.extend(get_king_moves(piece_state, board))
                    }
                }
            }
        }
    }

    moves
}

pub fn order_moves(moves: Vec<Move>) -> Vec<Move> {
    
}