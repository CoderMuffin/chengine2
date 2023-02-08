//export CARGO_HOME=/home/runner/StaticDistinctSlope/cargo_stash
//use rand::seq::{IteratorRandom, SliceRandom};
mod chengine;
use crate::chengine::*;

static DEPTH: u8 = 5;
static PERSPECTIVE: Color = Color::Black;

enum InputResult {
    Move((Square, Square)),
    Undo,
    NoChange
}
use InputResult::*;

fn input_move(
    board: &mut Board,
    color: Color,
    stdin: &std::io::Stdin,
    last_move: &Option<(Square, Square)>,
    computers: (&mut Computer, &mut Computer),
    moves: &mut Vec<(Square, Square, MoveData)>,
) -> Option<InputResult> {
    board.display(PERSPECTIVE);
    let mut line_buf = String::new();
    stdin.read_line(&mut line_buf).ok()?;
    let mut iter = line_buf.split(" ");
    match iter.next()?.trim() {
        "go" => {
            let now = std::time::Instant::now();
            let to_move = match color {
                Color::White => computers.0.get_move(board, last_move, DEPTH),
                Color::Black => computers.1.get_move(board, last_move, DEPTH),
            };
            println!(
                "Found move\nMinimum value: {}\nDepth: {}\nTime: {:.2?}",
                to_move.0,
                DEPTH,
                now.elapsed()
            );
            moves.push((
                to_move.1 .0,
                to_move.1 .1,
                board.exec_move(&to_move.1 .0, &to_move.1 .1),
            ));
            board.highlight_move = to_move.1;
            Some(Move(to_move.1))
        }
        "move" => {
            let from = Square::new(iter.next()?.trim())?;
            let to = Square::new(iter.next()?.trim())?;

            let mut valid_moves = board.get_moves(color);
            board.filter_checks(&mut valid_moves, color);
            if valid_moves.iter().any(|x| *x == (from, to)) {
                moves.push((from, to, board.exec_move(&from, &to)));
                board.highlight_move = (from, to);
                Some(Move((from, to)))
            } else {
                None
            }
        }
        "query" => {
            let sq = Square::new(iter.next()?.trim())?;
            board.highlight_piece = Some(sq);

            Some(NoChange)
        }
        "undo" => {
            if let Some(old_move) = moves.pop() {
                board.unexec_move(&old_move.0, &old_move.1, old_move.2);
                Some(Undo)
            } else {
                Some(NoChange)
            }
        }
        _ => None,
    }
}

trait StdinExtension {
    fn wait_for_enter(&self) -> std::io::Result<()>;
}

impl StdinExtension for std::io::Stdin {
    fn wait_for_enter(&self) -> std::io::Result<()> {
        let mut buf = String::new();
        self.read_line(&mut buf)?;
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let mut board = Board::new();
    let mut current_color = Color::White;
    let stdin = std::io::stdin();
    let mut last_move = None;

    let mut moves = Vec::new();
    let mut computer_white: Computer = Computer::new(Color::White, &OPENING_BOOK);
    let mut computer_black: Computer = Computer::new(Color::Black, &OPENING_BOOK);

    loop {
        match input_move(
            &mut board,
            current_color,
            &stdin,
            &last_move,
            (&mut computer_white, &mut computer_black),
            &mut moves,
        ) {
            Some(Move(new_move)) => {
                    println!(
                    "Move: {} to {}\nEval (+white, -black): {}\nWhite in check: {}\nBlack in check: {}",
                    new_move.0,
                    new_move.1,
                    board.eval(Color::White),
                    board.king_in_check(Color::White),
                    board.king_in_check(Color::Black),
                );

                last_move = Some(new_move);

                if board.eval(Color::White) == CHECKMATE {
                    println!("White wins");
                    break;
                } else if board.eval(Color::Black) == CHECKMATE {
                    println!("Black wins");
                    break;
                }
                current_color = !current_color;
            },
            Some(Undo) => {
                current_color = !current_color;
            }
            Some(NoChange) => {},
            None => {
                println!("Error");
            }
        }
    }
    Ok(())
}
