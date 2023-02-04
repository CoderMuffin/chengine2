//export CARGO_HOME=/home/runner/StaticDistinctSlope/cargo_stash
//use rand::seq::{IteratorRandom, SliceRandom};
mod chengine;
use crate::chengine::*;

static PERSPECTIVE: Color = Color::Black;

fn input_move(
    board: &mut Board,
    color: Color,
    stdin: &std::io::Stdin,
) -> std::io::Result<(Square, Square)> {
    let mut from;
    let mut to;
    loop {
        from = String::new();
        to = String::new();
        stdin.read_line(&mut from)?;
        if from.starts_with("?") {
            if let Some(sq) = Square::new(&from[1..]) {
                board.highlight_piece = Some(sq);
                board.display(PERSPECTIVE);
                continue;
            } else {
                println!("Cannot get moves for {}", &from[1..]);
                continue;
            }
        } else {
            board.highlight_piece = None;
        }
        stdin.read_line(&mut to)?;
        let (fromsq, tosq) = match (Square::new(&from), Square::new(&to)) {
            (Some(a), Some(b)) => (a, b),
            _ => {
                println!("Invalid");
                board.display(PERSPECTIVE);
                continue;
            }
        };

        let mut moves = board.get_moves(color);
        board.filter_checks(&mut moves, color);
        if moves.iter().any(|x| *x == (fromsq, tosq)) {
            return Ok((fromsq, tosq));
        }
        println!("Invalid");
        board.display(PERSPECTIVE);
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

// fn board_damage_minification() -> Board {
//     let mut pieces = [[None; 8]; 8];
//     pieces[7][7] = Some(Piece::new('k', Color::White));
//     pieces[2][1] = Some(Piece::new('k', Color::Black));
//     pieces[1][1] = Some(Piece::new('p', Color::Black));
//     pieces[1][2] = Some(Piece::new('r', Color::White));
//     pieces[2][2] = Some(Piece::new('n', Color::White));
//     Board::from(pieces, Square { x: 7, y: 7 }, Square { x: 1, y: 2 })
// }

// fn board_rook_checkmate() -> Board {
//     let mut pieces = [[None; 8]; 8];
//     pieces[7][7] = Some(Piece::new('k', Color::White));
//     pieces[2][1] = Some(Piece::new('k', Color::Black));
//     pieces[1][2] = Some(Piece::new('r', Color::White));
//     Board::from(pieces, Square { x: 7, y: 7 }, Square { x: 1, y: 2 })
// }

fn main() -> std::io::Result<()> {
    let depth = 5;
    let mut board = Board::new();
    let mut current_color = Color::White;
    let stdin = std::io::stdin();
    let mut computer1 = Computer::new(Color::White, &OPENING_BOOK);
    let mut computer2 = Computer::new(Color::Black, &OPENING_BOOK);
    computer1.following_opening = true;
    computer2.following_opening = false;
    //computer.following_opening = false;
    let mut last_move = None;
    //board.exec_move(&Square::new("e2").unwrap(), &Square::new("e4").unwrap());
    //board.exec_move(&Square::new("e7").unwrap(), &Square::new("e5").unwrap());
    // board.exec_move(&Square::new("d8"), &Square::new("e4"));
    // board.exec_move(&Square::new("d1"), &Square::new("d4"));
    // board.exec_move(&Square::new("h1"), &Square::new("f5"));
    //let rng = &mut rand::thread_rng();
    board.display(PERSPECTIVE);
    //println!("{}", board.fen(Color::White));

    // let mut pieces = [[None; 8]; 8];
    // pieces[0][3] = Some(Piece::new('k', Color::Black));
    // pieces[4][3] = Some(Piece::new('q', Color::Black));
    // pieces[6][3] = Some(Piece::new('b', Color::White));
    // pieces[7][3] = Some(Piece::new('r', Color::White));
    // pieces[7][4] = Some(Piece::new('k', Color::White));
    // println!("{}", Board::from(pieces, Square { x: 4, y: 7 }, Square { x: 3, y: 0 }).fen());
    loop {
        //println!("{:?}",pieces);
        // let mut moves = Vec::new();
        // let mut piece = &(Square { x: 16, y: 16 }, Piece::new('q', Color::White));
        // while moves.len() == 0 {
        //     piece = pieces.choose(rng).unwrap();
        //     moves = piece.1.get_moves(&board, piece.0);
        // }
        //println!("{:?}", moves);

        // let now = std::time::Instant::now();
        // let best = match current_color {
        //     Color::White => &mut computer1,
        //     Color::Black => &mut computer2
        // }.get_move(&board, last_move, depth);
        // println!("Found move\nMinimum value: {}\nDepth: {}\nTime: {:.2?}", best.0, depth, now.elapsed());
        // last_move = Some(best.1);
        if current_color == Color::White {
            let now = std::time::Instant::now();
            let best = computer1.get_move(&board, last_move, depth);
            println!(
                "Found move\nMinimum value: {}\nDepth: {}\nTime: {:.2?}",
                best.0,
                depth,
                now.elapsed()
            );
            last_move = Some(best.1);
        } else {
            last_move = Some(input_move(&mut board, current_color, &stdin)?);
        }
        let (from, to) = last_move.unwrap();
        board.exec_move(&from, &to);
        board.highlight_move = (from, to);
        //stdin.wait_for_enter()?;
        //board.exec_move(&piece.0, moves.choose(rng).unwrap());
        println!(
            "Move: {} to {}\nEval (+white, -black): {}\nWhite in check: {}\nBlack in check: {}\nUsing opening book: {}",
            from, to,
            board.eval(Color::White),
            board.king_in_check(Color::White),
            board.king_in_check(Color::Black),
            computer1.following_opening
        );
        board.display(PERSPECTIVE);
        if board.eval(Color::White) == CHECKMATE {
            println!("White wins");
            break;
        } else if board.eval(Color::Black) == CHECKMATE {
            println!("Black wins");
            break;
        }
        current_color = !current_color;
        //stdin.wait_for_enter()?;
    }
    Ok(())
}
