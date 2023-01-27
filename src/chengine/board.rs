use crate::chengine::*;
use std::fmt;

#[derive(Clone)]
pub struct Board {
    pieces: [[Option<Piece>; 8]; 8],
    pub highlight_move: (Square, Square),
    pub highlight_piece: Option<Square>,
    curr_points: i32,
    king_white: Square,
    king_black: Square,
    pub piece_count: u8
    // past_states: Vec<Board> //FOR DEBUG ONLY REMOVE ASAP
}

impl Board {
    pub fn new() -> Board {
        let mut pieces = [[None; 8]; 8];
        pieces[1][0] = Some(Piece::new('p', Color::White));
        pieces[1][1] = Some(Piece::new('p', Color::White));
        pieces[1][2] = Some(Piece::new('p', Color::White));
        pieces[1][3] = Some(Piece::new('p', Color::White));
        pieces[1][4] = Some(Piece::new('p', Color::White));
        pieces[1][5] = Some(Piece::new('p', Color::White));
        pieces[1][6] = Some(Piece::new('p', Color::White));
        pieces[1][7] = Some(Piece::new('p', Color::White));

        pieces[0][0] = Some(Piece::new('r', Color::White));
        pieces[0][1] = Some(Piece::new('n', Color::White));
        pieces[0][2] = Some(Piece::new('b', Color::White));
        pieces[0][3] = Some(Piece::new('q', Color::White));
        pieces[0][4] = Some(Piece::new('k', Color::White));
        pieces[0][5] = Some(Piece::new('b', Color::White));
        pieces[0][6] = Some(Piece::new('n', Color::White));
        pieces[0][7] = Some(Piece::new('r', Color::White));

        pieces[6][0] = Some(Piece::new('p', Color::Black));
        pieces[6][1] = Some(Piece::new('p', Color::Black));
        pieces[6][2] = Some(Piece::new('p', Color::Black));
        pieces[6][3] = Some(Piece::new('p', Color::Black));
        pieces[6][4] = Some(Piece::new('p', Color::Black));
        pieces[6][5] = Some(Piece::new('p', Color::Black));
        pieces[6][6] = Some(Piece::new('p', Color::Black));
        pieces[6][7] = Some(Piece::new('p', Color::Black));

        pieces[7][0] = Some(Piece::new('r', Color::Black));
        pieces[7][1] = Some(Piece::new('n', Color::Black));
        pieces[7][2] = Some(Piece::new('b', Color::Black));
        pieces[7][3] = Some(Piece::new('q', Color::Black));
        pieces[7][4] = Some(Piece::new('k', Color::Black));
        pieces[7][5] = Some(Piece::new('b', Color::Black));
        pieces[7][6] = Some(Piece::new('n', Color::Black));
        pieces[7][7] = Some(Piece::new('r', Color::Black));

        Board {
            highlight_move: (Square { x: 16, y: 16 }, Square { x: 16, y: 16 }),
            highlight_piece: None,
            curr_points: 0,
            pieces: pieces,
            king_white: Square::new("e1").unwrap(),
            king_black: Square::new("e8").unwrap(),
            piece_count: 32
            // past_states: Vec::new()
        }
    }

    pub fn from(pieces: [[Option<Piece>; 8]; 8], king_white: Square, king_black: Square) -> Board {
        Board {
            highlight_move: (Square { x: 16, y: 16 }, Square { x: 16, y: 16 }),
            highlight_piece: None,
            curr_points: 0,
            pieces: pieces,
            king_white: king_white,
            king_black: king_black,
            piece_count: pieces.into_iter().flatten().fold(0, |a, b| a + match b {
                Some(_) => 1,
                None => 0
            })
            // past_states: Vec::new()
        }
    }

    pub fn fen(&self, who_to_move: Color) -> String {
        let mut empty: u8 = 0;
        let mut fen: String = String::new();
        for y in (0..8).rev() {
            for x in 0..8 {
                if let Some(piece) = self.piece_at_xy(x, y) {
                    if empty != 0 {
                        fen += &empty.to_string();
                        empty = 0;
                    }
                    fen.push(match piece.color {
                        Color::White => piece.id.to_ascii_uppercase(),
                        Color::Black => piece.id
                    });
                } else {
                    empty += 1;
                }
            }
            if empty != 0 {
                fen += &empty.to_string();
                empty = 0;
            }
            if y != 0 {
                fen.push('/');
            }
        }
        fen += "_" + match who_to_move {
            Color::White => "w",
            Color::Black => "b"
        } + "_-_-_0_1";
        fen
    }

    pub fn occupied(&self, square: &Square) -> bool {
        self.pieces[square.y as usize][square.x as usize].is_some()
    }

    pub fn is_color(&self, square: &Square, color: Color) -> bool {
        match self.piece_at(square) {
            Some(piece) => piece.color == color,
            None => false,
        }
    }

    pub fn piece_at(&self, square: &Square) -> Option<Piece> {
        self.pieces[square.y as usize][square.x as usize]
    }

    pub fn piece_at_xy(&self, x: u8, y: u8) -> Option<Piece> {
        self.pieces[y as usize][x as usize]
    }

    pub fn king_in_check(&self, color: Color) -> bool {
        let sq = match color {
            Color::White => self.king_white,
            Color::Black => self.king_black,
        };
        // self.piece_at(&sq).unwrap().in_check(&self, sq)
        match self.piece_at(&sq) {
            Some(piece) => piece.in_check(&self, sq),
            None => {
                // for state in &self.past_states {
                //     println!("{}", state);
                // }
                println!("{} {}", self, sq);
                panic!("No king!");
            }
        }
    }

    pub fn exec_move(&mut self, from: &Square, to: &Square) -> (i32, Piece, Option<Piece>, bool) {
        let x = to.x as usize;
        let y = to.y as usize;
        // self.past_states.push(self.clone());
        let (taken, mut points) = match self.pieces[y][x] {
            Some(taken) => {
                self.piece_count -= 1;
                (Some(taken), taken.points)
            },
            None => (None, 0),
        };
        self.pieces[y][x] = self.pieces[from.y as usize][from.x as usize];
        let mut promoted = false;
        if self.pieces[y][x].unwrap().id == 'p' {
            if y == match self.pieces[y][x].unwrap().color {
                Color::White => 7,
                Color::Black => 0,
            } {
                let piece = Piece::new('q', self.pieces[y][x].unwrap().color);
                self.pieces[y][x] = Some(piece);
                points += piece.points;
                promoted = true;
            }
        }
        self.pieces[from.y as usize][from.x as usize] = None;
        let mut moved = self.pieces[y][x].as_mut().unwrap();
        moved.has_moved = true;
        if moved.id == 'k' {
            match moved.color {
                Color::White => self.king_white = *to,
                Color::Black => self.king_black = *to,
            }
        }
        points *= if moved.color == Color::White { 1 } else { -1 };
        self.curr_points += points;
        (points, *moved, taken, promoted)
    }

    pub fn unexec_move(
        &mut self,
        from: &Square,
        to: &Square,
        (points, moved, taken, promoted): (i32, Piece, Option<Piece>, bool),
    ) {
        if taken.is_some() {
            self.piece_count += 1;
        }
        if promoted {
            self.pieces[from.y as usize][from.x as usize] = Some(Piece::new('p', moved.color))
        } else {
            self.pieces[from.y as usize][from.x as usize] = Some(moved);
        }
        self.pieces[to.y as usize][to.x as usize] = taken;
        if moved.id == 'k' {
            match moved.color {
                Color::White => self.king_white = *from,
                Color::Black => self.king_black = *from,
            }
        }
        self.curr_points -= points;
    }

    pub fn get_pieces(&self, color: Color) -> Vec<(Square, Piece)> {
        let mut result = Vec::with_capacity(16);
        for y in 0..8 {
            for x in 0..8 {
                if let Some(piece) = self.pieces[y][x] {
                    if piece.color == color {
                        result.push((
                            Square {
                                x: x as u8,
                                y: y as u8,
                            },
                            piece,
                        ));
                    }
                }
            }
        }
        result
    }

    pub fn get_moves(&self, color: Color) -> Vec<(Square, Square)> {
        let mut moves = Vec::new();
        for (pos, piece) in self.get_pieces(color) {
            piece.get_moves(self, pos, &mut moves);
        }
        let mut board_clone = self.clone();
        moves.retain(move |(from, to)| {
            let data = board_clone.exec_move(&from, &to);
            let result = !board_clone.king_in_check(color);
            board_clone.unexec_move(&from, &to, data);
            result
        });
        moves
    }

    pub fn is_in_checkmate(&self, color: Color) -> bool {
        if !self.king_in_check(color) {
            return false;
        }
        self.get_moves(color).len() == 0
    }

    pub fn eval(&self, color: Color) -> f32 {
        if self.is_in_checkmate(!color) {
            CHECKMATE
        } else if self.is_in_checkmate(color) {
            -CHECKMATE
        } else {
            let points: f32 = match color {
                Color::White => self.curr_points,
                Color::Black => -self.curr_points
            } as f32;
            points
        }
    }

    pub fn display(&self, perspective: Color) {
        let mut moves = Vec::new();
        if let Some(piece_square) = self.highlight_piece {
            if let Some(piece) = self.piece_at(&piece_square) {
                piece.get_moves(&self, piece_square, &mut moves);
            }
        }
        let mut forward_y = 0usize..8;
        let mut backward_y = (0usize..8).rev();
        for y in match perspective {
            Color::White => &mut backward_y as &mut dyn Iterator<Item = usize>,
            Color::Black => &mut forward_y as &mut dyn Iterator<Item = usize>
        } {
            print!("+---+---+---+---+---+---+---+---+\n");
            let mut forward_x = 0usize..8;
            let mut backward_x = (0usize..8).rev();
            for x in match perspective {
                Color::White => &mut forward_x as &mut dyn Iterator<Item = usize>,
                Color::Black => &mut backward_x as &mut dyn Iterator<Item = usize>
            } {
                print!("|");
                let highlight_color = if (self.highlight_move.0.x == x as u8
                    && self.highlight_move.0.y == y as u8)
                    || (self.highlight_move.1.x == x as u8 && self.highlight_move.1.y == y as u8)
                {
                    "\x1b[1;103m"
                } else if moves.iter().any(|v| {
                    v.1 == Square {
                        x: x as u8,
                        y: y as u8,
                    }
                }) {
                    "\x1b[1;42m"
                } else {
                    ""
                };
                if let Some(piece) = self.pieces[y][x] {
                    print!(
                        "{} {} \x1b[0m",
                        highlight_color,
                        if piece.color == Color::White {
                            piece.id.to_ascii_uppercase()
                        } else {
                            piece.id
                        }
                    );
                } else {
                    print!("{}   \x1b[0m", highlight_color);
                }
            }
            print!("| {}\n", y + 1);
        }
        print!(
            "+---+---+---+---+---+---+---+---+\n  {}\n",
            match perspective {
                Color::White => "a   b   c   d   e   f   g   h",
                Color::Black => "h   g   f   e   d   c   b   a"
            }
        );
        println!();
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut moves = Vec::new();
        if let Some(piece_square) = self.highlight_piece {
            if let Some(piece) = self.piece_at(&piece_square) {
                piece.get_moves(&self, piece_square, &mut moves);
            }
        }
        for y in (0..8).rev() {
            write!(f, "+---+---+---+---+---+---+---+---+\n")?;
            for x in 0..8 {
                write!(f, "|")?;
                let highlight_color = if (self.highlight_move.0.x == x as u8
                    && self.highlight_move.0.y == y as u8)
                    || (self.highlight_move.1.x == x as u8 && self.highlight_move.1.y == y as u8)
                {
                    "\x1b[1;103m"
                } else if moves.iter().any(|v| {
                    v.1 == Square {
                        x: x as u8,
                        y: y as u8,
                    }
                }) {
                    "\x1b[1;42m"
                } else {
                    ""
                };
                if let Some(piece) = self.pieces[y][x] {
                    write!(
                        f,
                        "{} {} \x1b[0m",
                        highlight_color,
                        if piece.color == Color::White {
                            piece.id.to_ascii_uppercase()
                        } else {
                            piece.id
                        }
                    )?;
                } else {
                    write!(f, "{}   \x1b[0m", highlight_color)?;
                }
            }
            write!(f, "| {}\n", y + 1)?;
        }
        write!(
            f,
            "+---+---+---+---+---+---+---+---+\n  a   b   c   d   e   f   g   h\n"
        )?;
        Ok(())
    }
}
