#[derive(Default, Clone, Debug)]
pub struct Graph {
    nodes: Vec<Node>,
}

#[derive(Default, Clone, Debug)]
struct Node {
    adjacent: Vec<usize>,
}

impl Graph {
    pub fn new(num_nodes: usize) -> Self {
        Self {
            nodes: vec![Node { adjacent: vec![] }; num_nodes],
        }
    }

    pub fn add_edge(&mut self, start: usize, end: usize) {
        assert!(
            start < self.nodes.len(),
            "Node {} does not exist in this graph",
            start
        );
        assert!(
            end < self.nodes.len(),
            "Node {} does not exist in this graph",
            end
        );
        self.nodes[start].adjacent.push(end);
        self.nodes[end].adjacent.push(start);
    }

    pub fn adjacent(&self, node: usize) -> Vec<usize> {
        if let Some(node) = self.nodes.get(node) {
            node.adjacent.clone()
        } else {
            Vec::new()
        }
    }
}
