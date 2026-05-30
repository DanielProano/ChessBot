use crate::moves::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Color {
    White,
    Black,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
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
            Color::White => match Self {
                Piece::Pawn => 'P'.to_string(),
                Piece::Rook => 'R'.to_string(),
                Piece::Knight => 'N'.to_string(),
                Piece::Bishop => 'B'.to_string(),
                Piece::Queen => 'Q'.to_string(),
                Piece::King => 'K'.to_string(),
            },
            Color::Black => match Self {
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
pub struct CastlingRights {
    castle_kingside: bool,
    castle_queenside: bool,
}

pub struct EnPassant {
    target: Square,
}

pub struct DrawConditions {
    draw: bool,
    fifty_move_counter: u32,
    threefold_counter: u32,
}

pub struct Time {
    alloted_time: u32,
    cur_time: u32,
}

pub struct Square {
    pub row: u32,
    pub column: u32,
    pub piece_state: Option<PieceState>,
}

pub struct PieceState {
    pub id: u32,
    pub color: Color,
    pub piece: Piece,
    pub has_moved: bool
}

pub struct ColorState {
    pub color: Color,
    pub in_check: bool,
    pub en_passant: Option<EnPassant>,
    pub castling: CastlingRights,
}

pub struct Board {
    pub board: [[Square; 8]; 8],
}

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
        let mut unique_id = 0;

        for row in 6..8 {
            for col in 0..8 {
                white_piece_states.push(PieceState {
                    id: unique_id,
                    piece_state: Some(START_BOARD.board[row][col]).piece_state,
                    color: Color::White
                });
                unique_id += 1;
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
                black_piece_states.push(PieceState {
                    id: unique_id,
                    piece_state: Some(START_BOARD.board[row][col]).piece_state,
                    color: Color::Black
                });
                unique_id += 1;
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
        // Row 8 (Black back rank)
        [
            Square { row: 8, column: 1, piece_state: Some(PieceState { id: 0, color: Color::Black, piece: Piece::Rook }) },
            Square { row: 8, column: 2, piece_state: Some(PieceState { id: 1, color: Color::Black, piece: Piece::Knight }) },
            Square { row: 8, column: 3, piece_state: Some(PieceState { id: 2, color: Color::Black, piece: Piece::Bishop }) },
            Square { row: 8, column: 4, piece_state: Some(PieceState { id: 3, color: Color::Black, piece: Piece::Queen }) },
            Square { row: 8, column: 5, piece_state: Some(PieceState { id: 4, color: Color::Black, piece: Piece::King }) },
            Square { row: 8, column: 6, piece_state: Some(PieceState { id: 5, color: Color::Black, piece: Piece::Bishop }) },
            Square { row: 8, column: 7, piece_state: Some(PieceState { id: 6, color: Color::Black, piece: Piece::Knight }) },
            Square { row: 8, column: 8, piece_state: Some(PieceState { id: 7, color: Color::Black, piece: Piece::Rook }) },
        ],
        // Row 7 (Black pawns)
        std::array::from_fn(|col| Square {
            row: 7,
            column: (col + 1) as u32,
            piece_state: Some(PieceState { id: (8 + col) as u32, color: Color::Black, piece: Piece::Pawn }),
        }),
        // Rows 6-3 (Empty)
        [Square { row: 6, column: 1, piece_state: None }; 8],
        [Square { row: 5, column: 1, piece_state: None }; 8],
        [Square { row: 4, column: 1, piece_state: None }; 8],
        [Square { row: 3, column: 1, piece_state: None }; 8],
        // Row 2 (White pawns)
        std::array::from_fn(|col| Square {
            row: 2,
            column: (col + 1) as u32,
            piece_state: Some(PieceState { id: (16 + col) as u32, color: Color::White, piece: Piece::Pawn }),
        }),
        // Row 1 (White back rank)
        [
            Square { row: 1, column: 1, piece_state: Some(PieceState { id: 24, color: Color::White, piece: Piece::Rook }) },
            Square { row: 1, column: 2, piece_state: Some(PieceState { id: 25, color: Color::White, piece: Piece::Knight }) },
            Square { row: 1, column: 3, piece_state: Some(PieceState { id: 26, color: Color::White, piece: Piece::Bishop }) },
            Square { row: 1, column: 4, piece_state: Some(PieceState { id: 27, color: Color::White, piece: Piece::Queen }) },
            Square { row: 1, column: 5, piece_state: Some(PieceState { id: 28, color: Color::White, piece: Piece::King }) },
            Square { row: 1, column: 6, piece_state: Some(PieceState { id: 29, color: Color::White, piece: Piece::Bishop }) },
            Square { row: 1, column: 7, piece_state: Some(PieceState { id: 30, color: Color::White, piece: Piece::Knight }) },
            Square { row: 1, column: 8, piece_state: Some(PieceState { id: 31, color: Color::White, piece: Piece::Rook }) },
        ],
    ],
};

pub static EMPTY_BOARD: Board = Board {
    board: std::array::from_fn(|row| {
        std::array::from_fn(|col| Square {
            row: (8 - row) as u32,
            column: (col + 1) as u32,
            piece_state: None,
        })
    }),
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
    pub fn to_board(&self) -> Option<Board> {
        if !FEN::check_fen(&self.fen_str) {
            return None;
        }
        let sections: Vec<&str> = self.fen_str.split(' ').collect();

        let mut board: Board = EMPTY_BOARD;
        for (row_idx, row_str) in sections[0].split('/').enumerate().rev() {
            let mut col_idx: u32 = 0;
            for c in row_str.chars() {
                if c.is_digit(10) {
                    col_idx += c.to_digit(10).unwrap() as u32;
                } else {
                    board.board[row_idx][col_idx] = match c {
                        'P' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(piece_state::WhitePawn),
                        },
                        'R' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(piece_state::WhiteRook),
                        },
                        'N' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(piece_state::WhiteKnight),
                        },
                        'B' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(piece_state::WhiteBishop),
                        },
                        'Q' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(piece_state::WhiteQueen),
                        },
                        'K' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(piece_state::WhiteKing),
                        },
                        'p' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(piece_state::BlackPawn),
                        },
                        'r' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(piece_state::BlackRook),
                        },
                        'n' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(piece_state::BlackKnight),
                        },
                        'b' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(piece_state::BlackBishop),
                        },
                        'q' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(piece_state::BlackQueen),
                        },
                        'k' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(piece_state::BlackKing),
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
}

#[wasm_bindgen]
impl FEN {
    pub fn new(fen_str: &str) -> Option<FEN> {
        if !FEN::check_fen(fen_str) {
            return None;
        }
        Some(FEN {
            fen_str: fen_str.to_string(),
        })
    }

    pub fn check_fen(fen_str: &str) -> bool {
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

    pub fn to_board_state(self) -> Option<BoardState> {
        if !FEN::check_fen(&self.fen_str) {
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
            fifty_move_counter: fen_sections[4].parse::<u32>().unwrap(),
            threefold_counter: 0,
        };

        if draw.fifty_move_counter == 50 {
            draw.draw = true;
        }

        Some(BoardState {
            board: board,
            active_color: color,
            white_color_state: 
            draw: draw,
            time: None,
        })
    }
}
