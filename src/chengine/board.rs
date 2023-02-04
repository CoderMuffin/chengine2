use crate::chengine::*;
use std::fmt;

#[derive(Clone)]
pub struct CastleInfo {
    pub kingside: bool,
    pub queenside: bool,
}

impl CastleInfo {
    pub fn both() -> Self {
        Self {
            kingside: true,
            queenside: true,
        }
    }

    pub fn neither() -> Self {
        Self {
            kingside: false,
            queenside: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CastleMoveData {
    Queenside,
    Kingside,
    None,
}

#[derive(Clone)]
pub struct Board {
    pieces: [[Option<Piece>; 8]; 8],
    pub highlight_move: (Square, Square),
    pub highlight_piece: Option<Square>,
    curr_points: i32,
    king_white: Square,
    king_black: Square,
    pub piece_count: u8,
    castle_white: CastleInfo,
    castle_black: CastleInfo, // past_states: Vec<Board> //FOR DEBUG ONLY REMOVE ASAP
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
            piece_count: 32,
            castle_white: CastleInfo::both(),
            castle_black: CastleInfo::both(), // past_states: Vec::new()
        }
    }

    #[allow(dead_code)]
    pub fn from(pieces: [[Option<Piece>; 8]; 8], king_white: Square, king_black: Square) -> Board {
        Board {
            highlight_move: (Square { x: 16, y: 16 }, Square { x: 16, y: 16 }),
            highlight_piece: None,
            curr_points: 0,
            pieces: pieces,
            king_white: king_white,
            king_black: king_black,
            castle_white: CastleInfo::neither(),
            castle_black: CastleInfo::neither(),
            piece_count: pieces.into_iter().flatten().fold(0, |a, b| {
                a + match b {
                    Some(_) => 1,
                    None => 0,
                }
            }), // past_states: Vec::new()
        }
    }

    #[allow(dead_code)]
    pub fn from_fen(fen: &str) -> Self {
        let mut pieces = [[None; 8]; 8];
        let mut piece_count = 0;

        let mut king_white = None;
        let mut king_black = None;
        for (index_y_inv, line) in fen.split("/").enumerate() {
            let mut index_x = 0;
            let index_y = 7 - index_y_inv;
            for char in line.chars() {
                if let Some(skip) = char.to_digit(10) {
                    index_x += skip as usize;
                } else {
                    pieces[index_y][index_x] = Some(Piece::new(
                        char.to_ascii_lowercase(),
                        match char.is_uppercase() {
                            true => Color::White,
                            false => Color::Black,
                        },
                    ));
                    piece_count += 1;
                    if char.to_ascii_lowercase() == 'k' {
                        match char.is_uppercase() {
                            true => {
                                king_white = Some(Square {
                                    x: index_x as u8,
                                    y: index_y as u8,
                                })
                            }
                            false => {
                                king_black = Some(Square {
                                    x: index_x as u8,
                                    y: index_y as u8,
                                })
                            }
                        }
                    }
                    index_x += 1;
                }
            }
        }
        Self {
            highlight_move: (Square { x: 16, y: 16 }, Square { x: 16, y: 16 }),
            highlight_piece: None,
            curr_points: 0,
            pieces: pieces,
            king_white: king_white.expect("No white king on board!"),
            king_black: king_black.expect("No black king on board!"),
            castle_white: CastleInfo::both(),
            castle_black: CastleInfo::both(),
            piece_count: piece_count,
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
                        Color::Black => piece.id,
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
        fen += "_";
        fen += match who_to_move {
            Color::White => "w",
            Color::Black => "b",
        };
        fen += "_-_-_0_1";
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

    pub fn square_value(&self, square: &Square) -> i32 {
        match self.pieces[square.y as usize][square.x as usize] {
            Some(piece) => piece.points,
            None => 0
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
            Some(_) => Piece::in_check(&self, sq, color),
            None => {
                // for state in &self.past_states {
                //     println!("{}", state);
                // }
                println!("{} {}", self, sq);
                panic!("No king!");
            }
        }
    }

    pub fn exec_move(
        &mut self,
        from: &Square,
        to: &Square,
    ) -> (i32, Piece, Option<Piece>, bool, CastleMoveData) {
        let tx = to.x as usize;
        let ty = to.y as usize;
        let fx = from.x as usize;
        let fy = from.y as usize;
        let mut castle_data = CastleMoveData::None;
        // self.past_states.push(self.clone());

        //get value
        let (taken, mut points) = match self.pieces[ty][tx] {
            Some(taken) => {
                self.piece_count -= 1;
                (Some(taken), taken.points)
            }
            None => (None, 0),
        };

        // println!("{} {:?} {:?} {:?} {:?}", self, fy, fx, ty, tx);
        //move piece
        let mut moved = self.pieces[fy][fx].expect("no piece to move!");
        moved.has_moved = true;
        self.pieces[ty][tx] = Some(moved);
        self.pieces[fy][fx] = None;

        //test for promotion
        let mut promoted = false;
        if moved.id == 'p' {
            if ty
                == match moved.color {
                    Color::White => 7,
                    Color::Black => 0,
                }
            {
                let piece = Piece::new('q', moved.color);
                self.pieces[ty][tx] = Some(piece);
                points += piece.points;
                promoted = true;
            }
        }

        //test for castling or king move or pawn move
        if moved.id == 'k' {
            match moved.color {
                Color::White => {
                    self.king_white = *to;
                    self.castle_white = CastleInfo::neither();
                }
                Color::Black => {
                    self.king_black = *to;
                    self.castle_black = CastleInfo::neither();
                }
            }
            match tx as i8 - fx as i8 {
                2 => {
                    //kingside
                    self.pieces[ty][5] = self.pieces[ty][7];
                    self.pieces[ty][7] = None;
                    castle_data = CastleMoveData::Kingside;
                    points += 3;
                }
                -2 => {
                    //queenside
                    self.pieces[ty][3] = self.pieces[ty][0];
                    self.pieces[ty][0] = None;
                    castle_data = CastleMoveData::Queenside;
                    points += 2;
                }
                _ => {}
            }
        } else if moved.id == 'p' {
            let incr = (fy as i32 - ty as i32).abs();
            //moved.points += incr;
            //points += incr;
        } else if moved.id == 'r' {
            if tx == 0 || tx == 7 {
                match moved.color {
                    Color::White => {
                        if tx == 0 {
                            self.castle_white.queenside = false;
                        } else {
                            self.castle_white.kingside = false;
                        }
                    }
                    Color::Black => {
                        if tx == 0 {
                            self.castle_white.queenside = false;
                        } else {
                            self.castle_white.kingside = false;
                        }
                    }
                }
            }
        }
        points *= if moved.color == Color::White { 1 } else { -1 };
        self.curr_points += points;
        (points, moved, taken, promoted, castle_data)
    }

    pub fn unexec_move(
        &mut self,
        from: &Square,
        to: &Square,
        (points, moved, taken, promoted, castle_data): (
            i32,
            Piece,
            Option<Piece>,
            bool,
            CastleMoveData,
        ),
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
        match castle_data {
            CastleMoveData::Kingside => {
                self.pieces[from.y as usize][7] = self.pieces[to.y as usize][5];
                self.pieces[from.y as usize][5] = None;
                self.curr_points -= 3;
            }
            CastleMoveData::Queenside => {
                self.pieces[from.y as usize][0] = self.pieces[to.y as usize][3];
                self.pieces[from.y as usize][3] = None;
                self.curr_points -= 2;
            }
            CastleMoveData::None => {}
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

    pub fn filter_checks(&self, moves: &mut Vec<(Square, Square)>, color: Color) {
        let mut board_clone = self.clone();
        moves.retain(move |(from, to)| {
            let data = board_clone.exec_move(&from, &to);
            let result = !board_clone.king_in_check(color);
            board_clone.unexec_move(&from, &to, data);
            result
        });
    }

    pub fn get_moves(&self, color: Color) -> Vec<(Square, Square)> {
        let mut moves = Vec::new();
        for (pos, piece) in self.get_pieces(color) {
            piece.get_moves(self, pos, &mut moves);
        }
        self.filter_checks(&mut moves, color);
        moves
    }

    pub fn can_castle(&self, color: Color) -> &CastleInfo {
        match color {
            Color::White => &self.castle_white,
            Color::Black => &self.castle_black,
        }
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
                Color::Black => -self.curr_points,
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
            Color::Black => &mut forward_y as &mut dyn Iterator<Item = usize>,
        } {
            print!("+---+---+---+---+---+---+---+---+\n");
            let mut forward_x = 0usize..8;
            let mut backward_x = (0usize..8).rev();
            for x in match perspective {
                Color::White => &mut forward_x as &mut dyn Iterator<Item = usize>,
                Color::Black => &mut backward_x as &mut dyn Iterator<Item = usize>,
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
                Color::Black => "h   g   f   e   d   c   b   a",
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
