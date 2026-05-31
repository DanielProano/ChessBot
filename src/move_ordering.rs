use crate::moves::*;
use crate::pieces::*;

pub struct MoveList {
    pub moves: [Move; 218],
    pub move_count: usize
}

impl MoveList {
    pub fn push(&mut self, mv: Move) {
        self.push(mv);
        self.move_count += 1;
    }

    pub fn extend(&mut self, moves: Vec<Move>) {
        for mv in moves {
            self.push(mv);
        }
    }

    pub fn order_moves(&mut self) {
        
    }

    pub fn generate_moves(&mut self, board_state: &BoardState,board: &Board) {
        for row in 0..=7 {
            for col in 0..=7 {
                if let Some(piece_state) = board.board[row][col].piece_state {
                    match piece_state.piece {
                        Piece::Pawn => {
                            match piece_state.color {
                                Color::White => self.extend(get_white_pawn_moves(piece_state, board)),
                                Color::Black => self.extend(get_black_pawn_moves(piece_state, board))
                            }
                        },
                        Piece::Bishop => {
                            self.extend(get_bishop_moves(piece_state, board))
                        },
                        Piece::Knight => {
                            self.extend(get_knight_moves(piece_state, board))
                        },
                        Piece::Rook => {
                            self.extend(get_rook_moves(piece_state, board))
                        },
                        Piece::Queen => {
                            self.extend(get_queen_moves(piece_state, board));
                        },
                        Piece::King => {
                            self.extend(get_king_moves(piece_state, board))
                        }
                    }
                }
            }
        }
    }
}