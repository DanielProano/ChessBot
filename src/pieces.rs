use wasm_bindgen::prelude::*;
use crate::moves::*;

#[wasm_bindgen]
pub enum Color {
    White,
    Black,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum Piece {
    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen,
    BlackKing,
}

impl Piece {
    pub fn to_char(self) -> String {
        match self {
            Piece::WhitePawn => 'P'.to_string(),
            Piece::WhiteRook => 'R'.to_string(),
            Piece::WhiteKnight => 'N'.to_string(),
            Piece::WhiteBishop => 'B'.to_string(),
            Piece::WhiteQueen => 'Q'.to_string(),
            Piece::WhiteKing => 'K'.to_string(),
            Piece::BlackPawn => 'p'.to_string(),
            Piece::BlackRook => 'r'.to_string(),
            Piece::BlackKnight => 'n'.to_string(),
            Piece::BlackBishop => 'b'.to_string(),
            Piece::BlackQueen => 'q'.to_string(),
            Piece::BlackKing => 'k'.to_string(),
        }
    }

    pub fn to_color(self) -> Color {
        match self {
            Piece::WhitePawn => Color::White,
            Piece::WhiteRook => Color::White,
            Piece::WhiteKnight => Color::White,
            Piece::WhiteBishop => Color::White,
            Piece::WhiteQueen => Color::White,
            Piece::WhiteKing => Color::White,
            Piece::BlackPawn => Color::Black,
            Piece::BlackRook => Color::Black,
            Piece::BlackKnight => Color::Black,
            Piece::BlackBishop => Color::Black,
            Piece::BlackQueen => Color::Black,
            Piece::BlackKing => Color::Black,
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


#[derive(Copy, Clone)]
pub struct Square {
    pub row: u32,
    pub column: u32,
    pub piece: Option<Piece>
}

pub struct PieceState {
    pub id: u32,
    pub color: Color,
    pub piece: Piece,
    pub square: Square,
    pub moves: Option<Vec<Move>>
}

pub struct ColorState {
    pub color: Color,
    pub in_check: bool,
    pub pieces: Vec<PieceState>,
    pub time: Option<Time>,
    pub en_passant: Option<EnPassant>,
    pub castling: CastlingRights,
}

#[derive(Clone, Copy)]
pub struct Board {
    pub board: [[Square; 8]; 8],
}

pub struct BoardState {
    pub board: Board,
    pub active_color: Color,
    pub states: Vec<ColorState>,
    pub previous_state: Option<Box<BoardState>>,
    pub next_state: Option<Box<BoardState>>,
    pub draw: DrawConditions,
    pub time: Option<Time>
}

#[wasm_bindgen]
impl BoardState {
    fn start() -> Self {
        let mut white_pieces = vec![];
        let mut unique_id = 0;

        for row in 6..8 {
            for col in 0..8 {
                white_pieces.push(PieceState {
                    id: unique_id,
                    piece: Some(START_BOARD.board[row][col]).piece,
                    square: START_BOARD.board[row][col],
                    moves: None
                });
                unique_id += 1;
            }
        }

        let white_color_state: ColorState = ColorState {
            color: Color::White,
            in_check: false,
            pieces: white_pieces,
            time: None,
            en_passant: None,
            castling: CastlingRights { castle_kingside: true, castle_queenside: true }
        };

        let mut black_pieces = vec![];
        for row in 0..2 {
            for col in 0..8 {
                black_pieces.push(PieceState {
                    id: unique_id,
                    piece: Some(START_BOARD.board[row][col]).piece,
                    square: START_BOARD.board[row][col],
                    moves: None
                });
                unique_id += 1;
            }
        }

        let black_color_state: ColorState = ColorState {
            color: Color::Black,
            in_check: false,
            pieces: black_pieces,
            time: None,
            en_passant: None,
            castling: CastlingRights { castle_kingside: true, castle_queenside: true }
        };

        Self { 
            board: START_BOARD, 
            active_color: Color::White, 
            states: vec![white_color_state, black_color_state],
            previous_state: None, 
            next_state: None,
            draw: DrawConditions { 
                draw: false, 
                fifty_move_counter: 0, 
                threefold_counter: 0 
            }, 
            time: None
        }
    }

    fn update(fen: FEN) {

    }
}

pub static START_BOARD: Board = Board { board: [
    [Square{ row: 8, column: 1, piece: Some(Piece::BlackRook)}, Square{ row: 8, column: 2, piece: Some(Piece::BlackKnight)}, Square{ row: 8, column: 3, piece: Some(Piece::BlackBishop)}, Square{ row: 8, column: 4, piece: Some(Piece::BlackQueen)}, Square{ row: 8, column: 5, piece: Some(Piece::BlackKing)}, Square{ row: 8, column: 6, piece: Some(Piece::BlackBishop)}, Square{ row: 8, column: 7, piece: Some(Piece::BlackKnight)}, Square{ row: 8, column: 8, piece: Some(Piece::BlackRook)}],
    [Square{ row: 7, column: 1, piece: Some(Piece::BlackPawn)}, Square{ row: 7, column: 2, piece: Some(Piece::BlackPawn)}, Square{ row: 7, column: 3, piece: Some(Piece::BlackPawn)}, Square{ row: 7, column: 4, piece: Some(Piece::BlackPawn)}, Square{ row: 7, column: 5, piece: Some(Piece::BlackPawn)}, Square{ row: 7, column: 6, piece: Some(Piece::BlackPawn)}, Square{ row: 7, column: 7, piece: Some(Piece::BlackPawn)}, Square{ row: 7, column: 8, piece: Some(Piece::BlackPawn)}],
    [Square{ row: 6, column: 1, piece: None}, Square{ row: 6, column: 2, piece: None}, Square{ row: 6, column: 3, piece: None}, Square{ row: 6, column: 4, piece: None}, Square{ row: 6, column: 5, piece: None}, Square{ row: 6, column: 6, piece: None}, Square{ row: 6, column: 7, piece: None}, Square{ row: 6, column: 8, piece: None}],
    [Square{ row: 5, column: 1, piece: None}, Square{ row: 5, column: 2, piece: None}, Square{ row: 5, column: 3, piece: None}, Square{ row: 5, column: 4, piece: None}, Square{ row: 5, column: 5, piece: None}, Square{ row: 5, column: 6, piece: None}, Square{ row: 5, column: 7, piece: None}, Square{ row: 5, column: 8, piece: None}],
    [Square{ row: 4, column: 1, piece: None}, Square{ row: 4, column: 2, piece: None}, Square{ row: 4, column: 3, piece: None}, Square{ row: 4, column: 4, piece: None}, Square{ row: 4, column: 5, piece: None}, Square{ row: 4, column: 6, piece: None}, Square{ row: 4, column: 7, piece: None}, Square{ row: 4, column: 8, piece: None}],
    [Square{ row: 3, column: 1, piece: None}, Square{ row: 3, column: 2, piece: None}, Square{ row: 3, column: 3, piece: None}, Square{ row: 3, column: 4, piece: None}, Square{ row: 3, column: 5, piece: None}, Square{ row: 3, column: 6, piece: None}, Square{ row: 3, column: 7, piece: None}, Square{ row: 3, column: 8, piece: None}],
    [Square{ row: 2, column: 1, piece: Some(Piece::WhitePawn)}, Square{ row: 2, column: 2, piece: Some(Piece::WhitePawn)}, Square{ row: 2, column: 3, piece: Some(Piece::WhitePawn)}, Square{ row: 2, column: 4, piece: Some(Piece::WhitePawn)}, Square{ row: 2, column: 5, piece: Some(Piece::WhitePawn)}, Square{ row: 2, column: 6, piece: Some(Piece::WhitePawn)}, Square{ row: 2, column: 7, piece: Some(Piece::WhitePawn)}, Square{ row: 2, column: 8, piece: Some(Piece::WhitePawn)}],
    [Square{ row: 1, column: 1, piece: Some(Piece::WhiteRook)}, Square{ row: 1, column: 2, piece: Some(Piece::WhiteKnight)}, Square{ row: 1, column: 3, piece: Some(Piece::WhiteBishop)}, Square{ row: 1, column: 4, piece: Some(Piece::WhiteQueen)}, Square{ row: 1, column: 5, piece: Some(Piece::WhiteKing)}, Square{ row: 1, column: 6, piece: Some(Piece::WhiteBishop)}, Square{ row: 1, column: 7, piece: Some(Piece::WhiteKnight)}, Square{ row: 1, column: 8, piece: Some(Piece::WhiteRook)}],
] };

pub static EMPTY_BOARD: Board = Board { board: [
    [Square{ row: 8, column: 1, piece: None}, Square{ row: 8, column: 2, piece: None}, Square{ row: 8, column: 3, piece: None}, Square{ row: 8, column: 4, piece: None}, Square{ row: 8, column: 5, piece: None}, Square{ row: 8, column: 6, piece: None}, Square{ row: 8, column: 7, piece: None}, Square{ row: 8, column: 8, piece: None}],
    [Square{ row: 7, column: 1, piece: None}, Square{ row: 7, column: 2, piece: None}, Square{ row: 7, column: 3, piece: None}, Square{ row: 7, column: 4, piece: None}, Square{ row: 7, column: 5, piece: None}, Square{ row: 7, column: 6, piece: None}, Square{ row: 7, column: 7, piece: None}, Square{ row: 7, column: 8, piece: None}],
    [Square{ row: 6, column: 1, piece: None}, Square{ row: 6, column: 2, piece: None}, Square{ row: 6, column: 3, piece: None}, Square{ row: 6, column: 4, piece: None}, Square{ row: 6, column: 5, piece: None}, Square{ row: 6, column: 6, piece: None}, Square{ row: 6, column: 7, piece: None}, Square{ row: 6, column: 8, piece: None}],
    [Square{ row: 5, column: 1, piece: None}, Square{ row: 5, column: 2, piece: None}, Square{ row: 5, column: 3, piece: None}, Square{ row: 5, column: 4, piece: None}, Square{ row: 5, column: 5, piece: None}, Square{ row: 5, column: 6, piece: None}, Square{ row: 5, column: 7, piece: None}, Square{ row: 5, column: 8, piece: None}],
    [Square{ row: 4, column: 1, piece: None}, Square{ row: 4, column: 2, piece: None}, Square{ row: 4, column: 3, piece: None}, Square{ row: 4, column: 4, piece: None}, Square{ row: 4, column: 5, piece: None}, Square{ row: 4, column: 6, piece: None}, Square{ row: 4, column: 7, piece: None}, Square{ row: 4, column: 8, piece: None}],
    [Square{ row: 3, column: 1, piece: None}, Square{ row: 3, column: 2, piece: None}, Square{ row: 3, column: 3, piece: None}, Square{ row: 3, column: 4, piece: None}, Square{ row: 3, column: 5, piece: None}, Square{ row: 3, column: 6, piece: None}, Square{ row: 3, column: 7, piece: None}, Square{ row: 3, column: 8, piece: None}],
    [Square{ row: 2, column: 1, piece: None}, Square{ row: 2, column: 2, piece: None}, Square{ row: 2, column: 3, piece: None}, Square{ row: 2, column: 4, piece: None}, Square{ row: 2, column: 5, piece: None}, Square{ row: 2, column: 6, piece: None}, Square{ row: 2, column: 7, piece: None}, Square{ row: 2, column: 8, piece: None}],
    [Square{ row: 1, column: 1, piece: None}, Square{ row: 1, column: 2, piece: None}, Square{ row: 1, column: 3, piece: None}, Square{ row: 1, column: 4, piece: None}, Square{ row: 1, column: 5, piece: None}, Square{ row: 1, column: 6, piece: None}, Square{ row: 1, column: 7, piece: None}, Square{ row: 1, column: 8, piece: None}],
] };

pub fn simple_algebraic_to_grid(notation: &str) -> Option<EnPassant> {
    let mut square = Square { row: 0, column: 0, piece: None };

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
        }
        else if char.is_numeric() {
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
        if !FEN::check_fen(&self.fen_str) { return None; }
        let sections: Vec<&str> = self.fen_str.split(' ').collect();

        let mut board: Board = EMPTY_BOARD;
        for (row_idx, row_str) in sections[0].split('/').enumerate().rev() {
            let mut col_idx: u32 = 0;
            for c in row_str.chars() {
                if c.is_digit(10) {
                    col_idx += c.to_digit(10).unwrap() as u32;
                } else {
                    board.board[row_idx][col_idx] = match c {
                        'P' => Square { row: row_idx + 1, column: col_idx + 1, piece: Some(Piece::WhitePawn) },
                        'R' => Square { row: row_idx + 1, column: col_idx + 1, piece: Some(Piece::WhiteRook) },
                        'N' => Square { row: row_idx + 1, column: col_idx + 1, piece: Some(Piece::WhiteKnight) },
                        'B' => Square { row: row_idx + 1, column: col_idx + 1, piece: Some(Piece::WhiteBishop) },
                        'Q' => Square { row: row_idx + 1, column: col_idx + 1, piece: Some(Piece::WhiteQueen) },
                        'K' => Square { row: row_idx + 1, column: col_idx + 1, piece: Some(Piece::WhiteKing) },
                        'p' => Square { row: row_idx + 1, column: col_idx + 1, piece: Some(Piece::BlackPawn) },
                        'r' => Square { row: row_idx + 1, column: col_idx + 1, piece: Some(Piece::BlackRook) },
                        'n' => Square { row: row_idx + 1, column: col_idx + 1, piece: Some(Piece::BlackKnight) },
                        'b' => Square { row: row_idx + 1, column: col_idx + 1, piece: Some(Piece::BlackBishop) },
                        'q' => Square { row: row_idx + 1, column: col_idx + 1, piece: Some(Piece::BlackQueen) },
                        'k' => Square { row: row_idx + 1, column: col_idx + 1, piece: Some(Piece::BlackKing) },
                        _ => Square { row: row_idx + 1, column: col_idx + 1, piece: None },
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
        if !FEN::check_fen(fen_str) { return None; }
        Some(FEN { fen_str: fen_str.to_string() })
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
                    'r' | 'n' | 'b' | 'q' | 'k' | 'p' |
                    'R' | 'N' | 'B' | 'Q' | 'K' | 'P' => item_count += 1,
                    '1'..='8' => item_count += c.to_digit(10).unwrap() as i32,
                    _ => return false,
                }
            }
            if item_count != 8 {
                return false
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
            castling: castling, 
            en_passant: en_passant, 
            previous_state: None, 
            next_state: None,
            draw: draw,
            time: None
        })
    }
}

