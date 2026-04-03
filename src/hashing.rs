use cozy_chess::{Board, Move, Color, Piece, Square, BitBoard};
use wasm_bindgen::prelude::*;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

static mut GAME_STAGE: i32 = 1;
const PIECE_TYPES: usize = 12;
const BOARD_SQUARES: usize = 64;


pub struct Zobrist {
    pub piece_keys: [[u64; BOARD_SQUARES]; PIECE_TYPES],
    pub side_to_move_key: u64,
}

impl Zobrist {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut piece_keys = [[0u64; BOARD_SQUARES]; PIECE_TYPES];

        for piece in 0.. PIECE_TYPES {
            for square in 0..BOARD_SQUARES {
                piece_keys[piece][square] = rng.random();
            }
        }

        Zobrist {
            piece_keys,
            side_to_move_key: rng.random(),
        }
    }

    pub fn hash_position(&self, board: &Board) -> u64 {
        let mut  hash: u64 = 0;

        for square_index in 0..64 {
            if let Some(square) = Square::try_index(square_index) {
                if let Some(piece) = board.piece_on(square) {
                    // Ensure piece_index is valid
                    let piece_index = match piece {
                        Piece::Pawn => 0,
                        Piece::Knight => 1,
                        Piece::Bishop => 2,
                        Piece::Rook => 3,
                        Piece::Queen => 4,
                        Piece::King => 5,
                        _ => continue,
                    };

                    if piece_index >= PIECE_TYPES || square_index >= BOARD_SQUARES {
                        eprintln!("Piece_index or square_index out of bounds");
                        continue;
                    }

                    hash ^= self.piece_keys[piece_index][square_index];
                }
            }
        }

        if board.side_to_move() == Color::Black {
            hash ^= self.side_to_move_key;
        }
        hash
    }

    pub fn update_hash(
        &self,
        hash: u64,
        from_square: usize,
        to_square: usize,
        piece: Piece,
        side_to_move: Color,
    ) -> u64 {
        let piece_index = piece as usize;
        let mut new_hash = hash;

        new_hash ^= self.piece_keys[piece_index][from_square];
        new_hash ^= self.piece_keys[piece_index][to_square];
        new_hash ^= self.side_to_move_key;
        new_hash
    }
}


#[derive(Clone)]
pub struct Entry {
    pub score: i32,
    pub depth: i32,
    pub flag: flag_type,
    pub best_move: String,
}
#[derive(Clone, Copy, PartialEq)]
pub enum flag_type {
    Exact,
    Lower,
    Upper,
}
pub struct transposition_table {
    table: Mutex<HashMap<u64, Entry>>,
}
