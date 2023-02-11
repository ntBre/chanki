use super::*;

#[test]
fn mov() {
    let pgn = Pgn::load("test.pgn").unwrap();
    let mut board = Board::new();
    board.play(&pgn);
    assert_eq!(
        board.to_string(),
        "8/8/2R2P1p/p3k3/6PP/r7/2pK4/8 w KQkq - 0 1"
    );
}
