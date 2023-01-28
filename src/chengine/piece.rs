use crate::chengine::*;

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    pub id: char,
    pub points: i32,
    pub has_moved: bool,
    pub color: Color,
}

impl Piece {
    pub fn new(id: char, color: Color) -> Piece {
        Piece {
            id: id,
            color: color,
            has_moved: false,
            points: match id {
                'k' => 99999,
                'q' => 850,
                'r' => 500,
                'b' => 350,
                'n' => 300,
                'p' => 100,
                _ => panic!("Invalid piece '{}'", id),
            },
        }
    }

    pub fn get_moves(&self, board: &Board, from: Square, moves: &mut Vec<(Square, Square)>) {
        let pawn_dir = if self.color == Color::White { 1 } else { -1 };
        if self.id == 'p' {
            if let Some(dest) = from + (0, pawn_dir) {
                if !board.occupied(&dest) {
                    if let Some(dest_double) = dest + (0, pawn_dir) {
                        if !self.has_moved && !board.occupied(&dest_double) {
                            moves.push((from, dest_double));
                        }
                    }
                    moves.push((from, dest));
                }
            }
            if let Some(dest) = from + (1, pawn_dir) {
                if board.is_color(&dest, !self.color) {
                    moves.push((from, dest));
                }
            }
            if let Some(dest) = from + (-1, pawn_dir) {
                if board.is_color(&dest, !self.color) {
                    moves.push((from, dest));
                }
            }
        } else if self.id == 'n' {
            for &square in &[
                (1, 2),
                (-1, 2),
                (1, -2),
                (-1, -2),
                (2, 1),
                (-2, 1),
                (2, -1),
                (-2, -1),
            ] {
                if let Some(dest) = from + square {
                    if !board.is_color(&dest, self.color) {
                        moves.push((from, dest));
                    }
                }
            }
        } else if self.id == 'b' {
            for signs in &[(1i8, 1i8), (1, -1), (-1, -1), (-1, 1)] {
                for xy in 1i8.. {
                    if let Some(dest) = from + (xy * signs.0, xy * signs.1) {
                        if !board.occupied(&dest) {
                            moves.push((from, dest));
                        } else {
                            if board.is_color(&dest, !self.color) {
                                moves.push((from, dest))
                            }
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        } else if self.id == 'r' {
            for signs in &[(1i8, 0i8), (-1, 0), (0, -1), (0, 1)] {
                for xy in 1i8.. {
                    if let Some(dest) = from + (xy * signs.0, xy * signs.1) {
                        if !board.occupied(&dest) {
                            moves.push((from, dest));
                        } else {
                            if board.is_color(&dest, !self.color) {
                                moves.push((from, dest))
                            }
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        } else if self.id == 'q' {
            for signs in &[
                (1i8, 0i8),
                (-1, 0),
                (0, -1),
                (0, 1),
                (1, 1),
                (1, -1),
                (-1, -1),
                (-1, 1),
            ] {
                for xy in 1i8.. {
                    if let Some(dest) = from + (xy * signs.0, xy * signs.1) {
                        if !board.occupied(&dest) {
                            moves.push((from, dest));
                        } else {
                            if board.is_color(&dest, !self.color) {
                                moves.push((from, dest))
                            }
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        } else if self.id == 'k' {
            for &dir in &[
                (1i8, 0i8),
                (-1, 0),
                (0, -1),
                (0, 1),
                (1, 1),
                (1, -1),
                (-1, -1),
                (-1, 1),
            ] {
                if let Some(dest) = from + dir {
                    if !board.is_color(&dest, self.color) {
                        moves.push((from, dest));
                    }
                }
            }

            // if !self.has_moved {
            //     let rank = match self.color {
            //         Color::White => 1,
            //         Color::Black => 8
            //     };
            //     let rook_a = board.piece_at(&Square { x: 0, y: rank });
            //     let rook_h = board.piece_at(&Square { x: 7, y: rank });
            //     if rook_h. { //if it hasn't moved it must be our own, so we don't need to check that
            //         for file in [5, 6] {

            //         }
            //     }
            // }
        }
    }

    pub fn in_check(&self, board: &Board, square: Square) -> bool {
        //assumes it is a king for efficiency
        for signs in &[
            (1i8, 0i8, 'r'),
            (-1, 0, 'r'),
            (0, -1, 'r'),
            (0, 1, 'r'),
            (1, 1, 'b'),
            (1, -1, 'b'),
            (-1, -1, 'b'),
            (-1, 1, 'b'),
        ] {
            for xy in 1i8.. {
                if let Some(dest) = square + (xy * signs.0, xy * signs.1) {
                    if let Some(piece) = board.piece_at(&dest) {
                        if (piece.id == 'q' || piece.id == signs.2) && piece.color == !self.color {
                            return true;
                        } else if piece.id == 'k' && xy == 1 && piece.color == !self.color {
                            return true;
                        } else {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        }
        for &jump in &[
            (1, 2),
            (-1, 2),
            (1, -2),
            (-1, -2),
            (2, 1),
            (-2, 1),
            (2, -1),
            (-2, -1),
        ] {
            if let Some(dest) = square + jump {
                if let Some(piece) = board.piece_at(&dest) {
                    if piece.id == 'n' && piece.color != self.color {
                        return true;
                    }
                }
            }
        }
        if self.color == Color::Black {
            if let Some(dest) = square + (-1, -1) {
                if let Some(piece) = board.piece_at(&dest) {
                    if piece.id == 'p' && piece.color == Color::White {
                        return true;
                    }
                }
            }
            if let Some(dest) = square + (1, -1) {
                if let Some(piece) = board.piece_at(&dest) {
                    if piece.id == 'p' && piece.color == Color::White {
                        return true;
                    }
                }
            }
        } else if self.color == Color::White {
            if let Some(dest) = square + (-1, 1) {
                if let Some(piece) = board.piece_at(&dest) {
                    if piece.id == 'p' && piece.color == Color::Black {
                        return true;
                    }
                }
            }
            if let Some(dest) = square + (1, 1) {
                if let Some(piece) = board.piece_at(&dest) {
                    if piece.id == 'p' && piece.color == Color::Black {
                        return true;
                    }
                }
            }
        }
        return false;
    }
}
