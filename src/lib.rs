use pyo3::prelude::*;
use rayon::prelude::*;
use cozy_chess::{Board, Move, Color, Piece, Square, BitBoard};
use pyo3::exceptions::PyValueError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rand::Rng;
use std::thread;
use std::time::{Duration, Instant};
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicI32, AtomicBool, Ordering};
use chess::Color::White;
use pyo3::types::PyString;
use std::str::FromStr;
use pyo3::exceptions::socket::timeout;
use pyo3::indoc::eprintdoc;
pub mod search;
pub mod best_move;
pub mod hashing;
pub mod moves;
pub mod material;
pub mod pieces;
pub mod move_ordering;

lazy_static! {
    static ref ZOBRIST: Zobrist = Zobrist::new();
    static ref TABLE: transposition_table = transposition_table::new();
}

static mut GAME_STAGE: i32 = 1;
const PIECE_TYPES: usize = 12;
const BOARD_SQUARES: usize = 64;

#[derive(Debug, Clone, Copy)]
pub enum Color2 {
    White,
    Black,
}

// Implement IntoPy for Color
impl IntoPy<PyObject> for Color2 {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            Color2::White => PyString::new(py, "white").into_py(py),
            Color2::Black => PyString::new(py, "black").into_py(py),
        }
    }
}

// Convert from Python to Color (for receiving data from Python)
impl<'a> FromPyObject<'a> for Color2 {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let color_str: String = ob.extract()?;
        match color_str.as_str() {
            "white" => Ok(Color2::White),
            "black" => Ok(Color2::Black),
            _ => Err(pyo3::exceptions::PyValueError::new_err("Invalid color")),
        }
    }
}

#[pyfunction]
fn update_FEN(fen: String, opp_move: String) -> PyResult<String> {
    let mut board = match Board::from_fen(&fen, false) {
        Ok(b) => b,
        Err(_) => return Err(pyo3::exceptions::PyValueError::new_err("Rust: bad FEN string")),
    };

    let parsed_move = opp_move.parse::<Move>();
    if let Ok(m) = parsed_move {
        if board.is_legal(m) {
            board.play(m);
            let updated_fen = board.to_string();
            println!("fen: {}", updated_fen);
            Ok(updated_fen)
        } else {
            Err(PyValueError::new_err("Rust: illegal move"))
        }
    } else {
        Err(pyo3::exceptions::PyValueError::new_err("Rust: invalid move"))
    }
}

static mut SEARCH: bool = false;

pub struct AlphaBeta;

#[pyfunction]
fn find_best_move(fen: String, my_time: i32, game_on: bool, color_in: Color2) -> PyResult<String> {
    let color = match color_in {
        Color2::White => Color::White,
        Color2::Black => Color::Black,
    };
    let mut board = match Board::from_fen(&fen, false) {
        Ok(b) => Arc::new(Mutex::new(b)),
        Err(_) => return Err(pyo3::exceptions::PyValueError::new_err("Rust: bad FEN string")),
    };
    let hash = ZOBRIST.hash_position(&board.lock().unwrap());
    if let Some(entry) = TABLE.get(hash) {
        println!("In table best move: {}", entry.best_move);
        return Ok(entry.best_move.to_string());
    }

    if !game_on {
        return Ok("END".to_string());
    }
    let eval = calculate_material(&board.lock().unwrap());
    let time_limit = determine_time(eval, my_time, game_on, color);
    println!("time limit {}", time_limit);
    let best_move = AlphaBeta::start_alpha_beta_search(Arc::clone(&board), time_limit, game_on, color);
    println!("Made it past alphas");

    if time_limit == 0 {
        return Ok("END".to_string());
    }

    Ok(best_move.to_string())
}

fn color_to_str(color:Color) -> &'static str {
    match color {
        Color::White => "white",
        Color::Black=> "black",
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MoveType {
    PreviousBest,
    Check,
    Capture,
    Normal,
}

impl AlphaBeta {
    fn start_alpha_beta_search(board: Arc<Mutex<Board>>, time_limit: i32, game_on: bool, color: Color) -> Move {
        let start = Instant::now();
        let best_move = Arc::new(Mutex::new(None));
        let fallback = Move {
            from: cozy_chess::Square::A1,
            to: cozy_chess::Square::A1,
            promotion: None,
        };

        // Spawn thread for continuous search
        thread::spawn({
            println!("start alpha spawned thread!");
            let board = Arc::clone(&board);
            let best_move = Arc::clone(&best_move);
            let table = transposition_table::new();
            let mut best_score = -i32::MAX;
            move || {
                let mut depth = 1;
                let mut alpha = -i32::MAX;
                let mut beta = i32::MAX;

                while start.elapsed().as_millis() < time_limit as u128 {
                    // Get a fresh snapshot of the board state each iteration
                    let current_board = {
                        let board_locked = board.lock().unwrap();
                        board_locked.clone()
                    };

                    let candidate_move = AlphaBeta::iterative_deepening(
                        &current_board,
                        depth,
                        start,
                        time_limit,
                        color,
                        &table,
                    );
                    println!("Depth {} candidate: {}", depth, candidate_move);

                    {
                        // Update the shared board state with the candidate move.
                        let mut board_locked = board.lock().unwrap();
                        board_locked.play_unchecked(candidate_move);
                    }
                    // Update the shared best move
                    *best_move.lock().unwrap() = Some(candidate_move);

                    depth += 1; // Increase depth for the next iteration
                    if !game_on {
                        println!("Game over detected. Exiting search loop.");
                        break;
                    }
                }
            }
        });

        while start.elapsed().as_millis() < time_limit as u128 && game_on {
            println!("start alpha start while loop happened");
            if let Some(best) = best_move.lock().unwrap().clone() {
                println!("best in while loop in start alpha {}", best);;
                return best;
            }

            std::thread::sleep(std::time::Duration::from_millis(5));
        }

        let best = best_move.lock().unwrap().unwrap_or(fallback);
        println!("end best move{}", best);
        best
    }

    fn categorize_moves(board: &Board, previous_best: Option<Move>, color: Color) -> Vec<Move> {
        let mut total = Vec::new();
        //println!("categorize color {}", color);
        board.generate_moves(|moves| {
            total.extend(moves);
            false
        });
        let mut best_move = previous_best;
        let mut checks = Vec::new();
        let mut captures = Vec::new();
        let mut normal = Vec::new();

        if let Some(entry) = TABLE.get(ZOBRIST.hash_position(&board)) {
            if let Some(m) = Move::from_str(&entry.best_move).ok() {
                best_move = Some(m);
            }
        }

        let mut sorted_moves = Vec::new();

        for m in total {
            let mut new_board = board.clone();
            new_board.play_unchecked(m);
            let captured = new_board.piece_on(m.to);

            if Some(m) == best_move {
                sorted_moves.push(m);
            } else if new_board.checkers() != BitBoard::EMPTY {
                checks.push(m);
            } else if captured.is_some() {
                captures.push(m);
            } else {
                normal.push(m);
            }
        }
        if let Some(bm) = best_move {
            sorted_moves.push(bm);
        }
        sorted_moves.extend(checks);
        sorted_moves.extend(captures);
        sorted_moves.extend(normal);
        // println!("Total moves:");
        // for move_ in &sorted_moves {
        //     println!("{}", move_);
        // }

        sorted_moves
    }

    fn iterative_deepening(board: &Board, depth: i32, start: Instant, limit: i32, color: Color, tt: &transposition_table) -> Move {
        let fallback = Move {
            from: cozy_chess::Square::A1,
            to: cozy_chess::Square::A1,
            promotion: None,
        };
        // let current_color = if depth > 1 {
        //     if color == Color::White { Color::Black } else { Color::White }
        // } else {
        //     color
        // };
        //println!("iterative deepening color: {}", color);

        let mut best_move = Arc::new(Mutex::new(None));
        let previous_best = best_move.lock().unwrap().clone();
        let moves = Self::categorize_moves(&board, previous_best, color);
        let mut max_eval = Arc::new(AtomicI32::new(-i32::MAX));
        let mut root_move = Arc::new(Mutex::new(None));

        if moves.is_empty() {
            return fallback;
        }
        let root_clone = Arc::clone(&root_move);
        moves.par_iter().for_each(|m| {
            let mut new_board = board.clone();
            if new_board.is_legal(*m) {
                new_board.play_unchecked(*m);

                let eval = Self::alpha_beta_search(&new_board, depth - 1, -i32::MAX, i32::MAX, false, previous_best, color, tt);

                let mut best_move_lock = best_move.lock().unwrap();
                let current_max = max_eval.load(Ordering::Relaxed);
                if eval > current_max {
                    max_eval.store(eval, Ordering::Relaxed);
                    *best_move_lock = Some(*m);
                    let mut root_move_lock = root_clone.lock().unwrap();
                    *root_move_lock = Some(*m);
                }
            }
        });

        if start.elapsed().as_millis() < limit as u128 {
            return root_move.lock().unwrap().unwrap_or(moves[0]);
        }

        root_move.lock().unwrap().unwrap_or(moves[0])
    }

    fn alpha_beta_search(board: &Board, depth: i32, mut alpha: i32, mut beta: i32, max_player: bool, previous_best: Option<Move>, color: Color, tt: &transposition_table) -> i32 {
        //println!("SEARCHING...");
        let hash = board.hash();

        if let Some(entry) = tt.get(hash) {
            if entry.depth >= depth {
                match entry.flag {
                    flag_type::Exact => return entry.score,
                    flag_type::Lower => if entry.score > alpha { alpha = entry.score },
                    flag_type::Upper => if entry.score < beta { beta = entry.score },
                }
                if beta <= alpha {
                    return entry.score;
                }
            }
        }

        if depth == 0 {
            match calculate_material(&board) {
                Ok((white_pts, black_pts)) => {
                    if color == Color::White {
                        return white_pts;
                    } else {
                        return black_pts;
                    }
                }
                Err(e) => {
                    eprintln!("Error in SEARCH {}", e);
                    return 0;
                }
            }
        }

        //println!("Alpha Search Color {}", color);
        let moves = Self::categorize_moves(&board, previous_best, color);

        if max_player {
            let mut max_eval = -i32::MAX;
            for m in moves {
                let mut new_board = board.clone();
                if new_board.is_legal(m) {
                    new_board.play_unchecked(m);
                    let opponent_color = if color == Color::White { Color::Black } else { Color::White };
                    let eval = Self::alpha_beta_search(&new_board, depth - 1, alpha, beta, false, previous_best, opponent_color, tt);
                    max_eval = max_eval.max(eval);
                    alpha = alpha.max(eval);
                    if beta <= alpha {
                        break;
                    }
                }
            }

            tt.store(hash, Entry {
                score: max_eval,
                depth: depth as i32,
                flag: flag_type::Exact,
                best_move: "BestMove".to_string(),
            });

            max_eval
        } else {
            let mut min_eval = i32::MAX;
            for m in moves {
                let mut new_board = board.clone();
                if new_board.is_legal(m) {
                    new_board.play_unchecked(m);
                    let opponent_color = if color == Color::White { Color::Black } else { Color::White };
                    let eval = Self::alpha_beta_search(&new_board, depth - 1, alpha, beta, true, previous_best, opponent_color, tt);
                    min_eval = min_eval.min(eval);
                    beta = beta.min(eval);
                    if beta <= alpha {
                        break;
                    }
                }
            }

            tt.store(hash, Entry {
                score: min_eval,
                depth: depth as i32,
                flag: flag_type::Exact,
                best_move: "BestMove".to_string(),
            });

            min_eval
        }
    }
}

fn determine_time(eval: Result<(i32, i32), String>, time: i32, game_on: bool, mycolor: Color) -> i32 {
    if !game_on {
        return 0;
    }

    if time < 10_000 {
        return 200;
    }

    match eval {
        Ok((white_pts, black_pts)) => {
            if mycolor == Color::White {
                if white_pts < black_pts {
                    return 750;
                }
            } else {
                if black_pts < white_pts {
                    return 750;
                }
            }
        }
        Err(_) => {
            return 500;
        }
    }

    if unsafe { GAME_STAGE == 3 } {
        return 750;
    }

    500
}

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

impl transposition_table {
    pub fn new() -> Self {
        transposition_table {
            table: Mutex::new(HashMap::new()),
        }
    }

    pub fn store(&self, hash: u64, entry: Entry) {
        let mut table = self.table.lock().unwrap();
        table.insert(hash, entry);
    }

    pub fn get(&self, hash: u64) -> Option<Entry> {
        let table = self.table.lock().unwrap();
        table.get(&hash).map(|entry| {
            Entry {
               score: entry.score,
                depth: entry.depth,
                flag: entry.flag,
                best_move: entry.best_move.clone(),
            }
        })
    }
}

fn calculate_material(board: &Board) -> Result<(i32, i32), String> {
    let piece_values = [1, 3, 3, 5, 9, 0];
    let mut white_pts = 0;
    let mut black_pts = 0;

    let knight_pos_map: [i32; 64] = [
        -1, 1, -1, -1, -1, -1, 1, -1,
        -2, -1, 1, -1, -1, 1, -1, -2,
        -2, 0, 2, 1, 1, 2, 0, -2,
        -2, 1, 1, 3, 3, 1, 1, -2,
        -2, 1, 1, 3, 3, 1, 1, -2,
        -2, 0, 2, 1, 1, 2, 0, -2,
        -2, -1, 1, -1, -1, 1, -1, -2,
        -2, 1, -1, -1, -1, -1, 1, -2,
    ];

    let bishop_pos_map: [i32; 64] = [
        -2, -1, 1, -1, -1, 1, -1, -2,
        -1, 2, -1, 2, 2, -1, 2, -1,
        1, 1, 3, 2, 2, 3, 1, 1,
        -1, 2, 3, 4, 4, 3, 2, -1,
        -1, 2, 3, 4, 4, 3, 2, -1,
        1, 1, 3, 2, 2, 3, 1, 1,
        -1, 2, -1, 2, 2, -1, 2, -1,
        -2, -1, 1, -1, -1, 1, -1, -2,
    ];

    let queen_pos_map: [i32; 64] = [
        -2, -2, -2, 1, 1, -2, -2, -2,
        -2, 0, 1, 1, 1, 1, 0, -2,
        -2, 1, 2, 2, 2, 2, 1, -2,
        -2, 2, 2, 3, 3, 2, 2, -2,
        -2, 2, 2, 3, 3, 2, 2, -2,
        -2, 1, 2, 2, 2, 2, 1, -2,
        -2, 0, 1, 1, 1, 1, 0, -2,
        -2, -2, -2, 1, 1, -2, -2, -2,
    ];

    let king_begin_pos_map: [i32; 64] = [
        -1, 2, 2, 1, 1, 2, 2, -1,
        0, 0, 0, 0, 0, 0, 0, 0,
        -2, -2, -2, -2, -2, -2, -2, -2,
        -2, -2, -2, -2, -2, -2, -2, -2,
        -2, -2, -2, -2, -2, -2, -2, -2,
        -2, -2, -2, -2, -2, -2, -2, -2,
        0, 0, 0, 0, 0, 0, 0, 0,
        -1, 2, 2, 1, 1, 2, 2, -1,
    ];

    let king_end_pos_map: [i32; 64] = [
        -2, -2, -2, 1, 1, -2, -2, -2,
        -2, 0, 0, 1, 1, 0, 0, -2,
        -2, 0, 1, 2, 2, 1, 0, -2,
        -2, 1, 2, 3, 3, 2, 1, -2,
        -2, 1, 2, 3, 3, 2, 1, -2,
        -2, 0, 1, 2, 2, 1, 0, -2,
        -2, 0, 0, 1, 1, 0, 0, -2,
        -2, -2, -2, 1, 1, -2, -2, -2,
    ];

    let pawn_begin_pos_map: [i32; 64] = [
        2, 2, 2, 2, 2, 2, 2, 2,
        1, 1, 1, 1, 1, 1, 1, 1,
        0, 0, 1, 2, 2, 1, 0, 0,
        0, 0, 2, 3, 3, 2, 0, 0,
        0, 0, 2, 3, 3, 2, 0, 0,
        0, 0, 1, 2, 2, 1, 0, 0,
        1, 1, 1, 1, 1, 1, 1, 1,
        2, 2, 2, 2, 2, 2, 2, 2,
    ];

    for (piece, &value) in Piece::ALL.iter().zip(piece_values.iter()) {
        let piece_bb = board.pieces(*piece);
        let white_bb = piece_bb & board.colors(Color::White);
        let black_bb = piece_bb & board.colors(Color::Black);

        let white_count = white_bb.len() as i32;
        let black_count = black_bb.len() as i32;

        if *piece == Piece::Knight {
            for i in 0..64 {
                let square_bb = cozy_chess::BitBoard(1 << i);
                if (white_bb & square_bb) != cozy_chess::BitBoard(0) {
                    white_pts += knight_pos_map[i] * value;
                }
                if (black_bb & square_bb) != cozy_chess::BitBoard(0) {
                    black_pts += knight_pos_map[i] * value;
                }
            }
        } else if *piece == Piece::Bishop {
            for i in 0..64 {
                let square_bb = cozy_chess::BitBoard(1 << i);
                if (white_bb & square_bb) != cozy_chess::BitBoard(0) {
                    white_pts += bishop_pos_map[i] * value;
                }
                if (black_bb & square_bb) != cozy_chess::BitBoard(0) {
                    black_pts += bishop_pos_map[i] * value;
                }
            }
        } else if *piece == Piece::Queen {
            for i in 0..64 {
                let square_bb = cozy_chess::BitBoard(1 << i);
                if (white_bb & square_bb) != cozy_chess::BitBoard(0) {
                    white_pts += queen_pos_map[i] * value;
                }
                if (black_bb & square_bb) != cozy_chess::BitBoard(0) {
                    black_pts += queen_pos_map[i] * value;
                }
            }
        } else if *piece == Piece::King && unsafe { GAME_STAGE != 3 } {
            for i in 0..64 {
                let square_bb = cozy_chess::BitBoard(1 << i);
                if (white_bb & square_bb) != cozy_chess::BitBoard(0) {
                    white_pts += king_begin_pos_map[i] * value;
                }
                if (black_bb & square_bb) != cozy_chess::BitBoard(0) {
                    black_pts += king_begin_pos_map[i] * value;
                }
            }
        } else if *piece == Piece::King && unsafe { GAME_STAGE == 3 } {
            for i in 0..64 {
                let square_bb = cozy_chess::BitBoard(1 << i);
                if (white_bb & square_bb) != cozy_chess::BitBoard(0) {
                    white_pts += king_end_pos_map[i] * value;
                }
                if (black_bb & square_bb) != cozy_chess::BitBoard(0) {
                    black_pts += king_end_pos_map[i] * value;
                }
            }
        } else if *piece == Piece::Pawn && unsafe { GAME_STAGE != 3 } {
            for i in 0..64 {
                let square_bb = cozy_chess::BitBoard(1 << i);
                if (white_bb & square_bb) != cozy_chess::BitBoard(0) {
                    white_pts += pawn_begin_pos_map[i] * value;
                }
                if (black_bb & square_bb) != cozy_chess::BitBoard(0) {
                    black_pts += pawn_begin_pos_map[i] * value;
                }
            }
        } else {
            white_pts += white_count * value;
            black_pts += black_count * value;
        }
    }
    //println!("white eval: {}", white_pts);
    //println!("black eval: {}", black_pts);
    Ok((white_pts, black_pts))
}

#[pymodule]
fn chessbot(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(update_FEN, m)?)?;
    m.add_function(pyo3::wrap_pyfunction!(find_best_move, m)?)?;
    Ok(())
}
