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

    pub fn generate_moves(&mut self, board: &Board, color: Color, board_state: BoardState) {
        for row in 0..=7 {
            for col in 0..=7 {
                if let Some(piece_state) = board.board[row][col].piece_state {
                    if piece_state.color == color {
                        match piece_state.piece {
                            Piece::Pawn => {
                                match piece_state.color {
                                    Color::White => self.extend(get_white_pawn_moves(piece_state, board, board_state)),
                                    Color::Black => self.extend(get_black_pawn_moves(piece_state, board, board_state))
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

    // ---- generate_moves ----

    #[test]
    fn test_generate_moves_empty_board() {
        let board = empty_board();
        let board_state = empty_board_state();
        let mut move_list = MoveList::new();
        move_list.generate_moves(&board, Color::White, board_state);
        assert_eq!(move_list.move_count, 0);
    }

    #[test]
    fn test_generate_moves_only_generates_for_active_color() {
        let mut board = empty_board();
        let board_state = empty_board_state();
        place_piece(&mut board, 2, 1, Piece::Pawn, Color::White);
        place_piece(&mut board, 7, 1, Piece::Pawn, Color::Black);

        let mut white_moves = MoveList::new();
        white_moves.generate_moves(&board, Color::White, board_state);

        let mut black_moves = MoveList::new();
        black_moves.generate_moves(&board, Color::Black, board_state);

        // white should not generate black's moves and vice versa
        assert!(white_moves.move_count > 0);
        assert!(black_moves.move_count > 0);
        assert_eq!(white_moves.move_count, black_moves.move_count);
    }

    #[test]
    fn test_starting_position_white_has_20_moves() {
        // in the starting position white has exactly 20 legal moves
        // (16 pawn moves + 4 knight moves)
        let mut move_list = MoveList::new();
        let board_state = empty_board_state();
        move_list.generate_moves(&START_BOARD, Color::White, board_state);
        assert_eq!(move_list.move_count, 20);
    }

    #[test]
    fn test_starting_position_black_has_20_moves() {
        let mut move_list = MoveList::new();
        let board_state = empty_board_state();
        move_list.generate_moves(&START_BOARD, Color::Black, board_state);
        assert_eq!(move_list.move_count, 20);
    }

    // ---- score_moves ----

    #[test]
    fn test_score_moves_capture_scores_higher() {
        let mut board = empty_board();
        // white pawn can either move forward or capture a black piece
        place_piece(&mut board, 2, 4, Piece::Pawn, Color::White);
        place_piece(&mut board, 3, 5, Piece::Queen, Color::Black); // capturable queen

        let mut move_list = MoveList::new();
        let board_state = empty_board_state();
        move_list.generate_moves(&board, Color::White, board_state);
        move_list.score_moves(&board);

        // at least one score should be positive (capturing the queen)
        let max_score = move_list.score[..move_list.move_count].iter().copied().max().unwrap();
        assert!(max_score > 0, "capturing a queen should produce a positive score");
    }

    #[test]
    fn test_score_moves_scores_all_generated_moves() {
        let mut move_list = MoveList::new();
        let board_state = empty_board_state();
        move_list.generate_moves(&START_BOARD, Color::White, board_state);
        let count = move_list.move_count;
        move_list.score_moves(&START_BOARD);

        // score array beyond move_count should still be 0 (untouched)
        for i in count..218 {
            assert_eq!(move_list.score[i], 0, "ungenerated moves should not be scored");
        }
    }

    // ---- order_moves ----

    #[test]
    fn test_order_moves_descending() {
        let mut board = empty_board();
        place_piece(&mut board, 2, 4, Piece::Pawn, Color::White);
        place_piece(&mut board, 3, 5, Piece::Queen, Color::Black);

        let mut move_list = MoveList::new();
        let board_state = empty_board_state();
        move_list.generate_moves(&board, Color::White, board_state);
        move_list.score_moves(&board);
        move_list.order_moves();

        // scores should be in descending order
        for i in 0..move_list.move_count.saturating_sub(1) {
            assert!(
                move_list.score[i] >= move_list.score[i + 1],
                "moves should be ordered highest score first"
            );
        }
    }

    #[test]
    fn test_order_moves_capture_first() {
        let mut board = empty_board();
        place_piece(&mut board, 2, 4, Piece::Pawn, Color::White);
        place_piece(&mut board, 3, 5, Piece::Queen, Color::Black);

        let mut move_list = MoveList::new();
        let board_state = empty_board_state();
        move_list.generate_moves(&board, Color::White, board_state);
        move_list.score_moves(&board);
        move_list.order_moves();

        // the first move should be the capture (highest score)
        let first_move = move_list.moves[0];
        assert!(
            first_move.captured_piece.is_some(),
            "capture should be ordered first"
        );
    }

    // ---- push / extend ----

    #[test]
    fn test_push_increments_count() {
        let mut move_list = MoveList::new();
        let board_state = empty_board_state();
        assert_eq!(move_list.move_count, 0);
        move_list.generate_moves(&START_BOARD, Color::White, board_state);
        assert!(move_list.move_count > 0);
    }

    #[test]
    fn test_new_move_list_is_empty() {
        let move_list = MoveList::new();
        assert_eq!(move_list.move_count, 0);
    }
}