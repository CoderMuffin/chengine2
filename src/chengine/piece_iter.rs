use crate::chengine::*;

pub struct PieceIter<'a> {
    board: &'a Board,
    x: u8,
    y: u8,
    color: Color,
}

impl PieceIter<'_> {
    pub fn new(board: &Board, color: Color) -> PieceIter {
        PieceIter {
            board: board,
            x: 0,
            y: 0,
            color: color,
        }
    }
}

impl Iterator for PieceIter<'_> {
    type Item = (Square, Piece);

    fn next(&mut self) -> Option<Self::Item> {
        while self.y < 8 {
            while self.x < 8 {
                if let Some(piece) = self.board.piece_at_xy(self.y, self.x) {
                    if piece.color == self.color {
                        self.x += 1;
                        return Some((
                            Square {
                                x: self.x,
                                y: self.y,
                            },
                            piece,
                        ));
                    }
                }
                self.x += 1;
            }
            self.x = 0;
            self.y += 1;
        }
        None
    }
}
