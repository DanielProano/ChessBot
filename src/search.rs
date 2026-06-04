use crate::material::calculate_material;
use crate::move_ordering::*;
use crate::pieces::*;
use crate::moves::*;

use std::cmp::max;
use std::i32;


pub struct PVS;

impl PVS {
    fn search(&self, mut alpha: i32, beta: i32, depth_left: i32, board: &Board, color: Color, board_state: BoardState) -> i32 {
        if depth_left == 0 {
            return match color {
                Color::White => calculate_material(board),
                Color::Black => - calculate_material(board)
            }
        }

        let mut move_list = MoveList::new();
        move_list.generate_moves(board, color, board_state);

        if move_list.move_count == 0 {
            return i32::MIN + 1;
        }

        for index in 0..move_list.move_count {
            let score: i32 = if index == 0 {
                let new_board = self.setup_new_board(board, index, &move_list);
                -self.search(-beta, -alpha, depth_left - 1, &new_board, self.switch_color(color), board_state)
            } else {
                let new_board = self.setup_new_board(board, index, &move_list);
                let mut search_result = -self.search(-alpha - 1, -alpha, depth_left - 1, &new_board, self.switch_color(color), board_state);

                if search_result > alpha && search_result < beta {
                    search_result = -self.search(-beta, -alpha, depth_left - 1, &new_board, self.switch_color(color), board_state);
                }
                search_result
            };

            alpha = max(alpha, score);
            if alpha >= beta {
                break;
            }
        }

        alpha
    }
    
    fn setup_new_board(&self, board: &Board, index: usize, move_list: &MoveList) -> Board {
        let mut new_board: Board = board.clone();
        let mv = move_list.moves[index];
        let prev_square = mv.previous_square;
        let new_square = mv.current_square;
        
        if let Some(captured) = mv.captured_piece {
            if let Some(mut square) = access_board(&new_board, captured.location.0, captured.location.1) {
                square.piece_state = None;
            }
        }

        if let Some(mut square) = access_board(board, prev_square.row, prev_square.column) {
            if let Some(mut state) = square.piece_state {
                state.location = (new_square.row, new_square.column);
                state.has_moved = true;

                new_board.board[new_square.row - 1][new_square.column - 1].piece_state = Some(state);
            }

            new_board.board[prev_square.row - 1][prev_square.column - 1].piece_state = None;
        }

        if let Some(square) = access_board(&new_board, new_square.row, new_square.column) {
            if let Some(state) = square.piece_state {
                if state.piece == Piece::King && mv.castling == true {
                    match state.color {
                        Color::White => {
                            if state.location == (1, 3) {
                                let rook_state = new_board.board[0][0].piece_state;
                                new_board.board[0][0].piece_state = None;
                                new_board.board[0][3].piece_state = rook_state;
                            } else if state.location == (1, 7) {
                                let rook_state = new_board.board[0][7].piece_state;
                                new_board.board[0][7].piece_state = None;
                                new_board.board[0][5].piece_state = rook_state;
                            }
                        },
                        Color::Black => {
                            if state.location == (8, 3) {
                                let rook_state = new_board.board[7][0].piece_state;
                                new_board.board[7][0].piece_state = None;
                                new_board.board[7][3].piece_state = rook_state;
                            } else if state.location == (8, 7) {
                                let rook_state = new_board.board[7][7].piece_state;
                                new_board.board[7][7].piece_state = None;
                                new_board.board[7][5].piece_state = rook_state;
                            }
                        }
                    }
                }
            }
        }

        
        new_board
    }

    fn switch_color(&self, color: Color) -> Color {
        match color {
            Color::White => Color::Black,
            Color::Black => Color::White
        }
    }

    pub fn best_move(&self, depth_left: i32, board: &Board, color: Color, board_state: BoardState) -> Option<Move> {
        let mut move_list = MoveList::new();
        move_list.generate_moves(board, color, board_state);

        if move_list.move_count == 0 {
            return None;
        }

        let mut best_move = move_list.moves[0];
        let mut best_score = i32::MIN + 1;
        let mut alpha = i32::MIN + 1;
        let beta = i32::MAX;

        for index in 0..move_list.move_count {
            let new_board = self.setup_new_board(board, index, &move_list);
            let score = -self.search(-beta, -alpha, depth_left - 1, &new_board, self.switch_color(color), board_state);

            if score > best_score {
                best_score = score;
                best_move = move_list.moves[index];
                alpha = score;
            }
        }

        Some(best_move)
    }

    fn perft(&self, board: &Board, color: Color, depth: u32, board_state: BoardState) -> u64 {
        if depth == 0 {
            return 1;
        }

        let mut move_list = MoveList::new();
        move_list.generate_moves(board, color, board_state);

        let mut nodes: u64 = 0;
        
        for index in 0..move_list.move_count {
            let new_board = self.setup_new_board(board, index, &move_list);
            let next_color = match color {
                Color::White => Color::Black,
                Color::Black => Color::White,
            };
            nodes += self.perft(&new_board, next_color, depth - 1, board_state);
        }

        nodes
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

    // ---- best_move ----

    #[test]
    fn test_best_move_returns_none_on_empty_board() {
        let pvs = PVS;
        let board = empty_board();
        let board_state = empty_board_state();
        assert!(pvs.best_move(1, &board, Color::White, board_state).is_none());
    }

    #[test]
    fn test_best_move_returns_some_from_start() {
        let pvs = PVS;
        let board_state = empty_board_state();
        assert!(pvs.best_move(1, &START_BOARD, Color::White, board_state).is_some());
    }

    #[test]
    fn test_best_move_captures_free_queen() {
        // white king + pawn that can capture black queen, nothing else
        let mut board = empty_board();
        place_piece(&mut board, 1, 5, Piece::King, Color::White);
        place_piece(&mut board, 8, 5, Piece::King, Color::Black);
        place_piece(&mut board, 4, 4, Piece::Pawn, Color::White);
        place_piece(&mut board, 5, 5, Piece::Queen, Color::Black);

        let pvs = PVS;
        let board_state = empty_board_state();
        let mv = pvs.best_move(1, &board, Color::White, board_state).unwrap();
        assert!(
            mv.captured_piece.is_some(),
            "should capture the free queen"
        );
        assert_eq!(
            mv.captured_piece.unwrap().piece,
            Piece::Queen,
            "captured piece should be the queen"
        );
    }

    #[test]
    fn test_best_move_depth_2_still_returns_move() {
        let pvs = PVS;
        let board_state = empty_board_state();
        let result = pvs.best_move(2, &START_BOARD, Color::White, board_state);
        assert!(result.is_some());
    }

    #[test]
    fn test_best_move_prefers_capture_over_quiet() {
        let mut board = empty_board();
        place_piece(&mut board, 1, 5, Piece::King, Color::White);
        place_piece(&mut board, 8, 5, Piece::King, Color::Black);
        place_piece(&mut board, 4, 4, Piece::Rook, Color::White);
        place_piece(&mut board, 4, 7, Piece::Queen, Color::Black); // free queen on same row

        let pvs = PVS;
        let board_state = empty_board_state();
        let mv = pvs.best_move(1, &board, Color::White, board_state).unwrap();
        assert!(mv.captured_piece.is_some(), "should prefer capturing the queen");
    }

    // ---- switch_color ----

    #[test]
    fn test_switch_color_white_to_black() {
        let pvs = PVS;
        assert_eq!(pvs.switch_color(Color::White), Color::Black);
    }

    #[test]
    fn test_switch_color_black_to_white() {
        let pvs = PVS;
        assert_eq!(pvs.switch_color(Color::Black), Color::White);
    }

    #[test]
    fn test_switch_color_twice_returns_original() {
        let pvs = PVS;
        assert_eq!(pvs.switch_color(pvs.switch_color(Color::White)), Color::White);
        assert_eq!(pvs.switch_color(pvs.switch_color(Color::Black)), Color::Black);
    }

    // ---- setup_new_board ----

    #[test]
    fn test_setup_new_board_moves_piece() {
        let mut board = empty_board();
        place_piece(&mut board, 2, 4, Piece::Pawn, Color::White);

        let pvs = PVS;
        let mut move_list = MoveList::new();
        let board_state = empty_board_state();
        move_list.generate_moves(&board, Color::White, board_state);

        assert!(move_list.move_count > 0);
        let new_board = pvs.setup_new_board(&board, 0, &move_list);

        // origin square should be empty
        let prev = move_list.moves[0].previous_square;
        assert!(
            new_board.board[prev.row - 1][prev.column - 1].piece_state.is_none(),
            "origin square should be cleared"
        );

        // destination square should have the piece
        let dest = move_list.moves[0].current_square;
        assert!(
            new_board.board[dest.row - 1][dest.column - 1].piece_state.is_some(),
            "destination square should have the piece"
        );
    }

    #[test]
    fn test_setup_new_board_does_not_mutate_original() {
        let mut board = empty_board();
        place_piece(&mut board, 2, 4, Piece::Pawn, Color::White);
        let original = board.clone();

        let pvs = PVS;
        let mut move_list = MoveList::new();
        let board_state = empty_board_state();
        move_list.generate_moves(&board, Color::White, board_state);
        pvs.setup_new_board(&board, 0, &move_list);

        assert_eq!(board, original, "original board should not be mutated");
    }

    // ---- search depth 0 ----

    #[test]
    fn test_search_depth_0_white_returns_material() {
        let pvs = PVS;
        let mut board = empty_board();
        let board_state = empty_board_state();
        place_piece(&mut board, 1, 5, Piece::King, Color::White);
        place_piece(&mut board, 8, 5, Piece::King, Color::Black);
        place_piece(&mut board, 4, 4, Piece::Queen, Color::White);

        // at depth 0 white is up a queen so score should be positive
        let score = pvs.search(i32::MIN + 1, i32::MAX, 0, &board, Color::White, board_state);
        assert!(score > 0, "white up a queen should score positive at depth 0");
    }

    #[test]
    fn test_search_depth_0_black_flips_score() {
        let pvs = PVS;
        let mut board = empty_board();
        let board_state = empty_board_state();
        place_piece(&mut board, 1, 5, Piece::King, Color::White);
        place_piece(&mut board, 8, 5, Piece::King, Color::Black);
        place_piece(&mut board, 4, 4, Piece::Queen, Color::White);

        // from black's perspective white is winning so score should be negative
        let score = pvs.search(i32::MIN + 1, i32::MAX, 0, &board, Color::Black, board_state);
        assert!(score < 0, "white up a queen should score negative for black at depth 0");
    }

    #[test]
    fn test_search_no_moves_returns_min() {
        let pvs = PVS;
        let board = empty_board();
        let board_state = empty_board_state();
        let score = pvs.search(i32::MIN + 1, i32::MAX, 3, &board, Color::White, board_state);
        assert_eq!(score, i32::MIN + 1, "no legal moves should return MIN+1");
    }
    // ---- START_BOARD structure ----

    #[test]
    fn test_start_board_white_back_rank() {
        let back_rank = [
            Piece::Rook, Piece::Knight, Piece::Bishop, Piece::Queen,
            Piece::King, Piece::Bishop, Piece::Knight, Piece::Rook,
        ];
        for (col, &piece) in back_rank.iter().enumerate() {
            let sq = START_BOARD.board[0][col];
            let state = sq.piece_state.expect("back rank should be occupied");
            assert_eq!(state.color, Color::White, "col {} should be white", col);
            assert_eq!(state.piece, piece, "col {} piece mismatch", col);
            assert_eq!(sq.row, 1);
            assert_eq!(sq.column, col + 1);
        }
    }

    #[test]
    fn test_start_board_black_back_rank() {
        let back_rank = [
            Piece::Rook, Piece::Knight, Piece::Bishop, Piece::Queen,
            Piece::King, Piece::Bishop, Piece::Knight, Piece::Rook,
        ];
        for (col, &piece) in back_rank.iter().enumerate() {
            let sq = START_BOARD.board[7][col];
            let state = sq.piece_state.expect("back rank should be occupied");
            assert_eq!(state.color, Color::Black, "col {} should be black", col);
            assert_eq!(state.piece, piece, "col {} piece mismatch", col);
            assert_eq!(sq.row, 8);
            assert_eq!(sq.column, col + 1);
        }
    }

    #[test]
    fn test_start_board_white_pawns() {
        for col in 0..8 {
            let sq = START_BOARD.board[1][col];
            let state = sq.piece_state.expect("pawn rank should be occupied");
            assert_eq!(state.color, Color::White);
            assert_eq!(state.piece, Piece::Pawn);
            assert_eq!(sq.row, 2);
            assert_eq!(sq.column, col + 1);
        }
    }

    #[test]
    fn test_start_board_black_pawns() {
        for col in 0..8 {
            let sq = START_BOARD.board[6][col];
            let state = sq.piece_state.expect("pawn rank should be occupied");
            assert_eq!(state.color, Color::Black);
            assert_eq!(state.piece, Piece::Pawn);
            assert_eq!(sq.row, 7);
            assert_eq!(sq.column, col + 1);
        }
    }

    #[test]
    fn test_start_board_middle_ranks_empty() {
        for row in 2..6 {
            for col in 0..8 {
                assert!(
                    START_BOARD.board[row][col].piece_state.is_none(),
                    "row {} col {} should be empty", row, col
                );
            }
        }
    }

    #[test]
    fn test_start_board_has_32_pieces() {
        let count = START_BOARD.board.iter().flatten()
            .filter(|sq| sq.piece_state.is_some())
            .count();
        assert_eq!(count, 32);
    }

    #[test]
    fn test_start_board_has_16_white_pieces() {
        let count = START_BOARD.board.iter().flatten()
            .filter_map(|sq| sq.piece_state)
            .filter(|s| s.color == Color::White)
            .count();
        assert_eq!(count, 16);
    }

    #[test]
    fn test_start_board_has_16_black_pieces() {
        let count = START_BOARD.board.iter().flatten()
            .filter_map(|sq| sq.piece_state)
            .filter(|s| s.color == Color::Black)
            .count();
        assert_eq!(count, 16);
    }

    #[test]
    fn test_start_board_piece_locations_match_square_coords() {
        for row in 0..8 {
            for col in 0..8 {
                let sq = START_BOARD.board[row][col];
                assert_eq!(sq.row, row + 1, "square row mismatch at [{row}][{col}]");
                assert_eq!(sq.column, col + 1, "square col mismatch at [{row}][{col}]");
                if let Some(state) = sq.piece_state {
                    assert_eq!(state.location.0, row + 1, "piece location row mismatch at [{row}][{col}]");
                    assert_eq!(state.location.1, col + 1, "piece location col mismatch at [{row}][{col}]");
                }
            }
        }
    }

    #[test]
    fn test_start_board_no_pieces_have_moved() {
        for sq in START_BOARD.board.iter().flatten() {
            if let Some(state) = sq.piece_state {
                assert!(!state.has_moved, "no piece should have moved at start");
                assert!(!state.dead, "no piece should be dead at start");
            }
        }
    }

    // ---- EMPTY_BOARD structure ----

    #[test]
    fn test_empty_board_has_no_pieces() {
        let count = EMPTY_BOARD.board.iter().flatten()
            .filter(|sq| sq.piece_state.is_some())
            .count();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_empty_board_square_coords_correct() {
        for row in 0..8 {
            for col in 0..8 {
                let sq = EMPTY_BOARD.board[row][col];
                assert_eq!(sq.row, row + 1, "row mismatch at [{row}][{col}]");
                assert_eq!(sq.column, col + 1, "col mismatch at [{row}][{col}]");
            }
        }
    }

    // ---- Piece::to_char ----

    #[test]
    fn test_white_piece_chars() {
        assert_eq!(Piece::Pawn.to_char(Color::White), "P");
        assert_eq!(Piece::Rook.to_char(Color::White), "R");
        assert_eq!(Piece::Knight.to_char(Color::White), "N");
        assert_eq!(Piece::Bishop.to_char(Color::White), "B");
        assert_eq!(Piece::Queen.to_char(Color::White), "Q");
        assert_eq!(Piece::King.to_char(Color::White), "K");
    }

    #[test]
    fn test_black_piece_chars() {
        assert_eq!(Piece::Pawn.to_char(Color::Black), "p");
        assert_eq!(Piece::Rook.to_char(Color::Black), "r");
        assert_eq!(Piece::Knight.to_char(Color::Black), "n");
        assert_eq!(Piece::Bishop.to_char(Color::Black), "b");
        assert_eq!(Piece::Queen.to_char(Color::Black), "q");
        assert_eq!(Piece::King.to_char(Color::Black), "k");
    }

    // ---- Color / Piece equality ----

    #[test]
    fn test_color_equality() {
        assert_eq!(Color::White, Color::White);
        assert_eq!(Color::Black, Color::Black);
        assert_ne!(Color::White, Color::Black);
    }

    #[test]
    fn test_piece_equality() {
        assert_eq!(Piece::King, Piece::King);
        assert_ne!(Piece::King, Piece::Queen);
    }

     #[test]
    fn perft_depth_1() {
        let pvs = PVS;
        let board_state = empty_board_state();
        assert_eq!(pvs.perft(&START_BOARD, Color::White, 1, board_state), 20);
    }

    #[test]
    fn perft_depth_2() {
        let pvs = PVS;
        let board_state = empty_board_state();
        assert_eq!(pvs.perft(&START_BOARD, Color::White, 2, board_state), 400);
    }

    #[test]
    fn perft_depth_3() {
        let pvs = PVS;
        let board_state = empty_board_state();
        assert_eq!(pvs.perft(&START_BOARD, Color::White, 3, board_state), 8902);
    }

     #[test]
    fn perft_depth_4() {
        let pvs = PVS;
        let board_state = empty_board_state();
        assert_eq!(pvs.perft(&START_BOARD, Color::White, 4, board_state), 197281);
    }

    #[test]
    fn perft_depth_5() {
        let pvs = PVS;
        let board_state = empty_board_state();
        assert_eq!(pvs.perft(&START_BOARD, Color::White, 5, board_state), 4865609)
    }
}