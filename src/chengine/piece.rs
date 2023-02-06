use crate::chengine::*;

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    pub id: char,
    pub points: i32,
    pub has_moved: bool,
    pub color: Color,
}

impl Piece {
    pub const VALUE_KING: i32 = 99999;
    pub const VALUE_QUEEN: i32 = 850;
    pub const VALUE_ROOK: i32 = 500;
    pub const VALUE_BISHOP: i32 = 350;
    pub const VALUE_KNIGHT: i32 = 300;
    pub const VALUE_PAWN: i32 = 100;
    
    pub fn new(id: char, color: Color) -> Piece {
        Piece {
            id: id,
            color: color,
            has_moved: false,
            points: match id {
                'k' => Self::VALUE_KING,
                'q' => Self::VALUE_QUEEN,
                'r' => Self::VALUE_ROOK,
                'b' => Self::VALUE_BISHOP,
                'n' => Self::VALUE_KNIGHT,
                'p' => Self::VALUE_PAWN,
                _ => panic!("Invalid piece '{}'", id)
            }
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
            for square in [
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
            for signs in [(1i8, 1i8), (1, -1), (-1, -1), (-1, 1)] {
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
            for signs in [(1i8, 0i8), (-1, 0), (0, -1), (0, 1)] {
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
            for signs in [
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
            for dir in [
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

            if !Piece::in_check(board, from, self.color) {
                let castle_info = board.can_castle(self.color);
                if castle_info.kingside {
                    let mut can_castle = true;
                    let dest = (from + (-2, 0)).unwrap();
                    for sq in [(from + (-1, 0)).unwrap(), dest] {
                        if board.occupied(&sq) || Piece::in_check(board, sq, self.color) {
                            can_castle = false;
                            break;
                        }
                    }
                    if can_castle {
                        moves.push((from, dest));
                    }
                }
                if castle_info.queenside {
                    let mut can_castle = true;
                    let dest = (from + (2, 0)).unwrap();
                    for sq in [(from + (1, 0)).unwrap(), dest] {
                        if board.occupied(&sq) || Piece::in_check(board, sq, self.color) {
                            can_castle = false;
                            break;
                        }
                    }
                    if can_castle {
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

    pub fn in_check(board: &Board, square: Square, color: Color) -> bool {
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
                        if (piece.id == 'q' || piece.id == signs.2) && piece.color == !color {
                            return true;
                        } else if piece.id == 'k' && xy == 1 && piece.color == !color {
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
                    if piece.id == 'n' && piece.color != color {
                        return true;
                    }
                }
            }
        }
        if color == Color::Black {
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
        } else if color == Color::White {
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
