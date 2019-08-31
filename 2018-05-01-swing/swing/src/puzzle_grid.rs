use std::cmp::Ordering;
use std::ops::Sub;

/// An integer point on the grid.
/// These points should be no larger than about 180 for everything to work right
//#[derive(PartialEq, Eq, Hash)]
struct GridPoint {
    x: u16,
    y: u16,
}

impl Sub<GridPoint> for GridPoint {
    type Output = GridVector;

    fn sub(self, other: GridPoint) -> GridVector {
        GridVector {
            x: (self.x.wrapping_sub(other.x)) as i16,
            y: (self.y.wrapping_sub(other.y)) as i16,
        }
    }
}

/// A direction between integer points on the grid.
#[derive(Debug)]
struct GridVector {
    x: i16,
    y: i16,
}

impl GridVector {
    /// Find the squared length of the vector
    #[inline]
    fn sqlen(&self) -> u16 {
        (self.x.pow(2) + self.y.pow(2)) as u16
    }

    /// The (scalar) cross product of two vectors
    #[inline]
    fn cross_product(&self, other: &GridVector) -> i16 {
        self.x * other.y - other.x * self.y
    }

    /// The dot product of two vectors
    #[inline]
    fn dot_product(&self, other: &GridVector) -> i16 {
        self.x * other.x + self.y * other.y
    }

    /// Using this vector as a base, compare two vectors.
    /// For example, with (1, 0) as the base, (-1, 0) is greater than (0, 1),
    /// Since going from (1, 0) to (-1, 0) counterclockwise passes through (0, 1).
    /// Vectors that are pointing the same direction are considered equal, including 0 vectors.
    fn cmp(&self, other1: &GridVector, other2: &GridVector) -> Ordering {
        // We'll divide our work into cases.
        // Imagine that self is along the positive x axis.
        // Our basic four cases are based on whether the other vectors
        // are in the upper or lower half of the plane.

        match (self.cross_product(other1).cmp(&0),
               self.dot_product(other1).cmp(&0),
               self.cross_product(other2).cmp(&0),
               self.dot_product(other2).cmp(&0),
               other1.cross_product(other2).cmp(&0),) {

            // First equal to zero
            (Ordering::Equal, Ordering::Equal, _, _, _) => Ordering::Equal,

            // First on positive x-axis
            (Ordering::Equal, Ordering::Greater, Ordering::Equal, Ordering::Greater,  _) => Ordering::Equal,

            (Ordering::Equal, Ordering::Greater, _, _,  _) => Ordering::Less,

            // First in upper half of the plane
            (Ordering::Greater, _, Ordering::Equal, Ordering::Equal, _) => Ordering::Equal,

            (Ordering::Greater, _, Ordering::Equal, Ordering::Greater, _) => Ordering::Greater,

            (Ordering::Greater, _, Ordering::Greater, _, Ordering::Less) => Ordering::Greater,
            (Ordering::Greater, _, Ordering::Greater, _, Ordering::Equal) => Ordering::Equal,
            (Ordering::Greater, _, Ordering::Greater, _, Ordering::Greater) => Ordering::Less,

            (Ordering::Greater, _, _, _, _) => Ordering::Less,

            // First on negative x-axis
            (Ordering::Equal, Ordering::Less, Ordering::Equal, Ordering::Equal, _) => Ordering::Equal,

            (Ordering::Equal, Ordering::Less, Ordering::Equal, Ordering::Greater, _) => Ordering::Greater,

            (Ordering::Equal, Ordering::Less, Ordering::Greater, _, _) => Ordering::Greater,

            (Ordering::Equal, Ordering::Less, Ordering::Equal, Ordering::Less, _) => Ordering::Equal,
            (Ordering::Equal, Ordering::Less, _, _, _) => Ordering::Less,


            // First in lower half of the plane
            (Ordering::Less, _, Ordering::Equal, Ordering::Equal, _) => Ordering::Equal,

            (Ordering::Less, _, Ordering::Equal, _, _) => Ordering::Greater,

            (Ordering::Less, _, Ordering::Greater, _, _) => Ordering::Greater,

            (Ordering::Less, _, Ordering::Less, _, Ordering::Less) => Ordering::Greater,
            (Ordering::Less, _, Ordering::Less, _, Ordering::Equal) => Ordering::Equal,
            (Ordering::Less, _, Ordering::Less, _, Ordering::Greater) => Ordering::Less,
        }
    }
}

#[cfg(test)]
macro_rules! TestOrderingBasic {
    ($v1:expr, $v2:expr, $v3:expr) => {
        println!("{:?}, {:?}, {:?}", $v1, $v2, $v3);
        assert_eq!($v1.cmp(&$v2, &$v3), Ordering::Less, "{:?} < {:?} :> {:?}", $v2, $v3, $v1);
        assert_eq!($v1.cmp(&$v3, &$v2), Ordering::Greater, "{:?} > {:?} :> {:?}", $v3, $v2, $v1);
    };
}

#[cfg(test)]
macro_rules! TestOrdering {
    ($v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr, $v6:expr, $v7:expr, $v8:expr) => {

        TestOrderingBasic! { $v1, $v1, $v2 }
        TestOrderingBasic! { $v1, $v1, $v3 }
        TestOrderingBasic! { $v1, $v1, $v4 }
        TestOrderingBasic! { $v1, $v1, $v5 }
        TestOrderingBasic! { $v1, $v1, $v6 }
        TestOrderingBasic! { $v1, $v1, $v7 }
        TestOrderingBasic! { $v1, $v1, $v8 }
        TestOrderingBasic! { $v1, $v2, $v3 }
        TestOrderingBasic! { $v1, $v2, $v4 }
        TestOrderingBasic! { $v1, $v2, $v5 }
        TestOrderingBasic! { $v1, $v2, $v6 }
        TestOrderingBasic! { $v1, $v2, $v7 }
        TestOrderingBasic! { $v1, $v2, $v8 }
        TestOrderingBasic! { $v1, $v3, $v4 }
        TestOrderingBasic! { $v1, $v3, $v5 }
        TestOrderingBasic! { $v1, $v3, $v6 }
        TestOrderingBasic! { $v1, $v3, $v7 }
        TestOrderingBasic! { $v1, $v3, $v8 }
        TestOrderingBasic! { $v1, $v4, $v5 }
        TestOrderingBasic! { $v1, $v4, $v6 }
        TestOrderingBasic! { $v1, $v4, $v7 }
        TestOrderingBasic! { $v1, $v4, $v8 }
        TestOrderingBasic! { $v1, $v5, $v6 }
        TestOrderingBasic! { $v1, $v5, $v7 }
        TestOrderingBasic! { $v1, $v5, $v8 }
        TestOrderingBasic! { $v1, $v6, $v7 }
        TestOrderingBasic! { $v1, $v6, $v8 }
        TestOrderingBasic! { $v1, $v7, $v8 }

        assert_eq!($v1.cmp(&$v1, &$v1), Ordering::Equal, "{:?} == {:?} :> {:?}", $v1, $v1, $v1);
        assert_eq!($v1.cmp(&$v2, &$v2), Ordering::Equal, "{:?} == {:?} :> {:?}", $v2, $v2, $v2);
        assert_eq!($v1.cmp(&$v3, &$v3), Ordering::Equal, "{:?} == {:?} :> {:?}", $v3, $v3, $v3);
        assert_eq!($v1.cmp(&$v4, &$v4), Ordering::Equal, "{:?} == {:?} :> {:?}", $v1, $v4, $v4);
        assert_eq!($v1.cmp(&$v5, &$v5), Ordering::Equal, "{:?} == {:?} :> {:?}", $v1, $v5, $v5);
        assert_eq!($v1.cmp(&$v7, &$v6), Ordering::Equal, "{:?} == {:?} :> {:?}", $v1, $v6, $v6);
        assert_eq!($v1.cmp(&$v7, &$v7), Ordering::Equal, "{:?} == {:?} :> {:?}", $v1, $v7, $v7);
        assert_eq!($v1.cmp(&$v8, &$v8), Ordering::Equal, "{:?} == {:?} :> {:?}", $v1, $v8, $v8);

    };
}

#[test]
fn vector_cmp_test() {
    let v1 = GridVector { x: 1, y: 0};
    let v2 = GridVector { x: 1, y: 1};
    let v3 = GridVector { x: 0, y: 1};
    let v4 = GridVector { x: -1, y: 1};
    let v5 = GridVector { x: -1, y: 0};
    let v6 = GridVector { x: -1, y: -1};
    let v7 = GridVector { x: 0, y: -1};
    let v8 = GridVector { x: 1, y: -1};

    TestOrdering! {v1, v2, v3, v4, v5, v6, v7, v8}
    TestOrdering! {v2, v3, v4, v5, v6, v7, v8, v1}
    TestOrdering! {v3, v4, v5, v6, v7, v8, v1, v2}
    TestOrdering! {v4, v5, v6, v7, v8, v1, v2, v3}
    TestOrdering! {v5, v6, v7, v8, v1, v2, v3, v4}
    TestOrdering! {v6, v7, v8, v1, v2, v3, v4, v5}
    TestOrdering! {v7, v8, v1, v2, v3, v4, v5, v6}
    TestOrdering! {v8, v1, v2, v3, v4, v5, v6, v7}
}