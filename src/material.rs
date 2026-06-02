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
    -2, -1, 0, -1, -1, 0, -1, -2,
    -2, 2, 1, 1, 1, 1, 2, -2,
    -2, 0, 2, 1, 1, 2, 0, -2,
    -2, 1, 1, 3, 3, 1, 1, -2,
    -2, 1, 1, 3, 3, 1, 1, -2,
    -2, 0, 2, 1, 1, 2, 0, -2,
    -2, 2, 1, 1, 1, 1, 2, -2,
    -2, -1, 0, -1, -1, 0, -1, -2,
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
            if let Some(piece_state) = board.board[row - 1][col - 1].piece_state {
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
            if let Some(state) = board.board[row - 1][col - 1].piece_state {
                material += match state.piece {
                    Piece::Pawn => 1,
                    Piece::Knight | Piece::Bishop => 3,
                    Piece::Rook => 5,
                    Piece::Queen => 9,
                    Piece::King => 0,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pieces::*;

    fn empty_board() -> Board {
        EMPTY_BOARD
    }

    fn place_piece(board: &mut Board, row: usize, col: usize, piece: Piece, color: Color) {
        board.board[row - 1][col - 1].piece_state = Some(PieceState {
            color,
            piece,
            location: (row, col),
            has_moved: false,
            dead: false,
        });
    }

    // ---- get_piece_value ----

    #[test]
    fn test_pawn_value() {
        assert_eq!(get_piece_value(Piece::Pawn), 1);
    }

    #[test]
    fn test_queen_value() {
        assert_eq!(get_piece_value(Piece::Queen), 9);
    }

    #[test]
    fn test_king_value() {
        assert_eq!(get_piece_value(Piece::King), 0);
    }

    // ---- evaluate_phase ----

    #[test]
    fn test_opening_phase() {
        // START_BOARD has full material, should be Opening
        let phase = evaluate_phase(&START_BOARD);
        assert!(matches!(phase, GamePhase::Opening));
    }

    #[test]
    fn test_endgame_phase() {
        // just two kings on the board = very low material
        let mut board = empty_board();
        place_piece(&mut board, 1, 5, Piece::King, Color::White);
        place_piece(&mut board, 8, 5, Piece::King, Color::Black);
        let phase = evaluate_phase(&board);
        assert!(matches!(phase, GamePhase::Endgame));
    }

    #[test]
    fn test_middlegame_phase() {
        let mut board = empty_board();
        place_piece(&mut board, 1, 5, Piece::King, Color::White);
        place_piece(&mut board, 8, 5, Piece::King, Color::Black);
        place_piece(&mut board, 1, 1, Piece::Rook, Color::White);
        place_piece(&mut board, 1, 2, Piece::Rook, Color::White);
        place_piece(&mut board, 8, 1, Piece::Rook, Color::Black);
        place_piece(&mut board, 8, 2, Piece::Rook, Color::Black);
        // 4 rooks = 20 material, should be Middlegame
        let phase = evaluate_phase(&board);
        assert!(matches!(phase, GamePhase::Middlegame));
    }

    // ---- calculate_material ----

    #[test]
    fn test_empty_board_score_is_zero() {
        assert_eq!(calculate_material(&empty_board()), 0);
    }

    #[test]
    fn test_equal_material_near_zero() {
        // one white queen vs one black queen, same square index = score of 0
        let mut board = empty_board();
        place_piece(&mut board, 4, 4, Piece::Queen, Color::White);
        place_piece(&mut board, 5, 4, Piece::Queen, Color::Black);
        // material cancels out; position bonuses may differ slightly but score near 0
        let score = calculate_material(&board);
        // both queens present so white and black cancel on material (9 - 9 = 0)
        // just assert it's in a reasonable range
        assert!(score.abs() < 10);
    }

    #[test]
    fn test_white_up_a_queen() {
        let mut board = empty_board();
        place_piece(&mut board, 1, 5, Piece::King, Color::White);
        place_piece(&mut board, 8, 5, Piece::King, Color::Black);
        place_piece(&mut board, 4, 4, Piece::Queen, Color::White);
        let score = calculate_material(&board);
        assert!(score > 0, "white is up material, score should be positive");
    }

    #[test]
    fn test_black_up_a_rook() {
        let mut board = empty_board();
        place_piece(&mut board, 1, 5, Piece::King, Color::White);
        place_piece(&mut board, 8, 5, Piece::King, Color::Black);
        place_piece(&mut board, 4, 1, Piece::Rook, Color::Black);
        let score = calculate_material(&board);
        assert!(score < 0, "black is up material, score should be negative");
    }

    #[test]
    fn test_starting_position_is_balanced() {
        // starting position is perfectly symmetric, score should be 0
        println!("{}", calculate_material(&START_BOARD));
        assert_eq!(calculate_material(&START_BOARD), 0);
    }
}