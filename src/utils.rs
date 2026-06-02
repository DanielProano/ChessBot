use crate::pieces::*;

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
        assert_eq!(result.target.column, 1);
        assert_eq!(result.target.row, 1);
    }

    #[test]
    fn test_h8_converts_correctly() {
        let result = simple_algebraic_to_grid("h8").unwrap();
        assert_eq!(result.target.column, 8);
        assert_eq!(result.target.row, 8);
    }

    #[test]
    fn test_e4_converts_correctly() {
        let result = simple_algebraic_to_grid("e4").unwrap();
        assert_eq!(result.target.column, 5);
        assert_eq!(result.target.row, 4);
    }

    #[test]
    fn test_d6_converts_correctly() {
        let result = simple_algebraic_to_grid("d6").unwrap();
        assert_eq!(result.target.column, 4);
        assert_eq!(result.target.row, 6);
    }

    #[test]
    fn test_all_columns_map_correctly() {
        let cols = [
            ("a1", 1), ("b1", 2), ("c1", 3), ("d1", 4),
            ("e1", 5), ("f1", 6), ("g1", 7), ("h1", 8),
        ];
        for (notation, expected_col) in cols {
            let result = simple_algebraic_to_grid(notation).unwrap();
            assert_eq!(result.target.column, expected_col, "failed for {}", notation);
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
            assert_eq!(result.target.row, expected_row, "failed for {}", notation);
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
        assert!(result.target.piece_state.is_none());
    }
}