use crate::moves::{EMPTY_MOVE, Move, get_white_pawn_moves, get_black_pawn_moves, get_bishop_moves, get_knight_moves, get_rook_moves, get_queen_moves, get_king_moves};
use crate::pieces::*;
use crate::material::*;

pub struct MoveList {
    pub moves: [Move; 218],
    pub score: [i32; 218],
    pub move_count: usize
}

impl MoveList {
    pub fn new() -> Self {
        Self {
            moves: [EMPTY_MOVE; 218],
            score: [0; 218],
            move_count: 0
        }
    }

    pub fn push(&mut self, mv: Move) {
        self.moves[self.move_count] = mv;
        self.move_count += 1;
    }

    pub fn extend(&mut self, moves: Vec<Move>) {
        for mv in moves {
            self.push(mv);
        }
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

    pub fn score_moves(&mut self, board: &Board) {
        let count = self.move_count;

        for idx in 0..count {
            let mv: Move = self.moves[idx];
            let prev_square = mv.previous_square;
            let new_square = mv.current_square;
            let mut new_board = board.clone();

            new_board.board[prev_square.row - 1][prev_square.column - 1].piece_state = None;
            new_board.board[new_square.row - 1][new_square.column - 1] = new_square;
            self.score[idx] = calculate_material(&new_board);
        }
    }


    pub fn order_moves(&mut self) {
        let count = self.move_count;

        let mut pairs: Vec<(i32, Move)> = self.score[..count]
            .iter()
            .copied()
            .zip(self.moves[..count].iter().copied())
            .collect();

        pairs.sort_unstable_by_key(|&(score, _)| std::cmp::Reverse(score));

        for (idx, (score, mv)) in pairs.into_iter().enumerate() {
            self.score[idx] = score;
            self.moves[idx] = mv;
        }
    }
}