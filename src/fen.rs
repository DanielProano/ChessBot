use wasm_bindgen::prelude::*;

use crate::pieces::*;
use crate::utils::*;

#[wasm_bindgen]
pub struct FEN {
    fen_str: String,
}

impl FEN {
    pub fn new(fen_str: &str) -> Option<FEN> {
        let fen = FEN { fen_str: fen_str.to_string() };
        if !fen.fen_is_valid(fen_str) {
            return None;
        }
        Some(fen)
    }

    pub fn to_board(&self) -> Option<Board> {
        if !self.fen_is_valid(&self.fen_str) {
            return None;
        }
        let sections: Vec<&str> = self.fen_str.split(' ').collect();

        let mut board: Board = EMPTY_BOARD;
        let rows: Vec<&str> = sections[0].split('/').collect();
        for (row_idx, row_str) in rows.iter().rev().enumerate() {
            let mut col_idx: usize = 0;
            for c in row_str.chars() {
                if c.is_digit(10) {
                    col_idx += c.to_digit(10).unwrap() as usize;
                } else {
                    board.board[row_idx][col_idx] = match c {
                        'P' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(PieceState {color: Color::White, piece: Piece::Pawn, location: (row_idx + 1, col_idx + 1), has_moved: false, dead: false }),
                        },
                        'R' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(PieceState {color: Color::White, piece: Piece::Rook, location: (row_idx + 1, col_idx + 1), has_moved: false, dead: false }),
                        },
                        'N' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(PieceState {color: Color::White, piece: Piece::Knight, location: (row_idx + 1, col_idx + 1), has_moved: false, dead: false }),
                        },
                        'B' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(PieceState {color: Color::White, piece: Piece::Bishop, location: (row_idx + 1, col_idx + 1), has_moved: false, dead: false }),
                        },
                        'Q' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(PieceState {color: Color::White, piece: Piece::Queen, location: (row_idx + 1, col_idx + 1), has_moved: false, dead: false }),
                        },
                        'K' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(PieceState {color: Color::White, piece: Piece::King, location: (row_idx+ 1, col_idx + 1), has_moved: false, dead: false }),
                        },
                        'p' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(PieceState {color: Color::Black, piece: Piece::Pawn, location: (row_idx + 1, col_idx + 1), has_moved: false, dead: false }),
                        },
                        'r' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(PieceState {color: Color::Black, piece: Piece::Rook, location: (row_idx + 1, col_idx + 1), has_moved: false, dead: false }),
                        },
                        'n' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(PieceState {color: Color::Black, piece: Piece::Knight, location: (row_idx + 1, col_idx + 1), has_moved: false, dead: false }),
                        },
                        'b' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(PieceState {color: Color::Black, piece: Piece::Bishop, location: (row_idx + 1, col_idx + 1), has_moved: false, dead: false }),
                        },
                        'q' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(PieceState {color: Color::Black, piece: Piece::Queen, location: (row_idx + 1, col_idx + 1), has_moved: false, dead: false }),
                        },
                        'k' => Square {
                            row: row_idx + 1,
                            column: col_idx + 1,
                            piece_state: Some(PieceState {color: Color::Black, piece: Piece::King, location: (row_idx + 1, col_idx + 1), has_moved: false, dead: false }),
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

    pub fn from_board(board: &Board) -> String {
        let mut fen_string = String::new();

        for row in (0..8).rev() {
            let mut empty_count = 0;

            for col in 0..8 {
                if let Some(state) = board.board[row][col].piece_state {
                    if empty_count > 0 {
                        fen_string.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    let c = match (state.piece, state.color) {
                        (Piece::Pawn,   Color::White) => 'P',
                        (Piece::Rook,   Color::White) => 'R',
                        (Piece::Knight, Color::White) => 'N',
                        (Piece::Bishop, Color::White) => 'B',
                        (Piece::Queen,  Color::White) => 'Q',
                        (Piece::King,   Color::White) => 'K',
                        (Piece::Pawn,   Color::Black) => 'p',
                        (Piece::Rook,   Color::Black) => 'r',
                        (Piece::Knight, Color::Black) => 'n',
                        (Piece::Bishop, Color::Black) => 'b',
                        (Piece::Queen,  Color::Black) => 'q',
                        (Piece::King,   Color::Black) => 'k',
                    };
                    fen_string.push(c);
                } else {
                    empty_count += 1;
                }
            }

            if empty_count > 0 {
                fen_string.push_str(&empty_count.to_string());
            }

            if row > 0 {
                fen_string.push('/');
            }
        }

        fen_string
    }
    
    pub fn fen_is_valid(&self, fen_str: &str) -> bool {
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
        if !self.fen_is_valid(&self.fen_str.to_string()) {
            return None;
        }

        let fen_sections: Vec<&str> = self.fen_str.split(' ').collect();
        let board = self.to_board().unwrap();
        let color: Color = match fen_sections[1].chars().next() {
            Some('w') => Color::White,
            Some('b') => Color::Black,
            _ => panic!("Invalid color"),
        };

        let mut white_castling = CastlingRights { castle_kingside: false, castle_queenside: false };
        let mut black_castling = CastlingRights { castle_kingside: false, castle_queenside: false };


        for char in fen_sections[2].chars() {
            match char {
                '-' => break,
                'K' => white_castling.castle_kingside = true,
                'Q' => white_castling.castle_queenside = true,
                'k' => black_castling.castle_kingside = true,
                'q' => black_castling.castle_queenside = true,
                _ => panic!("Problem parsing FEN"),
            }
        }

        let en_passant: Option<Square> = simple_algebraic_to_grid(fen_sections[3]);

        let mut draw: DrawConditions = DrawConditions {
            draw: false,
            fifty_move_counter: fen_sections[4].parse::<usize>().unwrap(),
            threefold_counter: 0,
        };

        if draw.fifty_move_counter == 50 {
            draw.draw = true;
        }

        let white_color_state = ColorState {
            color: Color::White,
            in_check: false,
            en_passant: None,
            castling: white_castling,
        };

        let black_color_state = ColorState {
            color: Color::Black,
            in_check: false,
            en_passant: None,
            castling: black_castling,
        };

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

#[cfg(test)]
mod tests {
    use super::*;

    const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    #[test]
    fn test1_valid_fen() {
        let fen = FEN::new(STARTING_FEN);
        assert!(fen.is_some());
    }

    #[test]
    fn test2_valid_fen() {
        let fen = FEN::new("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
        assert!(fen.is_some());
    }

    #[test]
    fn test3_valid_fen() {
        let fen = FEN::new("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");
        assert!(fen.is_some());
    }

    #[test]
    fn test_invalid_fen() {
        let fen = FEN::new("invalid");
        assert!(fen.is_none());
    }

    #[test]
    fn test_to_board_starting_position() {
        let fen = FEN::new(STARTING_FEN).unwrap();
        let board = fen.to_board().unwrap();

        println!("{:?}", board.board);
        assert_eq!(board.board, START_BOARD.board);
    }

    #[test]
    fn test_board_to_fen() {
        let fen = FEN::new(STARTING_FEN).unwrap();

        assert_eq!(fen.to_board().unwrap().board, START_BOARD.board);
    }
}
