use std::collections::HashMap;
use std::env;
use std::time::Instant;
mod board;
use board::*;

fn main() {
    let mut previous_boards = HashMap::new();

    // Sample board
    let board3 = Board::new(6,6, (5,2), vec![
        Piece::marked((0,2), 2, Direction::Horizontal),
        Piece::new((0,3), 2, Direction::Horizontal),
        Piece::new((0,4), 2, Direction::Vertical),
        Piece::new((1,4), 2, Direction::Vertical),
        Piece::new((2,0), 2, Direction::Vertical),
        Piece::new((2,2), 2, Direction::Vertical),
        Piece::new((2,4), 2, Direction::Horizontal),
        Piece::new((2,5), 2, Direction::Horizontal),
        Piece::new((3,0), 3, Direction::Horizontal),
        Piece::new((3,3), 2, Direction::Horizontal),
        Piece::new((3,1), 2, Direction::Vertical),
        Piece::new((5,2), 3, Direction::Vertical),
    ]);

    let now = Instant::now();
    
    let (mut board, steps) = solve(board3, &mut previous_boards);
    println!("Total steps: {}", steps);
    
    let mut history = vec![];
    while let Some(entry) = previous_boards.get(&board) {
        
        if let Some(prev_move) = entry {
            history.push(prev_move.to_owned());
            board = board.undo(prev_move);
        } else { break; }
    }
    history.reverse();
    println!("Total time: {} ms", now.elapsed().as_millis());

    let args: Vec<String> = env::args().skip(1).collect();
    
    if !args.is_empty() && args[0] == "--verbose" {
        history.iter().for_each(|step| println!("{}", step));
    }
   
}


/// Solve a given board and return the number of steps and the final board
fn solve(start: Board, visited: &mut HashMap<Board, Option<Move>>) -> (Board, u32) {
    let mut boards = start.future_boards();
    visited.insert(start, None);
    let mut steps = 0;
    loop {
        let mut new_boards = vec![];
        // remove the board configurations that we already have visited
        // and add the new ones to our transposition table
        boards = boards
            .into_iter()
            .filter(|(board, mov)| {
                if visited.contains_key(board) {
                    false
                } else {
                    visited.insert(board.to_owned(), Some(*mov));
                    true
                }
            })
            .collect();

        steps += 1;

        for (board, _) in boards {
            if board.is_won { return (board, steps); }
            new_boards.append(
                &mut board.future_boards()
            );
        }

        boards = new_boards;
    }
}
