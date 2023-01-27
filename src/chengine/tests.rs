use crate::chengine::*;

#[test]
fn discovered_check() {
    let mut pieces = [[None; 8]; 8];
    pieces[0][3] = Some(Piece::new('k', Color::Black));
    pieces[4][3] = Some(Piece::new('q', Color::Black));
    pieces[6][3] = Some(Piece::new('b', Color::White));
    pieces[7][3] = Some(Piece::new('r', Color::White));
    pieces[7][4] = Some(Piece::new('k', Color::White));
    let board = Board::from(pieces, Square { x: 4, y: 7 }, Square { x: 3, y: 0 });
    assert_move_made(
        &board,
        Color::White,
        (Square { x: 3, y: 6 }, Square { x: 6, y: 3 }),
    );
}

#[test]
fn fork() {
    let mut pieces = [[None; 8]; 8];
    pieces[0][1] = Some(Piece::new('k', Color::White));
    pieces[4][1] = Some(Piece::new('r', Color::White));
    pieces[3][4] = Some(Piece::new('n', Color::Black));
    pieces[5][5] = Some(Piece::new('k', Color::Black));
    let board = Board::from(pieces, Square { x: 1, y: 0 }, Square { x: 5, y: 5 });
    assert_move_made(
        &board,
        Color::Black,
        (Square { x: 4, y: 3 }, Square { x: 2, y: 2 }),
    );
}

#[test]
fn piece_count_from() {
    let mut pieces = [[None; 8]; 8];
    pieces[0][1] = Some(Piece::new('k', Color::White));
    pieces[4][1] = Some(Piece::new('r', Color::White));
    pieces[3][4] = Some(Piece::new('n', Color::Black));
    pieces[5][5] = Some(Piece::new('k', Color::Black));
    let board = Board::from(pieces, Square { x: 1, y: 0 }, Square { x: 5, y: 5 });
    assert_eq!(board.piece_count, 4);
}

#[test]
fn rook_endgame() {
    let mut pieces = [[None; 8]; 8];
    pieces[2][1] = Some(Piece::new('k', Color::White));
    pieces[7][7] = Some(Piece::new('r', Color::White));
    pieces[3][4] = Some(Piece::new('p', Color::Black));
    pieces[0][1] = Some(Piece::new('k', Color::Black));
    let board = Board::from(pieces, Square { x: 1, y: 2 }, Square { x: 1, y: 0 });
    assert_move_made(
        &board,
        Color::White,
        (Square { x: 7, y: 7 }, Square { x: 7, y: 0 }),
    );
}

#[test]
fn board_promotion_test() {
    let mut pieces = [[None; 8]; 8];
    pieces[7][7] = Some(Piece::new('k', Color::Black));
    pieces[5][7] = Some(Piece::new('k', Color::White));
    pieces[1][2] = Some(Piece::new('p', Color::Black));
    pieces[1][1] = Some(Piece::new('p', Color::White));
    pieces[1][0] = Some(Piece::new('p', Color::White));
    let board = Board::from(pieces, Square { x: 7, y: 5 }, Square { x: 7, y: 7 });
    assert_move_made(
        &board,
        Color::Black,
        (Square { x: 2, y: 1 }, Square { x: 2, y: 0 }),
    );
}

fn assert_move_made(board: &Board, color: Color, expected: (Square, Square)) {
    let mut computer = Computer::new(color, &OPENING_BOOK);
    computer.following_opening = false;
    assert_eq!(computer.get_move(board, None, 6).1, expected);
}
