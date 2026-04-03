use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn find_best_move(fen: String) -> String {

    // Parse FEN 

    // Iterative Deepening + Alpha Beta Search

    // Return result
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