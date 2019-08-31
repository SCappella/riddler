mod disjoint_set;
mod graph;
mod hex_graph;
mod hex_solver;

use hex_graph::HexGraph;
use hex_solver::HexBoard;
use std::rc::Rc;

fn first_board() -> HexBoard {
    let graph = Rc::new(HexGraph::new(3, 3));
    let mut board = HexBoard::new(graph);
    board.add_black(0, 1);
    board.add_black(1, 0);
    board.add_white(0, 2);

    board
}

fn second_board() -> HexBoard {
    let graph = Rc::new(HexGraph::new(5, 5));
    let mut board = HexBoard::new(graph);
    board.add_white(2, 0);
    board.add_white(2, 2);
    board.add_white(4, 4);
    board.add_black(1, 2);
    board.add_black(3, 3);
    board.add_black(4, 2);

    board
}

fn third_board() -> HexBoard {
    let graph = Rc::new(HexGraph::new(5, 5));
    let mut board = HexBoard::new(graph);
    board.add_white(0, 0);
    board.add_white(0, 2);
    board.add_white(2, 2);
    board.add_black(1, 1);
    board.add_black(1, 2);
    board.add_black(0, 3);

    board
}

fn main() {
    let first_board = first_board();
    let second_board = second_board();
    let third_board = third_board();

    println!(
        "First board moves: {:?}",
        first_board.find_winning_moves_white()
    );

    println!(
        "Second board moves: {:?}",
        second_board.find_winning_moves_white()
    );

    println!(
        "Third board moves: {:?}",
        third_board.find_winning_moves_white()
    );
}
