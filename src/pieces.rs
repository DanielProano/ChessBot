use wasm_bindgen::prelude::*;

use crate::fen::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl Piece {
    pub fn to_char(self, color: Color) -> String {
        match color {
            Color::White => match self {
                Piece::Pawn => 'P'.to_string(),
                Piece::Rook => 'R'.to_string(),
                Piece::Knight => 'N'.to_string(),
                Piece::Bishop => 'B'.to_string(),
                Piece::Queen => 'Q'.to_string(),
                Piece::King => 'K'.to_string(),
            },
            Color::Black => match self {
                Piece::Pawn => 'p'.to_string(),
                Piece::Rook => 'r'.to_string(),
                Piece::Knight => 'n'.to_string(),
                Piece::Bishop => 'b'.to_string(),
                Piece::Queen => 'q'.to_string(),
                Piece::King => 'k'.to_string(),
            },
        }
    }
}

// Only tracks whether rooks or kings have moved
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CastlingRights {
    pub castle_kingside: bool,
    pub castle_queenside: bool
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrawConditions {
    pub draw: bool,
    pub fifty_move_counter: usize,
    pub threefold_counter: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Time {
    pub alloted_time: u32,
    pub cur_time: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PieceState {
    pub color: Color,
    pub piece: Piece,
    pub location: (usize, usize),
    pub has_moved: bool,
    pub dead: bool
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square {
    pub row: usize,
    pub column: usize,
    pub piece_state: Option<PieceState>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    pub board: [[Square; 8]; 8],
}

pub struct CheckMask {
    pub check_mask: [[bool; 8]; 8]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorState {
    pub color: Color,
    pub in_check: bool,
    pub en_passant: Option<Square>,
    pub castling: CastlingRights
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoardState {
    pub board: Board,
    pub active_color: Color,
    pub white_state: ColorState,
    pub black_state: ColorState,
    pub draw: DrawConditions,
    pub time: Option<Time>,
}

#[wasm_bindgen]
impl BoardState {
    fn start() -> Self {
        let mut white_piece_states = vec![];

        for row in 6..8 {
            for col in 0..8 {
                if let Some(state) = START_BOARD.board[row][col].piece_state {
                    white_piece_states.push(PieceState {
                        color: Color::White,
                        piece: state.piece,
                        location: (row, col),
                        has_moved: false,
                        dead: false
                    });
                }
            }
        }

        let white_color_state: ColorState = ColorState {
            color: Color::White,
            in_check: false,
            en_passant: None,
            castling: CastlingRights {
                castle_kingside: true,
                castle_queenside: true,
            },
        };

        let mut black_piece_states = vec![];
        for row in 0..2 {
            for col in 0..8 {
                if let Some(state) = START_BOARD.board[row][col].piece_state {
                    black_piece_states.push(PieceState {
                        color: Color::Black,
                        piece: state.piece,
                        location: (row, col),
                        has_moved: false,
                        dead: false
                    });
                }
            }
        }

        let black_color_state: ColorState = ColorState {
            color: Color::Black,
            in_check: false,
            en_passant: None,
            castling: CastlingRights {
                castle_kingside: true,
                castle_queenside: true,
            },
        };

        Self {
            board: START_BOARD,
            active_color: Color::White,
            black_state: black_color_state,
            white_state: white_color_state,
            draw: DrawConditions {
                draw: false,
                fifty_move_counter: 0,
                threefold_counter: 0,
            },
            time: None,
        }
    }
}

pub static START_BOARD: Board = Board {
    board: [
        [
            Square { row: 1, column: 1, piece_state: Some(PieceState {color: Color::White, piece: Piece::Rook, location: (1, 1), has_moved: false, dead: false  }) },
            Square { row: 1, column: 2, piece_state: Some(PieceState {color: Color::White, piece: Piece::Knight, location: (1, 2), has_moved: false, dead: false  }) },
            Square { row: 1, column: 3, piece_state: Some(PieceState {color: Color::White, piece: Piece::Bishop, location: (1, 3), has_moved: false, dead: false  }) },
            Square { row: 1, column: 4, piece_state: Some(PieceState {color: Color::White, piece: Piece::Queen, location: (1, 4), has_moved: false, dead: false  }) },
            Square { row: 1, column: 5, piece_state: Some(PieceState {color: Color::White, piece: Piece::King, location: (1, 5), has_moved: false, dead: false  }) },
            Square { row: 1, column: 6, piece_state: Some(PieceState {color: Color::White, piece: Piece::Bishop, location: (1, 6), has_moved: false, dead: false  }) },
            Square { row: 1, column: 7, piece_state: Some(PieceState {color: Color::White, piece: Piece::Knight, location: (1, 7), has_moved: false, dead: false  }) },
            Square { row: 1, column: 8, piece_state: Some(PieceState {color: Color::White, piece: Piece::Rook, location: (1, 8), has_moved: false, dead: false  }) },
        ],
        [
            Square { row: 2, column: 1, piece_state: Some(PieceState {color: Color::White, piece: Piece::Pawn, location: (2, 1), has_moved: false, dead: false }) },
            Square { row: 2, column: 2, piece_state: Some(PieceState {color: Color::White, piece: Piece::Pawn, location: (2, 2), has_moved: false, dead: false}) },
            Square { row: 2, column: 3, piece_state: Some(PieceState {color: Color::White, piece: Piece::Pawn, location: (2, 3), has_moved: false, dead: false }) },
            Square { row: 2, column: 4, piece_state: Some(PieceState {color: Color::White, piece: Piece::Pawn, location: (2, 4), has_moved: false, dead: false }) },
            Square { row: 2, column: 5, piece_state: Some(PieceState {color: Color::White, piece: Piece::Pawn, location: (2, 5), has_moved: false, dead: false }) },
            Square { row: 2, column: 6, piece_state: Some(PieceState {color: Color::White, piece: Piece::Pawn, location: (2, 6), has_moved: false, dead: false }) },
            Square { row: 2, column: 7, piece_state: Some(PieceState {color: Color::White, piece: Piece::Pawn, location: (2, 7), has_moved: false, dead: false }) },
            Square { row: 2, column: 8, piece_state: Some(PieceState {color: Color::White, piece: Piece::Pawn, location: (2, 8), has_moved: false, dead: false }) },
        ],
        [
            Square { row: 3, column: 1, piece_state: None },
            Square { row: 3, column: 2, piece_state: None },
            Square { row: 3, column: 3, piece_state: None },
            Square { row: 3, column: 4, piece_state: None },
            Square { row: 3, column: 5, piece_state: None },
            Square { row: 3, column: 6, piece_state: None },
            Square { row: 3, column: 7, piece_state: None },
            Square { row: 3, column: 8, piece_state: None },
        ],
        [
            Square { row: 4, column: 1, piece_state: None },
            Square { row: 4, column: 2, piece_state: None },
            Square { row: 4, column: 3, piece_state: None },
            Square { row: 4, column: 4, piece_state: None },
            Square { row: 4, column: 5, piece_state: None },
            Square { row: 4, column: 6, piece_state: None },
            Square { row: 4, column: 7, piece_state: None },
            Square { row: 4, column: 8, piece_state: None },
        ],
        [
            Square { row: 5, column: 1, piece_state: None },
            Square { row: 5, column: 2, piece_state: None },
            Square { row: 5, column: 3, piece_state: None },
            Square { row: 5, column: 4, piece_state: None },
            Square { row: 5, column: 5, piece_state: None },
            Square { row: 5, column: 6, piece_state: None },
            Square { row: 5, column: 7, piece_state: None },
            Square { row: 5, column: 8, piece_state: None },
        ],
        [
            Square { row: 6, column: 1, piece_state: None },
            Square { row: 6, column: 2, piece_state: None },
            Square { row: 6, column: 3, piece_state: None },
            Square { row: 6, column: 4, piece_state: None },
            Square { row: 6, column: 5, piece_state: None },
            Square { row: 6, column: 6, piece_state: None },
            Square { row: 6, column: 7, piece_state: None },
            Square { row: 6, column: 8, piece_state: None },
        ],
        [
            Square { row: 7, column: 1, piece_state: Some(PieceState {color: Color::Black, piece: Piece::Pawn, location: (7, 1), has_moved: false, dead: false }) },
            Square { row: 7, column: 2, piece_state: Some(PieceState {color: Color::Black, piece: Piece::Pawn, location: (7, 2), has_moved: false, dead: false}) },
            Square { row: 7, column: 3, piece_state: Some(PieceState {color: Color::Black, piece: Piece::Pawn, location: (7, 3), has_moved: false, dead: false }) },
            Square { row: 7, column: 4, piece_state: Some(PieceState {color: Color::Black, piece: Piece::Pawn, location: (7, 4), has_moved: false, dead: false }) },
            Square { row: 7, column: 5, piece_state: Some(PieceState {color: Color::Black, piece: Piece::Pawn, location: (7, 5), has_moved: false, dead: false }) },
            Square { row: 7, column: 6, piece_state: Some(PieceState {color: Color::Black, piece: Piece::Pawn, location: (7, 6), has_moved: false, dead: false }) },
            Square { row: 7, column: 7, piece_state: Some(PieceState {color: Color::Black, piece: Piece::Pawn, location: (7, 7), has_moved: false, dead: false }) },
            Square { row: 7, column: 8, piece_state: Some(PieceState {color: Color::Black, piece: Piece::Pawn, location: (7, 8), has_moved: false, dead: false }) },
        ],
        [
            Square { row: 8, column: 1, piece_state: Some(PieceState {color: Color::Black, piece: Piece::Rook, location: (8, 1), has_moved: false, dead: false }) },
            Square { row: 8, column: 2, piece_state: Some(PieceState {color: Color::Black, piece: Piece::Knight, location: (8, 2), has_moved: false, dead: false}) },
            Square { row: 8, column: 3, piece_state: Some(PieceState {color: Color::Black, piece: Piece::Bishop, location: (8, 3), has_moved: false, dead: false }) },
            Square { row: 8, column: 4, piece_state: Some(PieceState {color: Color::Black, piece: Piece::Queen, location: (8, 4), has_moved: false, dead: false }) },
            Square { row: 8, column: 5, piece_state: Some(PieceState {color: Color::Black, piece: Piece::King, location: (8, 5), has_moved: false, dead: false }) },
            Square { row: 8, column: 6, piece_state: Some(PieceState {color: Color::Black, piece: Piece::Bishop, location: (8, 6), has_moved: false, dead: false }) },
            Square { row: 8, column: 7, piece_state: Some(PieceState {color: Color::Black, piece: Piece::Knight, location: (8, 7), has_moved: false, dead: false }) },
            Square { row: 8, column: 8, piece_state: Some(PieceState {color: Color::Black, piece: Piece::Rook, location: (8, 8), has_moved: false, dead: false }) },
        ],
    ],
};

pub static EMPTY_BOARD: Board = Board {
    board: 
    [
        [
            Square { row: 1, column: 1, piece_state: None },
            Square { row: 1, column: 2, piece_state: None },
            Square { row: 1, column: 3, piece_state: None },
            Square { row: 1, column: 4, piece_state: None },
            Square { row: 1, column: 5, piece_state: None },
            Square { row: 1, column: 6, piece_state: None },
            Square { row: 1, column: 7, piece_state: None },
            Square { row: 1, column: 8, piece_state: None },
        ],
        [
            Square { row: 2, column: 1, piece_state: None },
            Square { row: 2, column: 2, piece_state: None },
            Square { row: 2, column: 3, piece_state: None },
            Square { row: 2, column: 4, piece_state: None },
            Square { row: 2, column: 5, piece_state: None },
            Square { row: 2, column: 6, piece_state: None },
            Square { row: 2, column: 7, piece_state: None },
            Square { row: 2, column: 8, piece_state: None },
        ],
        [
            Square { row: 3, column: 1, piece_state: None },
            Square { row: 3, column: 2, piece_state: None },
            Square { row: 3, column: 3, piece_state: None },
            Square { row: 3, column: 4, piece_state: None },
            Square { row: 3, column: 5, piece_state: None },
            Square { row: 3, column: 6, piece_state: None },
            Square { row: 3, column: 7, piece_state: None },
            Square { row: 3, column: 8, piece_state: None },
        ],
        [
            Square { row: 4, column: 1, piece_state: None },
            Square { row: 4, column: 2, piece_state: None },
            Square { row: 4, column: 3, piece_state: None },
            Square { row: 4, column: 4, piece_state: None },
            Square { row: 4, column: 5, piece_state: None },
            Square { row: 4, column: 6, piece_state: None },
            Square { row: 4, column: 7, piece_state: None },
            Square { row: 4, column: 8, piece_state: None },
        ],
        [
            Square { row: 5, column: 1, piece_state: None },
            Square { row: 5, column: 2, piece_state: None },
            Square { row: 5, column: 3, piece_state: None },
            Square { row: 5, column: 4, piece_state: None },
            Square { row: 5, column: 5, piece_state: None },
            Square { row: 5, column: 6, piece_state: None },
            Square { row: 5, column: 7, piece_state: None },
            Square { row: 5, column: 8, piece_state: None },
        ],
        [
            Square { row: 6, column: 1, piece_state: None },
            Square { row: 6, column: 2, piece_state: None },
            Square { row: 6, column: 3, piece_state: None },
            Square { row: 6, column: 4, piece_state: None },
            Square { row: 6, column: 5, piece_state: None },
            Square { row: 6, column: 6, piece_state: None },
            Square { row: 6, column: 7, piece_state: None },
            Square { row: 6, column: 8, piece_state: None },
        ],
        [
            Square { row: 7, column: 1, piece_state: None },
            Square { row: 7, column: 2, piece_state: None },
            Square { row: 7, column: 3, piece_state: None },
            Square { row: 7, column: 4, piece_state: None },
            Square { row: 7, column: 5, piece_state: None },
            Square { row: 7, column: 6, piece_state: None },
            Square { row: 7, column: 7, piece_state: None },
            Square { row: 7, column: 8, piece_state: None },
        ],
        [
            Square { row: 8, column: 1, piece_state: None },
            Square { row: 8, column: 2, piece_state: None },
            Square { row: 8, column: 3, piece_state: None },
            Square { row: 8, column: 4, piece_state: None },
            Square { row: 8, column: 5, piece_state: None },
            Square { row: 8, column: 6, piece_state: None },
            Square { row: 8, column: 7, piece_state: None },
            Square { row: 8, column: 8, piece_state: None },
        ]
    ]
};

#[cfg(test)]
mod tests {
    use super::*;

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
}