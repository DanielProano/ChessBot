use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Color {
    White,
    Black,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum Pieces {
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

#[wasm_bindgen]
impl Pieces {
    pub fn to_char(self) -> String {
        match self {
            Pieces::WhitePawn => 'P'.to_string(),
            Pieces::WhiteRook => 'R'.to_string(),
            Pieces::WhiteKnight => 'N'.to_string(),
            Pieces::WhiteBishop => 'B'.to_string(),
            Pieces::WhiteQueen => 'Q'.to_string(),
            Pieces::WhiteKing => 'K'.to_string(),
            Pieces::BlackPawn => 'p'.to_string(),
            Pieces::BlackRook => 'r'.to_string(),
            Pieces::BlackKnight => 'n'.to_string(),
            Pieces::BlackBishop => 'b'.to_string(),
            Pieces::BlackQueen => 'q'.to_string(),
            Pieces::BlackKing => 'k'.to_string(),
        }
    }

    pub fn color(self) -> Color {
        match self {
            Pieces::WhitePawn => Color::White,
            Pieces::WhiteRook => Color::White,
            Pieces::WhiteKnight => Color::White,
            Pieces::WhiteBishop => Color::White,
            Pieces::WhiteQueen => Color::White,
            Pieces::WhiteKing => Color::White,
            Pieces::BlackPawn => Color::Black,
            Pieces::BlackRook => Color::Black,
            Pieces::BlackKnight => Color::Black,
            Pieces::BlackBishop => Color::Black,
            Pieces::BlackQueen => Color::Black,
            Pieces::BlackKing => Color::Black,
        }
    }
}

// Only tracks whether rooks or kings have moved
pub struct castling_rights {
    white_can_castle_left: bool,
    white_can_castle_right: bool,
    black_can_castle_left: bool,
    black_can_castle_right: bool,
}

pub struct en_passant {
    attacker: Square,
    victim: Square,
}

pub struct draw_conditions {
    draw: bool,
    fifty_move_counter: u32,
    threefold_counter: u32,
}

pub struct time {
    alloted_time: u32,
    cur_time: u32,
}


#[derive(Copy, Clone)]
pub struct Square {
    row: usize,
    column: usize,
    piece: Option<Pieces>
}

#[derive(Clone, Copy)]
pub struct Board {
    board: [[Square; 8]; 8],
}

#[wasm_bindgen]
pub struct BoardState {
    board: Board,
    active_color: Color,
    castling_rights: castling_rights,
    en_passant: Option<en_passant>,
    in_check: bool,
    previous_state: Option<Board>,
    draw: draw_conditions,
    time: Option<time>,
}

#[wasm_bindgen]
impl BoardState {
    fn start() -> Self {
        Self { 
            board: START_BOARD, active_color: Color::White, 
            castling_rights: castling_rights { white_can_castle_left: true, white_can_castle_right: true, black_can_castle_left: true, black_can_castle_right: true},
            en_passant: None, in_check: false, previous_state: None, draw: draw_conditions { draw: false, fifty_move_counter: 0, threefold_counter: 0 }, time: None}
    }

    fn update(fen: FEN) {

    }
}

pub static START_BOARD: Board = Board { board: [
    [Square{ row: 1, column: 1, piece: Some(Pieces::BlackRook)}, Square{ row: 1, column: 2, piece: Some(Pieces::BlackKnight)}, Square{ row: 1, column: 3, piece: Some(Pieces::BlackBishop)}, Square{ row: 1, column: 4, piece: Some(Pieces::BlackQueen)}, Square{ row: 1, column: 5, piece: Some(Pieces::BlackKing)}, Square{ row: 1, column: 6, piece: Some(Pieces::BlackBishop)}, Square{ row: 1, column: 7, piece: Some(Pieces::BlackKnight)}, Square{ row: 1, column: 8, piece: Some(Pieces::BlackRook)}],
    [Square{ row: 2, column: 1, piece: Some(Pieces::BlackPawn)}, Square{ row: 2, column: 2, piece: Some(Pieces::BlackPawn)}, Square{ row: 2, column: 3, piece: Some(Pieces::BlackPawn)}, Square{ row: 2, column: 4, piece: Some(Pieces::BlackPawn)}, Square{ row: 2, column: 5, piece: Some(Pieces::BlackPawn)}, Square{ row: 2, column: 6, piece: Some(Pieces::BlackPawn)}, Square{ row: 2, column: 7, piece: Some(Pieces::BlackPawn)}, Square{ row: 2, column: 8, piece: Some(Pieces::BlackPawn)}],
    [Square{ row: 3, column: 1, piece: None}, Square{ row: 3, column: 2, piece: None}, Square{ row: 3, column: 3, piece: None}, Square{ row: 3, column: 4, piece: None}, Square{ row: 3, column: 5, piece: None}, Square{ row: 3, column: 6, piece: None}, Square{ row: 3, column: 7, piece: None}, Square{ row: 3, column: 8, piece: None}],
    [Square{ row: 4, column: 1, piece: None}, Square{ row: 4, column: 2, piece: None}, Square{ row: 4, column: 3, piece: None}, Square{ row: 4, column: 4, piece: None}, Square{ row: 4, column: 5, piece: None}, Square{ row: 4, column: 6, piece: None}, Square{ row: 4, column: 7, piece: None}, Square{ row: 4, column: 8, piece: None}],
    [Square{ row: 5, column: 1, piece: None}, Square{ row: 5, column: 2, piece: None}, Square{ row: 5, column: 3, piece: None}, Square{ row: 5, column: 4, piece: None}, Square{ row: 5, column: 5, piece: None}, Square{ row: 5, column: 6, piece: None}, Square{ row: 5, column: 7, piece: None}, Square{ row: 5, column: 8, piece: None}],
    [Square{ row: 6, column: 1, piece: None}, Square{ row: 6, column: 2, piece: None}, Square{ row: 6, column: 3, piece: None}, Square{ row: 6, column: 4, piece: None}, Square{ row: 6, column: 5, piece: None}, Square{ row: 6, column: 6, piece: None}, Square{ row: 6, column: 7, piece: None}, Square{ row: 6, column: 8, piece: None}],
    [Square{ row: 7, column: 1, piece: Some(Pieces::WhitePawn)}, Square{ row: 7, column: 2, piece: Some(Pieces::WhitePawn)}, Square{ row: 7, column: 3, piece: Some(Pieces::WhitePawn)}, Square{ row: 7, column: 4, piece: Some(Pieces::WhitePawn)}, Square{ row: 7, column: 5, piece: Some(Pieces::WhitePawn)}, Square{ row: 7, column: 6, piece: Some(Pieces::WhitePawn)}, Square{ row: 7, column: 7, piece: Some(Pieces::WhitePawn)}, Square{ row: 7, column: 8, piece: Some(Pieces::WhitePawn)}],
    [Square{ row: 8, column: 1, piece: Some(Pieces::WhiteRook)}, Square{ row: 8, column: 2, piece: Some(Pieces::WhiteKnight)}, Square{ row: 8, column: 3, piece: Some(Pieces::WhiteBishop)}, Square{ row: 8, column: 4, piece: Some(Pieces::WhiteQueen)}, Square{ row: 8, column: 5, piece: Some(Pieces::WhiteKing)}, Square{ row: 8, column: 6, piece: Some(Pieces::WhiteBishop)}, Square{ row: 8, column: 7, piece: Some(Pieces::WhiteKnight)}, Square{ row: 8, column: 8, piece: Some(Pieces::WhiteRook)}],
] };

pub static EMPTY_BOARD: Board = Board { board: [
    [Square{ row: 1, column: 1, piece: None}, Square{ row: 1, column: 2, piece: None}, Square{ row: 1, column: 3, piece: None}, Square{ row: 1, column: 4, piece: None}, Square{ row: 1, column: 5, piece: None}, Square{ row: 1, column: 6, piece: None}, Square{ row: 1, column: 7, piece: None}, Square{ row: 1, column: 8, piece: None}],
    [Square{ row: 2, column: 1, piece: None}, Square{ row: 2, column: 2, piece: None}, Square{ row: 2, column: 3, piece: None}, Square{ row: 2, column: 4, piece: None}, Square{ row: 2, column: 5, piece: None}, Square{ row: 2, column: 6, piece: None}, Square{ row: 2, column: 7, piece: None}, Square{ row: 2, column: 8, piece: None}],
    [Square{ row: 3, column: 1, piece: None}, Square{ row: 3, column: 2, piece: None}, Square{ row: 3, column: 3, piece: None}, Square{ row: 3, column: 4, piece: None}, Square{ row: 3, column: 5, piece: None}, Square{ row: 3, column: 6, piece: None}, Square{ row: 3, column: 7, piece: None}, Square{ row: 3, column: 8, piece: None}],
    [Square{ row: 4, column: 1, piece: None}, Square{ row: 4, column: 2, piece: None}, Square{ row: 4, column: 3, piece: None}, Square{ row: 4, column: 4, piece: None}, Square{ row: 4, column: 5, piece: None}, Square{ row: 4, column: 6, piece: None}, Square{ row: 4, column: 7, piece: None}, Square{ row: 4, column: 8, piece: None}],
    [Square{ row: 5, column: 1, piece: None}, Square{ row: 5, column: 2, piece: None}, Square{ row: 5, column: 3, piece: None}, Square{ row: 5, column: 4, piece: None}, Square{ row: 5, column: 5, piece: None}, Square{ row: 5, column: 6, piece: None}, Square{ row: 5, column: 7, piece: None}, Square{ row: 5, column: 8, piece: None}],
    [Square{ row: 6, column: 1, piece: None}, Square{ row: 6, column: 2, piece: None}, Square{ row: 6, column: 3, piece: None}, Square{ row: 6, column: 4, piece: None}, Square{ row: 6, column: 5, piece: None}, Square{ row: 6, column: 6, piece: None}, Square{ row: 6, column: 7, piece: None}, Square{ row: 6, column: 8, piece: None}],
    [Square{ row: 7, column: 1, piece: None}, Square{ row: 7, column: 2, piece: None}, Square{ row: 7, column: 3, piece: None}, Square{ row: 7, column: 4, piece: None}, Square{ row: 7, column: 5, piece: None}, Square{ row: 7, column: 6, piece: None}, Square{ row: 7, column: 7, piece: None}, Square{ row: 7, column: 8, piece: None}],
    [Square{ row: 8, column: 1, piece: None}, Square{ row: 8, column: 2, piece: None}, Square{ row: 8, column: 3, piece: None}, Square{ row: 8, column: 4, piece: None}, Square{ row: 8, column: 5, piece: None}, Square{ row: 8, column: 6, piece: None}, Square{ row: 8, column: 7, piece: None}, Square{ row: 8, column: 8, piece: None}],
] };


#[wasm_bindgen]
pub struct FEN {
    fen_str: String,
}

impl FEN {
    pub fn to_board(&self) -> Option<Board> {
        if !FEN::check_fen(&self.fen_str) { return None; }
        let sections: Vec<&str> = self.fen_str.split(' ').collect();

        let mut board: Board = EMPTY_BOARD;
        for (row_idx, row_str) in sections[0].split('/').enumerate() {
            let mut col_idx: usize = 0;
            for c in row_str.chars() {
                if c.is_digit(10) {
                    col_idx += c.to_digit(10).unwrap() as usize;
                } else {
                    board.board[row_idx][col_idx] = match c {
                        'P' => Square { row: row_idx + 1, column: col_idx + 1, piece: Some(Pieces::WhitePawn) },
                        'R' => Square { row: row_idx + 1, column: col_idx + 1, piece: Some(Pieces::WhiteRook) },
                        'N' => Square { row: row_idx + 1, column: col_idx + 1, piece: Some(Pieces::WhiteKnight) },
                        'B' => Square { row: row_idx + 1, column: col_idx + 1, piece: Some(Pieces::WhiteBishop) },
                        'Q' => Square { row: row_idx + 1, column: col_idx + 1, piece: Some(Pieces::WhiteQueen) },
                        'K' => Square { row: row_idx + 1, column: col_idx + 1, piece: Some(Pieces::WhiteKing) },
                        'p' => Square { row: row_idx + 1, column: col_idx + 1, piece: Some(Pieces::BlackPawn) },
                        'r' => Square { row: row_idx + 1, column: col_idx + 1, piece: Some(Pieces::BlackRook) },
                        'n' => Square { row: row_idx + 1, column: col_idx + 1, piece: Some(Pieces::BlackKnight) },
                        'b' => Square { row: row_idx + 1, column: col_idx + 1, piece: Some(Pieces::BlackBishop) },
                        'q' => Square { row: row_idx + 1, column: col_idx + 1, piece: Some(Pieces::BlackQueen) },
                        'k' => Square { row: row_idx + 1, column: col_idx + 1, piece: Some(Pieces::BlackKing) },
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


    pub fn to_boardState(self) -> Option<BoardState> {
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

        let mut castling: castling_rights = castling_rights { 
            white_can_castle_left: false, white_can_castle_right: false,
            black_can_castle_left: false, black_can_castle_right: false,
        };

        for char in fen_sections[2].chars() {
            match char {
                '-' => break,
                'K' => castling.white_can_castle_right = true,
                'Q' => castling.white_can_castle_left = true,
                'k' => castling.black_can_castle_right = true,
                'q' => castling.black_can_castle_left = true,
                _ => panic!("Problem parsing FEN"),
            }
        }

        let mut draw: draw_conditions = draw_conditions {
            draw: false,
            fifty_move_counter: fen_sections[4].parse::<u32>().unwrap(),
            threefold_counter: fen_sections[5].parse::<u32>().unwrap(),
        };

        if draw.fifty_move_counter == 50 {
            draw.draw = true;
        }

        let en_passant: Option<en_passant>;

        Some(BoardState { board: board, active_color: color, castling_rights: castling, en_passant: None, in_check: false, previous_state: None, draw: draw, time: None })
    }
}

