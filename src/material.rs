use crate::pieces::*;

const PAWN_VALUE: i32 = 1;
const BISHOP_VALUE: i32 = 3;
const KNIGHT_VALUE: i32 = 3;
const ROOK_VALUE: i32 = 5;
const QUEEN_VALUE: i32 = 9;
const KING_VALUE: i32 = 0;

#[derive(Debug, Clone, Copy)]
pub enum GamePhase {
    Opening,
    Middlegame,
    Endgame,
}

const WHITE_PAWN_MAP: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    4, 4, 4, 4, 4, 4, 4, 4,
    0, 0, 3, 3, 3, 3, 0, 0,
    0, 0, 2, 3, 3, 2, 0, 0,
    0, 0, 2, 3, 3, 2, 0, 0,
    0, 0, 1, 2, 2, 1, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 0, 0, 0, 0, 0,
];

const BLACK_PAWN_MAP:  [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 1, 2, 2, 1, 0, 0,
    0, 0, 2, 3, 3, 2, 0, 0,
    0, 0, 2, 3, 3, 2, 0, 0,
    0, 0, 3, 3, 3, 3, 0, 0,
    4, 4, 4, 4, 4, 4, 4, 4,
    0, 0, 0, 0, 0, 0, 0, 0,
];


const BISHOP_MAP: [i32; 64] = [
    -2, -1, 1, -1, -1, 1, -1, -2,
    -1, 2, -1, 2, 2, -1, 2, -1,
    1, 1, 3, 2, 2, 3, 1, 1,
    -1, 2, 3, 4, 4, 3, 2, -1,
    -1, 2, 3, 4, 4, 3, 2, -1,
    1, 1, 3, 2, 2, 3, 1, 1,
    -1, 2, -1, 2, 2, -1, 2, -1,
    -2, -1, 1, -1, -1, 1, -1, -2,
];

const KNIGHT_MAP: [i32; 64] = [
    -1, -1, 0, -1, -1, 0, -1, -1,
    -2, 2, 1, 1, 1, 1, 2, -2,
    -2, 0, 2, 1, 1, 2, 0, -2,
    -2, 1, 1, 3, 3, 1, 1, -2,
    -2, 1, 1, 3, 3, 1, 1, -2,
    -2, 0, 2, 1, 1, 2, 0, -2,
    -2, 2, 1, 0, 0, 1, 2, -2,
    -2, 0, -1, -1, -1, 0, -1, -2,
];

const QUEEN_MAP: [i32; 64] = [
    -2, -2, -2, 1, 1, -2, -2, -2,
    -2, 0, 1, 1, 1, 1, 0, -2,
    -2, 1, 2, 2, 2, 2, 1, -2,
    -2, 2, 2, 3, 3, 2, 2, -2,
    -2, 2, 2, 3, 3, 2, 2, -2,
    -2, 1, 2, 2, 2, 2, 1, -2,
    -2, 0, 1, 1, 1, 1, 0, -2,
    -2, -2, -2, 1, 1, -2, -2, -2,
];

const KING_MAP: [i32; 64] = [
    3, 3, 2, 1, 1, 2, 3, 3,
    0, 0, 0, 0, 0, 0, 0, 0,
    -2, -2, -2, -2, -2, -2, -2, -2,
    -2, -2, -2, -2, -2, -2, -2, -2,
    -2, -2, -2, -2, -2, -2, -2, -2,
    -2, -2, -2, -2, -2, -2, -2, -2,
    0, 0, 0, 0, 0, 0, 0, 0,
    3, 3, 2, 1, 1, 2, 3, 3,
];

const END_KING_MAP: [i32; 64] = [
    -2, -2, -2, 1, 1, -2, -2, -2,
    -2, 0, 0, 1, 1, 0, 0, -2,
    -2, 0, 1, 2, 2, 1, 0, -2,
    -2, 1, 2, 3, 3, 2, 1, -2,
    -2, 1, 2, 3, 3, 2, 1, -2,
    -2, 0, 1, 2, 2, 1, 0, -2,
    -2, 0, 0, 1, 1, 0, 0, -2,
    -2, -2, -2, 1, 1, -2, -2, -2,
];

pub fn calculate_material(board: &Board) -> i32 {
    let mut score = 0;
    let phase = evaluate_phase(board);
    
    for row in 1..=8 {
        for col in 1..=8 {
            if let Some(piece_state) = board.board[row][col].piece {
                let index = (8 - row) * 8 + (col - 1);
                let material = get_piece_value(piece_state.piece);
                let position = get_position_bonus(piece_state, index, phase);
                let total = material + position;
                
                score += if piece_state.color == Color::White { total } else { -total };
            }
        }
    }
    
    score
}

fn evaluate_phase(board: &Board) -> GamePhase {
    let mut material = 0;
    
    for row in 1..=8 {
        for col in 1..=8 {
            if let Some(piece) = board.board[row][col].piece {
                material += match piece.piece {
                    Piece::Pawn => 1,
                    Piece::Knight | Piece::Bishop => 3,
                    Piece::Rook => 5,
                    Piece::Queen => 9,
                    _ => 0,
                };
            }
        }
    }
    
    if material < 12 {
        GamePhase::Endgame
    } else if material < 25 {
        GamePhase::Middlegame
    } else {
        GamePhase::Opening
    }
}

fn get_piece_value(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => PAWN_VALUE,
        Piece::Knight => KNIGHT_VALUE,
        Piece::Bishop => BISHOP_VALUE,
        Piece::Rook => ROOK_VALUE,
        Piece::Queen => QUEEN_VALUE,
        Piece::King => KING_VALUE,
    }
}

fn get_position_bonus(piece_state: PieceState, index: usize, phase: GamePhase) -> i32 {
    match (piece_state.piece, piece_state.color) {
        (Piece::Pawn, Color::White) => WHITE_PAWN_MAP[index],
        (Piece::Pawn, Color::Black) => BLACK_PAWN_MAP[index],
        (Piece::Bishop, _) => BISHOP_MAP[index],
        (Piece::Knight, _) => KNIGHT_MAP[index],
        (Piece::Rook, _) => 0,
        (Piece::Queen, _) => QUEEN_MAP[index],
        (Piece::King, _) => {
            match phase {
                GamePhase::Endgame => END_KING_MAP[index],
                _ => KING_MAP[index]
            }
        }
    }
}