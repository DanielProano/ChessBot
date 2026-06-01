use crate::moves::*;
use wasm_bindgen::prelude::*;

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
    castle_kingside: bool,
    castle_queenside: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnPassant {
    target: Square,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrawConditions {
    draw: bool,
    fifty_move_counter: usize,
    threefold_counter: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Time {
    alloted_time: u32,
    cur_time: u32,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorState {
    pub color: Color,
    pub in_check: bool,
    pub en_passant: Option<EnPassant>,
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

    fn update(fen: FEN) {}
}

pub static START_BOARD: Board = Board {
    board: [
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
            Square { row: 1, column: 1, piece_state: Some(PieceState {color: Color::White, piece: Piece::Rook, location: (1, 1), has_moved: false, dead: false  }) },
            Square { row: 1, column: 2, piece_state: Some(PieceState {color: Color::White, piece: Piece::Knight, location: (1, 2), has_moved: false, dead: false  }) },
            Square { row: 1, column: 3, piece_state: Some(PieceState {color: Color::White, piece: Piece::Bishop, location: (1, 3), has_moved: false, dead: false  }) },
            Square { row: 1, column: 4, piece_state: Some(PieceState {color: Color::White, piece: Piece::Queen, location: (1, 4), has_moved: false, dead: false  }) },
            Square { row: 1, column: 5, piece_state: Some(PieceState {color: Color::White, piece: Piece::King, location: (1, 5), has_moved: false, dead: false  }) },
            Square { row: 1, column: 6, piece_state: Some(PieceState {color: Color::White, piece: Piece::Bishop, location: (1, 6), has_moved: false, dead: false  }) },
            Square { row: 1, column: 7, piece_state: Some(PieceState {color: Color::White, piece: Piece::Knight, location: (1, 7), has_moved: false, dead: false  }) },
            Square { row: 1, column: 8, piece_state: Some(PieceState {color: Color::White, piece: Piece::Rook, location: (1, 8), has_moved: false, dead: false  }) },
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

pub fn simple_algebraic_to_grid(notation: &str) -> Option<EnPassant> {
    let mut square = Square {
        row: 0,
        column: 0,
        piece_state: None,
    };

    if notation.chars().next() == Some('-') {
        return None;
    }

    if notation.len() != 2 {
        panic!("Invalid algebraic length");
    }

    for char in notation.chars() {
        if char.is_alphabetic() {
            match char {
                'a' => square.column = 1,
                'b' => square.column = 2,
                'c' => square.column = 3,
                'd' => square.column = 4,
                'e' => square.column = 5,
                'f' => square.column = 6,
                'g' => square.column = 7,
                'h' => square.column = 8,
                _ => panic!("Invalid algebraic syntax"),
            }
        } else if char.is_numeric() {
            match char {
                '1' => square.row = 1,
                '2' => square.row = 2,
                '3' => square.row = 3,
                '4' => square.row = 4,
                '5' => square.row = 5,
                '6' => square.row = 6,
                '7' => square.row = 7,
                '8' => square.row = 8,
                _ => panic!("Invalid algebraic syntax"),
            }
        } else {
            panic!("Invalid algebraic syntax")
        }
    }

    Some(EnPassant { target: square })
}

#[wasm_bindgen]
pub struct FEN {
    fen_str: String,
}

impl FEN {
    pub fn new(&self, fen_str: &str) -> Option<FEN> {
        if !self.check_fen(fen_str) {
            return None;
        }
        Some(FEN {
            fen_str: fen_str.to_string(),
        })
    }

    pub fn to_board(&self) -> Option<Board> {
        if !self.check_fen(&self.fen_str) {
            return None;
        }
        let sections: Vec<&str> = self.fen_str.split(' ').collect();

        let mut board: Board = EMPTY_BOARD;
        for (row_idx, row_str) in sections[0].split('/').enumerate().rev() {
            let mut col_idx: usize = 0;
            for c in row_str.chars() {
                if c.is_digit(10) {
                    col_idx += c.to_digit(10).unwrap() as usize;
                } else {
                    board.board[row_idx - 1][col_idx - 1] = match c {
                        'P' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(PieceState {color: Color::White, piece: Piece::Pawn, location: (row_idx, col_idx), has_moved: false, dead: false }),
                        },
                        'R' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(PieceState {color: Color::White, piece: Piece::Rook, location: (row_idx, col_idx), has_moved: false, dead: false }),
                        },
                        'N' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(PieceState {color: Color::White, piece: Piece::Knight, location: (row_idx, col_idx), has_moved: false, dead: false }),
                        },
                        'B' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(PieceState {color: Color::White, piece: Piece::Bishop, location: (row_idx, col_idx), has_moved: false, dead: false }),
                        },
                        'Q' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(PieceState {color: Color::White, piece: Piece::Queen, location: (row_idx, col_idx), has_moved: false, dead: false }),
                        },
                        'K' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(PieceState {color: Color::White, piece: Piece::King, location: (row_idx, col_idx), has_moved: false, dead: false }),
                        },
                        'p' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(PieceState {color: Color::Black, piece: Piece::Pawn, location: (row_idx, col_idx), has_moved: false, dead: false }),
                        },
                        'r' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(PieceState {color: Color::Black, piece: Piece::Rook, location: (row_idx, col_idx), has_moved: false, dead: false }),
                        },
                        'n' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(PieceState {color: Color::Black, piece: Piece::Knight, location: (row_idx, col_idx), has_moved: false, dead: false }),
                        },
                        'b' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(PieceState {color: Color::Black, piece: Piece::Bishop, location: (row_idx, col_idx), has_moved: false, dead: false }),
                        },
                        'q' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(PieceState {color: Color::Black, piece: Piece::Queen, location: (row_idx, col_idx), has_moved: false, dead: false }),
                        },
                        'k' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(PieceState {color: Color::Black, piece: Piece::King, location: (row_idx, col_idx), has_moved: false, dead: false }),
                        },
                        _ => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: None,
                        },
                    };
                    col_idx += 1;
                }
            }
        }
        Some(board)
    }

    pub fn check_fen(&self, fen_str: &str) -> bool {
        let sections: Vec<&str> = fen_str.split(' ').collect();
        if sections.len() != 6 || sections[0].split('/').count() != 8 {
            return false;
        }

        if sections[1].chars().next() != Some('w') && sections[1].chars().next() != Some('b') {
            return false;
        }

        for char in sections[2].chars() {
            if char != 'K' && char != 'Q' && char != 'k' && char != 'q' && char != '-' {
                return false;
            }
        }

        for char in sections[3].chars() {
            if char != '-' && !char.is_alphanumeric() {
                return false;
            }
        }

        if !sections[4].parse::<f64>().is_ok() || !sections[5].parse::<f64>().is_ok() {
            return false;
        }

        for row in sections[0].split('/') {
            let mut item_count = 0;
            for c in row.chars() {
                match c {
                    'r' | 'n' | 'b' | 'q' | 'k' | 'p' | 'R' | 'N' | 'B' | 'Q' | 'K' | 'P' => {
                        item_count += 1
                    }
                    '1'..='8' => item_count += c.to_digit(10).unwrap() as i32,
                    _ => return false,
                }
            }

            if item_count != 8 {
                return false;
            }
        }

        true
    }

    pub fn to_board_state(&self) -> Option<BoardState> {
        if !self.check_fen(&self.fen_str.to_string()) {
            return None;
        }

        let fen_sections: Vec<&str> = self.fen_str.split(' ').collect();
        let board = self.to_board().unwrap();
        let color: Color = match fen_sections[1].chars().next() {
            Some('w') => Color::White,
            Some('b') => Color::Black,
            _ => panic!("Invalid color"),
        };

        for char in fen_sections[2].chars() {
            match char {
                '-' => break,
                'K' => castling.castle_kingside = true,
                'Q' => castling.castle_queenside = true,
                'k' => castling.castle_kingside = true,
                'q' => castling.castle_queenside = true,
                _ => panic!("Problem parsing FEN"),
            }
        }

        let en_passant: Option<EnPassant> = simple_algebraic_to_grid(fen_sections[3]);

        let mut draw: DrawConditions = DrawConditions {
            draw: false,
            fifty_move_counter: fen_sections[4].parse::<usize>().unwrap(),
            threefold_counter: 0,
        };

        if draw.fifty_move_counter == 50 {
            draw.draw = true;
        }

        Some(BoardState {
            board: board,
            active_color: color,
            white_state: white_color_state,
            black_state: black_color_state,
            draw: draw,
            time: None,
        })
    }
}
