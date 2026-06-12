use core::num;

use crate::pieces::*;
use crate::moves::*;

pub fn simple_algebraic_to_grid(notation: &str) -> Option<Square> {
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

    Some(square)
}


pub fn access_board(board: &Board, row: usize, column: usize) -> Option<Square> {
    if !in_bounds(row, column) {
        return None;
    }

    let normalized_row = row - 1;
    let normalized_col = column - 1;

    Some(board.board[normalized_row][normalized_col])
}

pub fn in_bounds(row: usize, col: usize) -> bool {
    if row <= 8 && row >= 1 && col <= 8 && col >= 1 {
        return true;
    }

    return false;
}

pub fn square_is_attacked(square: Square, active_color: Color, board: &Board) -> bool {
    let cur_row = square.row as i32;
    let cur_col = square.column as i32;

    for &(row, col) in &KNIGHT_DELTAS {
        let target_row = (cur_row + row) as usize;
        let target_col = (cur_col + col) as usize;

        if in_bounds(target_row, target_col) {
            if let Some(square) = access_board(&board, target_row, target_col) {
                if let Some(state) = square.piece_state {
                    if state.piece == Piece::Knight && state.color != active_color {
                        return true;
                    }
                }
            }
        }
    }

    if row_is_attacked(square, active_color, board) {
        return true;
    }

    if column_is_attacked(square, active_color, board) {
        return true;
    }

    if diagonal_is_attacked(square, active_color, board) {
        return true;
    }

    let pawn_row_offset = match active_color {
        Color::White => -1, 
        Color::Black => 1,
    };

    let pawn_deltas = [(pawn_row_offset, 1), (pawn_row_offset, -1)];

    for &(row, col) in &pawn_deltas {
        let target_row = (cur_row + row) as usize;
        let target_col = (cur_col + col) as usize;

        if in_bounds(target_row, target_col) {
            if let Some(square) = access_board(board, target_row, target_col) {
                if let Some(state) = square.piece_state {
                    if state.piece == Piece::Pawn && state.color != active_color {
                        return true;
                    }
                }
            }
        }
    }

    for &(row, col) in &KING_DELTAS {
        let target_row = (cur_row + row) as usize;
        let target_col = (cur_col + col) as usize;

        if in_bounds(target_row, target_col) {
            if let Some(square) = access_board(board, target_row, target_col) {
                if let Some(state) = square.piece_state {
                    if state.piece == Piece::King && state.color != active_color {
                        return true;
                    }
                }
            }
        }
    }

    false
}

pub fn column_is_attacked(square: Square, active_color: Color, board: &Board) -> bool {
    let initial_row = square.row;
    let cur_col = square.column;

    for row in (initial_row + 1)..=8 {
        if in_bounds(row, cur_col) {
            if let Some(square) = access_board(board, row, cur_col) {
                if let Some(state) = square.piece_state {
                    if (state.piece == Piece::Rook || state.piece == Piece::Queen) && state.color != active_color {
                        return true;
                    }
                    break;
                }
            }
        }
    }

    for row in (0..initial_row).rev() {
        if in_bounds(row, cur_col) {
            if let Some(square) = access_board(board, row, cur_col) {
                if let Some(state) = square.piece_state {
                    if (state.piece == Piece::Rook || state.piece == Piece::Queen) && state.color != active_color {
                        return true;
                    }
                    break;
                }
            }
        }
    }

    return false;
}

pub fn row_is_attacked(square: Square, active_color: Color, board: &Board) -> bool {
    let cur_row = square.row;
    let initial_col = square.column;

    for col in (initial_col + 1)..=8 {
        if in_bounds(cur_row, col) {
            if let Some(square) = access_board(board, cur_row, col) {
                if let Some(state) = square.piece_state {
                    if (state.piece == Piece::Rook || state.piece == Piece::Queen) && state.color != active_color {
                        return true;
                    }
                    break;
                }
            }
        }
    }

    for col in (0..initial_col).rev() {
        if in_bounds(cur_row, col) {
            if let Some(square) = access_board(board, cur_row, col) {
                if let Some(state) = square.piece_state {
                    if (state.piece == Piece::Rook || state.piece == Piece::Queen) && state.color != active_color {
                        return true;
                    }
                    break;
                }
            }
        }
    }
    
    return false;
}

pub fn diagonal_is_attacked(square: Square, active_color: Color, board: &Board) -> bool {
    let initial_row: i32 = square.row as i32;
    let initial_col: i32 = square.column as i32;

    for &(r_idx, c_idx) in &BISHOP_DELTAS {
        for multiplier in 1..=8 {
            let target_row = (initial_row + r_idx * multiplier) as usize;
            let target_col = (initial_col + c_idx * multiplier) as usize;

            if !in_bounds(target_row, target_col) {
                break;
            }

            if let Some(square) = access_board(board, target_row, target_col) {
                if let Some(state) = square.piece_state {
                    if (state.piece == Piece::Queen || state.piece == Piece::Bishop) && state.color != active_color {
                        return true;
                    }
                    break;
                }
            }
        }
    }

    return false;
}

pub fn is_legal_move(mv: &Move, board: &Board, color: Color) -> bool {
    let mut test_board = board.clone();
    test_board.board[mv.previous_square.row - 1][mv.previous_square.column - 1].piece_state = None;
    test_board.board[mv.current_square.row - 1][mv.current_square.column - 1] = mv.current_square;
    if let Some(captured) = mv.captured_piece {
        test_board.board[captured.location.0 - 1][captured.location.1 - 1].piece_state = None;
    }
    let Some(king_square) = find_king(&test_board, color) else {
        return true;
    };

    !square_is_attacked(king_square, color, &test_board)
}

pub fn find_king(board: &Board, color: Color) -> Option<Square> {
    for row in 1..=8 {
        for col in 1..=8 {
            if let Some(square) = access_board(board, row, col) {
                if let Some(state) = square.piece_state {
                    if state.piece == Piece::King && state.color == color {
                        return Some(square);
                    }
                }
            }
        }
    }

    None
}

pub fn create_check_mask(board: &Board, color: Color) -> CheckMask {
    let Some(king_square) = find_king(board, color) else {
        return CheckMask { check_mask: [[true; 8]; 8] };
    };

   
    get_checks(king_square, color, board)
}

pub fn get_checks(king_square: Square, active_color: Color, board: &Board, ) -> CheckMask {
    if king_square.piece_state.is_none() {
        return CheckMask { check_mask: [[true; 8]; 8] }
    }

    let cur_row = king_square.row;
    let cur_col = king_square.column;
    let mut num_attacking_pieces = 0;
    let mut mask = CheckMask { check_mask: [[false; 8]; 8] };

    for &(row, col) in &KNIGHT_DELTAS {
        let target_row = (cur_row as i32 + row) as usize;
        let target_col = (cur_col as i32 + col) as usize;

        if in_bounds(target_row, target_col) {
            if let Some(square) = access_board(&board, target_row, target_col) {
                if let Some(state) = square.piece_state {
                    if state.piece == Piece::Knight && state.color != active_color {
                        mask.check_mask[target_row - 1][target_col - 1] = true;
                        num_attacking_pieces += 1;
                    }
                }
            }
        }
    }


    for target_row in (cur_row + 1)..=8 {
        if in_bounds(target_row, cur_col) {
            if let Some(square) = access_board(board, target_row, cur_col) {
                if let Some(state) = square.piece_state {
                    if (state.piece == Piece::Rook || state.piece == Piece::Queen) && state.color != active_color {
                        for trace_row in cur_row..=target_row {
                            mask.check_mask[trace_row - 1][cur_col - 1] = true;
                        }
                        num_attacking_pieces += 1;
                    }
                    break;
                }
            }
        }
    }

    for target_row in (0..cur_row).rev() {
        if in_bounds(target_row, cur_col) {
            if let Some(square) = access_board(board, target_row, cur_col) {
                if let Some(state) = square.piece_state {
                    if (state.piece == Piece::Rook || state.piece == Piece::Queen) && state.color != active_color {
                        for trace_row in target_row..=cur_row {
                            mask.check_mask[trace_row - 1][cur_col - 1] = true;
                        }
                        num_attacking_pieces += 1;
                    }
                    break;
                }
            }
        }
    }

    for target_col in (cur_col + 1)..=8 {
        if in_bounds(cur_row, target_col) {
            if let Some(square) = access_board(board, cur_row, target_col) {
                if let Some(state) = square.piece_state {
                    if (state.piece == Piece::Rook || state.piece == Piece::Queen) && state.color != active_color {
                        for trace_col in cur_col..=target_col {
                            mask.check_mask[cur_row - 1][trace_col - 1] = true;
                        }
                        num_attacking_pieces += 1;
                    }
                    break;
                }
            }
        }
    }

    for target_col in (0..cur_col).rev() {
        if in_bounds(cur_row, target_col) {
            if let Some(square) = access_board(board, cur_row, target_col) {
                if let Some(state) = square.piece_state {
                    if (state.piece == Piece::Rook || state.piece == Piece::Queen) && state.color != active_color {
                        for trace_col in target_col..=cur_col {
                            mask.check_mask[cur_row - 1][trace_col - 1] = true;
                        }
                        num_attacking_pieces += 1;
                    }
                    break;
                }
            }
        }
    }

    for &(r_idx, c_idx) in &BISHOP_DELTAS {
        for multiplier in 1..=8 {
            let target_row = (cur_row as i32 + r_idx * multiplier) as usize;
            let target_col = (cur_col as i32 + c_idx * multiplier) as usize;

            if !in_bounds(target_row, target_col) {
                break;
            }

            if let Some(square) = access_board(board, target_row, target_col) {
                if let Some(state) = square.piece_state {
                    if (state.piece == Piece::Queen || state.piece == Piece::Bishop) && state.color != active_color {
                        mask.check_mask[(cur_row - 1) as usize][(cur_col - 1) as usize] = true;
                        
                        for step in 1..=multiplier {
                            let trace_row = (cur_row as i32 + r_idx * step) as usize;
                            let trace_col = (cur_col as i32 + c_idx * step) as usize;
                            mask.check_mask[trace_row - 1][trace_col - 1] = true;
                        }
                        num_attacking_pieces += 1;
                    }
                    break;
                }
            }
        }
    }

    let pawn_row_offset: i32 = match active_color {
        Color::White => -1, 
        Color::Black => 1,
    };

    let pawn_deltas = [(pawn_row_offset, 1), (pawn_row_offset, -1)];

    for &(row, col) in &pawn_deltas {
        let target_row = (cur_row as i32 + row) as usize;
        let target_col = (cur_col as i32 + col) as usize;

        if in_bounds(target_row, target_col) {
            if let Some(square) = access_board(board, target_row, target_col) {
                if let Some(state) = square.piece_state {
                    if state.piece == Piece::Pawn && state.color != active_color {
                        mask.check_mask[target_row - 1][target_col - 1] = true;
                        num_attacking_pieces += 1;
                    }
                }
            }
        }
    }

    if num_attacking_pieces == 0 {
        return CheckMask { check_mask: [[true; 8]; 8] };
    }

    if num_attacking_pieces > 1 {
        return CheckMask { check_mask: [[false; 8]; 8] };
    }

    mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dash_returns_none() {
        assert!(simple_algebraic_to_grid("-").is_none());
    }

    #[test]
    fn test_a1_converts_correctly() {
        let result = simple_algebraic_to_grid("a1").unwrap();
        assert_eq!(result.column, 1);
        assert_eq!(result.row, 1);
    }

    #[test]
    fn test_h8_converts_correctly() {
        let result = simple_algebraic_to_grid("h8").unwrap();
        assert_eq!(result.column, 8);
        assert_eq!(result.row, 8);
    }

    #[test]
    fn test_e4_converts_correctly() {
        let result = simple_algebraic_to_grid("e4").unwrap();
        assert_eq!(result.column, 5);
        assert_eq!(result.row, 4);
    }

    #[test]
    fn test_d6_converts_correctly() {
        let result = simple_algebraic_to_grid("d6").unwrap();
        assert_eq!(result.column, 4);
        assert_eq!(result.row, 6);
    }

    #[test]
    fn test_all_columns_map_correctly() {
        let cols = [
            ("a1", 1), ("b1", 2), ("c1", 3), ("d1", 4),
            ("e1", 5), ("f1", 6), ("g1", 7), ("h1", 8),
        ];
        for (notation, expected_col) in cols {
            let result = simple_algebraic_to_grid(notation).unwrap();
            assert_eq!(result.column, expected_col, "failed for {}", notation);
        }
    }

    #[test]
    fn test_all_rows_map_correctly() {
        let rows = [
            ("a1", 1), ("a2", 2), ("a3", 3), ("a4", 4),
            ("a5", 5), ("a6", 6), ("a7", 7), ("a8", 8),
        ];
        for (notation, expected_row) in rows {
            let result = simple_algebraic_to_grid(notation).unwrap();
            assert_eq!(result.row, expected_row, "failed for {}", notation);
        }
    }

    #[test]
    #[should_panic(expected = "Invalid algebraic length")]
    fn test_too_long_panics() {
        simple_algebraic_to_grid("e44");
    }

    #[test]
    #[should_panic(expected = "Invalid algebraic length")]
    fn test_empty_panics() {
        simple_algebraic_to_grid("");
    }

    #[test]
    #[should_panic(expected = "Invalid algebraic syntax")]
    fn test_invalid_column_panics() {
        simple_algebraic_to_grid("z4");
    }

    #[test]
    #[should_panic(expected = "Invalid algebraic syntax")]
    fn test_invalid_row_panics() {
        simple_algebraic_to_grid("a9");
    }

    #[test]
    #[should_panic(expected = "Invalid algebraic syntax")]
    fn test_special_char_panics() {
        simple_algebraic_to_grid("!4");
    }

    #[test]
    fn test_result_has_no_piece_state() {
        let result = simple_algebraic_to_grid("e4").unwrap();
        assert!(result.piece_state.is_none());
    }

    
}