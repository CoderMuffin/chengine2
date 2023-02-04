use crate::chengine::*;
use reqwest;
use serde::Deserialize;
use serde_json;

pub struct Computer {
    pub following_opening: bool,
    seek_opening: usize,
    curr_opening: &'static Opening,
    color: Color,
}

#[derive(Deserialize, Debug)]
struct EndgameResponse {
    moves: Vec<EndgameMove>,
}
#[derive(Deserialize, Debug)]
struct EndgameMove {
    uci: String,
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

    pub fn probe_tablebase(&self, board: &Board) -> Result<(Square, Square), &str> {
        let req = "http://tablebase.lichess.ovh/standard?fen=".to_string() + &board.fen(self.color);
        println!("{:?}", req);
        let res_text = match reqwest::blocking::get(req) {
            Ok(r) => match r.text() {
                Ok(r2) => r2,
                Err(..) => return Err("Couldn't get text from response"),
            },
            Err(..) => return Err("GET request failed"),
        };
        match serde_json::from_str::<EndgameResponse>(&res_text) {
            Ok(endgame_res) => Ok((
                Square::new(&endgame_res.moves[0].uci[0..2]).unwrap(),
                Square::new(&endgame_res.moves[0].uci[2..4]).unwrap(),
            )),
            Err(..) => {
                println!("JSON parse error: response '{}'", res_text);
                Err("JSON parse error")
            }
        }
    }

    fn quiescence(board: &mut Board, curr_color: Color, mut alpha: f32, beta: f32) -> f32 {
        let stand_pat = board.eval(curr_color);
        if stand_pat >= beta {
            return beta;
        }
        if alpha < stand_pat {
            alpha = stand_pat;
        }
        for (from, to) in board.get_moves(curr_color) {
            if !board.occupied(&to) { //should be faster than retain, maybe bench this?
                continue;
            }
            let move_data = board.exec_move(&from, &to);
            let score = -Self::quiescence(board, !curr_color, -beta, -alpha);
            board.unexec_move(&from, &to, move_data);
    
            if score >= beta {
                return beta;
            }
            if score > alpha {
               alpha = score;
            }
        }
        return alpha;
    }

    fn move_sort(board: &Board, a: &(Square, Square), b: &(Square, Square)) -> std::cmp::Ordering {
        board.square_value(&b.1).partial_cmp(&board.square_value(&a.1)).unwrap()
    }

    fn negamax(board: &mut Board, curr_color: Color, mut alpha: f32, beta: f32, depth: u8) -> f32 {
        if depth == 0 {
            return Self::quiescence(board, curr_color, alpha, beta);
        }
        let mut best = f32::NEG_INFINITY; // +1 to avoid overflow on negate
        let mut moves = board.get_moves(curr_color);
        moves.sort_by(|a, b| Self::move_sort(board, a, b));
        for (from, to) in moves {
            let move_data = board.exec_move(&from, &to);
            let score = -Self::negamax(board, !curr_color, -beta, -alpha, depth - 1);

            if score > best {
                best = score;
            }
            
            //this has to come before the break as the board is shared state
            board.unexec_move(&from, &to, move_data); 
            if best > alpha {
                alpha = best;
                if alpha >= beta {
                    break;
                }
            }
        }
        return best;
    }

    fn negamax_with_move(
        board: &Board,
        curr_color: Color,
        mut alpha: f32,
        beta: f32,
        depth: u8,
    ) -> (f32, Option<(Square, Square)>) {
        let mut best = (f32::NEG_INFINITY, None);
        let mut moves = board.get_moves(curr_color);
        moves.sort_by(|a, b| Self::move_sort(board, a, b));
        for (from, to) in moves {
            let mut board_copy = board.clone();
            board_copy.exec_move(&from, &to);
            let score = -Self::negamax(&mut board_copy, !curr_color, -beta, -alpha, depth - 1);
            //score = -score;
            //board.unexec_move(&from, &to, move_data);
            if score > best.0 {
                best = (score, Some((from, to)));
            }
            
            if best.0 > alpha {
                alpha = best.0;
                if alpha >= beta {
                    break;
                }
            }
        }
        return best;
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
            return (
                f32::INFINITY,
                self.probe_tablebase(board).expect("Communication error"),
            );
        }
        match Self::negamax_with_move(board, self.color, f32::NEG_INFINITY, f32::INFINITY, depth) {
            (a, Some(b)) => {
                //let b0 = b[0];
                //println!("{} {:?}", a, b.into_iter().map(|x| (x.0.disp(), x.1.disp())).collect::<Vec<_>>());
                (a, b)
            },
            _ => panic!(),
        }
    }
}
