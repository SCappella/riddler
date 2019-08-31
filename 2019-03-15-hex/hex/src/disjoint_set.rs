#[derive(Default, Clone, Debug)]
pub struct DisjointSet {
    parents: Vec<usize>,
    ranks: Vec<usize>,
}

impl DisjointSet {
    /// Test if the disjoint set is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.parents.is_empty()
    }

    /// Find the length of the disjoint set
    #[inline]
    pub fn len(&self) -> usize {
        self.parents.len()
    }

    /// Add a single element to the DisjointSet. Returns the index of the new element.
    #[inline]
    pub fn add_elem(&mut self) -> usize {
        let new_elem = self.len();
        self.parents.push(new_elem);
        self.ranks.push(0);
        new_elem
    }

    /// Find the root of the given element. Will panic if the given element is not in the disjoint set.
    #[inline]
    fn find_root(&mut self, mut x: usize) -> usize {
        assert!(
            x < self.len(),
            "The element `{}` does not exist in this disjoint set",
            x
        );

        while self.parents[x] != x {
            let next = self.parents[x];
            self.parents[x] = self.parents[next];
            x = next;
        }

        x
    }

    /// Add a connection between nodes `x` and `y`. Will panic if these nodes are not in the disjoint set.
    #[inline]
    pub fn union(&mut self, x: usize, y: usize) {
        let mut x_root = self.find_root(x);
        let mut y_root = self.find_root(y);

        // The elements are in the same connected commponent already
        if x_root == y_root {
            return;
        }

        if self.ranks[x_root] < self.ranks[y_root] {
            std::mem::swap(&mut x_root, &mut y_root);
        }

        self.parents[y_root] = x_root;
        if self.ranks[x_root] == self.ranks[y_root] {
            self.ranks[x_root] += 1;
        }
    }

    /// Test if `x` and `y` are in the same connected component. Will panic if `x` and `y` are not in the disjoint set.
    #[inline]
    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find_root(x) == self.find_root(y)
    }
}
