enum Square {
    Up,
    Down,
    Left,
    Right,
    Vertical,
    Horizontal,
    Number(u64),
    Death,
    Goal,
}

impl From<char> for Square {
    fn from(c: char) -> Self {
        match c {
            'u' => Square::Up,
            'd' => Square::Down,
            'l' => Square::Left,
            'r' => Square::Right,
            'v' => Square::Vertical,
            'h' => Square::Horizontal,
            'x' => Square::Death,
            'g' => Square::Goal,
            x => x
                .to_digit(10)
                .map_or(Square::Death, |x| Square::Number(x.into())),
        }
    }
}

impl Square {
    fn value(&self) -> u64 {
        match *self {
            Square::Number(x) => x,
            _ => 0,
        }
    }
}

struct Grid<T> {
    height: usize,
    width: usize,
    grid: Box<[T]>,
}

impl<T> Grid<T> {
    fn get_index(&self, row: usize, column: usize) -> Option<usize> {
        if column > self.width {
            return None;
        }

        let index = row.checked_mul(self.width)?.checked_add(column)?;

        if index < self.grid.len() {
            Some(index)
        } else {
            None
        }
    }
}

impl Grid<Square> {
    fn new(rows: &[String]) -> Self {
        let height = rows.len();
        let width = rows.get(0).map_or(0, |s| s.len());
        assert!(rows.iter().all(|s| s.len() == width));

        let mut grid = Vec::with_capacity(height * width);

        for row in rows {
            for c in row.chars() {
                grid.push(c.into());
            }
        }

        Self {
            height,
            width,
            grid: grid.into_boxed_slice(),
        }
    }
}

type BoxedSlice<T> = Box<[T]>;

struct Graph {
    nodes: BoxedSlice<BoxedSlice<(u64, usize)>>,
    goals: Vec<usize>,
}

impl From<Grid<Square>> for Graph {
    fn from(grid: Grid<Square>) -> Self {
        // include the outside of the grid
        let size = grid.width * grid.height + 1;

        let mut nodes = vec![vec![]; size];
        let mut goals = Vec::new();

        for (index, square) in grid.grid.iter().enumerate() {
            let row = index / grid.width;
            let column = index % grid.width;

            // we can go up
            match *square {
                Square::Up | Square::Vertical | Square::Number(_) => {
                    if let Some(up_row) = row.checked_sub(1) {
                        if let Some(up_index) = grid.get_index(up_row, column) {
                            nodes[index].push((grid.grid[up_index].value(), up_index));
                        } else {
                            nodes[size - 1].push((square.value(), index));
                        }
                    } else {
                        nodes[size - 1].push((square.value(), index));
                    }
                }
                _ => {}
            }

            // we can go down
            match *square {
                Square::Down | Square::Vertical | Square::Number(_) => {
                    if let Some(down_row) = row.checked_add(1) {
                        if let Some(down_index) = grid.get_index(down_row, column) {
                            nodes[index].push((grid.grid[down_index].value(), down_index));
                        } else {
                            nodes[size - 1].push((square.value(), index));
                        }
                    } else {
                        nodes[size - 1].push((square.value(), index));
                    }
                }
                _ => {}
            }

            // we can go left
            match *square {
                Square::Left | Square::Horizontal | Square::Number(_) => {
                    if let Some(left_column) = column.checked_sub(1) {
                        if let Some(left_index) = grid.get_index(row, left_column) {
                            nodes[index].push((grid.grid[left_index].value(), left_index));
                        } else {
                            nodes[size - 1].push((square.value(), index));
                        }
                    } else {
                        nodes[size - 1].push((square.value(), index));
                    }
                }
                _ => {}
            }

            // we can go right
            match *square {
                Square::Right | Square::Horizontal | Square::Number(_) => {
                    if let Some(right_column) = column.checked_add(1) {
                        if let Some(right_index) = grid.get_index(row, right_column) {
                            nodes[index].push((grid.grid[right_index].value(), right_index));
                        } else {
                            nodes[size - 1].push((square.value(), index));
                        }
                    } else {
                        nodes[size - 1].push((square.value(), index));
                    }
                }
                _ => {}
            }

            // goal
            if let Square::Goal = *square {
                goals.push(index);
            }
        }

        Self {
            nodes: nodes.into_iter().map(|v| v.into_boxed_slice()).collect(),
            goals,
        }
    }
}

impl Graph {
    fn find_path(&self) -> Option<(u64, u64, Vec<usize>)> {
        use std::cmp::Reverse;

        let mut queue = std::collections::BinaryHeap::new();

        let mut min_costs = vec![(u64::max_value(), u64::max_value()); self.nodes.len()];
        let mut prev_node = vec![None; self.nodes.len()];

        // start on the outside of the grid
        queue.push(Reverse((0, 0, self.nodes.len() - 1)));

        while let Some(Reverse((cost, length, position))) = queue.pop() {
            if (cost, length) > min_costs[position] {
                continue;
            }

            if self.goals.contains(&position) {
                let mut path = Vec::new();
                path.push(position);

                let mut curr = position;

                while let Some(prev) = prev_node[curr] {
                    path.push(prev);
                    curr = prev;
                }

                path.reverse();

                return Some((cost, length, path));
            }

            for &(edge_cost, next_position) in self.nodes[position].iter() {
                let next_cost = cost + edge_cost;
                if (next_cost, length + 1) >= min_costs[next_position] {
                    continue;
                }

                min_costs[next_position] = (next_cost, length + 1);
                prev_node[next_position] = Some(position);
                queue.push(Reverse((next_cost, length + 1, next_position)));
            }
        }

        None
    }
}

macro_rules! read_types_to_eol {
    ($($t: ty),*) => {
        {
            use std::io::Read;
            let mut string = String::new();
            std::io::stdin().read_to_string(&mut string).expect("Error while reading stdin");
            string.lines().map(|line|
                {
                    let mut values = line.split_whitespace();
                    (
                        $(
                            values.next().expect("Not enough input").parse::<$t>().expect("Unable to parse")
                        ),*
                    )
                }
            ).collect::<Vec<_>>()
        };
    };
}

fn main() {
    let rows = read_types_to_eol!(String);

    let grid = Grid::new(&rows);

    let graph = Graph::from(grid);

    if let Some((cost, length, path)) = graph.find_path() {
        println!("cost: {}", cost);
        println!("length: {}", length);
        println!("path: {:?}", path);
    } else {
        println!("No path found");
    }
}
