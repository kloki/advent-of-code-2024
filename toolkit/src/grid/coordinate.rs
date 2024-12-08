use std::{
    fmt,
    ops::{Add, Sub},
};

pub enum CoordinateError {
    Negative,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub row: usize,
    pub column: usize,
}

impl Coordinate {
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    pub fn in_bounds(&self, max_row: usize, max_column: usize) -> bool {
        self.row <= max_row && self.column <= max_column
    }
}

impl From<(usize, usize)> for Coordinate {
    fn from(tuple: (usize, usize)) -> Self {
        Self::new(tuple.0, tuple.1)
    }
}
impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.row, self.column)
    }
}

impl Sub for Coordinate {
    type Output = Distance;

    fn sub(self, other: Coordinate) -> Distance {
        Distance {
            column: self.column as isize - other.column as isize,
            row: self.row as isize - other.row as isize,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Distance {
    pub row: isize,
    pub column: isize,
}

impl Add<Distance> for Coordinate {
    type Output = Result<Coordinate, CoordinateError>;

    fn add(self, distance: Distance) -> Result<Coordinate, CoordinateError> {
        let new_row = self.row as isize + distance.row;
        let new_column = self.column as isize + distance.column;

        if new_row < 0 || new_column < 0 {
            return Err(CoordinateError::Negative);
        }

        Ok(Coordinate {
            row: new_row as usize,
            column: new_column as usize,
        })
    }
}

impl Sub<Distance> for Coordinate {
    type Output = Result<Coordinate, CoordinateError>;

    fn sub(self, distance: Distance) -> Result<Coordinate, CoordinateError> {
        let new_row = self.row as isize - distance.row;
        let new_column = self.column as isize - distance.column;

        if new_row < 0 || new_column < 0 {
            return Err(CoordinateError::Negative);
        }

        Ok(Coordinate {
            row: new_row as usize,
            column: new_column as usize,
        })
    }
}
