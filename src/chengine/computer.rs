use crate::chengine::*;
use reqwest;
use serde::Deserialize;

pub struct Computer {
    pub following_opening: bool,
    seek_opening: usize,
    curr_opening: &'static Opening,
    color: Color,
}

#[derive(Deserialize, Debug)]
struct EndgameResponse {
    dtz: i32,
    mainline: Vec<EndgameMove>
}
#[derive(Deserialize, Debug)]
struct EndgameMove {
    uci: String,
    san: String,
    dtz: i32
}

impl Computer {
    pub fn new(color: Color, opening: &'static Opening) -> Computer {
        Computer {
            following_opening: true,
            seek_opening: 0,
            curr_opening: opening,
            color: color,
        }
    }

    pub fn probe_tablebase(board: &Board) -> Result<(Square, Square), reqwest::Error> {
        println!("{:?}", &board.fen());
        let res = reqwest::blocking::get("http://tablebase.lichess.ovh/standard/mainline?fen=".to_string() + &board.fen())?.json::<EndgameResponse>()?;
        println!("{:?}", res);
        Ok((
            Square::new(&res.mainline[0].uci[0..2]).unwrap(),
            Square::new(&res.mainline[0].uci[2..4]).unwrap()
        ))
    }

    // fn negamax(board: &Board, curr_color: Color, mut alpha: i32, beta: i32, depth: u8) -> i32 {
    //     if depth == 0 {
    //         return board.eval(curr_color);
    //     }
    //     let mut max = i32::MIN + 1; // +1 to avoid overflow on negate
    //     for piece in board.get_pieces_iter(curr_color) {
    //         for square in piece.1.get_moves(board, piece.0) {
    //             let mut board_copy = board.clone();
    //             board_copy.exec_move(&piece.0, &square);
    //             let score = -Self::negamax(&board_copy, !curr_color, -beta, -alpha, depth - 1);
    //             if score > max {
    //                 max = score;
    //             }
    //             if score > alpha {
    //                 alpha = score;
    //                 if alpha >= beta {
    //                    break;
    //                 }
    //             }
    //         }
    //     }
    //     return max;
    // }

    // fn negamax(board: &Board, curr_color: Color, mut alpha: i32, beta: i32, depth: u8) -> i32 {
    //     if depth == 0 {
    //         return board.eval(curr_color);
    //     }
    //     let mut max = i32::MIN + 1; // +1 to avoid overflow on negate
    //     for (from, to) in board.get_moves(curr_color) {
    //         let mut board_copy = board.clone();
    //         board_copy.exec_move(&from, &to);
    //         let score = -Self::negamax(&board_copy, !curr_color, -beta, -alpha, depth - 1);
    //         if score > max {
    //             max = score;
    //         }
    //         if score > alpha {
    //             alpha = score;
    //             if alpha >= beta {
    //                break;
    //             }
    //         }
    //     }
    //     return max;
    // }

    fn negamax(board: &mut Board, curr_color: Color, mut alpha: f32, beta: f32, depth: u8) -> f32 {
        if depth == 0 {
            return board.eval(curr_color);
        }
        let mut max = f32::NEG_INFINITY; // +1 to avoid overflow on negate
        for (from, to) in board.get_moves(curr_color) {
            let move_data = board.exec_move(&from, &to);
            let score = -Self::negamax(board, !curr_color, -beta, -alpha, depth - 1);
            board.unexec_move(&from, &to, move_data);
            if score > max {
                max = score;
            }
            if score > alpha {
                alpha = score;
                if alpha >= beta {
                    break;
                }
            }
        }
        return max;
    }

    fn negamax_with_move(
        board: &Board,
        curr_color: Color,
        mut alpha: f32,
        beta: f32,
        depth: u8,
    ) -> (f32, Option<(Square, Square)>) {
        let mut max = (f32::NEG_INFINITY, None);
        for (from, to) in board.get_moves(curr_color) {
            let mut board_copy = board.clone();
            board_copy.exec_move(&from, &to);
            let score = -Self::negamax(&mut board_copy, !curr_color, -beta, -alpha, depth - 1);
            //println!("Possible move {} to {} earns min of (+adv, -dis) {}", piece.0, square, score);
            if score > max.0 {
                max = (score, Some((from, to)));
            }
            if score > alpha {
                alpha = score;
                if alpha >= beta {
                    break;
                }
            }
        }
        return max;
    }

    pub fn get_next_from_opening(
        &mut self,
        maybe_last_move: Option<(Square, Square)>,
    ) -> Option<(Square, Square)> {
        if self.seek_opening == self.curr_opening.moves.len() {
            match maybe_last_move {
                None => {
                    //my go
                    if self.curr_opening.next.len() == 0 {
                        return None;
                    } else {
                        self.curr_opening = &self.curr_opening.next[0];
                        self.seek_opening = 0;
                        return self.get_next_from_opening(None);
                    }
                }
                Some(last_move) => {
                    //their go
                    let mut found_opening = false;
                    for opening in &self.curr_opening.next {
                        if opening.moves[0] == last_move {
                            self.curr_opening = opening;
                            found_opening = true;
                            break;
                        }
                    }
                    if found_opening {
                        self.seek_opening = 1; //skip opponent move, NOTE = 1 NOT += 1
                        return self.get_next_from_opening(None);
                    } else {
                        return None;
                    }
                }
            }
        } else {
            match maybe_last_move {
                None => return Some(self.curr_opening.moves[self.seek_opening]), //my go
                Some(last_move) => {
                    //their go
                    if self.curr_opening.moves[self.seek_opening] == last_move {
                        self.seek_opening += 1;
                        return self.get_next_from_opening(None);
                    } else {
                        return None;
                    }
                }
            }
        }
    }

    pub fn get_move(
        &mut self,
        board: &Board,
        maybe_last_move: Option<(Square, Square)>,
        depth: u8,
    ) -> (f32, (Square, Square)) {
        if self.following_opening {
            if let Some(opening_move) = self.get_next_from_opening(maybe_last_move) {
                self.seek_opening += 1;
                return (f32::INFINITY, opening_move);
            } else {
                self.following_opening = false;
            }
        }
        if board.piece_count <= 7 {
            return (f32::INFINITY, Self::probe_tablebase(board).expect("Communication error"));
        }
        match Self::negamax_with_move(board, self.color, f32::NEG_INFINITY, f32::INFINITY, depth) {
            (a, Some(b)) => (a, b),
            _ => panic!(),
        }
    }
}
