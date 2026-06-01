use crate::material::calculate_material;
use crate::move_ordering::*;
use crate::pieces::*;
use crate::moves::*;

use std::cmp::max;
use std::i32;


pub struct PVS;

impl PVS {
    fn search(&self, mut alpha: i32, beta: i32, depth_left: i32, board: &Board, color: Color) -> i32 {
        if depth_left == 0 {
            return match color {
                Color::White => calculate_material(board),
                Color::Black => - calculate_material(board)
            }
        }

        let mut move_list = MoveList::new();
        move_list.generate_moves(board, color);

        if move_list.move_count == 0 {
            return i32::MIN + 1;
        }

        for index in 0..move_list.move_count {
            let mut score: i32 = i32::MIN + 1;
            if index == 0 {
                let new_board = self.setup_new_board(board, index, &move_list);
                score = -self.search(-beta, -alpha, depth_left - 1, &new_board, self.switch_color(color));
            } else {
                let new_board = self.setup_new_board(board, index, &move_list);
                score = -self.search(-alpha - 1, -alpha, depth_left - 1, &new_board, self.switch_color(color));

                if score > alpha && score < beta {
                    score = -self.search(-beta, -alpha, depth_left - 1, &new_board, self.switch_color(color));
                }
            }

            alpha = max(alpha, score);
            if alpha >= beta {
                break;
            }
        }

        alpha
    }
    
    fn setup_new_board(&self, board: &Board, index: usize, move_list: &MoveList) -> Board {
        let mut new_board = board.clone();
        let prev_square = move_list.moves[index].previous_square;
        let new_square = move_list.moves[index].current_square;
        new_board.board[new_square.row - 1][new_square.column - 1] = new_square;
        new_board.board[prev_square.row - 1][prev_square.column - 1].piece_state = None;

        new_board
    }

    fn switch_color(&self, color: Color) -> Color {
        match color {
            Color::White => Color::Black,
            Color::Black => Color::White
        }
    }

    pub fn best_move(&self, depth_left: i32, board: &Board, color: Color) -> Option<Move> {
        let mut move_list = MoveList::new();
        move_list.generate_moves(board, color);

        if move_list.move_count == 0 {
            return None;
        }

        let mut best_move = move_list.moves[0];
        let mut best_score = i32::MIN + 1;
        let mut alpha = i32::MIN + 1;
        let beta = i32::MAX;

        for index in 0..move_list.move_count {
            let new_board = self.setup_new_board(board, index, &move_list);
            let score = -self.search(-beta, -alpha, depth_left - 1, &new_board, self.switch_color(color));

            if score > best_score {
                best_score = score;
                best_move = move_list.moves[index];
                alpha = score;
            }
        }

        Some(best_move)
    }
}
