use crate::graph::Graph;

#[derive(Debug)]
pub struct HexGraph {
    graph: Graph,
    /// The width of the hex board
    pub width: usize,
    /// The height of the hex board
    pub height: usize,
}

impl HexGraph {
    /// create a new quadrilateral hex grid including each side as a node
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 0, "The width must be greater than zero");
        assert!(height > 0, "The height must be greater than zero");

        // the four edges of the grid plus the grid
        let mut graph = Graph::new(4 + width * height);

        // add connections from the edges
        for column in 0..height {
            // bottom edge
            graph.add_edge(0, 4 + column);
            // top edge
            graph.add_edge(1, 4 + (height - 1) * width + column);
        }
        for row in 0..width {
            // left edge
            graph.add_edge(2, 4 + row * width);
            // right edge
            graph.add_edge(3, 4 + row * width + (width - 1));
        }

        for row in 0..height {
            for column in 0..width {
                // below and to the left
                if let Some(left) = column.checked_sub(1) {
                    if let Some(below) = row.checked_sub(1) {
                        graph.add_edge(4 + row * width + column, 4 + below * width + left);
                    }
                }
                // below
                if let Some(below) = row.checked_sub(1) {
                    graph.add_edge(4 + row * width + column, 4 + below * width + column);
                }
                // left
                if let Some(left) = column.checked_sub(1) {
                    graph.add_edge(4 + row * width + column, 4 + row * width + left);
                }
                // the other three get connected later
            }
        }

        Self {
            graph,
            height,
            width,
        }
    }

    /// Get the index of the given row and column. Panics if the input is out of bounds.
    #[inline]
    pub fn index(&self, column: usize, row: usize) -> usize {
        assert!(column < self.height, "Column {} is out of bounds", column);
        assert!(row < self.height, "Row {} is out of bounds", row);
        4 + self.width * row + column
    }

    /// Return the indexes of the nodes adjacent to the given node. Panics if the given node is out of bounds.
    #[inline]
    pub fn adjacent(&self, column: usize, row: usize) -> Vec<usize> {
        assert!(column < self.height, "Column {} is out of bounds", column);
        assert!(row < self.height, "Row {} is out of bounds", row);
        self.graph.adjacent(self.index(column, row))
    }
}
