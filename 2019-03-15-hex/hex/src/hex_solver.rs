use crate::disjoint_set::DisjointSet;
use crate::hex_graph::HexGraph;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct HexBoard {
    graph: Rc<HexGraph>,
    // take a graph index and return a set index
    black_nodes: BTreeMap<usize, usize>,
    white_nodes: BTreeMap<usize, usize>,
    black_set: DisjointSet,
    white_set: DisjointSet,
    open_moves: BTreeSet<(usize, usize)>,
}

impl HexBoard {
    pub fn new(graph: Rc<HexGraph>) -> Self {
        let mut black_set = DisjointSet::default();
        // add the black sides
        let first_black = black_set.add_elem();
        let second_black = black_set.add_elem();
        let mut white_set = DisjointSet::default();

        // add the white sides
        let first_white = white_set.add_elem();
        let second_white = white_set.add_elem();

        let mut black_nodes = BTreeMap::new();
        black_nodes.insert(0, first_black);
        black_nodes.insert(1, second_black);

        let mut white_nodes = BTreeMap::new();
        white_nodes.insert(2, first_white);
        white_nodes.insert(3, second_white);

        Self {
            graph: graph.clone(),
            black_nodes,
            white_nodes,
            black_set,
            white_set,
            open_moves: (0..graph.width)
                .flat_map(|i| (0..graph.height).map(move |j| (i, j)))
                .collect(),
        }
    }

    /// Check if black has won.
    #[inline]
    pub fn black_win(&mut self) -> bool {
        self.black_set.connected(0, 1)
    }

    /// Check if white has won.
    #[inline]
    pub fn white_win(&mut self) -> bool {
        self.white_set.connected(0, 1)
    }

    /// Make a move for black. Panics if the move is out of bounds or if a move has already been made there.
    #[inline]
    pub fn add_black(&mut self, column: usize, row: usize) {
        assert!(
            self.open_moves.contains(&(column, row)),
            "The move ({}, {}) has already been made.",
            column,
            row
        );
        self.open_moves.remove(&(column, row));

        let new_black = self.black_set.add_elem();
        let index = self.graph.index(column, row);
        self.black_nodes.insert(index, new_black);

        let mut black_set = std::mem::replace(&mut self.black_set, DisjointSet::default());
        self.graph
            .adjacent(column, row)
            .into_iter()
            .filter_map(|adj_index| self.black_nodes.get(&adj_index))
            .for_each(|&adj_index| black_set.union(adj_index, new_black));
        self.black_set = black_set;
    }

    /// Make a move for white. Panics if the move is out of bounds or if a move has already been made there.
    #[inline]
    pub fn add_white(&mut self, column: usize, row: usize) {
        assert!(
            self.open_moves.contains(&(column, row)),
            "The move ({}, {}) has already been made.",
            column,
            row
        );
        self.open_moves.remove(&(column, row));

        let new_white = self.white_set.add_elem();
        let index = self.graph.index(column, row);
        self.white_nodes.insert(index, new_white);

        let mut white_set = std::mem::replace(&mut self.white_set, DisjointSet::default());
        self.graph
            .adjacent(column, row)
            .into_iter()
            .filter_map(|adj_index| self.white_nodes.get(&adj_index))
            .for_each(|&adj_index| white_set.union(adj_index, new_white));
        self.white_set = white_set;
    }

    /// Check if this board is winning for white, with black to play
    fn is_winning_black(&mut self) -> bool {
        // If white won with the last move, we're done
        if self.white_win() {
            return true;
        }

        // This board is winning for white if the results of all moves are winning with white to play
        self.open_moves.iter().all(|&(column, row)| {
            let mut new_board = self.clone();
            new_board.add_black(column, row);
            new_board.is_winning_white()
        })
    }

    /// Check if this board is winning for white, with white to play
    fn is_winning_white(&mut self) -> bool {
        // If black won with the last move, we're done
        if self.black_win() {
            return false;
        }

        // This board is winning for white if the result of any move is winning with black to play
        self.open_moves.iter().any(|&(column, row)| {
            let mut new_board = self.clone();
            new_board.add_white(column, row);
            new_board.is_winning_black()
        })
    }

    /// Find all the winning moves for white, with white to play
    pub fn find_winning_moves_white(self) -> Vec<(usize, usize)> {
        let mut winning_moves = Vec::new();

        for &(column, row) in &self.open_moves {
            let mut new_board = self.clone();
            new_board.add_white(column, row);
            if new_board.is_winning_black() {
                winning_moves.push((column, row));
            }
        }

        winning_moves
    }
}
