use std::ops::Sub;

/// An integer point on the grid.
//#[derive(PartialEq, Eq, Hash)]
struct GridPoint {
    x: u8,
    y: u8,
}

impl Sub<GridPoint> for GridPoint {
    type Output = GridVector;

    fn sub(self, other: GridPoint) -> GridVector {
        GridVector {
            x_diff: (self.x.wrapping_sub(other.x)) as i8,
            y_diff: (self.y.wrapping_sub(other.y)) as i8,
        }
    }
}

/// A direction between integer points on the grid.
//#[derive(PartialEq, Eq, Hash)]
struct GridVector {
    x_diff: i8,
    y_diff: i8,
}

impl GridVector {
    fn sqlen(&self) -> u8 {
        (self.x_diff.pow(2) + self.y_diff.pow(2)) as u8
    }
}
