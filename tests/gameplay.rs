extern crate fueros;
use fueros::board::{Board, Edge, EdgeValue};

#[test]
fn board_construction() {
    let board = Board::with_size(7, 9);
    matches!(
        board.get_edge(Edge::Horizontal { x: 6, y: 9 }),
        Some(EdgeValue::Set(_))
    );
    matches!(
        board.get_edge(Edge::Horizontal { x: 5, y: 9 }),
        Some(EdgeValue::Unset)
    );
    matches!(
        board.get_edge(Edge::Horizontal { x: 6, y: 8 }),
        Some(EdgeValue::Unset)
    );
}
