use std::fmt::Debug;

struct Graph<T> {
    data: Vec<(T, Vec<usize>)>,
}

impl<T> Graph<T> {
    #[inline]
    fn new(nodes: Vec<T>) -> Self {
        Self {
            data: nodes.into_iter().map(|t| (t, Vec::new())).collect(),
        }
    }

    #[inline]
    fn add_edge(&mut self, start: usize, end: usize) {
        self.data[start].1.push(end)
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    fn get_neighbors(&self, start: usize) -> &[usize] {
        &self.data[start].1
    }

    fn longest_simple_path(&self) -> Vec<usize> {
        let mut longest_path = Vec::new();
        let mut tasks: Vec<Vec<usize>> = vec![(0..self.len()).rev().collect()];

        let mut path = vec![];
        let mut in_path = vec![false; self.len()];

        loop {
            while let Some(next_node) = tasks.last_mut().unwrap().pop() {
                path.push(next_node);
                in_path[next_node] = true;
                tasks.push(
                    self.get_neighbors(next_node)
                        .iter()
                        .rev()
                        .filter(|&&node| !in_path[node])
                        .copied()
                        .collect(),
                );
            }

            if path.len() > longest_path.len() {
                longest_path = path.clone();
            }

            if let Some(popped_node) = path.pop() {
                in_path[popped_node] = false;
                tasks.pop();
            } else {
                break;
            }
        }

        longest_path
    }
}

impl<T: Debug> Graph<T> {
    fn longest_simple_path_debug(&self) {
        let mut longest_path = Vec::new();
        let mut count = 1;

        let mut tasks: Vec<Vec<usize>> = vec![(0..self.len()).rev().collect()];

        let mut path = vec![];
        let mut in_path = vec![false; self.len()];

        loop {
            while let Some(next_node) = tasks.last_mut().unwrap().pop() {
                path.push(next_node);
                in_path[next_node] = true;
                tasks.push(
                    self.get_neighbors(next_node)
                        .iter()
                        .rev()
                        .filter(|&&node| !in_path[node])
                        .copied()
                        .collect(),
                );
            }

            if path.len() >= longest_path.len() {
                if path.len() == longest_path.len() {
                    count += 1;
                    println!("{}", count);
                } else {
                    count = 1;
                    println!("{}", count);
                    longest_path = path.clone();
                }
            }

            if let Some(popped_node) = path.pop() {
                in_path[popped_node] = false;
                tasks.pop();
            } else {
                break;
            }
        }
        println!(
            "{:?}",
            longest_path
                .into_iter()
                .map(|node| &self.data[node].0)
                .collect::<Vec<_>>()
        );
    }
}

fn main() {
    let digraphs_all = [
        "AK", "AL", "AR", "AS", "AZ", "CA", "CO", "CT", "DC", "DE", "FL", "FM", "GA", "GU", "HI",
        "IA", "ID", "IL", "IN", "KS", "KY", "LA", "MA", "MD", "ME", "MH", "MI", "MN", "MO", "MP",
        "MS", "MT", "NC", "ND", "NE", "NH", "NJ", "NM", "NV", "NY", "OH", "OK", "OR", "PA", "PR",
        "PW", "RI", "SC", "SD", "TN", "TX", "UT", "VA", "VI", "VT", "WA", "WI", "WV", "WY",
    ];

    let digraphs_states = [
        "AK", "AL", "AR", "AZ", "CA", "CO", "CT", "DE", "FL", "GA", "HI", "IA", "ID", "IL", "IN",
        "KS", "KY", "LA", "MA", "MD", "ME", "MI", "MN", "MO", "MS", "MT", "NC", "ND", "NE", "NH",
        "NJ", "NM", "NV", "NY", "OH", "OK", "OR", "PA", "RI", "SC", "SD", "TN", "TX", "UT", "VA",
        "VT", "WA", "WI", "WV", "WY",
    ];


    // switch this
    let digraphs = digraphs_all;
    //let digraphs = digraphs_states;

    let mut graph = Graph::new(digraphs.iter().collect());
    for (i, digraph_1) in digraphs.iter().enumerate() {
        for (j, digraph_2) in digraphs.iter().enumerate() {
            if digraph_1.as_bytes()[1] == digraph_2.as_bytes()[0] {
                graph.add_edge(i, j);
            }
        }
    }

    graph.longest_simple_path_debug();
}
