use std::fmt;
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub row: usize,
    pub column: usize,
}

impl Coordinate {
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
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
